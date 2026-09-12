use crate::color::{ColorChannel, channel::color_channel};

/// CIE XYZ tristimulus coordinates relative to a D50 reference white.
///
/// Uses relative scaling with reference-white `y = 1.0`, rather than 100.
/// Physical tristimulus values are nonnegative. X and Z have no universal
/// `0.0..=1.0` bound; values above one can be valid. Extended calculations
/// may also produce negative coordinates.
/// Use `new` to configure channel names and bounds. Values are not automatically clamped.
/// Use finite channel values.
///
/// `Display` writes CSS `color(xyz-d50...)` with three decimal places by default;
/// use `{:.N}` to choose precision. Non-finite channels are written as `none`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XyzD50 {
    /// X tristimulus value, normally nonnegative, with no fixed upper bound.
    pub x: ColorChannel,
    /// Relative luminance, nominally `0.0..=1.0`; values above white may exceed one.
    pub y: ColorChannel,
    /// Z tristimulus value, normally nonnegative, with no fixed upper bound.
    pub z: ColorChannel,
}

impl XyzD50 {
    /// Creates channels with this color space's bounds, without validating or clamping values.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: color_channel("x", x, 0.0..),
            y: color_channel("y", y, 0.0..),
            z: color_channel("z", z, 0.0..),
        }
    }
}

crate::color::formatting::impl_display!(
    XyzD50, "color(xyz-d50 ", |color| [
        *color.x => "",
        *color.y => "",
        *color.z => ""
    ]
);

crate::color::conversions::impl_colorspace!(
    infallible XyzD50,
    [x, y, z],
    crate::color::rgb_conversion::xyz50_to_linear,
    crate::color::rgb_conversion::linear_to_xyz50
);
