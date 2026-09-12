//! Dracula. See `REGISTRATION` for pinned upstream sources.
//!
//! Dracula declares darker chrome and lighter control surfaces. Bright ANSI red supplies critical. The source comment colour is reserved for subtle text and trace.

define_palette! {
    Main, "Dracula",
    identity("dracula/main", "dracula", "Dracula", "main", "Main", Dark, None);
    sources: crate::palette::sources::DRACULA_MAIN;
    colors {
        BG = crate::color::Color::hex(0x282a36),
        FG = crate::color::Color::hex(0xf8f8f2),
        SELECTION = crate::color::Color::hex(0x44475a),
        COMMENT = crate::color::Color::hex(0x6272a4),
        CYAN = crate::color::Color::hex(0x8be9fd),
        GREEN = crate::color::Color::hex(0x50fa7b),
        ORANGE = crate::color::Color::hex(0xffb86c),
        PINK = crate::color::Color::hex(0xff79c6),
        PURPLE = crate::color::Color::hex(0xbd93f9),
        RED = crate::color::Color::hex(0xff5555),
        YELLOW = crate::color::Color::hex(0xf1fa8c),
        COLOR_0 = crate::color::Color::hex(0x21222c),
        COLOR_1 = crate::color::Color::hex(0xff5555),
        COLOR_2 = crate::color::Color::hex(0x50fa7b),
        COLOR_3 = crate::color::Color::hex(0xf1fa8c),
        COLOR_4 = crate::color::Color::hex(0xbd93f9),
        COLOR_5 = crate::color::Color::hex(0xff79c6),
        COLOR_6 = crate::color::Color::hex(0x8be9fd),
        COLOR_7 = crate::color::Color::hex(0xf8f8f2),
        COLOR_8 = crate::color::Color::hex(0x6272a4),
        COLOR_9 = crate::color::Color::hex(0xff6e6e),
        COLOR_10 = crate::color::Color::hex(0x69ff94),
        COLOR_11 = crate::color::Color::hex(0xffffa5),
        COLOR_12 = crate::color::Color::hex(0xd6acff),
        COLOR_13 = crate::color::Color::hex(0xff92df),
        COLOR_14 = crate::color::Color::hex(0xa4ffff),
        COLOR_15 = crate::color::Color::hex(0xffffff),
        TEMP_QUOTES = crate::color::Color::hex(0xe9f284),
        TEMP_PROPERTY_QUOTES = crate::color::Color::hex(0x8be9fe),
        LINE_HIGHLIGHT = crate::color::Color::hex_alpha(0x44475a75),
        NON_TEXT = crate::color::Color::hex_alpha(0xffffff1a),
        WHITE = crate::color::Color::hex(0xffffff),
        TAB_DROP_BG = crate::color::Color::hex_alpha(0x44475a70),
        BGLIGHTER = crate::color::Color::hex(0x424450),
        BGLIGHT = crate::color::Color::hex(0x343746),
        BGDARK = crate::color::Color::hex(0x21222c),
        BGDARKER = crate::color::Color::hex(0x191a21),
    }
    accents {
        Cyan = CYAN => ("cyan", "Cyan"),
        Green = GREEN => ("green", "Green"),
        Orange = ORANGE => ("orange", "Orange"),
        Pink = PINK => ("pink", "Pink"),
        Purple = PURPLE => ("purple", "Purple"),
        Red = RED => ("red", "Red"),
        Yellow = YELLOW => ("yellow", "Yellow"),
    }
    default PURPLE => "purple";
    roles(primary) {
        surface: crate::theme_variant::SurfaceColors {
            background: Self::BG,
            surface: Self::BGLIGHT,
            raised: Self::BGLIGHTER,
            overlay: Self::BGDARK,
            hover: Self::SELECTION,
        },
        surface_alt: crate::theme_variant::SurfaceColors {
            background: Self::BGDARKER,
            surface: Self::BG,
            raised: Self::BGLIGHT,
            overlay: Self::BGDARK,
            hover: Self::SELECTION,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::FG,
            muted: Self::FG,
            subtle: Self::COMMENT,
            on_primary: crate::palette::action_text(primary),
            on_secondary: crate::palette::action_text(Self::PINK),
        },
        primary: crate::theme_variant::ActionColors {
            normal: primary,
            hover: crate::palette::primary_hover(primary),
            pressed: crate::palette::action_pressed(primary),
            muted: Self::BGLIGHT,
        },
        secondary: crate::theme_variant::ActionColors {
            normal: Self::PINK,
            hover: crate::palette::primary_hover(Self::PINK),
            pressed: crate::palette::action_pressed(Self::PINK),
            muted: Self::BGLIGHT,
        },
        status: crate::theme_variant::StatusColors {
            success: Self::GREEN,
            warning: Self::YELLOW,
            error: Self::RED,
            critical: Self::COLOR_9,
            info: Self::CYAN,
            trace: Self::COMMENT,
        },
        border: Self::COMMENT,
        border_muted: Self::BGLIGHT,
        focus: primary,
    }
}
