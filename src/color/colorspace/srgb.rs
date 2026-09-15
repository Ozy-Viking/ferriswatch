use crate::color::Channel;

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
    pub(super) r: Channel<f32>,
    /// Green channel, nominally `0.0..=1.0`; must be finite.
    pub(super) g: Channel<f32>,
    /// Blue channel, nominally `0.0..=1.0`; must be finite.
    pub(super) b: Channel<f32>,
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
            r: Channel::color_channel("r", r, 0.0..=1.0),
            g: Channel::color_channel("g", g, 0.0..=1.0),
            b: Channel::color_channel("b", b, 0.0..=1.0),
        })
    }

    /// Borrows the r channel and its bounds.

    pub fn r_channel(&self) -> &Channel<f32> {

        &self.r
    }

    pub fn r(&self) -> f32 {

        *self.r
    }

    /// Borrows the g channel and its bounds.

    pub fn g_channel(&self) -> &Channel<f32> {

        &self.g
    }

    pub fn g(&self) -> f32 {

        *self.g
    }

    /// Borrows the b channel and its bounds.

    pub fn b_channel(&self) -> &Channel<f32> {

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
            r: Channel::color_channel("r", linear_to_srgb(color.r()), 0.0..=1.0),
            g: Channel::color_channel("g", linear_to_srgb(color.g()), 0.0..=1.0),
            b: Channel::color_channel("b", linear_to_srgb(color.b()), 0.0..=1.0),
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
    pub Channel<u8>,
    /// Green channel in `0..=255`.
    pub Channel<u8>,
    /// Blue channel in `0..=255`.
    pub Channel<u8>,
);

impl Rgb {
    /// Creates an encoded sRGB color from a packed `0xRRGGBB` integer.
    ///
    /// # Panics
    /// Panics if the value exceeds `0xFFFFFF`; alpha bytes are not accepted.
    ///
    /// ```
    /// use ferriswatch::color::Rgb;
    /// const ROSEWATER: Rgb = Rgb::hex(0xf5e0dc);
    /// assert_eq!(ROSEWATER.to_hex(), "#F5E0DC");
    /// ```

    pub const fn from_hex(value: u32) -> Self {

        assert!(value <= 0xFFFFFF, "RGB hex value must fit in 24 bits");

        Self::new((value >> 16) as u8, (value >> 8) as u8, value as u8)
    }

    /// Shorthand for [`Self::from_hex`], including its 24-bit input requirement.

    pub const fn hex(value: u32) -> Self {

        Self::from_hex(value)
    }

    /// Parses three or six ASCII hex digits, with an optional leading `#`.
    ///
    /// Letter case is ignored. Three digits expand by repeating each digit.
    /// Whitespace, `0x` prefixes, and alpha components are not accepted.
    ///
    /// # Errors
    /// Returns [`ColorError::InvalidLength`] for other digit counts, or
    /// [`ColorError::InvalidHex`] for non-hex characters.

    pub fn from_hex_str(value: &str) -> ColorResult<Self> {

        let [r, g, b, _] = parse_hex(value, false)?;

        Ok(Self::new(r, g, b))
    }

    /// Returns uppercase CSS hex (`#RRGGBB`), like [`Self::to_upper_hex`].

    pub fn to_hex(&self) -> String {

        self.to_upper_hex()
    }

    /// Returns lowercase CSS hex (`#rrggbb`), with two digits per channel.

    pub fn to_lower_hex(&self) -> String {

        format!("{self:#x}")
    }

    /// Returns uppercase CSS hex (`#RRGGBB`), with two digits per channel.

    pub fn to_upper_hex(&self) -> String {

        format!("{self:#X}")
    }

    /// Creates byte channels with names and inclusive bounds `0..=255`.

    pub const fn new(r: u8, g: u8, b: u8) -> Self {

        Self(
            Channel::byte_color_channel("r", r),
            Channel::byte_color_channel("g", g),
            Channel::byte_color_channel("b", b),
        )
    }

    /// Borrows the red channel and its bounds.

