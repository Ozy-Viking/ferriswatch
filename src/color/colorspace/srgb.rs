use crate::color::{ColorChannel, channel::color_channel};

use super::{ColorSpace, LinearSrgb};
use crate::color::{Clamp, ColorError, ColorResult, floats_eq};
use std::fmt;

/// An sRGB color with encoded floating-point channels and no alpha.
///
/// RGB channels are nominally `0.0..=1.0`. The constructor accepts any finite
/// value, including negative values and values above one for colors outside
/// the sRGB gamut. Values are not clamped automatically.
///
/// `Display` writes CSS `color(srgb...)` with three decimal places by default;
/// use `{:.N}` to choose precision. Non-finite channels are written as `none`.
/// Hex formatting with `{:x}` or `{:X}` clamps channels to `0..=1` and rounds
/// to 8 bits. The alternate forms `{:#x}` and `{:#X}` include a CSS `#` prefix.
#[derive(Debug, Clone, Copy)]
pub struct Srgb {
    /// Red channel, nominally `0.0..=1.0`; must be finite.
    pub(super) r: ColorChannel,
    /// Green channel, nominally `0.0..=1.0`; must be finite.
    pub(super) g: ColorChannel,
    /// Blue channel, nominally `0.0..=1.0`; must be finite.
    pub(super) b: ColorChannel,
}

impl Srgb {
    pub fn new(r: f32, g: f32, b: f32) -> ColorResult<Self> {
        if !r.is_finite() {
            return Err(ColorError::InvalidColorChannel("r", r));
        }
        if !g.is_finite() {
            return Err(ColorError::InvalidColorChannel("g", g));
        }
        if !b.is_finite() {
            return Err(ColorError::InvalidColorChannel("b", b));
        }

        Ok(Self {
            r: color_channel("r", r, 0.0..=1.0),
            g: color_channel("g", g, 0.0..=1.0),
            b: color_channel("b", b, 0.0..=1.0),
        })
    }

    /// Borrows the r channel and its bounds.
    pub fn r_channel(&self) -> &ColorChannel {
        &self.r
    }

    pub fn r(&self) -> f32 {
        *self.r
    }

    /// Borrows the g channel and its bounds.
    pub fn g_channel(&self) -> &ColorChannel {
        &self.g
    }

    pub fn g(&self) -> f32 {
        *self.g
    }

    /// Borrows the b channel and its bounds.
    pub fn b_channel(&self) -> &ColorChannel {
        &self.b
    }

    pub fn b(&self) -> f32 {
        *self.b
    }

    pub fn is_in_srgb_gamut(&self) -> bool {
        self.r.in_bounds() && self.g.in_bounds() && self.b.in_bounds()
    }

    pub fn r_u8_clamped(&self) -> u8 {
        (*self.r.clamp() * 255.0).round() as u8
    }

    pub fn g_u8_clamped(&self) -> u8 {
        (*self.g.clamp() * 255.0).round() as u8
    }

    pub fn b_u8_clamped(&self) -> u8 {
        (*self.b.clamp() * 255.0).round() as u8
    }
}

impl PartialEq for Srgb {
    fn eq(&self, other: &Self) -> bool {
        floats_eq(&self.r, &other.r) && floats_eq(&self.g, &other.g) && floats_eq(&self.b, &other.b)
    }
}

impl Eq for Srgb {}

impl From<LinearSrgb> for Srgb {
    fn from(color: LinearSrgb) -> Self {
        // The signed encoding compresses large magnitudes, so finite linear
        // channels always produce finite encoded channels.
        Self {
            r: color_channel("r", linear_to_srgb(color.r()), 0.0..=1.0),
            g: color_channel("g", linear_to_srgb(color.g()), 0.0..=1.0),
            b: color_channel("b", linear_to_srgb(color.b()), 0.0..=1.0),
        }
    }
}

impl TryFrom<Srgb> for LinearSrgb {
    type Error = ColorError;

    fn try_from(color: Srgb) -> ColorResult<Self> {
        Self::new(
            srgb_to_linear(color.r()),
            srgb_to_linear(color.g()),
            srgb_to_linear(color.b()),
        )
    }
}

impl Clamp for Srgb {
    fn clamp(mut self) -> Self {
        self.r = self.r.clamp();
        self.g = self.g.clamp();
        self.b = self.b.clamp();
        self
    }
}
impl ColorSpace for Srgb {
    fn try_into_linear_srgb_raw(self) -> ColorResult<LinearSrgb> {
        LinearSrgb::new(
            srgb_to_linear(*self.r),
            srgb_to_linear(*self.g),
            srgb_to_linear(*self.b),
        )
    }

    fn try_from_linear_srgb_raw(color: LinearSrgb) -> ColorResult<Self> {
        Ok(Self::from(color))
    }
}

impl From<Rgb> for Srgb {
    fn from(rgb: Rgb) -> Srgb {
        Srgb::new(rgb.r_f32(), rgb.g_f32(), rgb.b_f32())
            .expect("byte channels always convert to finite sRGB")
    }
}

