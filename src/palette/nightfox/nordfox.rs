//! Nordfox. See `REGISTRATION` for pinned upstream sources.
//!
//! Nordfox base palette and declared UI roles. Resolved bright/dim shades are
//! retained where upstream syntax exports require them. Syntect roles follow
//! Nightfox's TextMate/Bat export where an equivalent scope exists, with native
//! semantic roles used for otherwise-unrepresented concepts.

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
        RED_DIM = crate::color::Color::hex(0xa54e56),
        BLUE_DIM = crate::color::Color::hex(0x668aab),
        PINK_DIM = crate::color::Color::hex(0xa96ca5),
        YELLOW_BRIGHT = crate::color::Color::hex(0xf0d399),
        BLUE_BRIGHT = crate::color::Color::hex(0x8cafd2),
        MAGENTA_BRIGHT = crate::color::Color::hex(0xc895bf),
        CYAN_BRIGHT = crate::color::Color::hex(0x93ccdc),
        ORANGE_BRIGHT = crate::color::Color::hex(0xd89079),
        PINK_BRIGHT = crate::color::Color::hex(0xd092ce),
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
            orange: Self::ORANGE,
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
