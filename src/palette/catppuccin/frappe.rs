//! Catppuccin Frappé. Pinned palette source is listed in the registration.

define_palette! {
    Frappe, "Catppuccin Frappé",
    identity("catppuccin/frappe", "catppuccin", "Catppuccin", "frappe", "Frappé", Dark, None);
    support(Dark);
    sources: super::SOURCE;
    colors {
        ROSEWATER = crate::color::Color::hex(0xf2d5cf),
        FLAMINGO = crate::color::Color::hex(0xeebebe),
        PINK = crate::color::Color::hex(0xf4b8e4),
        MAUVE = crate::color::Color::hex(0xca9ee6),
        RED = crate::color::Color::hex(0xe78284),
        MAROON = crate::color::Color::hex(0xea999c),
        PEACH = crate::color::Color::hex(0xef9f76),
        YELLOW = crate::color::Color::hex(0xe5c890),
        GREEN = crate::color::Color::hex(0xa6d189),
        TEAL = crate::color::Color::hex(0x81c8be),
        SKY = crate::color::Color::hex(0x99d1db),
        SAPPHIRE = crate::color::Color::hex(0x85c1dc),
        BLUE = crate::color::Color::hex(0x8caaee),
        LAVENDER = crate::color::Color::hex(0xbabbf1),
        TEXT = crate::color::Color::hex(0xc6d0f5),
        SUBTEXT_1 = crate::color::Color::hex(0xb5bfe2),
        SUBTEXT_0 = crate::color::Color::hex(0xa5adce),
        OVERLAY_2 = crate::color::Color::hex(0x949cbb),
        OVERLAY_1 = crate::color::Color::hex(0x838ba7),
        OVERLAY_0 = crate::color::Color::hex(0x737994),
        SURFACE_2 = crate::color::Color::hex(0x626880),
        SURFACE_1 = crate::color::Color::hex(0x51576d),
        SURFACE_0 = crate::color::Color::hex(0x414559),
        BASE = crate::color::Color::hex(0x303446),
        MANTLE = crate::color::Color::hex(0x292c3c),
        CRUST = crate::color::Color::hex(0x232634),
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
