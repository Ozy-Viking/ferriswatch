/// LMS cone-response coordinates.
///
/// The range depends on the transform and normalization used by the caller;
/// there is no universal `0.0..=1.0` bound.
/// Physical linear responses are normally nonnegative; extended color
/// calculations may produce negative values.
/// Expected ranges describe the color model; public fields do not validate or clamp values.
/// Use finite channel values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lms {
    /// Long-wavelength component; finite, with no fixed bounds.
    pub l: f32,
    /// Medium-wavelength component; finite, with no fixed bounds.
    pub m: f32,
    /// Short-wavelength component; finite, with no fixed bounds.
    pub s: f32,
}
