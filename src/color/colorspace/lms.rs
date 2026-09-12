use crate::color::{ColorChannel, channel::color_channel};

/// LMS cone-response coordinates.
///
/// Conversions use the Oklab cone-response matrices from CSS Color 4;
/// there is no universal `0.0..=1.0` bound.
/// Physical linear responses are normally nonnegative; extended color
/// calculations may produce negative values.
/// Use `new` to configure channel names and bounds. Values are not automatically clamped.
/// Use finite channel values.
///
/// `Display` writes descriptive `lms(l m s)` text, not CSS.
/// There is no standard CSS function for this intermediate space.
/// Precision defaults to three decimal places and can be set with `{:.N}`.
/// Non-finite channels are written as `none`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lms {
    /// Long-wavelength component; finite, with no fixed bounds.
    pub l: ColorChannel,
    /// Medium-wavelength component; finite, with no fixed bounds.
    pub m: ColorChannel,
    /// Short-wavelength component; finite, with no fixed bounds.
    pub s: ColorChannel,
}

impl Lms {
    /// Creates channels with this color space's bounds, without validating or clamping values.
    pub fn new(l: f32, m: f32, s: f32) -> Self {
        Self {
            l: color_channel("l", l, ..),
            m: color_channel("m", m, ..),
            s: color_channel("s", s, ..),
        }
    }
}

crate::color::formatting::impl_display!(Lms, "lms(", |color| [
    *color.l => "", *color.m => "", *color.s => ""
]);

crate::color::conversions::impl_colorspace!(
    infallible Lms,
    [l, m, s],
    crate::color::perceptual_conversion::lms_to_linear,
    crate::color::perceptual_conversion::linear_to_lms
);
