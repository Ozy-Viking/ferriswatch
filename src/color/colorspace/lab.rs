use crate::color::{ColorChannel, channel::color_channel};

/// A CIELAB color expressed as lightness and two opponent axes.
///
/// Conversions use the D50 reference white, matching CSS Lab.
/// The opponent axes have no fixed mathematical bounds; the ranges below
/// are nominal reference ranges, not gamut limits.
/// Use `new` to configure channel names and bounds. Values are not automatically clamped.
/// Use finite channel values.
///
/// `Display` writes CSS `lab(...)` with three decimal places by default;
/// use `{:.N}` to choose precision. Non-finite channels are written as `none`.
/// CSS output interprets these coordinates relative to D50.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lab {
    /// Lightness, nominally `0.0..=100.0`, from black to reference white.
    pub l: ColorChannel<f32>,
    /// Green-negative/red-positive axis, nominally `-125.0..=125.0`.
    pub a: ColorChannel<f32>,
    /// Blue-negative/yellow-positive axis, nominally `-125.0..=125.0`.
    pub b: ColorChannel<f32>,
}

impl Lab {
    /// Creates channels with this color space's bounds, without validating or clamping values.
    pub fn new(l: f32, a: f32, b: f32) -> Self {
        Self {
            l: color_channel("l", l, 0.0..=100.0),
            a: color_channel("a", a, ..),
            b: color_channel("b", b, ..),
        }
    }
}

crate::color::formatting::impl_display!(
    Lab, "lab(", |color| [
        *color.l => "",
        *color.a => "",
        *color.b => ""
    ]
);

crate::color::conversions::impl_colorspace!(
    Lab,
    [l, a, b],
    crate::color::perceptual_conversion::lab_to_linear,
    crate::color::perceptual_conversion::linear_to_lab
);
