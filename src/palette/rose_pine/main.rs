//! Rosé Pine Main. See the family module for its upstream source.

define_palette! {
    Main, "Rosé Pine Main",
    colors {
        NC = 0x16141f,
        BASE = 0x191724,
        SURFACE = 0x1f1d2e,
        OVERLAY = 0x26233a,
        MUTED = 0x6e6a86,
        SUBTLE = 0x908caa,
        TEXT = 0xe0def4,
        LOVE = 0xeb6f92,
        GOLD = 0xf6c177,
        ROSE = 0xebbcba,
        PINE = 0x31748f,
        FOAM = 0x9ccfd8,
        IRIS = 0xc4a7e7,
        LEAF = 0x95b1ac,
        HIGHLIGHT_LOW = 0x21202e,
        HIGHLIGHT_MED = 0x403d52,
        HIGHLIGHT_HIGH = 0x524f67,
    }
    accents {
        Love = LOVE,
        Gold = GOLD,
        Rose = ROSE,
        Pine = PINE,
        Foam = FOAM,
        Iris = IRIS,
        Leaf = LEAF,
    }
    default IRIS;
    roles(primary) {
        surface: crate::theme_variant::SurfaceColors {
            background: Self::BASE,
            surface: Self::SURFACE,
            raised: Self::HIGHLIGHT_MED,
            overlay: Self::OVERLAY,
            hover: Self::HIGHLIGHT_MED,
        },
        surface_alt: crate::theme_variant::SurfaceColors {
            background: Self::BASE,
            surface: Self::SURFACE,
            raised: Self::HIGHLIGHT_MED,
            overlay: Self::OVERLAY,
            hover: Self::HIGHLIGHT_MED,
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
            pressed: primary,
            muted: Self::HIGHLIGHT_MED,
        },
        secondary: crate::theme_variant::ActionColors {
            normal: Self::ROSE,
            hover: crate::palette::primary_hover(Self::ROSE),
            pressed: Self::ROSE,
            muted: Self::ROSE,
        },
        status: crate::theme_variant::StatusColors {
            success: Self::LEAF,
            warning: Self::GOLD,
            error: Self::LOVE,
            critical: Self::LOVE,
            info: Self::FOAM,
            trace: Self::MUTED,
        },
        border: Self::HIGHLIGHT_HIGH,
        border_muted: Self::HIGHLIGHT_LOW,
        focus: primary,
    }
}
