use crate::color::{
    A98Rgb, DisplayP3, Hsl, Hsv, Hwb, Lab, Lch, Lms, LmsPrime, Oklab, Oklch, ProPhotoRgb, Rec2020,
    Rgb, Srgb, Xyz, XyzD50, XyzD65,
};
use crate::color::{ColorChannel, channel::color_channel};

use crate::color::{Alpha, Clamp, ColorError, ColorResult, ColorSpace, LinearSrgb, floats_eq};

/// A linear-light sRGB color with a separate, unpremultiplied alpha channel.
///
/// RGB channels are nominally `0.0..=1.0`; [`Color::new`] accepts any finite
/// RGB value to preserve colors outside the sRGB gamut. Alpha must be finite
/// and in `0.0..=1.0`, from fully transparent to fully opaque.
#[derive(Debug, Clone, Copy)]
pub struct Color {
    /// Red channel, finite and nominally `0.0..=1.0`.
    r: ColorChannel<f32>,
    /// Green channel, finite and nominally `0.0..=1.0`.
    g: ColorChannel<f32>,
    /// Blue channel, finite and nominally `0.0..=1.0`.
    b: ColorChannel<f32>,
    /// Alpha channel, finite and in `0.0..=1.0`.
    a: ColorChannel<f32>,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        floats_eq(&self.r, &other.r)
            && floats_eq(&self.g, &other.g)
            && floats_eq(&self.b, &other.b)
            && floats_eq(&self.a, &other.a)
    }
}

impl Eq for Color {}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> ColorResult<Self> {
        if !r.is_finite() {
            return Err(ColorError::InvalidColorChannel("r", r));
        }
        if !g.is_finite() {
            return Err(ColorError::InvalidColorChannel("g", g));
        }
        if !b.is_finite() {
            return Err(ColorError::InvalidColorChannel("b", b));
        }

        if !a.is_finite() || !(0.0..=1.0).contains(&a) {
            return Err(ColorError::InvalidAlpha(a));
        }

