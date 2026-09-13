//! Duskfox. See `REGISTRATION` for pinned upstream sources.
//!
//! Nightfox base shades and declared UI roles. bg0 remains the floating surface, sel0 is hover, and red serves both error levels. Generated bright/dim shades are outside the raw import.

define_palette! {
    Duskfox, "Duskfox",
    identity("nightfox/duskfox", "nightfox", "Nightfox", "duskfox", "Duskfox", Dark, None);
    support(Dark);
    sources: super::SOURCE_DUSKFOX;
    colors {
        BLACK = crate::color::Color::hex(0x393552),
        RED = crate::color::Color::hex(0xeb6f92),
        GREEN = crate::color::Color::hex(0xa3be8c),
        YELLOW = crate::color::Color::hex(0xf6c177),
        BLUE = crate::color::Color::hex(0x569fba),
        MAGENTA = crate::color::Color::hex(0xc4a7e7),
        CYAN = crate::color::Color::hex(0x9ccfd8),
        WHITE = crate::color::Color::hex(0xe0def4),
        ORANGE = crate::color::Color::hex(0xea9a97),
        PINK = crate::color::Color::hex(0xeb98c3),
        COMMENT = crate::color::Color::hex(0x817c9c),
        BG_0 = crate::color::Color::hex(0x191726),
        BG_1 = crate::color::Color::hex(0x232136),
        BG_2 = crate::color::Color::hex(0x2d2a45),
        BG_3 = crate::color::Color::hex(0x373354),
        BG_4 = crate::color::Color::hex(0x4b4673),
        FG_0 = crate::color::Color::hex(0xeae8ff),
        FG_1 = crate::color::Color::hex(0xe0def4),
        FG_2 = crate::color::Color::hex(0xcdcbe0),
        FG_3 = crate::color::Color::hex(0x6e6a86),
        SEL_0 = crate::color::Color::hex(0x433c59),
        SEL_1 = crate::color::Color::hex(0x63577d),
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
