use crate::color::Channel;
use crate::color::{
    A98Rgb, DisplayP3, Hsl, Hsv, Hwb, Lab, Lch, Lms, LmsPrime, Oklab, Oklch, ProPhotoRgb, Rec2020,
    Rgb, Rgba, Srgb, Xyz, XyzD50, XyzD65,
};

use crate::color::{Alpha, Clamp, ColorError, ColorResult, ColorSpace, LinearSrgb, floats_eq};

/// A linear-light sRGB color with a separate, unpremultiplied alpha channel.
///
/// RGB channels are nominally `0.0..=1.0`; [`Color::new`] accepts any finite
/// RGB value to preserve colors outside the sRGB gamut. Alpha must be finite
/// and in `0.0..=1.0`, from fully transparent to fully opaque.
///
/// Infallible `From`/`Into` conversions to and from color spaces clamp the
/// destination channels to their declared ranges (wrapping hue). Byte RGB
/// additionally quantizes to the nearest byte. Unbounded channels are unchanged.
/// Alpha wrappers preserve alpha; bare spaces discard it or supply opaque alpha.
/// Use the explicit raw conversion methods to preserve extended channel values.
/// Directions that can reject non-finite or overflowing values retain `TryFrom`.
#[derive(Debug, Clone, Copy)]

pub struct Color {
    /// Red channel, finite and nominally `0.0..=1.0`.
    r: Channel<f32>,
    /// Green channel, finite and nominally `0.0..=1.0`.
    g: Channel<f32>,
    /// Blue channel, finite and nominally `0.0..=1.0`.
    b: Channel<f32>,
    /// Alpha channel, finite and in `0.0..=1.0`.
    a: Channel<f32>,
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
    /// Fully transparent black: all RGB channels and alpha are zero.
    ///
    /// Can be used in constant definitions without construction or validation.
    ///
    /// ```
    /// use ferriswatch::color::Color;
    /// const BACKGROUND: Color = Color::TRANSPARENT;
    /// assert_eq!(BACKGROUND.a(), 0.0);
    /// ```

    pub const TRANSPARENT: Self = Self {
        r: Channel::unit_color_channel("r", 0.0),
        g: Channel::unit_color_channel("g", 0.0),
        b: Channel::unit_color_channel("b", 0.0),
        a: Channel::unit_color_channel("alpha", 0.0),
    };

    /// Converts packed RGB or RGBA hex into linear sRGB with alpha.
    ///
    /// Values up to `0xFFFFFF` use [`Rgb::from_hex`] and opaque alpha; larger
    /// values use [`Rgba::from_hex`] with alpha in the low byte. Integers do not
    /// retain leading zeros. Use [`Self::from_hex_str`] or convert an explicit
    /// [`Rgba`] when a packed RGBA value fits in 24 bits.
    ///
    /// This conversion cannot fail. The result wrapper is retained for compatibility;
    /// use [`Self::hex`] for a direct opaque RGB value in constant definitions.

    pub const fn from_hex(value: u32) -> ColorResult<Self> {
        if value <= 0xFFFFFF {
            Ok(Self::hex(value))
        } else {
            Ok(Self::hex_alpha(value))
        }
    }

    /// Decodes packed `0xRRGGBB` hex into an opaque color, including in constants.
    ///
    /// RGB bytes are decoded to linear light and alpha is one. Integers do not
    /// retain digit counts: shorter literals are interpreted with leading zeros.
    /// Use [`Self::hex_alpha`] for packed RGBA or [`Self::from_rgba8`] for separate bytes.
    ///
    /// # Panics
    /// Panics if `value` exceeds `0xFFFFFF`.
    ///
    /// ```
    /// use ferriswatch::color::Color;
    /// const ROSEWATER: Color = Color::hex(0xf5e0dc);
    /// assert_eq!(ROSEWATER.a(), 1.0);
    /// ```

    pub const fn hex(value: u32) -> Self {
        assert!(value <= 0xFFFFFF, "RGB hex value must fit in 24 bits");

        Self::from_rgba8((value >> 16) as u8, (value >> 8) as u8, value as u8, 255)
    }