        Ok(Self {
            r: color_channel("r", r, 0.0..=1.0),
            g: color_channel("g", g, 0.0..=1.0),
            b: color_channel("b", b, 0.0..=1.0),
            a: color_channel("alpha", a, 0.0..=1.0),
        })
    }
    /// Borrows the r channel and its bounds.
    pub fn r_channel(&self) -> &ColorChannel<f32> {
        &self.r
    }

    pub fn r(&self) -> f32 {
        *self.r
    }

    /// Borrows the g channel and its bounds.
    pub fn g_channel(&self) -> &ColorChannel<f32> {
        &self.g
    }

    pub fn g(&self) -> f32 {
        *self.g
    }

    /// Borrows the b channel and its bounds.
    pub fn b_channel(&self) -> &ColorChannel<f32> {
        &self.b
    }

    pub fn b(&self) -> f32 {
        *self.b
    }

    /// Borrows the a channel and its bounds.
    pub fn a_channel(&self) -> &ColorChannel<f32> {
        &self.a
    }

    pub fn a(&self) -> f32 {
        *self.a
    }
    pub fn is_in_srgb_gamut(&self) -> bool {
        self.r.in_bounds() && self.g.in_bounds() && self.b.in_bounds()
    }
    pub fn clamp_to_srgb_gamut(mut self) -> Self {
        self.r = self.r.clamp();
        self.g = self.g.clamp();
        self.b = self.b.clamp();
        self
    }
    pub fn linear_srgb(&self) -> LinearSrgb {
        LinearSrgb::new(self.r(), self.g(), self.b()).expect("Color stores finite RGB values")
    }
    pub fn linear_srgba(&self) -> Alpha<LinearSrgb> {
        Alpha {
            color: self.linear_srgb(),
            alpha: self.a,
        }
    }
    pub fn to<T>(&self) -> ColorResult<T>
    where
        T: ColorSpace,
        <T as TryFrom<LinearSrgb>>::Error: Into<ColorError>,
        LinearSrgb: TryFrom<T>,
    {
        self.to_colorspace::<T>()
    }

    /// Converts to a color space without clamping, ignoring alpha.
    ///
    /// # Errors
    /// Returns the target color space's conversion error for unrepresentable values.
    ///
    /// ```
    /// use ferriswatch::color::{Color, Hsl};
    /// let red = Color::new(1.0, 0.0, 0.0, 1.0)?;
    /// let hsl = red.to_colorspace::<Hsl>()?;
    /// assert_eq!((*hsl.h, *hsl.s, *hsl.l), (0.0, 1.0, 0.5));
    /// # Ok::<(), ferriswatch::color::ColorError>(())
    /// ```
    pub fn to_colorspace<T>(&self) -> ColorResult<T>
    where
        T: ColorSpace,
        LinearSrgb: TryFrom<T>,
    {
        T::try_from_color(*self)
    }

    /// Converts to a color space and applies its channel bounds, ignoring alpha.
    /// Byte RGB clamps before quantization. Hue channels wrap within their range.
    ///
    /// # Errors
    /// Returns an error if conversion produces non-finite or unrepresentable coordinates.
    ///
    /// ```
    /// use ferriswatch::color::{Color, Rgb};
    /// let extended = Color::new(2.0, -0.5, 0.0, 1.0)?;
    /// assert_eq!(extended.to_clamped::<Rgb>()?, Rgb::new(255, 0, 0));
    /// # Ok::<(), ferriswatch::color::ColorError>(())
    /// ```
    pub fn to_clamped<T>(&self) -> ColorResult<T>
    where
        T: ColorSpace,
        LinearSrgb: TryFrom<T>,
    {
        T::try_from_linear_srgb_clamped(self.linear_srgb())
    }

    /// Converts from a color space and clamps the stored linear-sRGB channels.
    /// Alpha is set to one. Source components are not clamped before conversion.
    ///
    /// # Errors
    /// Returns the source conversion error for non-finite or overflowing coordinates.
    pub fn clamped_from<T>(color: T) -> ColorResult<Self>
    where
        T: ColorSpace,
        LinearSrgb: TryFrom<T>,
    {
        let linear = color.try_into_linear_srgb_raw()?;
        Ok(Self::from(linear.clamp()))
    }

    /// Converts to [`Rgb`] without clamping, ignoring alpha.
    ///
    /// # Errors
    /// Returns an error if the color is outside the sRGB gamut or conversion overflows.
    pub fn rgb(&self) -> ColorResult<Rgb> {
        self.to::<Rgb>()
    }

    /// Converts to [`Srgb`] without clamping, ignoring alpha.
    /// All finite linear-sRGB inputs are representable in this space.
    pub fn srgb(&self) -> Srgb {
        Srgb::from(self.linear_srgb())
    }

    /// Converts to [`A98Rgb`] without clamping, ignoring alpha.
    /// All finite linear-sRGB inputs are representable in this space.
    pub fn a98_rgb(&self) -> A98Rgb {
        A98Rgb::from(self.linear_srgb())
    }

    /// Converts to [`DisplayP3`] without clamping, ignoring alpha.
    /// All finite linear-sRGB inputs are representable in this space.
    pub fn display_p3(&self) -> DisplayP3 {
        DisplayP3::from(self.linear_srgb())
    }

    /// Converts to [`ProPhotoRgb`] without clamping, ignoring alpha.
    /// All finite linear-sRGB inputs are representable in this space.
    pub fn prophoto_rgb(&self) -> ProPhotoRgb {
        ProPhotoRgb::from(self.linear_srgb())
    }

    /// Converts to [`Rec2020`] without clamping, ignoring alpha.
    /// All finite linear-sRGB inputs are representable in this space.
    pub fn rec2020(&self) -> Rec2020 {
        Rec2020::from(self.linear_srgb())
    }

    /// Converts to [`Hsl`] without clamping, ignoring alpha.
    ///
    /// # Errors
    /// Returns an error if the converted coordinates are non-finite or overflow.
    ///
    /// ```
    /// use ferriswatch::color::Color;
    /// let red = Color::new(1.0, 0.0, 0.0, 0.5)?;
    /// let hsl = red.hsl()?;
    /// assert_eq!((*hsl.h, *hsl.s, *hsl.l), (0.0, 1.0, 0.5));
    /// # Ok::<(), ferriswatch::color::ColorError>(())
    /// ```
    pub fn hsl(&self) -> ColorResult<Hsl> {
        self.to::<Hsl>()
    }

    /// Converts to [`Hsv`] without clamping, ignoring alpha.
    ///
    /// # Errors
    /// Returns an error if the converted coordinates are non-finite or overflow.
    pub fn hsv(&self) -> ColorResult<Hsv> {
        self.to::<Hsv>()
    }

    /// Converts to [`Hwb`] without clamping, ignoring alpha.
    /// All finite linear-sRGB inputs are representable in this space.
    pub fn hwb(&self) -> Hwb {
        Hwb::from(self.linear_srgb())
    }

    /// Converts to [`Lab`] without clamping, ignoring alpha.
    ///
    /// # Errors
    /// Returns an error if the converted coordinates are non-finite or overflow.
    pub fn lab(&self) -> ColorResult<Lab> {
        self.to::<Lab>()
    }

    /// Converts to [`Lch`] without clamping, ignoring alpha.
    ///
    /// # Errors
    /// Returns an error if the converted coordinates are non-finite or overflow.
    pub fn lch(&self) -> ColorResult<Lch> {
        self.to::<Lch>()
    }

    /// Converts to [`Oklab`] without clamping, ignoring alpha.
    /// All finite linear-sRGB inputs are representable in this space.
    pub fn oklab(&self) -> Oklab {
        Oklab::from(self.linear_srgb())
    }

    /// Converts to [`Oklch`] without clamping, ignoring alpha.
    /// All finite linear-sRGB inputs are representable in this space.
    pub fn oklch(&self) -> Oklch {
        Oklch::from(self.linear_srgb())
    }

    /// Converts to [`Lms`] without clamping, ignoring alpha.
    /// All finite linear-sRGB inputs are representable in this space.
    pub fn lms(&self) -> Lms {
        Lms::from(self.linear_srgb())
    }

    /// Converts to [`LmsPrime`] without clamping, ignoring alpha.
    /// All finite linear-sRGB inputs are representable in this space.
    pub fn lms_prime(&self) -> LmsPrime {
        LmsPrime::from(self.linear_srgb())
    }

    /// Converts to [`Xyz`] without clamping, ignoring alpha.
    ///
    /// # Errors
    /// Returns an error if the converted coordinates are non-finite or overflow.
    pub fn xyz(&self) -> ColorResult<Xyz> {
        self.to::<Xyz>()
    }

    /// Converts to [`XyzD50`] without clamping, ignoring alpha.
    /// All finite linear-sRGB inputs are representable in this space.
    pub fn xyz_d50(&self) -> XyzD50 {
        XyzD50::from(self.linear_srgb())
    }

    /// Converts to [`XyzD65`] without clamping, ignoring alpha.
    ///
    /// # Errors
    /// Returns an error if the converted coordinates are non-finite or overflow.
    pub fn xyz_d65(&self) -> ColorResult<XyzD65> {
        self.to::<XyzD65>()
    }

    pub fn set_a(&mut self, alpha: f32) -> ColorResult<&mut Self> {
        if !alpha.is_finite() || !(0.0..=1.0).contains(&alpha) {
            return Err(ColorError::InvalidAlpha(alpha));
        }
        *self.a = alpha;
        Ok(self)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.linear_srgba(), f)
    }
}

