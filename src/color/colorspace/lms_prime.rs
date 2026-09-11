/// Nonlinear LMS intermediate coordinates.
///
/// The range depends on the transform and normalization used by the caller;
/// there is no universal `0.0..=1.0` bound.
/// In the Oklab conversion, these are the signed cube roots of linear LMS
/// coordinates. This struct does not select or apply a transform.
/// Expected ranges describe the color model; public fields do not validate or clamp values.
/// Use finite channel values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LmsPrime {
    /// Long-wavelength component; finite, with no fixed bounds.
    pub l: f32,
    /// Medium-wavelength component; finite, with no fixed bounds.
    pub m: f32,
    /// Short-wavelength component; finite, with no fixed bounds.
    pub s: f32,
}