    /// Decodes packed `0xRRGGBBAA` hex into a color, including in constants.
    ///
    /// RGB bytes are decoded to linear light. The final byte is alpha, divided
    /// by 255: zero is transparent and 255 is opaque. Every `u32` is interpreted
    /// as eight hex digits with leading zeros; integers do not retain digit counts.
    ///
    /// ```
    /// use ferriswatch::color::Color;
    /// const FADED: Color = Color::hex_alpha(0xf5e0dc80);
    /// const CLEAR: Color = Color::hex_alpha(0x00000000);
    /// assert_eq!(FADED.a(), 128.0 / 255.0);
    /// assert_eq!(CLEAR, Color::TRANSPARENT);
    /// ```

    pub const fn hex_alpha(value: u32) -> Self {
        Self::from_rgba8(
            (value >> 24) as u8,
            (value >> 16) as u8,
            (value >> 8) as u8,
            value as u8,
        )
    }

    /// Decodes encoded sRGB bytes and a linear alpha byte, including in constants.
    ///
    /// Each input is in `0..=255`. Alpha zero is transparent and 255 is opaque.

    pub const fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        use crate::color::srgb_byte_table::SRGB_BYTE_TO_LINEAR;

        Self {
            r: Channel::unit_color_channel("r", SRGB_BYTE_TO_LINEAR[r as usize]),
            g: Channel::unit_color_channel("g", SRGB_BYTE_TO_LINEAR[g as usize]),
            b: Channel::unit_color_channel("b", SRGB_BYTE_TO_LINEAR[b as usize]),
            a: Channel::unit_color_channel("alpha", a as f32 / 255.0),
        }
    }

    /// Parses RGB or RGBA hex and converts the encoded channels to linear light.
    ///
    /// Accepts 3 or 6 RGB digits and 4 or 8 RGBA digits, with optional `#`.
    /// Alpha is last in RGBA forms and defaults to one in RGB forms.
    ///
    /// # Errors
    /// Returns [`ColorError::InvalidLength`] for unsupported digit counts and
    /// [`ColorError::InvalidHex`] for invalid digits. Conversion errors propagate.
    ///
    /// ```
    /// use ferriswatch::color::Color;
    /// let color = Color::from_hex_str("#80402080")?;
    /// assert!((color.r() - 0.2158605).abs() < 0.000001);
    /// assert_eq!(color.a(), 128.0 / 255.0);
    /// # Ok::<(), ferriswatch::color::ColorError>(())
    /// ```

    pub fn from_hex_str(value: &str) -> ColorResult<Self> {
        match value.strip_prefix('#').unwrap_or(value).len() {
            3 | 6 => Ok(Self::from(Rgb::from_hex_str(value)?)),
            4 | 8 => Ok(Self::from(Rgba::from_hex_str(value)?)),
            length => Err(ColorError::InvalidLength(length)),
        }
    }

    /// Shorthand for [`Self::from_hex_str`], with the same parsing errors.

    pub fn hex_str(value: &str) -> ColorResult<Self> {
        Self::from_hex_str(value)
    }

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
            r: Channel::color_channel("r", r, 0.0..=1.0),
            g: Channel::color_channel("g", g, 0.0..=1.0),
            b: Channel::color_channel("b", b, 0.0..=1.0),
            a: Channel::color_channel("alpha", a, 0.0..=1.0),
        })
    }

    /// Borrows the r channel and its bounds.

    pub fn r_channel(&self) -> &Channel<f32> {
        &self.r
    }

    pub fn r(&self) -> f32 {
        *self.r
    }

    /// Borrows the g channel and its bounds.

    pub fn g_channel(&self) -> &Channel<f32> {
        &self.g
    }

    pub fn g(&self) -> f32 {
        *self.g
    }

    /// Borrows the b channel and its bounds.

    pub fn b_channel(&self) -> &Channel<f32> {
        &self.b
    }

    pub fn b(&self) -> f32 {
        *self.b
    }

    /// Borrows the a channel and its bounds.

    pub fn a_channel(&self) -> &Channel<f32> {
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
    /// Clamps linear-sRGB channels to `0..=1` and sets alpha to one.

    fn from(color: LinearSrgb) -> Self {
        let color = color.clamp();

        Self {
            r: *color.r_channel(),
            g: *color.g_channel(),
            b: *color.b_channel(),
            a: Channel::color_channel("alpha", 1.0, 0.0..=1.0),
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

        impl TryFrom<Alpha<$space>> for Color {
            type Error = ColorError;

            /// Converts without clamping and preserves alpha; may reject invalid channels.
            fn try_from(value: Alpha<$space>) -> ColorResult<Self> {
                let linear = value.color.try_into_linear_srgb_raw()?;
                Self::new(linear.r(), linear.g(), linear.b(), value.alpha())
            }
        }
    )+};
}

