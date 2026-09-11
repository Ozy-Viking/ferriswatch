//! Color-space representations, alpha aliases, and conversions through linear sRGB.

use super::{Alpha, Clamp, Color, ColorError, ColorResult};

mod a98_rgb;
pub use a98_rgb::A98Rgb;
mod display_p3;
pub use display_p3::DisplayP3;
mod hsl;
pub use hsl::Hsl;
mod hsv;
pub use hsv::Hsv;
mod hwb;
pub use hwb::Hwb;
mod lab;
pub use lab::Lab;
mod lch;
pub use lch::Lch;
mod lms;
pub use lms::Lms;
mod lms_prime;
pub use lms_prime::LmsPrime;
mod oklab;
pub use oklab::Oklab;
mod oklch;
pub use oklch::Oklch;
mod prophoto_rgb;
pub use prophoto_rgb::ProPhotoRgb;
mod rec2020;
pub use rec2020::Rec2020;
mod xyz;
pub use xyz::Xyz;
mod xyz_d50;
pub use xyz_d50::XyzD50;
mod xyz_d65;
pub use xyz_d65::XyzD65;
mod linear_srgb;
pub use linear_srgb::LinearSrgb;
mod srgb;
pub use srgb::{Rgb, Srgb};

pub type A98Rgba = Alpha<A98Rgb>;
pub type DisplayP3a = Alpha<DisplayP3>;
pub type Hsla = Alpha<Hsl>;
pub type Hsva = Alpha<Hsv>;
pub type Hwba = Alpha<Hwb>;
pub type Laba = Alpha<Lab>;
pub type Lcha = Alpha<Lch>;
pub type LinearSrgba = Alpha<LinearSrgb>;
pub type Lmsa = Alpha<Lms>;
pub type LmsPrimea = Alpha<LmsPrime>;
pub type Oklaba = Alpha<Oklab>;
pub type Oklcha = Alpha<Oklch>;
pub type ProPhotoRgba = Alpha<ProPhotoRgb>;
pub type Rec2020a = Alpha<Rec2020>;
pub type Rgba = Alpha<Rgb>;
pub type SRgba = Alpha<Srgb>;
pub type Xyza = Alpha<Xyz>;
pub type XyzD50a = Alpha<XyzD50>;
pub type XyzD65a = Alpha<XyzD65>;

pub trait ColorSpace: Sized + TryFrom<LinearSrgb> + Clamp
where
    LinearSrgb: TryFrom<Self>,
{
    fn try_into_color(self) -> ColorResult<Color>
    where
        <LinearSrgb as TryFrom<Self>>::Error: Into<ColorError>,
    {
        let color = LinearSrgb::try_from(self).map_err(Into::into)?;

        Color::new(color.r(), color.g(), color.b(), 1.0)
    }

    fn try_from_color(color: Color) -> ColorResult<Self>
    where
        <Self as TryFrom<LinearSrgb>>::Error: Into<ColorError>,
    {
        Self::try_from(color.linear_srgb()).map_err(Into::into)
    }
    fn try_into_linear_srgb_raw(self) -> ColorResult<LinearSrgb>;

    fn try_from_linear_srgb_raw(color: LinearSrgb) -> ColorResult<Self>;
}
