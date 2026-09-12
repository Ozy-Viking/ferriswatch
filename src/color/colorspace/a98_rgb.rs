use crate::color::{ColorChannel, channel::color_channel};

/// A Adobe RGB (1998) color with encoded RGB channels and no alpha.
///
/// Each channel is nominally `0.0..=1.0`. Extended values represent colors
/// outside this RGB gamut.
/// Use `new` to configure channel names and bounds. Values are not automatically clamped.
/// Use finite channel values.
///
/// `Display` writes CSS `color(a98-rgb...)` with three decimal places by default;
/// use `{:.N}` to choose precision. Non-finite channels are written as `none`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct A98Rgb {
    /// Red channel, nominally `0.0..=1.0`.
    pub r: ColorChannel,
    /// Green channel, nominally `0.0..=1.0`.
    pub g: ColorChannel,
    /// Blue channel, nominally `0.0..=1.0`.
    pub b: ColorChannel,
}

impl A98Rgb {
    /// Creates channels with this color space's bounds, without validating or clamping values.
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: color_channel("r", r, 0.0..=1.0),
            g: color_channel("g", g, 0.0..=1.0),
            b: color_channel("b", b, 0.0..=1.0),
        }
    }
}

crate::color::formatting::impl_display!(
    A98Rgb, "color(a98-rgb ", |color| [
        *color.r => "",
        *color.g => "",
        *color.b => ""
    ]
);

crate::color::conversions::impl_colorspace!(
    infallible A98Rgb,
    [r, g, b],
    crate::color::rgb_conversion::a98_to_linear,
    crate::color::rgb_conversion::linear_to_a98
);