fn srgb_to_linear(value: f32) -> f32 {
    let sign = value.signum();
    let value = value.abs();

    if value <= 0.04045 {
        sign * value / 12.92
    } else {
        sign * ((value + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_to_srgb(value: f32) -> f32 {
    let sign = value.signum();
    let value = value.abs();

    if value <= 0.0031308 {
        sign * value * 12.92
    } else {
        sign * (1.055 * value.powf(1.0 / 2.4) - 0.055)
    }
}

/// An encoded sRGB color with three 8-bit channels and no alpha.
///
/// Each channel ranges from `0..=255`.
///
/// `Display` writes CSS `rgb(r g b)` with integer channels. Hex formatting
/// supports `{:x}` and `{:X}`, with a `#` prefix for `{:#x}` and `{:#X}`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb(
    /// Red channel in `0..=255`.
    pub ColorChannel<u8>,
    /// Green channel in `0..=255`.
    pub ColorChannel<u8>,
    /// Blue channel in `0..=255`.
    pub ColorChannel<u8>,
);

impl Rgb {
    /// Creates byte channels with names and inclusive bounds `0..=255`.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(
            color_channel("r", r, 0..=255),
            color_channel("g", g, 0..=255),
            color_channel("b", b, 0..=255),
        )
    }

    /// Borrows the red channel and its bounds.
    pub fn r_channel(&self) -> &ColorChannel<u8> {
        &self.0
    }

    /// Borrows the green channel and its bounds.
    pub fn g_channel(&self) -> &ColorChannel<u8> {
        &self.1
    }

    /// Borrows the blue channel and its bounds.
    pub fn b_channel(&self) -> &ColorChannel<u8> {
        &self.2
    }

    pub fn r(&self) -> u8 {
        *self.0
    }

    pub fn g(&self) -> u8 {
        *self.1
    }

    pub fn b(&self) -> u8 {
        *self.2
    }

    pub fn r_f32(&self) -> f32 {
        *self.0 as f32 / 255.0
    }

    pub fn g_f32(&self) -> f32 {
        *self.1 as f32 / 255.0
    }

    pub fn b_f32(&self) -> f32 {
        *self.2 as f32 / 255.0
    }
}
impl TryFrom<Srgb> for Rgb {
    type Error = ColorError;

    fn try_from(color: Srgb) -> ColorResult<Self> {
        if !color.r.in_bounds() {
            return Err(ColorError::OutOfSrgbGamut("r", *color.r));
        }

        if !color.g.in_bounds() {
            return Err(ColorError::OutOfSrgbGamut("g", *color.g));
        }

        if !color.b.in_bounds() {
            return Err(ColorError::OutOfSrgbGamut("b", *color.b));
        }

        Ok(Self::new(
            (*color.r * 255.0).round() as u8,
            (*color.g * 255.0).round() as u8,
            (*color.b * 255.0).round() as u8,
        ))
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({} {} {})", self.r(), self.g(), self.b())
    }
}
impl fmt::LowerHex for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
        } else {
            write!(f, "{:02x}{:02x}{:02x}", self.0, self.1, self.2)
        }
    }
}

impl fmt::UpperHex for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
        } else {
            write!(f, "{:02X}{:02X}{:02X}", self.0, self.1, self.2)
        }
    }
}

crate::color::formatting::impl_display!(
    Srgb, "color(srgb ", |color| [
        *color.r => "",
        *color.g => "",
        *color.b => ""
    ]
);

impl fmt::Display for crate::color::Alpha<Rgb> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color = self.color();
        write!(f, "rgb({} {} {}", color.r(), color.g(), color.b())?;
        crate::color::formatting::finish(f, Some(self.alpha()))
    }
}

impl fmt::LowerHex for Srgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.rgb_u8_clamped(), f)
    }
}

impl fmt::UpperHex for Srgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.rgb_u8_clamped(), f)
    }
}

impl Srgb {
    fn rgb_u8_clamped(&self) -> Rgb {
        Rgb::new(
            self.r_u8_clamped(),
            self.g_u8_clamped(),
            self.b_u8_clamped(),
        )
    }
}

impl fmt::LowerHex for crate::color::Alpha<Rgb> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(self.color(), f)?;
        write!(f, "{:02x}", self.alpha_u8())
    }
}

impl fmt::UpperHex for crate::color::Alpha<Rgb> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(self.color(), f)?;
        write!(f, "{:02X}", self.alpha_u8())
    }
}

impl fmt::LowerHex for crate::color::Alpha<Srgb> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(self.color(), f)?;
        write!(f, "{:02x}", self.alpha_u8())
    }
}

impl fmt::UpperHex for crate::color::Alpha<Srgb> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(self.color(), f)?;
        write!(f, "{:02X}", self.alpha_u8())
    }
}

impl Clamp for Rgb {
    fn clamp(mut self) -> Self {
        self.0 = self.0.clamp();
        self.1 = self.1.clamp();
        self.2 = self.2.clamp();
        self
    }
}

impl Eq for Rgb {}

impl std::hash::Hash for Rgb {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&(self.r(), self.g(), self.b()), state);
    }
}

impl ColorSpace for Rgb {
    fn try_from_linear_srgb_clamped(color: LinearSrgb) -> ColorResult<Self> {
        Self::try_from(Srgb::from(color).clamp())
    }

    fn try_into_linear_srgb_raw(self) -> ColorResult<LinearSrgb> {
        LinearSrgb::try_from(Srgb::from(self))
    }

    fn try_from_linear_srgb_raw(color: LinearSrgb) -> ColorResult<Self> {
        Self::try_from(Srgb::from(color))
    }
}

impl TryFrom<LinearSrgb> for Rgb {
    type Error = ColorError;
    fn try_from(color: LinearSrgb) -> ColorResult<Self> {
        Self::try_from_linear_srgb_raw(color)
    }
}

impl TryFrom<Rgb> for LinearSrgb {
    type Error = ColorError;
    fn try_from(color: Rgb) -> ColorResult<Self> {
        color.try_into_linear_srgb_raw()
    }
}
