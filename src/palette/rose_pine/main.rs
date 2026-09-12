//! Rosé Pine Main. See the family module for its upstream source.

define_palette! {
    Main, "Rosé Pine",
    identity("rose_pine/main", "rose_pine", "Rosé Pine", "main", "Main", Dark, None);
    sources: crate::palette::sources::ROSE_PINE;
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
        surface: crate::theme_variant::SurfaceColors {
            background: Self::BASE,
            surface: Self::SURFACE,
            raised: Self::OVERLAY,
            overlay: Self::OVERLAY,
            hover: Self::HIGHLIGHT_MED,
        },
        surface_alt: crate::theme_variant::SurfaceColors {
            background: Self::NC,
            surface: Self::BASE,
            raised: Self::SURFACE,
            overlay: Self::OVERLAY,
            hover: Self::HIGHLIGHT_LOW,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::TEXT,
            muted: Self::SUBTLE,
            subtle: Self::MUTED,
            on_primary: crate::palette::action_text(primary),
            on_secondary: crate::palette::action_text(Self::ROSE),
        },
        primary: crate::theme_variant::ActionColors {
            normal: primary,
            hover: crate::palette::primary_hover(primary),
            pressed: crate::palette::action_pressed(primary),
            muted: Self::HIGHLIGHT_MED,
        },
        secondary: crate::theme_variant::ActionColors {
            normal: Self::ROSE,
            hover: crate::palette::primary_hover(Self::ROSE),
            pressed: crate::palette::action_pressed(Self::ROSE),
            muted: Self::SURFACE,
        },
        status: crate::theme_variant::StatusColors {
            success: Self::LEAF,
            warning: Self::GOLD,
            error: Self::LOVE,
            critical: Self::LOVE,
            info: Self::FOAM,
            trace: Self::SUBTLE,
        },
        border: Self::HIGHLIGHT_HIGH,
        border_muted: Self::HIGHLIGHT_LOW,
        focus: primary,
    }
}
