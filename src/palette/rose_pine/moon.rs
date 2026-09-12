//! Rosé Pine Moon. See the family module for its upstream source.

define_palette! {
    Moon, "Rosé Pine Moon",
    colors {
        NC = 0x1f1d30,
        BASE = 0x232136,
        SURFACE = 0x2a273f,
        OVERLAY = 0x393552,
        MUTED = 0x6e6a86,
        SUBTLE = 0x908caa,
        TEXT = 0xe0def4,
        LOVE = 0xeb6f92,
        GOLD = 0xf6c177,
        ROSE = 0xea9a97,
        PINE = 0x3e8fb0,
        FOAM = 0x9ccfd8,
        IRIS = 0xc4a7e7,
        LEAF = 0x95b1ac,
        HIGHLIGHT_LOW = 0x2a283e,
        HIGHLIGHT_MED = 0x44415a,
        HIGHLIGHT_HIGH = 0x56526e,
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
