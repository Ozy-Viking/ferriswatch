use super::ColorSpace;
use crate::color::{Clamp, ColorError, ColorResult, floats_eq};

/// An sRGB color with linear-light floating-point channels and no alpha.
///
/// RGB channels are nominally `0.0..=1.0`. The constructor accepts any finite
/// value, including negative values and values above one for colors outside
/// the sRGB gamut. Values are not clamped automatically.
#[derive(Debug, Clone, Copy)]
pub struct LinearSrgb {
    /// Red channel, nominally `0.0..=1.0`; must be finite.
    pub(in crate::color) r: f32,
    /// Green channel, nominally `0.0..=1.0`; must be finite.
    pub(in crate::color) g: f32,
    /// Blue channel, nominally `0.0..=1.0`; must be finite.
    pub(in crate::color) b: f32,
}

impl LinearSrgb {
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

    pub fn is_in_gamut(&self) -> bool {
        (0.0..=1.0).contains(&self.r)
            && (0.0..=1.0).contains(&self.g)
            && (0.0..=1.0).contains(&self.b)
    }
}

impl Clamp for LinearSrgb {
    fn clamp(mut self) -> Self {
        self.r = self.r.clamp(0.0, 1.0);
        self.g = self.g.clamp(0.0, 1.0);
        self.b = self.b.clamp(0.0, 1.0);
        self
    }
}

impl PartialEq for LinearSrgb {
    fn eq(&self, other: &Self) -> bool {
        floats_eq(&self.r, &other.r) && floats_eq(&self.g, &other.g) && floats_eq(&self.b, &other.b)
    }
}

impl Eq for LinearSrgb {}

impl ColorSpace for LinearSrgb {
    fn try_into_linear_srgb_raw(self) -> ColorResult<LinearSrgb> {
        Ok(self)
    }

    fn try_from_linear_srgb_raw(color: LinearSrgb) -> ColorResult<Self> {
        Ok(color)
    }
}
