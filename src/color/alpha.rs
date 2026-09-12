use crate::color::{ColorChannel, channel::color_channel};

use crate::color::{Clamp, ColorError, ColorResult, ColorSpace, LinearSrgb, floats_eq};

/// A color paired with a separate, unpremultiplied alpha channel.
///
/// The color retains the channel ranges of `C`. Alpha is a finite fraction
/// in `0.0..=1.0`, with zero fully transparent and one fully opaque.
/// Constructors and alpha setters reject invalid floating-point alpha values.
///
/// `Display` for the built-in color types inserts `/ alpha` before the closing
/// parenthesis and uses the same precision as the color channels. LMS output
/// remains descriptive text rather than CSS. For [`crate::color::Rgb`] and
/// [`crate::color::Srgb`], hex formatting appends a rounded 8-bit alpha byte,
/// giving RRGGBBAA; alternate hex formatting includes the `#` prefix.
///
/// # Examples
///
/// ```
/// use ferriswatch::color::{Alpha, Rgb};
///
/// let color = Alpha::new(Rgb::new(255, 128, 0), 0.5)?;
/// assert_eq!(color.alpha(), 0.5);
/// assert_eq!(*color.color(), Rgb::new(255, 128, 0));
/// # Ok::<(), ferriswatch::color::ColorError>(())
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Alpha<C> {
    /// Color channels in the ranges defined by `C`.
    pub(super) color: C,
    /// Opacity, finite and in `0.0..=1.0`.
    pub(super) alpha: ColorChannel,
}

impl<C> Alpha<C> {
    pub fn new(color: C, alpha: f32) -> ColorResult<Self> {
        if !alpha.is_finite() || !(0.0..=1.0).contains(&alpha) {
            return Err(ColorError::InvalidAlpha(alpha));
        }
        Ok(Self {
            color,
            alpha: color_channel("alpha", alpha, 0.0..=1.0),
        })
    }
    pub fn color(&self) -> &C {
        &self.color
    }

    pub fn color_mut(&mut self) -> &mut C {
        &mut self.color
    }

    pub fn set_color(&mut self, color: C) {
        self.color = color;
    }

    pub fn alpha_u8(&self) -> u8 {
        (*self.alpha.clamp() * 255.0).round() as u8
    }

    pub fn set_alpha_u8(&mut self, alpha: u8) -> &mut Self {
        *self.alpha = (alpha as f32) / 255.0;
        self
    }

    /// Borrows the opacity channel and its bounds.
    pub fn alpha_channel(&self) -> &ColorChannel {
        &self.alpha
    }

    pub fn alpha(&self) -> f32 {
        *self.alpha
    }

    pub fn set_alpha(&mut self, alpha: f32) -> ColorResult<&mut Self> {
        if !alpha.is_finite() || !(0.0..=1.0).contains(&alpha) {
            return Err(ColorError::InvalidAlpha(alpha));
        }
        *self.alpha = alpha;
        Ok(self)
    }
    pub fn with_color(mut self, color: C) -> Self {
        self.color = color;
        self
    }

    pub fn with_alpha(mut self, alpha: f32) -> ColorResult<Self> {
        self.set_alpha(alpha)?;
        Ok(self)
    }

    pub fn with_alpha_u8(mut self, alpha: u8) -> Self {
        self.set_alpha_u8(alpha);
        self
    }
    pub fn opaque(color: C) -> Self {
        Self {
            color,
            alpha: color_channel("alpha", 1.0, 0.0..=1.0),
        }
    }
}
impl<C: PartialEq> PartialEq for Alpha<C> {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && floats_eq(&self.alpha, &other.alpha)
    }
}

impl<C: Eq> Eq for Alpha<C> {}

impl<C: Clamp> Clamp for Alpha<C> {
    fn clamp(mut self) -> Self {
        self.color = self.color.clamp();
        self.alpha = self.alpha.clamp();
        self
    }
}

impl<C> AsMut<C> for Alpha<C>
where
    C: TryFrom<LinearSrgb> + Clamp + ColorSpace,
    LinearSrgb: TryFrom<C>,
{
    fn as_mut(&mut self) -> &mut C {
        &mut self.color
    }
}

impl<C> AsRef<C> for Alpha<C>
where
    C: TryFrom<LinearSrgb> + Clamp + ColorSpace,
    LinearSrgb: TryFrom<C>,
{
    fn as_ref(&self) -> &C {
        self.color()
    }
}

impl<C> AsRef<f32> for Alpha<C> {
    fn as_ref(&self) -> &f32 {
        &self.alpha
    }
}
