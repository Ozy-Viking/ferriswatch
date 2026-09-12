//! Catppuccin Macchiato. Pinned palette source is listed in the registration.

define_palette! {
    Macchiato, "Catppuccin Macchiato",
    identity("catppuccin/macchiato", "catppuccin", "Catppuccin", "macchiato", "Macchiato", Dark, None);
    sources: crate::palette::sources::CATPPUCCIN;
    colors {
        ROSEWATER = crate::color::Color::hex(0xf4dbd6),
        FLAMINGO = crate::color::Color::hex(0xf0c6c6),
        PINK = crate::color::Color::hex(0xf5bde6),
        MAUVE = crate::color::Color::hex(0xc6a0f6),
        RED = crate::color::Color::hex(0xed8796),
        MAROON = crate::color::Color::hex(0xee99a0),
        PEACH = crate::color::Color::hex(0xf5a97f),
        YELLOW = crate::color::Color::hex(0xeed49f),
        GREEN = crate::color::Color::hex(0xa6da95),
        TEAL = crate::color::Color::hex(0x8bd5ca),
        SKY = crate::color::Color::hex(0x91d7e3),
        SAPPHIRE = crate::color::Color::hex(0x7dc4e4),
        BLUE = crate::color::Color::hex(0x8aadf4),
        LAVENDER = crate::color::Color::hex(0xb7bdf8),
        TEXT = crate::color::Color::hex(0xcad3f5),
        SUBTEXT_1 = crate::color::Color::hex(0xb8c0e0),
        SUBTEXT_0 = crate::color::Color::hex(0xa5adcb),
        OVERLAY_2 = crate::color::Color::hex(0x939ab7),
        OVERLAY_1 = crate::color::Color::hex(0x8087a2),
        OVERLAY_0 = crate::color::Color::hex(0x6e738d),
        SURFACE_2 = crate::color::Color::hex(0x5b6078),
        SURFACE_1 = crate::color::Color::hex(0x494d64),
        SURFACE_0 = crate::color::Color::hex(0x363a4f),
        BASE = crate::color::Color::hex(0x24273a),
        MANTLE = crate::color::Color::hex(0x1e2030),
        CRUST = crate::color::Color::hex(0x181926),
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
