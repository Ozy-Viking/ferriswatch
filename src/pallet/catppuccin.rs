use crate::color::{Color, Rgb};

pub struct Mocha;

impl Mocha {
    pub const ROSEWATER: Color = Color::from(Rgb::from_hex(0xf5e0dc));
    pub const FLAMINGO: Rgb = Rgb::from_hex(0xf2cdcd);
    pub const PINK: Rgb = Rgb::from_hex(0xf5c2e7);
    pub const MAUVE: Rgb = Rgb::from_hex(0xcba6f7);
    pub const RED: Rgb = Rgb::from_hex(0xf38ba8);
    pub const MAROON: Rgb = Rgb::from_hex(0xeba0ac);
    pub const PEACH: Rgb = Rgb::from_hex(0xfab387);
    pub const YELLOW: Rgb = Rgb::from_hex(0xf9e2af);
    pub const GREEN: Rgb = Rgb::from_hex(0xa6e3a1);
    pub const TEAL: Rgb = Rgb::from_hex(0x94e2d5);
    pub const SKY: Rgb = Rgb::from_hex(0x89dceb);
    pub const SAPPHIRE: Rgb = Rgb::from_hex(0x74c7ec);
    pub const BLUE: Rgb = Rgb::from_hex(0x89b4fa);
    pub const LAVENDER: Rgb = Rgb::from_hex(0xb4befe);

    pub const TEXT: Rgb = Rgb::from_hex(0xcdd6f4);
    pub const SUBTEXT_1: Rgb = Rgb::from_hex(0xbac2de);
    pub const SUBTEXT_0: Rgb = Rgb::from_hex(0xa6adc8);

    pub const OVERLAY_2: Rgb = Rgb::from_hex(0x9399b2);
    pub const OVERLAY_1: Rgb = Rgb::from_hex(0x7f849c);
    pub const OVERLAY_0: Rgb = Rgb::from_hex(0x6c7086);

    pub const SURFACE_2: Rgb = Rgb::from_hex(0x585b70);
    pub const SURFACE_1: Rgb = Rgb::from_hex(0x45475a);
    pub const SURFACE_0: Rgb = Rgb::from_hex(0x313244);

    pub const BASE: Rgb = Rgb::from_hex(0x1e1e2e);
    pub const MANTLE: Rgb = Rgb::from_hex(0x181825);
    pub const CRUST: Rgb = Rgb::from_hex(0x11111b);
}
