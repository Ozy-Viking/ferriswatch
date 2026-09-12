use crate::color::Color;
mod catppuccin;

pub trait Palette {}

pub trait ScaledPalette: Palette {}
pub trait NamedPalette: Palette {
    fn color(&self, name: &str) -> Option<Color>;
}

pub trait Accent<P> {
    const ACCENT: Color;
    fn accent(&self) -> Color {
        Self::ACCENT
    }
}

pub struct NoAccent;
impl<P> Accent<P> for NoAccent {
    const ACCENT: Color = Color::TRANSPARENT;
}
