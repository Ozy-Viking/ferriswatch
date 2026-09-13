//! Catppuccin Mocha. Pinned palette source is listed in the registration.

define_palette! {
    Mocha, "Catppuccin Mocha",
    identity("catppuccin/mocha", "catppuccin", "Catppuccin", "mocha", "Mocha", Dark, None);
    sources: super::SOURCE;
    colors {
        ROSEWATER = crate::color::Color::hex(0xf5e0dc),
        FLAMINGO = crate::color::Color::hex(0xf2cdcd),
        PINK = crate::color::Color::hex(0xf5c2e7),
        MAUVE = crate::color::Color::hex(0xcba6f7),
        RED = crate::color::Color::hex(0xf38ba8),
        MAROON = crate::color::Color::hex(0xeba0ac),
        PEACH = crate::color::Color::hex(0xfab387),
        YELLOW = crate::color::Color::hex(0xf9e2af),
        GREEN = crate::color::Color::hex(0xa6e3a1),
        TEAL = crate::color::Color::hex(0x94e2d5),
        SKY = crate::color::Color::hex(0x89dceb),
        SAPPHIRE = crate::color::Color::hex(0x74c7ec),
        BLUE = crate::color::Color::hex(0x89b4fa),
        LAVENDER = crate::color::Color::hex(0xb4befe),
        TEXT = crate::color::Color::hex(0xcdd6f4),
        SUBTEXT_1 = crate::color::Color::hex(0xbac2de),
        SUBTEXT_0 = crate::color::Color::hex(0xa6adc8),
        OVERLAY_2 = crate::color::Color::hex(0x9399b2),
        OVERLAY_1 = crate::color::Color::hex(0x7f849c),
        OVERLAY_0 = crate::color::Color::hex(0x6c7086),
        SURFACE_2 = crate::color::Color::hex(0x585b70),
        SURFACE_1 = crate::color::Color::hex(0x45475a),
        SURFACE_0 = crate::color::Color::hex(0x313244),
        BASE = crate::color::Color::hex(0x1e1e2e),
        MANTLE = crate::color::Color::hex(0x181825),
        CRUST = crate::color::Color::hex(0x11111b),
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
