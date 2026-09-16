//! Solarized Light. See `REGISTRATION` for pinned upstream sources.
//!
//! The two neutral background steps repeat for raised content and hover. Solarized has no separate critical hue; critical repeats red.

define_palette! {
    Light, "Solarized Light",
    identity("solarized/light", "solarized", "Solarized", "light", "Light", Light, None);
    support(Light);
    sources: super::SOURCE_LIGHT;
    colors {
        BASE_03 = crate::color::Color::hex(0x002b36),
        BASE_02 = crate::color::Color::hex(0x073642),
        BASE_01 = crate::color::Color::hex(0x586e75),
        BASE_00 = crate::color::Color::hex(0x657b83),
        BASE_0 = crate::color::Color::hex(0x839496),
        BASE_1 = crate::color::Color::hex(0x93a1a1),
        BASE_2 = crate::color::Color::hex(0xeee8d5),
        BASE_3 = crate::color::Color::hex(0xfdf6e3),
        YELLOW = crate::color::Color::hex(0xb58900),
        ORANGE = crate::color::Color::hex(0xcb4b16),
        RED = crate::color::Color::hex(0xdc322f),
        MAGENTA = crate::color::Color::hex(0xd33682),
        VIOLET = crate::color::Color::hex(0x6c71c4),
        BLUE = crate::color::Color::hex(0x268bd2),
        CYAN = crate::color::Color::hex(0x2aa198),
        GREEN = crate::color::Color::hex(0x859900),
        VIM_GREEN = crate::color::Color::hex(0x719e07),
    }
    accents {
        Yellow = YELLOW => ("yellow", "Yellow"),
        Orange = ORANGE => ("orange", "Orange"),
        Red = RED => ("red", "Red"),
        Magenta = MAGENTA => ("magenta", "Magenta"),
        Violet = VIOLET => ("violet", "Violet"),
        Blue = BLUE => ("blue", "Blue"),
        Cyan = CYAN => ("cyan", "Cyan"),
        Green = GREEN => ("green", "Green"),
    }
    default BLUE => "blue";
    roles(primary) {
        surfaces: crate::theme_variant::SurfaceColors {
            background: Self::BASE_3,
            base: Self::BASE_2,
            raised: Self::BASE_2,
            overlay: Self::BASE_2,
            hover: Self::BASE_2,
        },
        surfaces_alt: crate::theme_variant::SurfaceColors {
            background: Self::BASE_2,
            base: Self::BASE_3,
            raised: Self::BASE_2,
            overlay: Self::BASE_2,
            hover: Self::BASE_2,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::BASE_00,
            muted: Self::BASE_01,
            subtle: Self::BASE_1,
        },
        text_alt: None,
        primary: crate::theme_variant::ActionColors {
            normal: crate::theme_variant::ColorPair::new(
                crate::palette::action_text(
                    primary,
                    Self::BASE_3,
                    Self::BASE_00,
                ),
                primary,
            ),
            hover: crate::theme_variant::ColorPair::new(
                crate::palette::action_text(
                    primary,
                    Self::BASE_3,
                    Self::BASE_00,
                ),
                crate::palette::primary_hover(primary),
            ),
            pressed: crate::theme_variant::ColorPair::new(
                crate::palette::action_text(
                    primary,
                    Self::BASE_3,
                    Self::BASE_00,
                ),
                crate::palette::action_pressed(primary),
            ),
            muted: crate::theme_variant::ColorPair::new(
                Self::BASE_01,
                Self::BASE_2,
            ),
            disabled: crate::theme_variant::ColorPair::new(
                Self::BASE_1,
                Self::BASE_2,
            ),
        },
        secondary: crate::theme_variant::ActionColors {
            normal: crate::theme_variant::ColorPair::new(
                crate::palette::action_text(
                    Self::CYAN,
                    Self::BASE_3,
                    Self::BASE_00,
                ),
                Self::CYAN,
            ),
            hover: crate::theme_variant::ColorPair::new(
                crate::palette::action_text(
                    Self::CYAN,
                    Self::BASE_3,
                    Self::BASE_00,
                ),
                crate::palette::primary_hover(Self::CYAN),
            ),
            pressed: crate::theme_variant::ColorPair::new(
                crate::palette::action_text(
                    Self::CYAN,
                    Self::BASE_3,
                    Self::BASE_00,
                ),
                crate::palette::action_pressed(Self::CYAN),
            ),
            muted: crate::theme_variant::ColorPair::new(
                Self::BASE_01,
                Self::BASE_2,
            ),
            disabled: crate::theme_variant::ColorPair::new(
                Self::BASE_1,
                Self::BASE_2,
            ),
        },
        status: crate::theme_variant::StatusColors {
            success: Self::GREEN,
            warning: Self::YELLOW,
            error: Self::RED,
            critical: Self::RED,
            info: Self::BLUE,
            debug: Self::BASE_01,
            trace: Self::BASE_00,
        },
        border: Self::BASE_1,
        border_muted: Self::BASE_2,
        focus: primary,
        chromatic: crate::theme_variant::ChromaticColors {
            red: Self::RED,
            orange: Self::ORANGE,
            yellow: Self::YELLOW,
            green: Self::GREEN,
            cyan: Self::CYAN,
            blue: Self::BLUE,
            purple: Self::VIOLET,
            pink: Self::MAGENTA,
        },
        syntax: crate::theme_variant::SyntaxColors {
            attribute: Self::BASE_0,
            boolean: Self::CYAN,
            builtin: Self::YELLOW,
            builtin_function: Self::BLUE,
            builtin_type: Self::YELLOW,
            comment: Self::BASE_01,
            constant: Self::CYAN,
            control_keyword: Self::VIM_GREEN,
            deleted: Self::RED,
            deprecated: Self::BASE_00,
            documentation: Self::BASE_01,
            escape: Self::RED,
            foreground: Self::BASE_00,
            function: Self::BLUE,
            heading: Self::ORANGE,
            inserted: Self::VIM_GREEN,
            invalid: Self::RED,
            keyword: Self::VIM_GREEN,
            link: Self::VIOLET,
            macro_name: Self::ORANGE,
            markup_bold: Self::BASE_00,
            markup_italic: Self::BASE_00,
            modifier: Self::YELLOW,
            namespace: Self::YELLOW,
            number: Self::CYAN,
            operator: Self::VIM_GREEN,
            parameter: Self::BLUE,
            property: Self::BLUE,
            punctuation: Self::BASE_00,
            string: Self::CYAN,
            tag: Self::BLUE,
            type_keyword: Self::YELLOW,
            type_name: Self::YELLOW,
            variable: Self::BLUE,
        },
    }
}
