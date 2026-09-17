//! Color-space representations, alpha aliases, and conversions through linear sRGB.
//!
//! `Display` uses [CSS Color 4](https://www.w3.org/TR/css-color-4/) syntax,
//! except for [`Lms`] and [`LmsPrime`], which have no standard CSS representation.
//! [`Hsv`] is displayed as equivalent HWB. CSS formatting interprets [`Lab`] and
//! [`Lch`] as D50 coordinates and [`Xyz`] as D65 coordinates.
//! Floating-point channels default to three decimal places; `{:.N}` sets precision.
//! Non-finite channels become the CSS missing component `none`. Finite values are
//! not clamped; formatting rounds them and is not a lossless serialization.
//! [`Rgb`] and [`Srgb`] also support lower/upper hex and `#` alternate prefixes.
//! Other spaces must be converted to sRGB before they can be represented as CSS hex.
//!
//! Construct colors with `new(...)` to configure their channel names and bounds.
//! Public components are [`super::Channel`] values: use `*color.h` to read
//! a number and `color.h.set_value(...)` to replace it without clamping.
//! [`Srgb`] and [`LinearSrgb`] retain numeric getters and expose immutable
//! channel access through `r_channel()`, `g_channel()`, and `b_channel()`.
//! [`Rgb`] stores byte channels, rather than a packed three-byte representation.
//!
//! ```
//! use ferriswatch_core::color::{Alpha, Hsl, Rgb};
//!
//! let color = Hsl::new(120.0, 0.5, 0.25);
//! assert_eq!(format!("{color:.1}"), "hsl(120.0 50.0% 25.0%)");
//! let rgba = Alpha::new(Rgb::new(255, 128, 0), 0.5)?;
//! assert_eq!(format!("{rgba}"), "rgb(255 128 0 / 0.500)");
//! assert_eq!(format!("{rgba:#X}"), "#FF800080");
//! # Ok::<(), ferriswatch_core::color::ColorError>(())
//! ```

use std::fmt::Display;

use super::{Alpha, Clamp, Color, ColorResult};

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

/// A color representation convertible to and from linear sRGB.
///
/// Floating-point raw conversions preserve extended channel values and reject
/// non-finite inputs or results that overflow `f32`. HSL and HSV also reject
/// extended colors at singular coordinates that require infinite saturation.
/// [`Rgb`] rejects out-of-gamut input and rounds representable channels to bytes.
/// Converting through [`Color`] sets alpha to one; input alpha is ignored when
/// extracting a color space. Alpha wrappers are not themselves color spaces.
///
/// [`Clamp`] applies channel constraints, not perceptual gamut mapping. RGB
/// channels use `0..=1`; hue wraps to `0..360`; cylindrical fractions use `0..=1`.
/// HWB also normalizes whiteness plus blackness to at most one. Lab/Oklab clamp
/// lightness, and LCh/OkLCh also clamp chroma to nonnegative values. XYZ clamps
/// only negative coordinates. Signed, unbounded LMS intermediates are unchanged.
/// Clamping does not validate non-finite values; conversion remains fallible.
///
/// # Examples
///
/// ```
/// use ferriswatch_core::color::{Color, Oklch, Srgb};
///
/// let source = Color::try_from(Srgb::new(1.0, 0.0, 0.0)?)?;
/// let polar: Oklch = source.oklch();
/// let restored = Color::try_from(polar)?;
/// assert!((restored.r() - source.r()).abs() < 0.00001);
/// # Ok::<(), ferriswatch_core::color::ColorError>(())
/// ```

pub trait ColorSpace: Sized + TryFrom<LinearSrgb> + Clamp + Display
where
    LinearSrgb: TryFrom<Self>,
{
    /// Converts to an opaque linear-sRGB color without clamping.

    fn try_into_color(self) -> ColorResult<Color> {
        let color = self.try_into_linear_srgb_raw()?;

        Color::new(color.r(), color.g(), color.b(), 1.0)
    }

    /// Converts the color channels, ignoring alpha, without clamping.

    fn try_from_color(color: Color) -> ColorResult<Self> {
        Self::try_from_linear_srgb_raw(color.linear_srgb())
    }

    /// Converts to linear sRGB without clamping; rejects non-finite or overflowing results.

    fn try_into_linear_srgb_raw(self) -> ColorResult<LinearSrgb>;

    /// Converts from linear sRGB without clamping; rejects unrepresentable results.

    fn try_from_linear_srgb_raw(color: LinearSrgb) -> ColorResult<Self>;

    /// Converts without clamping the input, then clamps the destination channels.
    ///
    /// Byte representations override this to clamp before quantization.

    fn try_from_linear_srgb_clamped(color: LinearSrgb) -> ColorResult<Self> {
        Ok(Self::try_from_linear_srgb_raw(color)?.clamp())
    }
}
