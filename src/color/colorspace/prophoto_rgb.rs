use crate::color::Channel;

/// A ProPhoto RGB color with encoded RGB channels and no alpha.
///
/// Each channel is nominally `0.0..=1.0`. Extended values represent colors
/// outside this RGB gamut.
/// Use `new` to configure channel names and bounds. Values are not automatically clamped.
/// Use finite channel values.
///
/// `Display` writes CSS `color(prophoto-rgb...)` with three decimal places by default;
/// use `{:.N}` to choose precision. Non-finite channels are written as `none`.
#[derive(Debug, Clone, Copy, PartialEq)]

pub struct ProPhotoRgb {
    /// Red channel, nominally `0.0..=1.0`.
    pub r: Channel<f32>,
    /// Green channel, nominally `0.0..=1.0`.
    pub g: Channel<f32>,
    /// Blue channel, nominally `0.0..=1.0`.
    pub b: Channel<f32>,
}

impl ProPhotoRgb {
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
    ProPhotoRgb, "color(prophoto-rgb ", |color| [
        *color.r => "",
        *color.g => "",
        *color.b => ""
    ]
);

crate::color::conversions::impl_colorspace!(
    infallible ProPhotoRgb,
    [r, g, b],
    crate::color::rgb_conversion::prophoto_to_linear,
    crate::color::rgb_conversion::linear_to_prophoto
);