impl_try_from_colorspace!(
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

impl From<Srgb> for Color {
    /// Decodes to linear light, clamps RGB to `0..=1`, and supplies opaque alpha.

    fn from(value: Srgb) -> Self {
        // Decode in f64 so every finite encoded f32 remains representable until clamping.
        let [r, g, b] = [value.r(), value.g(), value.b()].map(|v| {
            crate::color::rgb_conversion::decode_srgb(f64::from(v)).clamp(0.0, 1.0) as f32
        });

        Self {
            r: Channel::color_channel("r", r, 0.0..=1.0),
            g: Channel::color_channel("g", g, 0.0..=1.0),
            b: Channel::color_channel("b", b, 0.0..=1.0),
            a: Channel::color_channel("alpha", 1.0, 0.0..=1.0),
        }
    }
}

macro_rules! impl_from_alpha {
    ($($space:ty),+ $(,)?) => {$(
        impl From<Alpha<$space>> for Color {
            /// Converts and clamps RGB to `0..=1`, preserving alpha.
            fn from(value: Alpha<$space>) -> Self {
                let mut color = Self::from(value.color);
                color.a = value.alpha;
                color
            }
        }
    )+};
}

impl_from_alpha!(LinearSrgb, Srgb, Rgb);

impl From<Rgb> for Color {
    /// Decodes encoded byte channels to linear light and sets alpha to one.

    fn from(value: Rgb) -> Self {
        Self::from(LinearSrgb::from(value))
    }
}

macro_rules! impl_from_color {
    ($($space:ty),+ $(,)?) => {$(
        impl From<Color> for $space {
            /// Converts and clamps destination channels, wrapping hue and ignoring alpha.
            ///
            /// Floating-point transforms can round; this is not bitwise serialization.
            fn from(value: Color) -> Self {
                Self::from(value.linear_srgb()).clamp()
            }
        }
    )+};
}

impl_from_color!(
    LinearSrgb,
    Srgb,
    A98Rgb,
    DisplayP3,
    ProPhotoRgb,
    Rec2020,
    Hwb,
    Oklab,
    Oklch,
    Lms,
    LmsPrime,
    XyzD50,
);

impl<C: From<Color>> From<Color> for Alpha<C> {
    /// Converts and clamps destination channels, quantizing byte RGB, and preserves alpha.

    fn from(value: Color) -> Self {
        Self {
            color: C::from(value),
            alpha: value.a,
        }
    }
}

macro_rules! impl_try_from_color {
    ($($space:ty),+ $(,)?) => {$(
        impl TryFrom<Color> for $space {
            type Error = ColorError;

            /// Converts without clamping, ignoring alpha; rejects unrepresentable values.
            fn try_from(value: Color) -> ColorResult<Self> {
                Self::try_from(value.linear_srgb())
            }
        }
    )+};
}

impl_try_from_color!(Hsl, Hsv, Lab, Lch, Xyz, XyzD65);

impl From<Color> for Rgb {
    /// Encodes to sRGB, clamps to `0..=1`, and rounds to the nearest byte.
    ///
    /// `Into<Rgb>` has the same behavior. Alpha is discarded. Use [`Color::rgb`]
    /// to reject out-of-gamut colors instead of clamping them.

    fn from(value: Color) -> Self {
        let encoded = value.srgb();

        Self::new(
            encoded.r_u8_clamped(),
            encoded.g_u8_clamped(),
            encoded.b_u8_clamped(),
        )
    }
}
