use crate::color::{Alpha, ColorError, ColorResult, ColorSpace, LinearSrgb, floats_eq};

/// A linear-light sRGB color with a separate, unpremultiplied alpha channel.
///
/// RGB channels are nominally `0.0..=1.0`; [`Color::new`] accepts any finite
/// RGB value to preserve colors outside the sRGB gamut. Alpha must be finite
/// and in `0.0..=1.0`, from fully transparent to fully opaque.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Color {
    /// Red channel, finite and nominally `0.0..=1.0`.
    r: f32,
    /// Green channel, finite and nominally `0.0..=1.0`.
    g: f32,
    /// Blue channel, finite and nominally `0.0..=1.0`.
    b: f32,
    /// Alpha channel, finite and in `0.0..=1.0`.
    a: f32,
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

        Ok(Self { r, g, b, a })
    }
    pub fn r(&self) -> f32 {
        self.r
    }

    pub fn g(&self) -> f32 {
        self.g
    }

    pub fn b(&self) -> f32 {
        self.b
    }

    pub fn a(&self) -> f32 {
        self.a
    }
    pub fn is_in_srgb_gamut(&self) -> bool {
        (0.0..=1.0).contains(&self.r)
            && (0.0..=1.0).contains(&self.g)
            && (0.0..=1.0).contains(&self.b)
    }
    pub fn clamp_to_srgb_gamut(mut self) -> Self {
        self.r = self.r.clamp(0.0, 1.0);
        self.g = self.g.clamp(0.0, 1.0);
        self.b = self.b.clamp(0.0, 1.0);
        self
    }
    pub fn linear_srgb(&self) -> LinearSrgb {
        LinearSrgb {
            r: self.r(),
            g: self.g(),
            b: self.b(),
        }
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
        T::try_from_color(*self)
    }

    pub fn set_a(&mut self, alpha: f32) -> ColorResult<&mut Self> {
        if !alpha.is_finite() || !(0.0..=1.0).contains(&alpha) {
            return Err(ColorError::InvalidAlpha(alpha));
        }
        self.a = alpha;
        Ok(self)
    }
}
