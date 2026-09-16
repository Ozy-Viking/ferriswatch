//! Dawnfox. See `REGISTRATION` for pinned upstream sources.
//!
//! Dawnfox base palette and declared UI roles. Resolved bright/dim shades are
//! retained where upstream syntax exports require them. Syntect roles follow
//! Nightfox's TextMate/Bat export where an equivalent scope exists, with native
//! semantic roles used for otherwise-unrepresented concepts.

define_palette! {
    Dawnfox, "Dawnfox",
    identity("nightfox/dawnfox", "nightfox", "Nightfox", "dawnfox", "Dawnfox", Light, None);
    support(Light);
    sources: super::SOURCE_DAWNFOX;
    colors {
        BLACK = crate::color::Color::hex(0x575279),
        RED = crate::color::Color::hex(0xb4637a),
        GREEN = crate::color::Color::hex(0x618774),
        YELLOW = crate::color::Color::hex(0xea9d34),
        BLUE = crate::color::Color::hex(0x286983),
        MAGENTA = crate::color::Color::hex(0x907aa9),
        CYAN = crate::color::Color::hex(0x56949f),
        WHITE = crate::color::Color::hex(0xe5e9f0),
        ORANGE = crate::color::Color::hex(0xd7827e),
        PINK = crate::color::Color::hex(0xd685af),
        COMMENT = crate::color::Color::hex(0x9893a5),
        BG_0 = crate::color::Color::hex(0xebe5df),
        BG_1 = crate::color::Color::hex(0xfaf4ed),
        BG_2 = crate::color::Color::hex(0xebe0df),
        BG_3 = crate::color::Color::hex(0xebdfe4),
        BG_4 = crate::color::Color::hex(0xbdbfc9),
        FG_0 = crate::color::Color::hex(0x4c4769),
        FG_1 = crate::color::Color::hex(0x575279),
        FG_2 = crate::color::Color::hex(0x625c87),
        FG_3 = crate::color::Color::hex(0xa8a3b3),
        SEL_0 = crate::color::Color::hex(0xd0d8d8),
        SEL_1 = crate::color::Color::hex(0xb8cece),
        RED_BRIGHT = crate::color::Color::hex(0xc26d85),
        YELLOW_DIM = crate::color::Color::hex(0xdd9024),
        BLUE_DIM = crate::color::Color::hex(0x295e73),
        MAGENTA_DIM = crate::color::Color::hex(0x816b9a),
        CYAN_DIM = crate::color::Color::hex(0x50848c),
        ORANGE_DIM = crate::color::Color::hex(0xca6e69),
        PINK_DIM = crate::color::Color::hex(0xc9709e),
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
