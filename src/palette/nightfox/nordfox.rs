//! Nordfox. See `REGISTRATION` for pinned upstream sources.
//!
//! Nightfox base shades and declared UI roles. bg0 remains the floating surface, sel0 is hover, and red serves both error levels. Generated bright/dim shades are outside the raw import.

define_palette! {
    Nordfox, "Nordfox",
    identity("nightfox/nordfox", "nightfox", "Nightfox", "nordfox", "Nordfox", Dark, None);
    support(Dark);
    sources: super::SOURCE_NORDFOX;
    colors {
        BLACK = crate::color::Color::hex(0x3b4252),
        RED = crate::color::Color::hex(0xbf616a),
        GREEN = crate::color::Color::hex(0xa3be8c),
        YELLOW = crate::color::Color::hex(0xebcb8b),
        BLUE = crate::color::Color::hex(0x81a1c1),
        MAGENTA = crate::color::Color::hex(0xb48ead),
        CYAN = crate::color::Color::hex(0x88c0d0),
        WHITE = crate::color::Color::hex(0xe5e9f0),
        ORANGE = crate::color::Color::hex(0xc9826b),
        PINK = crate::color::Color::hex(0xbf88bc),
        COMMENT = crate::color::Color::hex(0x60728a),
        BG_0 = crate::color::Color::hex(0x232831),
        BG_1 = crate::color::Color::hex(0x2e3440),
        BG_2 = crate::color::Color::hex(0x39404f),
        BG_3 = crate::color::Color::hex(0x444c5e),
        BG_4 = crate::color::Color::hex(0x5a657d),
        FG_0 = crate::color::Color::hex(0xc7cdd9),
        FG_1 = crate::color::Color::hex(0xcdcecf),
        FG_2 = crate::color::Color::hex(0xabb1bb),
        FG_3 = crate::color::Color::hex(0x7e8188),
        SEL_0 = crate::color::Color::hex(0x3e4a5b),
        SEL_1 = crate::color::Color::hex(0x4f6074),
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
