/// An sRGB color expressed as hue, whiteness, and blackness.
///
/// The nominal range has `w + b <= 1.0`; larger sums require normalization
/// to represent an achromatic color. This struct does not perform that normalization.
/// Expected ranges describe the color model; public fields do not validate or clamp values.
/// Use finite channel values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hwb {
    /// Hue in degrees, normally `0.0..360.0`; 360 degrees is equivalent to zero.
    pub h: f32,
    /// Whiteness in `0.0..=1.0`.
    pub w: f32,
    /// Blackness in `0.0..=1.0`.
    pub b: f32,
}
