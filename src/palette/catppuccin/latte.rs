//! Catppuccin Latte. Pinned palette source is listed in the registration.

define_palette! {
    Latte, "Catppuccin Latte",
    identity("catppuccin/latte", "catppuccin", "Catppuccin", "latte", "Latte", Light, None);
    sources: super::SOURCE;
    colors {
        ROSEWATER = crate::color::Color::hex(0xdc8a78),
        FLAMINGO = crate::color::Color::hex(0xdd7878),
        PINK = crate::color::Color::hex(0xea76cb),
        MAUVE = crate::color::Color::hex(0x8839ef),
        RED = crate::color::Color::hex(0xd20f39),
        MAROON = crate::color::Color::hex(0xe64553),
        PEACH = crate::color::Color::hex(0xfe640b),
        YELLOW = crate::color::Color::hex(0xdf8e1d),
        GREEN = crate::color::Color::hex(0x40a02b),
        TEAL = crate::color::Color::hex(0x179299),
        SKY = crate::color::Color::hex(0x04a5e5),
        SAPPHIRE = crate::color::Color::hex(0x209fb5),
        BLUE = crate::color::Color::hex(0x1e66f5),
        LAVENDER = crate::color::Color::hex(0x7287fd),
        TEXT = crate::color::Color::hex(0x4c4f69),
        SUBTEXT_1 = crate::color::Color::hex(0x5c5f77),
        SUBTEXT_0 = crate::color::Color::hex(0x6c6f85),
        OVERLAY_2 = crate::color::Color::hex(0x7c7f93),
        OVERLAY_1 = crate::color::Color::hex(0x8c8fa1),
        OVERLAY_0 = crate::color::Color::hex(0x9ca0b0),
        SURFACE_2 = crate::color::Color::hex(0xacb0be),
        SURFACE_1 = crate::color::Color::hex(0xbcc0cc),
        SURFACE_0 = crate::color::Color::hex(0xccd0da),
        BASE = crate::color::Color::hex(0xeff1f5),
        MANTLE = crate::color::Color::hex(0xe6e9ef),
        CRUST = crate::color::Color::hex(0xdce0e8),
    }
    accents {
        Rosewater = ROSEWATER => ("rosewater", "Rosewater"),
        Flamingo = FLAMINGO => ("flamingo", "Flamingo"),
        Pink = PINK => ("pink", "Pink"),
        Mauve = MAUVE => ("mauve", "Mauve"),
        Red = RED => ("red", "Red"),
        Maroon = MAROON => ("maroon", "Maroon"),
        Peach = PEACH => ("peach", "Peach"),
        Yellow = YELLOW => ("yellow", "Yellow"),
        Green = GREEN => ("green", "Green"),
        Teal = TEAL => ("teal", "Teal"),
        Sky = SKY => ("sky", "Sky"),
        Sapphire = SAPPHIRE => ("sapphire", "Sapphire"),
        Blue = BLUE => ("blue", "Blue"),
        Lavender = LAVENDER => ("lavender", "Lavender"),
    }
    default MAUVE => "mauve";
    roles(primary) {
        surface: crate::theme_variant::SurfaceColors {
            background: Self::BASE,
            surface: Self::SURFACE_0,
            raised: Self::SURFACE_2,
            overlay: Self::OVERLAY_0,
            hover: Self::SURFACE_1,
        },
        surface_alt: crate::theme_variant::SurfaceColors {
            background: Self::CRUST,
            surface: Self::MANTLE,
            raised: Self::SURFACE_0,
            overlay: Self::BASE,
            hover: Self::SURFACE_1,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::TEXT,
            muted: Self::SUBTEXT_1,
            subtle: Self::OVERLAY_1,
            on_primary: crate::palette::action_text(primary),
            on_secondary: crate::palette::action_text(Self::BLUE),
        },
        primary: crate::theme_variant::ActionColors {
            normal: primary,
            hover: crate::palette::primary_hover(primary),
            pressed: crate::palette::action_pressed(primary),
            muted: Self::SURFACE_2,
        },
        secondary: crate::theme_variant::ActionColors {
            normal: Self::BLUE,
            hover: crate::palette::primary_hover(Self::BLUE),
            pressed: crate::palette::action_pressed(Self::BLUE),
            muted: Self::SURFACE_0,
        },
        status: crate::theme_variant::StatusColors {
            success: Self::GREEN,
            warning: Self::YELLOW,
            error: Self::RED,
            critical: Self::RED,
            info: Self::TEAL,
            trace: Self::SUBTEXT_0,
        },
        border: Self::OVERLAY_0,
        border_muted: Self::SURFACE_1,
        focus: primary,
    }
}
