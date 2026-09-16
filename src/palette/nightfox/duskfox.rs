//! Duskfox. See `REGISTRATION` for pinned upstream sources.
//!
//! Duskfox base palette and declared UI roles. Resolved bright/dim shades are
//! retained where upstream syntax exports require them. Syntect roles follow
//! Nightfox's TextMate/Bat export where an equivalent scope exists, with native
//! semantic roles used for otherwise-unrepresented concepts.

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
        BLUE_DIM = crate::color::Color::hex(0x4a869c),
        PINK_DIM = crate::color::Color::hex(0xd871a6),
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
            macro_name: Self::PINK_BRIGHT,
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
