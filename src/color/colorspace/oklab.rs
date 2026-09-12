use crate::color::{ColorChannel, channel::color_channel};

/// A Oklab color expressed as lightness and two opponent axes.
///
/// The opponent axes have no fixed mathematical bounds; the ranges below
/// are nominal reference ranges, not gamut limits.
/// Use `new` to configure channel names and bounds. Values are not automatically clamped.
/// Use finite channel values.
///
/// `Display` writes CSS `oklab(...)` with three decimal places by default;
/// use `{:.N}` to choose precision. Non-finite channels are written as `none`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Oklab {
    /// Lightness, nominally `0.0..=1.0`, from black to reference white.
    pub l: ColorChannel,
    /// Green-negative/red-positive axis, nominally `-0.4..=0.4`.
    pub a: ColorChannel,
    /// Blue-negative/yellow-positive axis, nominally `-0.4..=0.4`.
    pub b: ColorChannel,
}

impl Oklab {
    /// Creates channels with this color space's bounds, without validating or clamping values.
    pub fn new(l: f32, a: f32, b: f32) -> Self {
        Self {
            l: color_channel("l", l, 0.0..=1.0),
            a: color_channel("a", a, ..),
            b: color_channel("b", b, ..),
        }
    }
}

crate::color::formatting::impl_display!(
    Oklab, "oklab(", |color| [
        *color.l => "",
        *color.a => "",
        *color.b => ""
    ]
);

crate::color::conversions::impl_colorspace!(
    infallible Oklab,
    [l, a, b],
    crate::color::perceptual_conversion::oklab_to_linear,
    crate::color::perceptual_conversion::linear_to_oklab
);
