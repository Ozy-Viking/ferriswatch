use crate::color::Channel;

/// An sRGB color expressed as hue, saturation, and lightness.
///
/// Use `new` to configure channel names and bounds. Values are not automatically clamped.
/// Use finite channel values.
///
/// `Display` writes CSS `hsl(...)` with three decimal places by default;
/// use `{:.N}` to choose precision. Non-finite channels are written as `none`.
#[derive(Debug, Clone, Copy, PartialEq)]

pub struct Hsl {
    /// Hue in degrees, normally `0.0..360.0`; 360 degrees is equivalent to zero.
    pub h: Channel<f32>,
    /// Saturation in `0.0..=1.0`, from achromatic to fully saturated.
    pub s: Channel<f32>,
    /// Lightness in `0.0..=1.0`, from black to white.
    pub l: Channel<f32>,
}

impl Hsl {
    /// Creates channels with this color space's bounds, without validating or clamping values.

    pub fn new(h: f32, s: f32, l: f32) -> Self {
        Self {
            h: Channel::color_channel("h", h, 0.0..360.0)
                .with_wrapping()
                .expect("built-in hue range is valid"),
            s: Channel::color_channel("s", s, 0.0..=1.0),
            l: Channel::color_channel("l", l, 0.0..=1.0),
        }
    }
}

crate::color::formatting::impl_display!(
    Hsl, "hsl(", |color| [
        *color.h => "",
        f64::from(*color.s) * 100.0 => "%",
        f64::from(*color.l) * 100.0 => "%"
    ]
);

crate::color::conversions::impl_colorspace!(
    Hsl,
    [h, s, l],
    crate::color::cylindrical_conversion::hsl_to_linear,
    crate::color::cylindrical_conversion::linear_to_hsl
);
