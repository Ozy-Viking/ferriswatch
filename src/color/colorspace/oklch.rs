use crate::color::Channel;

/// A Oklab color expressed in cylindrical lightness, chroma, and hue coordinates.
///
/// Chroma has no fixed upper bound; its displayable maximum depends on
/// lightness, hue, and the target gamut.
/// Use `new` to configure channel names and bounds. Values are not automatically clamped.
/// Use finite channel values.
///
/// `Display` writes CSS `oklch(...)` with three decimal places by default;
/// use `{:.N}` to choose precision. Non-finite channels are written as `none`.
#[derive(Debug, Clone, Copy, PartialEq)]

pub struct Oklch {
    /// Lightness, nominally `0.0..=1.0`, from black to reference white.
    pub l: Channel<f32>,
    /// Nonnegative chroma, nominally `0.0..=0.4`; larger values are possible.
    pub c: Channel<f32>,
    /// Hue in degrees, normally `0.0..360.0`; 360 degrees is equivalent to zero. Has no effect when chroma is zero.
    pub h: Channel<f32>,
}

impl Oklch {
    /// Creates channels with this color space's bounds, without validating or clamping values.

    pub fn new(l: f32, c: f32, h: f32) -> Self {

        Self {
            l: Channel::color_channel("l", l, 0.0..=1.0),
            c: Channel::color_channel("c", c, 0.0..),
            h: Channel::color_channel("h", h, 0.0..360.0)
                .with_wrapping()
                .expect("built-in hue range is valid"),
        }
    }
}

crate::color::formatting::impl_display!(
    Oklch, "oklch(", |color| [
        *color.l => "",
        *color.c => "",
        *color.h => ""
    ]
);

crate::color::conversions::impl_colorspace!(
    infallible Oklch,
    [l, c, h],
    crate::color::perceptual_conversion::oklch_to_linear,
    crate::color::perceptual_conversion::linear_to_oklch
);
