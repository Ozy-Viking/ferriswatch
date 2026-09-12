use crate::color::Color;
use crate::palette::{Accent, Palette};
use crate::theme_variant::{ThemePalette, ThemeVariant, ThemeVariantColors};

/// Catppuccin Frappé palette.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Frappe;

impl Frappe {
    pub const ROSEWATER: Color = Color::hex(0xf2d5cf);
    pub const FLAMINGO: Color = Color::hex(0xeebebe);
    pub const PINK: Color = Color::hex(0xf4b8e4);
    pub const MAUVE: Color = Color::hex(0xca9ee6);
    pub const RED: Color = Color::hex(0xe78284);
    pub const MAROON: Color = Color::hex(0xea999c);
    pub const PEACH: Color = Color::hex(0xef9f76);
    pub const YELLOW: Color = Color::hex(0xe5c890);
    pub const GREEN: Color = Color::hex(0xa6d189);
    pub const TEAL: Color = Color::hex(0x81c8be);
    pub const SKY: Color = Color::hex(0x99d1db);
    pub const SAPPHIRE: Color = Color::hex(0x85c1dc);
    pub const BLUE: Color = Color::hex(0x8caaee);
    pub const LAVENDER: Color = Color::hex(0xbabbf1);
    pub const TEXT: Color = Color::hex(0xc6d0f5);
    pub const SUBTEXT_1: Color = Color::hex(0xb5bfe2);
    pub const SUBTEXT_0: Color = Color::hex(0xa5adce);
    pub const OVERLAY_2: Color = Color::hex(0x949cbb);
    pub const OVERLAY_1: Color = Color::hex(0x838ba7);
    pub const OVERLAY_0: Color = Color::hex(0x737994);
    pub const SURFACE_2: Color = Color::hex(0x626880);
    pub const SURFACE_1: Color = Color::hex(0x51576d);
    pub const SURFACE_0: Color = Color::hex(0x414559);
    pub const BASE: Color = Color::hex(0x303446);
    pub const MANTLE: Color = Color::hex(0x292c3c);
    pub const CRUST: Color = Color::hex(0x232634);
}

catppuccin_variant!(Frappe, "Catppuccin Frappé");
