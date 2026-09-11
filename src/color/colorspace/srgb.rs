use super::{ColorSpace, LinearSrgb};
use crate::color::{Clamp, ColorError, ColorResult, floats_eq};

/// An sRGB color with encoded floating-point channels and no alpha.
///
/// RGB channels are nominally `0.0..=1.0`. The constructor accepts any finite
/// value, including negative values and values above one for colors outside
/// the sRGB gamut. Values are not clamped automatically.
#[derive(Debug, Clone, Copy)]
pub struct Srgb {
    /// Red channel, nominally `0.0..=1.0`; must be finite.
    pub(super) r: f32,
    /// Green channel, nominally `0.0..=1.0`; must be finite.
    pub(super) g: f32,
    /// Blue channel, nominally `0.0..=1.0`; must be finite.
    pub(super) b: f32,
}

impl Srgb {
    pub fn new(r: f32, g: f32, b: f32) -> ColorResult<Self> {
        if !r.is_finite() {
            return Err(ColorError::InvalidColorChannel("r", r));
        }
        if !g.is_finite() {
            return Err(ColorError::InvalidColorChannel("g", g));
        }
        if !b.is_finite() {
            return Err(ColorError::InvalidColorChannel("b", b));
        }

        Ok(Self { r, g, b })
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

    pub fn is_in_srgb_gamut(&self) -> bool {
        (0.0..=1.0).contains(&self.r)
            && (0.0..=1.0).contains(&self.g)
            && (0.0..=1.0).contains(&self.b)
    }

    pub fn r_u8_clamped(&self) -> u8 {
        (self.r.clamp(0.0, 1.0) * 255.0).round() as u8
    }

    pub fn g_u8_clamped(&self) -> u8 {
        (self.g.clamp(0.0, 1.0) * 255.0).round() as u8
    }

    pub fn b_u8_clamped(&self) -> u8 {
        (self.b.clamp(0.0, 1.0) * 255.0).round() as u8
    }
}

impl PartialEq for Srgb {
    fn eq(&self, other: &Self) -> bool {
        floats_eq(&self.r, &other.r) && floats_eq(&self.g, &other.g) && floats_eq(&self.b, &other.b)
    }
}

impl Eq for Srgb {}

impl TryFrom<LinearSrgb> for Srgb {
    type Error = ColorError;

    fn try_from(color: LinearSrgb) -> ColorResult<Self> {
        Self::new(
            linear_to_srgb(color.r()),
            linear_to_srgb(color.g()),
            linear_to_srgb(color.b()),
        )
    }
}

impl TryFrom<Srgb> for LinearSrgb {
    type Error = ColorError;

    fn try_from(color: Srgb) -> ColorResult<Self> {
        Self::new(
            srgb_to_linear(color.r()),
            srgb_to_linear(color.g()),
            srgb_to_linear(color.b()),
        )
    }
}

impl Clamp for Srgb {
    fn clamp(mut self) -> Self {
        self.r = self.r.clamp(0.0, 1.0);
        self.g = self.g.clamp(0.0, 1.0);
        self.b = self.b.clamp(0.0, 1.0);
        self
    }
}

impl ColorSpace for Srgb {
    fn try_into_linear_srgb_raw(self) -> ColorResult<LinearSrgb> {
        LinearSrgb::new(
            srgb_to_linear(self.r),
            srgb_to_linear(self.g),
            srgb_to_linear(self.b),
        )
    }

    fn try_from_linear_srgb_raw(color: LinearSrgb) -> ColorResult<Self> {
        Self::new(
            linear_to_srgb(color.r()),
            linear_to_srgb(color.g()),
            linear_to_srgb(color.b()),
        )
    }
}

impl From<Rgb> for Srgb {
    fn from(rgb: Rgb) -> Srgb {
        Srgb {
            r: rgb.r_f32(),
            g: rgb.g_f32(),
            b: rgb.b_f32(),
        }
    }
}

fn srgb_to_linear(value: f32) -> f32 {
    let sign = value.signum();
    let value = value.abs();

    if value <= 0.04045 {
        sign * value / 12.92
    } else {
        sign * ((value + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_to_srgb(value: f32) -> f32 {
    let sign = value.signum();
    let value = value.abs();

    if value <= 0.0031308 {
        sign * value * 12.92
    } else {
        sign * (1.055 * value.powf(1.0 / 2.4) - 0.055)
    }
}

/// An encoded sRGB color with three 8-bit channels and no alpha.
///
/// Each channel ranges from `0..=255`.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct Rgb(
    /// Red channel in `0..=255`.
    pub u8,
    /// Green channel in `0..=255`.
    pub u8,
    /// Blue channel in `0..=255`.
    pub u8,
);

impl Rgb {
    pub fn r(&self) -> u8 {
        self.0
    }

    pub fn g(&self) -> u8 {
        self.1
    }

    pub fn b(&self) -> u8 {
        self.2
    }

    pub fn r_f32(&self) -> f32 {
        self.0 as f32 / 255.0
    }

    pub fn g_f32(&self) -> f32 {
        self.1 as f32 / 255.0
    }

    pub fn b_f32(&self) -> f32 {
        self.2 as f32 / 255.0
    }
}
impl TryFrom<Srgb> for Rgb {
    type Error = ColorError;

    fn try_from(color: Srgb) -> ColorResult<Self> {
        if !(0.0..=1.0).contains(&color.r) {
            return Err(ColorError::OutOfSrgbGamut("r", color.r));
        }

        if !(0.0..=1.0).contains(&color.g) {
            return Err(ColorError::OutOfSrgbGamut("g", color.g));
        }

        if !(0.0..=1.0).contains(&color.b) {
            return Err(ColorError::OutOfSrgbGamut("b", color.b));
        }

        Ok(Self(
            (color.r * 255.0).round() as u8,
            (color.g * 255.0).round() as u8,
            (color.b * 255.0).round() as u8,
        ))
    }
}
