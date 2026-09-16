//! Carbonfox. See `REGISTRATION` for pinned upstream sources.
//!
//! Carbonfox base palette and declared UI roles. Resolved bright/dim shades are
//! retained where upstream syntax exports require them. Syntect roles follow
//! Nightfox's TextMate/Bat export where an equivalent scope exists, with native
//! semantic roles used for otherwise-unrepresented concepts.

define_palette! {
    Carbonfox, "Carbonfox",
    identity("nightfox/carbonfox", "nightfox", "Nightfox", "carbonfox", "Carbonfox", Dark, None);
    support(Dark);
    sources: super::SOURCE_CARBONFOX;
    colors {
        BLACK = crate::color::Color::hex(0x282828),
        RED = crate::color::Color::hex(0xee5396),
        GREEN = crate::color::Color::hex(0x25be6a),
        YELLOW = crate::color::Color::hex(0x08bdba),
        BLUE = crate::color::Color::hex(0x78a9ff),
        MAGENTA = crate::color::Color::hex(0xbe95ff),
        CYAN = crate::color::Color::hex(0x33b1ff),
        WHITE = crate::color::Color::hex(0xdfdfe0),
        ORANGE = crate::color::Color::hex(0x3ddbd9),
        PINK = crate::color::Color::hex(0xff7eb6),
        SEL_0 = crate::color::Color::hex(0x2a2a2a),
        SEL_1 = crate::color::Color::hex(0x525253),
        BG_0 = crate::color::Color::hex(0x0c0c0c),
        BG_1 = crate::color::Color::hex(0x161616),
        BG_2 = crate::color::Color::hex(0x252525),
        BG_3 = crate::color::Color::hex(0x353535),
        BG_4 = crate::color::Color::hex(0x535353),
        FG_0 = crate::color::Color::hex(0xf9fbff),
        FG_1 = crate::color::Color::hex(0xf2f4f8),
        FG_2 = crate::color::Color::hex(0xb6b8bb),
        FG_3 = crate::color::Color::hex(0x7b7c7e),
        COMMENT = crate::color::Color::hex(0x6e6f70),
        RED_DIM = crate::color::Color::hex(0xca4780),
        BLUE_DIM = crate::color::Color::hex(0x6690d9),
        PINK_DIM = crate::color::Color::hex(0xd96b9b),
        YELLOW_BRIGHT = crate::color::Color::hex(0x2dc7c4),
        BLUE_BRIGHT = crate::color::Color::hex(0x8cb6ff),
        MAGENTA_BRIGHT = crate::color::Color::hex(0xc8a5ff),
        CYAN_BRIGHT = crate::color::Color::hex(0x52bdff),
        ORANGE_BRIGHT = crate::color::Color::hex(0x5ae0df),
        PINK_BRIGHT = crate::color::Color::hex(0xff91c1),
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
            warning: Self::MAGENTA,
            error: Self::RED,
            critical: Self::RED,
            info: Self::BLUE,
            debug: Self::ORANGE,
            trace: Self::COMMENT,
        },
        border: Self::BG_4,
        border_muted: Self::BG_2,
        focus: primary,
        chromatic: crate::theme_variant::ChromaticColors {
            red: Self::RED,
            orange: Self::RED,
            yellow: Self::GREEN,
            green: Self::GREEN,
            cyan: Self::YELLOW,
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
