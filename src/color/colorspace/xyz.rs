use crate::color::Channel;

/// CIE XYZ tristimulus coordinates.
///
/// Uses relative scaling with reference-white `y = 1.0`, rather than 100.
/// Physical tristimulus values are nonnegative. X and Z have no universal
/// `0.0..=1.0` bound; values above one can be valid. Extended calculations
/// may also produce negative coordinates.
/// Conversions use D65, matching the CSS `xyz` keyword.
/// Use `new` to configure channel names and bounds. Values are not automatically clamped.
/// Use finite channel values.
///
/// `Display` writes CSS `color(xyz...)` with three decimal places by default;
/// use `{:.N}` to choose precision. Non-finite channels are written as `none`.
/// CSS `xyz` means D65; callers must supply D65 coordinates for CSS output.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Xyz {
    /// X tristimulus value, normally nonnegative, with no fixed upper bound.
    pub x: Channel<f32>,
    /// Relative luminance, nominally `0.0..=1.0`; values above white may exceed one.
    pub y: Channel<f32>,
    /// Z tristimulus value, normally nonnegative, with no fixed upper bound.
    pub z: Channel<f32>,
}

impl Xyz {
    /// Creates channels with this color space's bounds, without validating or clamping values.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: Channel::color_channel("x", x, 0.0..),
            y: Channel::color_channel("y", y, 0.0..),
            z: Channel::color_channel("z", z, 0.0..),
        }
    }
}

crate::color::formatting::impl_display!(
    Xyz, "color(xyz ", |color| [
        *color.x => "",
        *color.y => "",
        *color.z => ""
    ]
);

crate::color::conversions::impl_colorspace!(
    Xyz,
    [x, y, z],
    crate::color::rgb_conversion::xyz_to_linear,
    crate::color::rgb_conversion::linear_to_xyz
);
