use crate::color::{ColorChannel, channel::color_channel};

use super::ColorSpace;
use crate::color::{Clamp, ColorError, ColorResult, floats_eq};

/// An sRGB color with linear-light floating-point channels and no alpha.
///
/// RGB channels are nominally `0.0..=1.0`. The constructor accepts any finite
/// value, including negative values and values above one for colors outside
/// the sRGB gamut. Values are not clamped automatically.
///
/// `Display` writes CSS `color(srgb-linear...)` with three decimal places by default;
/// use `{:.N}` to choose precision. Non-finite channels are written as `none`.
#[derive(Debug, Clone, Copy)]
pub struct LinearSrgb {
    /// Red channel, nominally `0.0..=1.0`; must be finite.
    pub(in crate::color) r: ColorChannel<f32>,
    /// Green channel, nominally `0.0..=1.0`; must be finite.
    pub(in crate::color) g: ColorChannel<f32>,
    /// Blue channel, nominally `0.0..=1.0`; must be finite.
    pub(in crate::color) b: ColorChannel<f32>,
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

        Ok(Self {
            r: color_channel("r", r, 0.0..=1.0),
            g: color_channel("g", g, 0.0..=1.0),
            b: color_channel("b", b, 0.0..=1.0),
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

    pub fn is_in_gamut(&self) -> bool {
        self.r.in_bounds() && self.g.in_bounds() && self.b.in_bounds()
    }
}

impl Clamp for LinearSrgb {
    fn clamp(mut self) -> Self {
        self.r = self.r.clamp();
        self.g = self.g.clamp();
        self.b = self.b.clamp();
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

crate::color::formatting::impl_display!(
    LinearSrgb, "color(srgb-linear ", |color| [
        *color.r => "",
        *color.g => "",
        *color.b => ""
    ]
);
