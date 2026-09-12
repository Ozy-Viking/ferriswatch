use crate::color::Color;
use crate::palette::{Accent, Palette};
use crate::theme_variant::{ThemePalette, ThemeVariant, ThemeVariantColors};

/// Catppuccin Macchiato palette.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Macchiato;

impl Macchiato {
    pub const ROSEWATER: Color = Color::hex(0xf4dbd6);
    pub const FLAMINGO: Color = Color::hex(0xf0c6c6);
    pub const PINK: Color = Color::hex(0xf5bde6);
    pub const MAUVE: Color = Color::hex(0xc6a0f6);
    pub const RED: Color = Color::hex(0xed8796);
    pub const MAROON: Color = Color::hex(0xee99a0);
    pub const PEACH: Color = Color::hex(0xf5a97f);
    pub const YELLOW: Color = Color::hex(0xeed49f);
    pub const GREEN: Color = Color::hex(0xa6da95);
    pub const TEAL: Color = Color::hex(0x8bd5ca);
    pub const SKY: Color = Color::hex(0x91d7e3);
    pub const SAPPHIRE: Color = Color::hex(0x7dc4e4);
    pub const BLUE: Color = Color::hex(0x8aadf4);
    pub const LAVENDER: Color = Color::hex(0xb7bdf8);
    pub const TEXT: Color = Color::hex(0xcad3f5);
    pub const SUBTEXT_1: Color = Color::hex(0xb8c0e0);
    pub const SUBTEXT_0: Color = Color::hex(0xa5adcb);
    pub const OVERLAY_2: Color = Color::hex(0x939ab7);
    pub const OVERLAY_1: Color = Color::hex(0x8087a2);
    pub const OVERLAY_0: Color = Color::hex(0x6e738d);
    pub const SURFACE_2: Color = Color::hex(0x5b6078);
    pub const SURFACE_1: Color = Color::hex(0x494d64);
    pub const SURFACE_0: Color = Color::hex(0x363a4f);
    pub const BASE: Color = Color::hex(0x24273a);
    pub const MANTLE: Color = Color::hex(0x1e2030);
    pub const CRUST: Color = Color::hex(0x181926);
}

catppuccin_variant!(Macchiato, "Catppuccin Macchiato");
