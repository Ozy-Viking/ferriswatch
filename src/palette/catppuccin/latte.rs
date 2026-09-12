use crate::color::Color;
use crate::palette::{Accent, Palette};
use crate::theme_variant::{ThemePalette, ThemeVariant, ThemeVariantColors};

/// Catppuccin Latte palette.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Latte;

impl Latte {
    pub const ROSEWATER: Color = Color::hex(0xdc8a78);
    pub const FLAMINGO: Color = Color::hex(0xdd7878);
    pub const PINK: Color = Color::hex(0xea76cb);
    pub const MAUVE: Color = Color::hex(0x8839ef);
    pub const RED: Color = Color::hex(0xd20f39);
    pub const MAROON: Color = Color::hex(0xe64553);
    pub const PEACH: Color = Color::hex(0xfe640b);
    pub const YELLOW: Color = Color::hex(0xdf8e1d);
    pub const GREEN: Color = Color::hex(0x40a02b);
    pub const TEAL: Color = Color::hex(0x179299);
    pub const SKY: Color = Color::hex(0x04a5e5);
    pub const SAPPHIRE: Color = Color::hex(0x209fb5);
    pub const BLUE: Color = Color::hex(0x1e66f5);
    pub const LAVENDER: Color = Color::hex(0x7287fd);
    pub const TEXT: Color = Color::hex(0x4c4f69);
    pub const SUBTEXT_1: Color = Color::hex(0x5c5f77);
    pub const SUBTEXT_0: Color = Color::hex(0x6c6f85);
    pub const OVERLAY_2: Color = Color::hex(0x7c7f93);
    pub const OVERLAY_1: Color = Color::hex(0x8c8fa1);
    pub const OVERLAY_0: Color = Color::hex(0x9ca0b0);
    pub const SURFACE_2: Color = Color::hex(0xacb0be);
    pub const SURFACE_1: Color = Color::hex(0xbcc0cc);
    pub const SURFACE_0: Color = Color::hex(0xccd0da);
    pub const BASE: Color = Color::hex(0xeff1f5);
    pub const MANTLE: Color = Color::hex(0xe6e9ef);
    pub const CRUST: Color = Color::hex(0xdce0e8);
}

catppuccin_variant!(Latte, "Catppuccin Latte");