impl From<LinearSrgb> for Color {
    /// Stores linear-sRGB channels with alpha set to one.
    fn from(color: LinearSrgb) -> Self {
        Self {
            r: *color.r_channel(),
            g: *color.g_channel(),
            b: *color.b_channel(),
            a: color_channel("alpha", 1.0, 0.0..=1.0),
        }
    }
}

macro_rules! impl_try_from_colorspace {
    ($($space:ty),+ $(,)?) => {$(
        impl TryFrom<$space> for Color {
            type Error = ColorError;

            /// Converts without clamping and sets alpha to one.
            fn try_from(color: $space) -> ColorResult<Self> {
                color.try_into_color()
            }
        }
    )+};
}

impl_try_from_colorspace!(
    Rgb,
    Srgb,
    A98Rgb,
    DisplayP3,
    ProPhotoRgb,
    Rec2020,
    Hsl,
    Hsv,
    Hwb,
    Lab,
    Lch,
    Oklab,
    Oklch,
    Lms,
    LmsPrime,
    Xyz,
    XyzD50,
    XyzD65,
);

impl<T> crate::color::ClampedFrom<T> for Color
where
    T: ColorSpace,
    LinearSrgb: TryFrom<T>,
{
    fn clamped_from(color: T) -> ColorResult<Self> {
        Self::clamped_from(color)
    }
}
