/// CIE XYZ tristimulus coordinates relative to a D50 reference white.
///
/// Uses relative scaling with reference-white `y = 1.0`, rather than 100.
/// Physical tristimulus values are nonnegative. X and Z have no universal
/// `0.0..=1.0` bound; values above one can be valid. Extended calculations
/// may also produce negative coordinates.
/// Expected ranges describe the color model; public fields do not validate or clamp values.
/// Use finite channel values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XyzD50 {
    /// X tristimulus value, normally nonnegative, with no fixed upper bound.
    pub x: f32,
    /// Relative luminance, nominally `0.0..=1.0`; values above white may exceed one.
    pub y: f32,
    /// Z tristimulus value, normally nonnegative, with no fixed upper bound.
    pub z: f32,
}
