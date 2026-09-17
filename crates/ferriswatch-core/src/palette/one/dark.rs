//! One Dark. See `REGISTRATION` for pinned upstream sources.
//!
//! Original Atom syntax palette. Background roles repeat because this source has one background; border and action states supply control separation. HSL conversion is recorded by the importer.

define_palette! {
    Dark, "One Dark",
    identity("one/dark", "one", "One", "dark", "Dark", Dark, None);
    support(Dark);
    sources: super::SOURCE_DARK;
    colors {
        MONO_1 = crate::color::Color::hex(0xabb2bf),
        MONO_2 = crate::color::Color::hex(0x828997),
        MONO_3 = crate::color::Color::hex(0x5c6370),
        HUE_1 = crate::color::Color::hex(0x56b6c2),
        HUE_2 = crate::color::Color::hex(0x61afef),
        HUE_3 = crate::color::Color::hex(0xc678dd),
        HUE_4 = crate::color::Color::hex(0x98c379),
        HUE_5 = crate::color::Color::hex(0xe06c75),
        HUE_5_2 = crate::color::Color::hex(0xbe5046),
        HUE_6 = crate::color::Color::hex(0xd19a66),
        HUE_6_2 = crate::color::Color::hex(0xe5c07b),
        SYNTAX_BG = crate::color::Color::hex(0x282c34),
        SYNTAX_ACCENT = crate::color::Color::hex(0x528bff),
        SYNTAX_DEPRECATED_FG = crate::color::Color::hex(0x523d14),
        SYNTAX_ILLEGAL_FG = crate::color::Color::hex(0xffffff),
    }
    accents {
        Accent = SYNTAX_ACCENT => ("accent", "Accent"),
        Hue1 = HUE_1 => ("hue_1", "Hue 1"),
        Hue2 = HUE_2 => ("hue_2", "Hue 2"),
        Hue3 = HUE_3 => ("hue_3", "Hue 3"),
        Hue4 = HUE_4 => ("hue_4", "Hue 4"),
        Hue5 = HUE_5 => ("hue_5", "Hue 5"),
        Hue6 = HUE_6 => ("hue_6", "Hue 6"),
    }
    default SYNTAX_ACCENT => "accent";
    roles(primary) {
        surfaces: crate::theme_variant::SurfaceColors {
            background: Self::SYNTAX_BG,
            base: Self::SYNTAX_BG,
            raised: Self::SYNTAX_BG,
            overlay: Self::SYNTAX_BG,
            hover: Self::SYNTAX_BG,
        },
        surfaces_alt: crate::theme_variant::SurfaceColors {
            background: Self::SYNTAX_BG,
            base: Self::SYNTAX_BG,
            raised: Self::SYNTAX_BG,
            overlay: Self::SYNTAX_BG,
            hover: Self::SYNTAX_BG,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::MONO_1,
            muted: Self::MONO_2,
            subtle: Self::MONO_3,
        },
        text_alt: None,
        primary: crate::theme_variant::ActionColors {
            normal: crate::theme_variant::ColorPair::new(
                crate::palette::action_text(
                    primary,
                    Self::SYNTAX_BG,
                    Self::MONO_1,
                ),
                primary,
            ),
            hover: crate::theme_variant::ColorPair::new(
                crate::palette::action_text(
                    primary,
                    Self::SYNTAX_BG,
                    Self::MONO_1,
                ),
                crate::palette::primary_hover(primary),
            ),
            pressed: crate::theme_variant::ColorPair::new(
                crate::palette::action_text(
                    primary,
                    Self::SYNTAX_BG,
                    Self::MONO_1,
                ),
                crate::palette::action_pressed(primary),
            ),
            muted: crate::theme_variant::ColorPair::new(
                Self::MONO_1,
                Self::SYNTAX_BG,
            ),
            disabled: crate::theme_variant::ColorPair::new(
                Self::MONO_3,
                Self::SYNTAX_BG,
            ),
        },
        secondary: crate::theme_variant::ActionColors {
            normal: crate::theme_variant::ColorPair::new(
                crate::palette::action_text(
                    Self::HUE_3,
                    Self::SYNTAX_BG,
                    Self::MONO_1,
                ),
                Self::HUE_3,
            ),
            hover: crate::theme_variant::ColorPair::new(
                crate::palette::action_text(
                    Self::HUE_3,
                    Self::SYNTAX_BG,
                    Self::MONO_1,
                ),
                crate::palette::primary_hover(Self::HUE_3),
            ),
            pressed: crate::theme_variant::ColorPair::new(
                crate::palette::action_text(
                    Self::HUE_3,
                    Self::SYNTAX_BG,
                    Self::MONO_1,
                ),
                crate::palette::action_pressed(Self::HUE_3),
            ),
            muted: crate::theme_variant::ColorPair::new(
                Self::MONO_1,
                Self::SYNTAX_BG,
            ),
            disabled: crate::theme_variant::ColorPair::new(
                Self::MONO_3,
                Self::SYNTAX_BG,
            ),
        },
        status: crate::theme_variant::StatusColors {
            success: Self::HUE_4,
            warning: Self::HUE_6_2,
            error: Self::HUE_5,
            critical: Self::HUE_5_2,
            info: Self::HUE_2,
            debug: Self::MONO_2,
            trace: Self::MONO_3,
        },
        border: Self::MONO_3,
        border_muted: Self::SYNTAX_BG,
        focus: primary,
        chromatic: crate::theme_variant::ChromaticColors {
            red: Self::HUE_5,
            orange: Self::HUE_6,
            yellow: Self::HUE_6_2,
            green: Self::HUE_4,
            cyan: Self::HUE_1,
            blue: Self::HUE_2,
            purple: Self::HUE_3,
            pink: Self::HUE_3,
        },
        syntax: crate::theme_variant::SyntaxColors {
            attribute: Self::HUE_6,
            boolean: Self::HUE_6,
            builtin: Self::MONO_1,
            builtin_function: Self::HUE_1,
            builtin_type: Self::HUE_1,
            comment: Self::MONO_3,
            constant: Self::HUE_6,
            control_keyword: Self::HUE_3,
            deleted: Self::HUE_5,
            deprecated: Self::SYNTAX_DEPRECATED_FG,
            documentation: Self::MONO_3,
            escape: Self::HUE_1,
            foreground: Self::MONO_1,
            function: Self::HUE_2,
            heading: Self::HUE_5,
            inserted: Self::HUE_4,
            invalid: Self::SYNTAX_ILLEGAL_FG,
            keyword: Self::HUE_3,
            link: Self::HUE_1,
            macro_name: Self::HUE_2,
            markup_bold: Self::HUE_6,
            markup_italic: Self::HUE_3,
            modifier: Self::HUE_3,
            namespace: Self::MONO_1,
            number: Self::HUE_6,
            operator: Self::MONO_1,
            parameter: Self::MONO_1,
            property: Self::MONO_1,
            punctuation: Self::MONO_1,
            string: Self::HUE_4,
            tag: Self::HUE_5,
            type_keyword: Self::HUE_3,
            type_name: Self::HUE_6_2,
            variable: Self::HUE_5,
        },
    }
}
