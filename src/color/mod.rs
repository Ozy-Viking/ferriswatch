mod channel;
pub use channel::{
    AdjacentValue, Channel, ChannelBuilder, ChannelError, ColorChannel, RangeErrorReason,
    WrappingRangeErrorReason, WrappingValue,
};
mod conversions;
mod cylindrical_conversion;
mod formatting;
mod matrices;
mod perceptual_conversion;
mod rgb_conversion;

pub mod colorspace;
pub use colorspace::*;
mod alpha;
pub use alpha::Alpha;
#[allow(clippy::module_inception)]
mod color;
pub use color::Color;

pub mod prelude {
    pub use super::{
        A98Rgb, A98Rgba, Alpha, Color, ColorError, ColorResult, ColorSpace, DisplayP3, DisplayP3a,
        Hsl, Hsla, Hsv, Hsva, Hwb, Hwba, Lab, Laba, Lch, Lcha, LinearSrgb, LinearSrgba, Lms,
        LmsPrime, LmsPrimea, Lmsa, Oklab, Oklaba, Oklch, Oklcha, ProPhotoRgb, ProPhotoRgba,
        Rec2020, Rec2020a, Rgb, Rgba, SRgba, Srgb, Xyz, XyzD50, XyzD50a, XyzD65, XyzD65a, Xyza,
    };
}

use std::convert::Infallible;

pub type ColorResult<T> = Result<T, ColorError>;

#[derive(Debug, Clone, miette::Diagnostic, thiserror::Error)]
pub enum ColorError {
    #[error("invalid length of {0}")]
    InvalidLength(usize),

    #[error("invalid hex")]
    InvalidHex,

    #[error("invalid hue: {0}")]
    InvalidHue(f32),

    #[error("invalid saturation: {0}")]
    InvalidSaturation(f32),

    #[error("invalid lightness: {0}")]
    InvalidLightness(f32),

    #[error("invalid chroma: {0}")]
    InvalidChroma(f32),
    #[error("invalid alpha: {0}")]
    InvalidAlpha(f32),
    #[error("invalid color channel value:{0}={1}")]
    InvalidColorChannel(&'static str, f32),
    #[error("color channel out of sRGB gamut: {0}={1}")]
    OutOfSrgbGamut(&'static str, f32),
    #[error("{0}")]
    ChannelError(#[from] ChannelError),
}

impl PartialEq for ColorError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::InvalidLength(l0), Self::InvalidLength(r0)) => l0 == r0,
            (
                Self::InvalidColorChannel(l_name, l_value),
                Self::InvalidColorChannel(r_name, r_value),
            )
            | (Self::OutOfSrgbGamut(l_name, l_value), Self::OutOfSrgbGamut(r_name, r_value)) => {
                l_name == r_name && floats_eq(l_value, r_value)
            }

            (Self::InvalidSaturation(l0), Self::InvalidSaturation(r0))
            | (Self::InvalidAlpha(l0), Self::InvalidAlpha(r0))
            | (Self::InvalidLightness(l0), Self::InvalidLightness(r0))
            | (Self::InvalidChroma(l0), Self::InvalidChroma(r0))
            | (Self::InvalidHue(l0), Self::InvalidHue(r0)) => floats_eq(l0, r0),
            (Self::InvalidHex, Self::InvalidHex) => true,
            _ => false,
        }
    }
}

impl Eq for ColorError {}

impl From<Infallible> for ColorError {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

fn floats_eq(a: &f32, b: &f32) -> bool {
    if a.is_nan() && b.is_nan() {
        true
    } else {
        a == b
    }
}

/// Applies the bounds of each channel, wrapping circular values with modulo.
///
/// Hue wraps into `0.0..360.0`; unbounded channels are left unchanged.
/// This operation does not validate values or perform perceptual gamut mapping.
pub trait Clamp {
    fn clamp(self) -> Self;
}

pub trait ClampedInto<T>: Sized {
    fn clamped_into(self) -> ColorResult<T>;
}

impl<T, U> ClampedInto<U> for T
where
    U: ClampedFrom<T>,
{
    fn clamped_into(self) -> ColorResult<U> {
        U::clamped_from(self)
    }
}

pub trait ClampedFrom<T>: Sized {
    fn clamped_from(value: T) -> ColorResult<Self>;
}

impl<T, U> ClampedFrom<T> for U
where
    T: ColorSpace,
    LinearSrgb: TryFrom<T> + TryFrom<U>,
    U: ColorSpace,
{
    fn clamped_from(value: T) -> ColorResult<Self> {
        let linear = value.try_into_linear_srgb_raw()?;
        U::try_from_linear_srgb_clamped(linear)
    }
}
