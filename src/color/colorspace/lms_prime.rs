use crate::color::{ColorChannel, channel::color_channel};

/// Nonlinear LMS intermediate coordinates.
///
/// Conversions use the Oklab cone-response matrices from CSS Color 4;
/// there is no universal `0.0..=1.0` bound.
/// In the Oklab conversion, these are the signed cube roots of linear LMS
/// coordinates. Conversions use signed cube roots, including for negative responses.
/// Use `new` to configure channel names and bounds. Values are not automatically clamped.
/// Use finite channel values.
///
/// `Display` writes descriptive `lms-prime(l m s)` text, not CSS.
/// There is no standard CSS function for this intermediate space.
/// Precision defaults to three decimal places and can be set with `{:.N}`.
/// Non-finite channels are written as `none`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LmsPrime {
    /// Long-wavelength component; finite, with no fixed bounds.
    pub l: ColorChannel<f32>,
    /// Medium-wavelength component; finite, with no fixed bounds.
    pub m: ColorChannel<f32>,
    /// Short-wavelength component; finite, with no fixed bounds.
    pub s: ColorChannel<f32>,
}

impl LmsPrime {
    /// Creates channels with this color space's bounds, without validating or clamping values.
    pub fn new(l: f32, m: f32, s: f32) -> Self {
        Self {
            l: color_channel("l", l, ..),
            m: color_channel("m", m, ..),
            s: color_channel("s", s, ..),
        }
    }
}

crate::color::formatting::impl_display!(LmsPrime, "lms-prime(", |color| [
    *color.l => "", *color.m => "", *color.s => ""
]);

crate::color::conversions::impl_colorspace!(
    infallible LmsPrime,
    [l, m, s],
    crate::color::perceptual_conversion::lms_prime_to_linear,
    crate::color::perceptual_conversion::linear_to_lms_prime
);
