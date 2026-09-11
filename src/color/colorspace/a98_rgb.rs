/// A Adobe RGB (1998) color with encoded RGB channels and no alpha.
///
/// Each channel is nominally `0.0..=1.0`. Extended values represent colors
/// outside this RGB gamut.
/// Expected ranges describe the color model; public fields do not validate or clamp values.
/// Use finite channel values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct A98Rgb {
    /// Red channel, nominally `0.0..=1.0`.
    pub r: f32,
    /// Green channel, nominally `0.0..=1.0`.
    pub g: f32,
    /// Blue channel, nominally `0.0..=1.0`.
    pub b: f32,
}
