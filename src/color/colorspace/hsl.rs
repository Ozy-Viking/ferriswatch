/// An sRGB color expressed as hue, saturation, and lightness.
///
/// Expected ranges describe the color model; public fields do not validate or clamp values.
/// Use finite channel values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsl {
    /// Hue in degrees, normally `0.0..360.0`; 360 degrees is equivalent to zero.
    pub h: f32,
    /// Saturation in `0.0..=1.0`, from achromatic to fully saturated.
    pub s: f32,
    /// Lightness in `0.0..=1.0`, from black to white.
    pub l: f32,
}
