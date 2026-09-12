use crate::color::{ColorChannel, channel::color_channel};

/// An sRGB color expressed as hue, whiteness, and blackness.
///
/// The nominal range has `w + b <= 1.0`; larger sums require normalization
/// to represent an achromatic color. `Clamp` normalizes the sum after clamping each channel.
/// Use `new` to configure channel names and bounds. Values are not automatically clamped.
/// Use finite channel values.
///
/// `Display` writes CSS `hwb(...)` with three decimal places by default;
/// use `{:.N}` to choose precision. Non-finite channels are written as `none`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hwb {
    /// Hue in degrees, normally `0.0..360.0`; 360 degrees is equivalent to zero.
    pub h: ColorChannel,
    /// Whiteness in `0.0..=1.0`.
    pub w: ColorChannel,
    /// Blackness in `0.0..=1.0`.
    pub b: ColorChannel,
}

impl Hwb {
    /// Creates channels with this color space's bounds, without validating or clamping values.
    pub fn new(h: f32, w: f32, b: f32) -> Self {
        Self {
            h: color_channel("h", h, 0.0..360.0)
                .with_wrapping()
                .expect("built-in hue range is valid"),
            w: color_channel("w", w, 0.0..=1.0),
            b: color_channel("b", b, 0.0..=1.0),
        }
    }
}

crate::color::formatting::impl_display!(
    Hwb, "hwb(", |color| [
        *color.h => "",
        f64::from(*color.w) * 100.0 => "%",
        f64::from(*color.b) * 100.0 => "%"
    ]
);

crate::color::conversions::impl_colorspace!(
    infallible Hwb,
    [h, w, b],
    crate::color::cylindrical_conversion::hwb_to_linear,
    crate::color::cylindrical_conversion::linear_to_hwb,
    normalize
);

impl Hwb {
    fn normalize(&mut self) {
        let sum = *self.w + *self.b;
        if sum > 1.0 {
            *self.w /= sum;
            *self.b /= sum;
        }
    }
}
