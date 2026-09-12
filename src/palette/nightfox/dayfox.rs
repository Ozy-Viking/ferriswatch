//! Dayfox. See `REGISTRATION` for pinned upstream sources.
//!
//! Nightfox base shades and declared UI roles. bg0 remains the floating surface, sel0 is hover, and red serves both error levels. Generated bright/dim shades are outside the raw import.

define_palette! {
    Dayfox, "Dayfox",
    identity("nightfox/dayfox", "nightfox", "Nightfox", "dayfox", "Dayfox", Light, None);
    sources: crate::palette::sources::NIGHTFOX_DAYFOX;
    colors {
        BLACK = crate::color::Color::hex(0x352c24),
        RED = crate::color::Color::hex(0xa5222f),
        GREEN = crate::color::Color::hex(0x396847),
        YELLOW = crate::color::Color::hex(0xac5402),
        BLUE = crate::color::Color::hex(0x2848a9),
        MAGENTA = crate::color::Color::hex(0x6e33ce),
        CYAN = crate::color::Color::hex(0x287980),
        WHITE = crate::color::Color::hex(0xf2e9e1),
        ORANGE = crate::color::Color::hex(0x955f61),
        PINK = crate::color::Color::hex(0xa440b5),
        COMMENT = crate::color::Color::hex(0x837a72),
        BG_0 = crate::color::Color::hex(0xe4dcd4),
        BG_1 = crate::color::Color::hex(0xf6f2ee),
        BG_2 = crate::color::Color::hex(0xdbd1dd),
        BG_3 = crate::color::Color::hex(0xd3c7bb),
        BG_4 = crate::color::Color::hex(0xaab0ad),
        FG_0 = crate::color::Color::hex(0x302b5d),
        FG_1 = crate::color::Color::hex(0x3d2b5a),
        FG_2 = crate::color::Color::hex(0x643f61),
        FG_3 = crate::color::Color::hex(0x824d5b),
        SEL_0 = crate::color::Color::hex(0xe7d2be),
        SEL_1 = crate::color::Color::hex(0xa4c1c2),
    }
    accents {
        Red = RED => ("red", "Red"),
        Green = GREEN => ("green", "Green"),
        Yellow = YELLOW => ("yellow", "Yellow"),
        Blue = BLUE => ("blue", "Blue"),
        Magenta = MAGENTA => ("magenta", "Magenta"),
        Cyan = CYAN => ("cyan", "Cyan"),
        Orange = ORANGE => ("orange", "Orange"),
        Pink = PINK => ("pink", "Pink"),
    }
    default BLUE => "blue";
    roles(primary) {
        surface: crate::theme_variant::SurfaceColors {
            background: Self::BG_1,
            surface: Self::BG_2,
            raised: Self::BG_3,
            overlay: Self::BG_0,
            hover: Self::SEL_0,
        },
        surface_alt: crate::theme_variant::SurfaceColors {
            background: Self::BG_0,
            surface: Self::BG_1,
            raised: Self::BG_2,
            overlay: Self::BG_0,
            hover: Self::SEL_0,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::FG_1,
            muted: Self::FG_2,
            subtle: Self::COMMENT,
            on_primary: crate::palette::action_text(primary),
            on_secondary: crate::palette::action_text(Self::MAGENTA),
        },
        primary: crate::theme_variant::ActionColors {
            normal: primary,
            hover: crate::palette::primary_hover(primary),
            pressed: crate::palette::action_pressed(primary),
            muted: Self::BG_2,
        },
        secondary: crate::theme_variant::ActionColors {
            normal: Self::MAGENTA,
            hover: crate::palette::primary_hover(Self::MAGENTA),
            pressed: crate::palette::action_pressed(Self::MAGENTA),
            muted: Self::BG_2,
        },
        status: crate::theme_variant::StatusColors {
            success: Self::GREEN,
            warning: Self::YELLOW,
            error: Self::RED,
            critical: Self::RED,
            info: Self::BLUE,
            trace: Self::FG_3,
        },
        border: Self::BG_4,
        border_muted: Self::BG_2,
        focus: primary,
    }
}
