//! Dayfox. See `REGISTRATION` for pinned upstream sources.
//!
//! Dayfox base palette and declared UI roles. Resolved bright/dim shades are
//! retained where upstream syntax exports require them. Syntect roles follow
//! Nightfox's TextMate/Bat export where an equivalent scope exists, with native
//! semantic roles used for otherwise-unrepresented concepts.

define_palette! {
    Dayfox, "Dayfox",
    identity("nightfox/dayfox", "nightfox", "Nightfox", "dayfox", "Dayfox", Light, None);
    support(Light);
    sources: super::SOURCE_DAYFOX;
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
        RED_BRIGHT = crate::color::Color::hex(0xb3434e),
        YELLOW_DIM = crate::color::Color::hex(0x924702),
        BLUE_DIM = crate::color::Color::hex(0x223d90),
        MAGENTA_DIM = crate::color::Color::hex(0x5e2baf),
        CYAN_DIM = crate::color::Color::hex(0x22676d),
        ORANGE_DIM = crate::color::Color::hex(0x7f5152),
        PINK_DIM = crate::color::Color::hex(0x8b369a),
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
        surfaces: crate::theme_variant::SurfaceColors {
            background: Self::BG_1,
            base: Self::BG_2,
            raised: Self::BG_3,
            overlay: Self::BG_0,
            hover: Self::SEL_0,
        },
        surfaces_alt: crate::theme_variant::SurfaceColors {
            background: Self::BG_0,
            base: Self::BG_1,
            raised: Self::BG_2,
            overlay: Self::BG_0,
            hover: Self::SEL_0,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::FG_1,
            muted: Self::FG_2,
            subtle: Self::COMMENT,
        },
        text_alt: None,
        primary: crate::theme_variant::ActionColors {
            normal: crate::theme_variant::ColorPair::new(
                Self::BG_0,
                primary,
            ),
            hover: crate::theme_variant::ColorPair::new(
                Self::BG_0,
                crate::palette::primary_hover(primary),
            ),
            pressed: crate::theme_variant::ColorPair::new(
                Self::BG_0,
                crate::palette::action_pressed(primary),
            ),
            muted: crate::theme_variant::ColorPair::new(
                Self::FG_3,
                Self::BG_0,
            ),
            disabled: crate::theme_variant::ColorPair::new(
                Self::COMMENT,
                Self::BG_0,
            ),
        },
        secondary: crate::theme_variant::ActionColors {
            normal: crate::theme_variant::ColorPair::new(
                Self::BG_0,
                Self::MAGENTA,
            ),
            hover: crate::theme_variant::ColorPair::new(
                Self::BG_0,
                crate::palette::primary_hover(Self::MAGENTA),
            ),
            pressed: crate::theme_variant::ColorPair::new(
                Self::BG_0,
                crate::palette::action_pressed(Self::MAGENTA),
            ),
            muted: crate::theme_variant::ColorPair::new(
                Self::FG_3,
                Self::BG_0,
            ),
            disabled: crate::theme_variant::ColorPair::new(
                Self::COMMENT,
                Self::BG_0,
            ),
        },
        status: crate::theme_variant::StatusColors {
            success: Self::GREEN,
            warning: Self::YELLOW,
            error: Self::RED,
            critical: Self::RED,
            info: Self::BLUE,
            debug: Self::GREEN,
            trace: Self::COMMENT,
        },
        border: Self::BG_4,
        border_muted: Self::BG_2,
        focus: primary,
        chromatic: crate::theme_variant::ChromaticColors {
            red: Self::RED,
            orange: Self::YELLOW,
            yellow: Self::YELLOW,
            green: Self::GREEN,
            cyan: Self::CYAN,
            blue: Self::BLUE,
            purple: Self::MAGENTA,
            pink: Self::PINK,
        },
        syntax: crate::theme_variant::SyntaxColors {
            attribute: Self::YELLOW,
            boolean: Self::YELLOW,
            builtin: Self::RED,
            builtin_function: Self::BLUE_DIM,
            builtin_type: Self::YELLOW,
            comment: Self::BG_4,
            constant: Self::YELLOW,
            control_keyword: Self::MAGENTA,
            deleted: Self::RED,
            deprecated: Self::FG_3,
            documentation: Self::BG_4,
            escape: Self::FG_2,
            foreground: Self::FG_0,
            function: Self::BLUE_DIM,
            heading: Self::YELLOW,
            inserted: Self::GREEN,
            invalid: Self::RED,
            keyword: Self::MAGENTA,
            link: Self::BLUE_DIM,
            macro_name: Self::PINK_DIM,
            markup_bold: Self::RED,
            markup_italic: Self::RED,
            modifier: Self::MAGENTA,
            namespace: Self::YELLOW,
            number: Self::YELLOW,
            operator: Self::CYAN,
            parameter: Self::PINK_DIM,
            property: Self::BLUE,
            punctuation: Self::FG_0,
            string: Self::GREEN,
            tag: Self::BLUE_DIM,
            type_keyword: Self::MAGENTA,
            type_name: Self::YELLOW,
            variable: Self::FG_0,
        },
    }
}
