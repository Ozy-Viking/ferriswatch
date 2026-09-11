use crate::color::{Clamp, ColorError, ColorResult, floats_eq};

/// A color paired with a separate, unpremultiplied alpha channel.
///
/// The color retains the channel ranges of `C`. Alpha is a finite fraction
/// in `0.0..=1.0`, with zero fully transparent and one fully opaque.
/// Constructors and alpha setters reject invalid floating-point alpha values.
///
/// # Examples
///
/// ```
/// use ferriswatch::color::{Alpha, Rgb};
///
/// let color = Alpha::new(Rgb(255, 128, 0), 0.5)?;
/// assert_eq!(color.alpha(), 0.5);
/// assert_eq!(*color.color(), Rgb(255, 128, 0));
/// # Ok::<(), ferriswatch::color::ColorError>(())
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Alpha<C> {
    /// Color channels in the ranges defined by `C`.
    pub(super) color: C,
    /// Opacity, finite and in `0.0..=1.0`.
    pub(super) alpha: f32,
}

impl<C> Alpha<C> {
    pub fn new(color: C, alpha: f32) -> ColorResult<Self> {
        if !alpha.is_finite() || !(0.0..=1.0).contains(&alpha) {
            return Err(ColorError::InvalidAlpha(alpha));
        }
        Ok(Self { color, alpha })
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
        (self.alpha.clamp(0.0, 1.0) * 255.0).round() as u8
    }

    pub fn set_alpha_u8(&mut self, alpha: u8) -> &mut Self {
        self.alpha = (alpha as f32) / 255.0;
        self
    }

    pub fn alpha(&self) -> f32 {
        self.alpha
    }

    pub fn set_alpha(&mut self, alpha: f32) -> ColorResult<&mut Self> {
        if !alpha.is_finite() || !(0.0..=1.0).contains(&alpha) {
            return Err(ColorError::InvalidAlpha(alpha));
        }
        self.alpha = alpha;
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
        Self { color, alpha: 1.0 }
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
        self.alpha = self.alpha.clamp(0.0, 1.0);
        self
    }
}
