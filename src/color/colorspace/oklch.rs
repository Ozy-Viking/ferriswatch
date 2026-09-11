/// A Oklab color expressed in cylindrical lightness, chroma, and hue coordinates.
///
/// Chroma has no fixed upper bound; its displayable maximum depends on
/// lightness, hue, and the target gamut.
/// Expected ranges describe the color model; public fields do not validate or clamp values.
/// Use finite channel values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Oklch {
    /// Lightness, nominally `0.0..=1.0`, from black to reference white.
    pub l: f32,
    /// Nonnegative chroma, nominally `0.0..=0.4`; larger values are possible.
    pub c: f32,
    /// Hue in degrees, normally `0.0..360.0`; 360 degrees is equivalent to zero. Has no effect when chroma is zero.
    pub h: f32,
}
