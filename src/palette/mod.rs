//! Raw palettes and their supported accent choices.
//!
#![doc = include_str!("../../docs/Palettes.md")]
use crate::color::Color;
#[macro_use]
mod macros;

pub mod catppuccin;
pub mod everforest;
pub mod gruvbox;
pub mod kanagawa;
pub mod rose_pine;
pub mod tokyo_night;

/// Darkens linear RGB by 15% while retaining the accent's alpha.
fn primary_hover(accent: Color) -> Color {
    Color::new(
        accent.r() * 0.85,
        accent.g() * 0.85,
        accent.b() * 0.85,
        accent.a(),
    )
    .expect("scaling valid colour channels preserves validity")
}

/// A collection of raw colours that can back a theme.
pub trait Palette {}

/// An accent choice supported by palette `P`.
pub trait Accent<P: Palette> {
    /// `None` lets the palette choose its default semantic colours.
    const ACCENT: Option<Color>;

    /// Display name of the explicit accent, or `None` when no accent is selected.
    const NAME: Option<&'static str>;

    fn accent(&self) -> Option<Color> {
        Self::ACCENT
    }
}

/// Use the palette's default semantic colours without an explicit accent.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NoAccent;
impl<P: Palette> Accent<P> for NoAccent {
    const ACCENT: Option<Color> = None;
    const NAME: Option<&'static str> = None;
}

// Choose the higher-contrast black/white foreground for the action's RGB.
// Transparent fills still depend on the application's composited background.
fn action_text(color: Color) -> Color {
    let luminance = 0.2126 * color.r().clamp(0.0, 1.0)
        + 0.7152 * color.g().clamp(0.0, 1.0)
        + 0.0722 * color.b().clamp(0.0, 1.0);
    if luminance > 0.17912878 {
        Color::hex(0x000000)
    } else {
        Color::hex(0xffffff)
    }
}
