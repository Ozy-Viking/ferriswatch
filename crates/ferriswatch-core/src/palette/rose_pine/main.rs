//! Rosé Pine Main. See the family module for its upstream source.

define_palette! {
    Main, "Rosé Pine",
    identity("rose_pine/main", "rose_pine", "Rosé Pine", "main", "Main", Dark, None);
    support(Dark);
    sources: super::SOURCE;
    colors {
        NC = crate::color::Color::hex(0x16141f),
        BASE = crate::color::Color::hex(0x191724),
        SURFACE = crate::color::Color::hex(0x1f1d2e),
        OVERLAY = crate::color::Color::hex(0x26233a),
        MUTED = crate::color::Color::hex(0x6e6a86),
        SUBTLE = crate::color::Color::hex(0x908caa),
        TEXT = crate::color::Color::hex(0xe0def4),
        LOVE = crate::color::Color::hex(0xeb6f92),
        GOLD = crate::color::Color::hex(0xf6c177),
        ROSE = crate::color::Color::hex(0xebbcba),
        PINE = crate::color::Color::hex(0x31748f),
        FOAM = crate::color::Color::hex(0x9ccfd8),
        IRIS = crate::color::Color::hex(0xc4a7e7),
        LEAF = crate::color::Color::hex(0x95b1ac),
        HIGHLIGHT_LOW = crate::color::Color::hex(0x21202e),
        HIGHLIGHT_MED = crate::color::Color::hex(0x403d52),
        HIGHLIGHT_HIGH = crate::color::Color::hex(0x524f67),
    }
    accents {
        Love = LOVE => ("love", "Love"),
        Gold = GOLD => ("gold", "Gold"),
        Rose = ROSE => ("rose", "Rose"),
        Pine = PINE => ("pine", "Pine"),
        Foam = FOAM => ("foam", "Foam"),
        Iris = IRIS => ("iris", "Iris"),
        Leaf = LEAF => ("leaf", "Leaf"),
    }
    default IRIS => "iris";
    roles(primary) {
        surfaces: crate::theme_variant::SurfaceColors {
            background: Self::BASE,
            base: Self::SURFACE,
            raised: Self::OVERLAY,
            overlay: Self::OVERLAY,
            hover: Self::HIGHLIGHT_MED,
        },
        surfaces_alt: crate::theme_variant::SurfaceColors {
            background: Self::NC,
            base: Self::BASE,
            raised: Self::SURFACE,
            overlay: Self::OVERLAY,
            hover: Self::HIGHLIGHT_LOW,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::TEXT,
            muted: Self::SUBTLE,
            subtle: Self::MUTED,
        },
        text_alt: None,
        primary: crate::theme_variant::ActionColors {
            normal: crate::theme_variant::ColorPair::new(
                Self::BASE,
                primary,
            ),
            hover: crate::theme_variant::ColorPair::new(
                Self::BASE,
                crate::palette::primary_hover(primary),
            ),
            pressed: crate::theme_variant::ColorPair::new(
                Self::BASE,
                crate::palette::action_pressed(primary),
            ),
            muted: crate::theme_variant::ColorPair::new(
                Self::MUTED,
                Self::SURFACE,
            ),
            disabled: crate::theme_variant::ColorPair::new(
                Self::MUTED,
                Self::SURFACE,
            ),
        },
        secondary: crate::theme_variant::ActionColors {
            normal: crate::theme_variant::ColorPair::new(
                Self::BASE,
                Self::ROSE,
            ),
            hover: crate::theme_variant::ColorPair::new(
                Self::BASE,
                crate::palette::primary_hover(Self::ROSE),
            ),
            pressed: crate::theme_variant::ColorPair::new(
                Self::BASE,
                crate::palette::action_pressed(Self::ROSE),
            ),
            muted: crate::theme_variant::ColorPair::new(
                Self::MUTED,
                Self::SURFACE,
            ),
            disabled: crate::theme_variant::ColorPair::new(
                Self::MUTED,
                Self::SURFACE,
            ),
        },
        status: crate::theme_variant::StatusColors {
            success: Self::LEAF,
            warning: Self::GOLD,
            error: Self::LOVE,
            critical: Self::LOVE,
            info: Self::FOAM,
            debug: Self::MUTED,
            trace: Self::IRIS,
        },
        border: Self::HIGHLIGHT_HIGH,
        border_muted: Self::HIGHLIGHT_LOW,
        focus: primary,
        chromatic: crate::theme_variant::ChromaticColors {
            red: Self::LOVE,
            orange: Self::GOLD,
            yellow: Self::GOLD,
            green: Self::PINE,
            cyan: Self::ROSE,
            blue: Self::FOAM,
            purple: Self::IRIS,
            pink: Self::LOVE,
        },
        syntax: crate::theme_variant::SyntaxColors {
            attribute: Self::IRIS,
            boolean: Self::ROSE,
            builtin: Self::LOVE,
            builtin_function: Self::ROSE,
            builtin_type: Self::FOAM,
            comment: Self::SUBTLE,
            constant: Self::GOLD,
            control_keyword: Self::PINE,
            deleted: Self::LOVE,
            deprecated: Self::SUBTLE,
            documentation: Self::SUBTLE,
            escape: Self::PINE,
            foreground: Self::TEXT,
            function: Self::ROSE,
            heading: Self::FOAM,
            inserted: Self::FOAM,
            invalid: Self::LOVE,
            keyword: Self::PINE,
            link: Self::IRIS,
            macro_name: Self::IRIS,
            markup_bold: Self::TEXT,
            markup_italic: Self::TEXT,
            modifier: Self::FOAM,
            namespace: Self::PINE,
            number: Self::GOLD,
            operator: Self::SUBTLE,
            parameter: Self::IRIS,
            property: Self::FOAM,
            punctuation: Self::SUBTLE,
            string: Self::GOLD,
            tag: Self::FOAM,
            type_keyword: Self::FOAM,
            type_name: Self::FOAM,
            variable: Self::TEXT,
        },
    }
}
