//! Duskfox. See `REGISTRATION` for pinned upstream sources.
//!
//! Nightfox base shades and declared UI roles. bg0 remains the floating
//! surface, sel0 is hover, and generated bright/dim shades used by upstream
//! semantic roles are preserved as their exact resolved values.

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
        RED_DIM = crate::color::Color::hex(0xd84f76),
        YELLOW_BRIGHT = crate::color::Color::hex(0xf9cb8c),
        BLUE_BRIGHT = crate::color::Color::hex(0x65b1cd),
        MAGENTA_BRIGHT = crate::color::Color::hex(0xccb1ed),
        CYAN_BRIGHT = crate::color::Color::hex(0xa6dae3),
        ORANGE_BRIGHT = crate::color::Color::hex(0xf0a4a2),
        PINK_BRIGHT = crate::color::Color::hex(0xf0a6cc),
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
            attribute: Self::ORANGE_BRIGHT,
            boolean: Self::ORANGE,
            builtin: Self::RED,
            builtin_function: Self::RED,
            builtin_type: Self::CYAN_BRIGHT,
            comment: Self::COMMENT,
            constant: Self::ORANGE_BRIGHT,
            control_keyword: Self::MAGENTA_BRIGHT,
            deleted: Self::RED,
            deprecated: Self::FG_3,
            documentation: Self::COMMENT,
            escape: Self::YELLOW_BRIGHT,
            foreground: Self::FG_1,
            function: Self::BLUE_BRIGHT,
            heading: Self::BLUE_BRIGHT,
            inserted: Self::GREEN,
            invalid: Self::RED,
            keyword: Self::MAGENTA,
            link: Self::ORANGE_BRIGHT,
            macro_name: Self::PINK_BRIGHT,
            markup_bold: Self::RED_DIM,
            markup_italic: Self::FG_1,
            modifier: Self::YELLOW,
            namespace: Self::CYAN_BRIGHT,
            number: Self::ORANGE,
            operator: Self::FG_2,
            parameter: Self::CYAN_BRIGHT,
            property: Self::BLUE,
            punctuation: Self::FG_2,
            string: Self::GREEN,
            tag: Self::MAGENTA,
            type_keyword: Self::YELLOW,
            type_name: Self::YELLOW,
            variable: Self::WHITE,
        },
    }
}
