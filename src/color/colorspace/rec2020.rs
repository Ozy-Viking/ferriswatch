use crate::color::Channel;

/// A ITU-R BT.2020 color with encoded RGB channels and no alpha.
///
/// Conversions use the CSS Color 4 signed BT.1886 gamma-2.4 transfer curve.
/// Each channel is nominally `0.0..=1.0`. Extended values represent colors
/// outside this RGB gamut.
/// Use `new` to configure channel names and bounds. Values are not automatically clamped.
/// Use finite channel values.
///
/// `Display` writes CSS `color(rec2020...)` with three decimal places by default;
/// use `{:.N}` to choose precision. Non-finite channels are written as `none`.
#[derive(Debug, Clone, Copy, PartialEq)]

pub struct Rec2020 {
    /// Red channel, nominally `0.0..=1.0`.
    pub r: Channel<f32>,
    /// Green channel, nominally `0.0..=1.0`.
    pub g: Channel<f32>,
    /// Blue channel, nominally `0.0..=1.0`.
    pub b: Channel<f32>,
}

impl Rec2020 {
    /// Creates channels with this color space's bounds, without validating or clamping values.

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: Channel::color_channel("r", r, 0.0..=1.0),
            g: Channel::color_channel("g", g, 0.0..=1.0),
            b: Channel::color_channel("b", b, 0.0..=1.0),
        }
    }
}

crate::color::formatting::impl_display!(
    Rec2020, "color(rec2020 ", |color| [
        *color.r => "",
        *color.g => "",
        *color.b => ""
    ]
);

crate::color::conversions::impl_colorspace!(
    infallible Rec2020,
    [r, g, b],
    crate::color::rgb_conversion::rec2020_to_linear,
    crate::color::rgb_conversion::linear_to_rec2020
);
