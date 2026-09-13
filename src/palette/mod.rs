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

pub mod ayu;
pub mod base16;
pub mod cobalt2;
pub mod dracula;
pub mod github;
pub mod horizon;
pub mod jetbrains;
pub mod material;
pub mod monokai;
pub mod night_owl;
pub mod nightfox;
pub mod nord;
pub mod oceanic_next;
pub mod one;
pub mod panda;
pub mod poimandres;
pub mod quiet_light;
pub mod shades_of_purple;
pub mod solarized;
pub mod synthwave_84;
pub mod tomorrow;
pub mod vscode;

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
///
/// Implementations must supply `ACCENT`, `ID`, and `NAME` together, or make all
/// three absent. IDs are ASCII snake_case and names must be nonempty.
/// Built-in factories panic if a custom implementation violates this contract.
pub trait Accent<P: Palette> {
    /// `None` lets the palette choose its default semantic colours.
    const ACCENT: Option<Color>;

    /// Stable ASCII snake_case ID. All metadata is absent for `NoAccent`.
    const ID: Option<&'static str>;

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
    const ID: Option<&'static str> = None;
    const NAME: Option<&'static str> = None;
}

// Choose the black/white foreground with the best worst-state contrast.
// Color channels are already linear sRGB, including values made by Color::hex.
// Decoding them as sRGB again would calculate the wrong luminance.
// Derived states darken RGB, so normal and pressed bound the luminance range.
// Alpha is retained; applications must also check compositing over their own canvas.
fn action_text(color: Color) -> Color {
    let luminance = 0.2126 * color.r().clamp(0.0, 1.0)
        + 0.7152 * color.g().clamp(0.0, 1.0)
        + 0.0722 * color.b().clamp(0.0, 1.0);
    let black_worst = (luminance * 0.70 + 0.05) / 0.05;
    let white_worst = 1.05 / (luminance + 0.05);
    if black_worst >= white_worst {
        Color::hex(0x000000)
    } else {
        Color::hex(0xffffff)
    }
}

// Accent implementations must supply either all metadata or none.
fn selected_accent<P: Palette, A: Accent<P>>() -> Option<crate::theme_variant::ResolvedAccent> {
    match (A::ACCENT, A::ID, A::NAME) {
        (None, None, None) => None,
        (Some(color), Some(id), Some(name)) => Some(
            crate::theme_variant::ResolvedAccent::new(id, name, color)
                .expect("Accent IDs must be snake_case and names nonempty"),
        ),
        _ => panic!("Accent must supply colour, ID and name together"),
    }
}

/// Pressed actions retain their hue and alpha, with a stronger darkening than hover.
fn action_pressed(color: Color) -> Color {
    Color::new(
        color.r() * 0.70,
        color.g() * 0.70,
        color.b() * 0.70,
        color.a(),
    )
    .expect("scaling valid colour channels preserves validity")
}
