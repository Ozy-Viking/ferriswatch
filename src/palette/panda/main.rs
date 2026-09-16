//! Panda. See `REGISTRATION` for pinned upstream sources.
//!
//! The pinned Brackets theme uses white for ordinary editor text; its source
//! variable named `foreground` is pink and is not the CodeMirror text colour.
//! CodeMirror token classes supply syntax semantics; no control colours are
//! authored by the source.

define_palette! {
    Main, "Panda",
    identity("panda/main", "panda", "Panda", "main", "Main", Dark, None);
    support(Dark);
    sources: super::SOURCE;
    colors {
        BACKGROUND = crate::color::Color::hex(0x292a2b),
        FOREGROUND = crate::color::Color::hex(0xff2c6d),
        COMMENT = crate::color::Color::hex(0x676b79),
        ERROR = crate::color::Color::hex(0xff2c6d),
        KEYWORD = crate::color::Color::hex(0xff75b5),
        WHITE = crate::color::Color::hex(0xf3f3f3),
        LIGHT = crate::color::Color::hex(0xe6e6e6),
        BLUE = crate::color::Color::hex(0x45a9f9),
        LIGHT_BLUE = crate::color::Color::hex(0x6fc1ff),
        PURPLE = crate::color::Color::hex(0xb084eb),
        GREEN = crate::color::Color::hex(0x19f9d8),
        RED = crate::color::Color::hex(0xff2c6d),
        ORANGE = crate::color::Color::hex(0xffb86c),
        LIGHT_ORANGE = crate::color::Color::hex(0xffcc95),
        PINK = crate::color::Color::hex(0xff75b5),
        LIGHT_PINK = crate::color::Color::hex(0xff9ac1),
    }
    accents {
        Blue = BLUE => ("blue", "Blue"),
        Purple = PURPLE => ("purple", "Purple"),
        Green = GREEN => ("green", "Green"),
        Red = RED => ("red", "Red"),
        Orange = ORANGE => ("orange", "Orange"),
        Pink = PINK => ("pink", "Pink"),
    }
    default PURPLE => "purple";
    roles(primary) {
        surfaces: crate::theme_variant::SurfaceColors {
            background: Self::BACKGROUND,
            base: Self::BACKGROUND,
            raised: Self::BACKGROUND,
            overlay: Self::BACKGROUND,
            hover: Self::BACKGROUND,
        },
        surfaces_alt: crate::theme_variant::SurfaceColors {
            background: Self::BACKGROUND,
            base: Self::BACKGROUND,
            raised: Self::BACKGROUND,
            overlay: Self::BACKGROUND,
            hover: Self::BACKGROUND,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::WHITE,
            muted: Self::LIGHT,
            subtle: Self::COMMENT,
        },
        text_alt: None,
        primary: crate::theme_variant::ActionColors {
            normal: crate::theme_variant::ColorPair::new(
                crate::palette::action_text(
                    primary,
                    Self::BACKGROUND,
                    Self::WHITE,
                ),
                primary,
            ),
            hover: crate::theme_variant::ColorPair::new(
                crate::palette::action_text(
                    primary,
                    Self::BACKGROUND,
                    Self::WHITE,
                ),
                crate::palette::primary_hover(primary),
            ),
            pressed: crate::theme_variant::ColorPair::new(
                crate::palette::action_text(
                    primary,
                    Self::BACKGROUND,
                    Self::WHITE,
                ),
                crate::palette::action_pressed(primary),
            ),
            muted: crate::theme_variant::ColorPair::new(
                Self::WHITE,
                Self::BACKGROUND,
            ),
            disabled: crate::theme_variant::ColorPair::new(
                Self::COMMENT,
                Self::BACKGROUND,
            ),
        },
        secondary: crate::theme_variant::ActionColors {
            normal: crate::theme_variant::ColorPair::new(
                crate::palette::action_text(
                    Self::PURPLE,
                    Self::BACKGROUND,
                    Self::WHITE,
                ),
                Self::PURPLE,
            ),
            hover: crate::theme_variant::ColorPair::new(
                crate::palette::action_text(
                    Self::PURPLE,
                    Self::BACKGROUND,
                    Self::WHITE,
                ),
                crate::palette::primary_hover(Self::PURPLE),
            ),
            pressed: crate::theme_variant::ColorPair::new(
                crate::palette::action_text(
                    Self::PURPLE,
                    Self::BACKGROUND,
                    Self::WHITE,
                ),
                crate::palette::action_pressed(Self::PURPLE),
            ),
            muted: crate::theme_variant::ColorPair::new(
                Self::WHITE,
                Self::BACKGROUND,
            ),
            disabled: crate::theme_variant::ColorPair::new(
                Self::COMMENT,
                Self::BACKGROUND,
            ),
        },
        status: crate::theme_variant::StatusColors {
            success: Self::GREEN,
            warning: Self::ORANGE,
            error: Self::ERROR,
            critical: Self::ERROR,
            info: Self::BLUE,
            debug: Self::LIGHT,
            trace: Self::COMMENT,
        },
        border: Self::COMMENT,
        border_muted: Self::BACKGROUND,
        focus: primary,
        chromatic: crate::theme_variant::ChromaticColors {
            red: Self::RED,
            orange: Self::ORANGE,
            yellow: Self::ORANGE,
            green: Self::GREEN,
            cyan: Self::GREEN,
            blue: Self::BLUE,
            purple: Self::PURPLE,
            pink: Self::PINK,
        },
        syntax: crate::theme_variant::SyntaxColors {
            attribute: Self::ORANGE,
            boolean: Self::ORANGE,
            builtin: Self::BLUE,
            builtin_function: Self::BLUE,
            builtin_type: Self::BLUE,
            comment: Self::COMMENT,
            constant: Self::ORANGE,
            control_keyword: Self::PINK,
            deleted: Self::RED,
            deprecated: Self::WHITE,
            documentation: Self::COMMENT,
            escape: Self::GREEN,
            foreground: Self::WHITE,
            function: Self::PURPLE,
            heading: Self::LIGHT,
            inserted: Self::GREEN,
            invalid: Self::RED,
            keyword: Self::PINK,
            link: Self::ORANGE,
            macro_name: Self::LIGHT,
            markup_bold: Self::WHITE,
            markup_italic: Self::WHITE,
            modifier: Self::PINK,
            namespace: Self::LIGHT_PINK,
            number: Self::ORANGE,
            operator: Self::LIGHT_ORANGE,
            parameter: Self::ORANGE,
            property: Self::LIGHT,
            punctuation: Self::LIGHT_ORANGE,
            string: Self::GREEN,
            tag: Self::RED,
            type_keyword: Self::PINK,
            type_name: Self::LIGHT_PINK,
            variable: Self::ORANGE,
        },
    }
}
