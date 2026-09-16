//! Dawnfox. See `REGISTRATION` for pinned upstream sources.
//!
//! Nightfox base shades and declared UI roles. bg0 remains the floating
//! surface, sel0 is hover, and generated bright/dim shades used by upstream
//! semantic roles are preserved as their exact resolved values.

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
            debug: Self::FG_2,
            trace: Self::FG_3,
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
            attribute: Self::ORANGE_DIM,
            boolean: Self::ORANGE,
            builtin: Self::RED,
            builtin_function: Self::RED,
            builtin_type: Self::CYAN_DIM,
            comment: Self::COMMENT,
            constant: Self::ORANGE_DIM,
            control_keyword: Self::MAGENTA_DIM,
            deleted: Self::RED,
            deprecated: Self::FG_3,
            documentation: Self::COMMENT,
            escape: Self::YELLOW_DIM,
            foreground: Self::FG_1,
            function: Self::BLUE_DIM,
            heading: Self::BLUE_DIM,
            inserted: Self::GREEN,
            invalid: Self::RED,
            keyword: Self::MAGENTA,
            link: Self::ORANGE_DIM,
            macro_name: Self::PINK_DIM,
            markup_bold: Self::RED_BRIGHT,
            markup_italic: Self::FG_1,
            modifier: Self::YELLOW,
            namespace: Self::CYAN_DIM,
            number: Self::ORANGE,
            operator: Self::FG_2,
            parameter: Self::CYAN_DIM,
            property: Self::BLUE,
            punctuation: Self::FG_2,
            string: Self::GREEN,
            tag: Self::MAGENTA,
            type_keyword: Self::YELLOW,
            type_name: Self::YELLOW,
            variable: Self::BLACK,
        },
    }
}