    pub fn r_channel(&self) -> &Channel<u8> {

        &self.0
    }

    /// Borrows the green channel and its bounds.

    pub fn g_channel(&self) -> &Channel<u8> {

        &self.1
    }

    /// Borrows the blue channel and its bounds.

    pub fn b_channel(&self) -> &Channel<u8> {

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

        Ok(LinearSrgb::from(self))
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

impl From<Rgb> for LinearSrgb {
    /// Decodes byte channels to linear light without clipping or quantization.

    fn from(color: Rgb) -> Self {

        Self {
            r: Channel::color_channel("r", srgb_to_linear(color.r_f32()), 0.0..=1.0),
            g: Channel::color_channel("g", srgb_to_linear(color.g_f32()), 0.0..=1.0),
            b: Channel::color_channel("b", srgb_to_linear(color.b_f32()), 0.0..=1.0),
        }
    }
}

impl std::str::FromStr for Rgb {
    type Err = ColorError;

    fn from_str(value: &str) -> ColorResult<Self> {

        Self::from_hex_str(value)
    }
}

impl crate::color::Alpha<Rgb> {
    /// Creates an sRGB color with alpha from packed `0xRRGGBBAA` bytes.
    ///
    /// The low byte is always alpha, including when leading RGB bytes are zero.
    ///
    /// ```
    /// use ferriswatch::color::Rgba;
    /// const COLOR: Rgba = Rgba::from_hex(0xf5e0dc80);
    /// assert_eq!(format!("{COLOR:#X}"), "#F5E0DC80");
    /// ```

    pub const fn from_hex(value: u32) -> Self {

        Self {
            color: Rgb::from_hex(value >> 8),
            alpha: Channel::alpha_byte_channel(value as u8),
        }
    }

    /// Shorthand for [`Self::from_hex`], using `0xRRGGBBAA` order.

    pub const fn hex(value: u32) -> Self {

        Self::from_hex(value)
    }

    /// Parses RGB or RGBA hex, with an optional leading `#`.
    ///
    /// Accepts 3, 4, 6, or 8 ASCII hex digits. Short forms repeat each digit;
    /// RGB forms default to opaque alpha. RGBA forms place alpha last.
    /// Whitespace and `0x` prefixes are not accepted.
    ///
    /// # Errors
    /// Returns [`ColorError::InvalidLength`] for other digit counts, or
    /// [`ColorError::InvalidHex`] for non-hex characters.

    pub fn from_hex_str(value: &str) -> ColorResult<Self> {

        let [r, g, b, a] = parse_hex(value, true)?;

        Ok(Self::from_hex(u32::from_be_bytes([r, g, b, a])))
    }
}

impl std::str::FromStr for crate::color::Alpha<Rgb> {
    type Err = ColorError;

    fn from_str(value: &str) -> ColorResult<Self> {

        Self::from_hex_str(value)
    }
}

fn parse_hex(value: &str, allow_alpha: bool) -> ColorResult<[u8; 4]> {

    let digits = value.strip_prefix('#').unwrap_or(value).as_bytes();

    let (channels, step) = match digits.len() {
        3 => (3, 1),
        6 => (3, 2),
        4 if allow_alpha => (4, 1),
        8 if allow_alpha => (4, 2),
        length => return Err(ColorError::InvalidLength(length)),
    };

    let nibble = |byte: u8| -> ColorResult<u8> {

        match byte {
            b'0'..=b'9' => Ok(byte - b'0'),
            b'a'..=b'f' => Ok(byte - b'a' + 10),
            b'A'..=b'F' => Ok(byte - b'A' + 10),
            _ => Err(ColorError::InvalidHex),
        }
    };

    let mut result = [255; 4];

    for (channel, digits) in result[..channels].iter_mut().zip(digits.chunks_exact(step)) {

        *channel = if step == 1 {

            nibble(digits[0])? * 17
        } else {

            nibble(digits[0])? * 16 + nibble(digits[1])?
        };
    }

    Ok(result)
}
