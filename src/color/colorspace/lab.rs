/// A CIELAB color expressed as lightness and two opponent axes.
///
/// Coordinates require a reference white, which this struct does not encode.
/// The opponent axes have no fixed mathematical bounds; the ranges below
/// are nominal reference ranges, not gamut limits.
/// Expected ranges describe the color model; public fields do not validate or clamp values.
/// Use finite channel values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lab {
    /// Lightness, nominally `0.0..=100.0`, from black to reference white.
    pub l: f32,
    /// Green-negative/red-positive axis, nominally `-125.0..=125.0`.
    pub a: f32,
    /// Blue-negative/yellow-positive axis, nominally `-125.0..=125.0`.
    pub b: f32,
}
