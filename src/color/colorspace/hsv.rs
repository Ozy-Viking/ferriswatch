use crate::color::{ColorChannel, channel::color_channel};

/// An sRGB color expressed as hue, saturation, and value.
///
/// Use `new` to configure channel names and bounds. Values are not automatically clamped.
/// Use finite channel values.
///
/// `Display` writes CSS `hwb(...)` with three decimal places by default;
/// use `{:.N}` to choose precision. Non-finite channels are written as `none`.
/// HSV is expressed as equivalent HWB because CSS has no HSV function.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsv {
    /// Hue in degrees, normally `0.0..360.0`; 360 degrees is equivalent to zero.
    pub h: ColorChannel<f32>,
    /// Saturation in `0.0..=1.0`, from achromatic to fully saturated.
    pub s: ColorChannel<f32>,
    /// Value in `0.0..=1.0`, equal to the largest encoded RGB channel.
    pub v: ColorChannel<f32>,
}

impl Hsv {
    /// Creates channels with this color space's bounds, without validating or clamping values.
    pub fn new(h: f32, s: f32, v: f32) -> Self {
        Self {
            h: color_channel("h", h, 0.0..360.0)
                .with_wrapping()
                .expect("built-in hue range is valid"),
            s: color_channel("s", s, 0.0..=1.0),
            v: color_channel("v", v, 0.0..=1.0),
        }
    }
}

crate::color::formatting::impl_display!(
    Hsv, "hwb(", |color| [
        *color.h => "",
        (1.0 - f64::from(*color.s)) * f64::from(*color.v) * 100.0 => "%",
        (1.0 - f64::from(*color.v)) * 100.0 => "%"
    ]
);

crate::color::conversions::impl_colorspace!(
    Hsv,
    [h, s, v],
    crate::color::cylindrical_conversion::hsv_to_linear,
    crate::color::cylindrical_conversion::linear_to_hsv
);
