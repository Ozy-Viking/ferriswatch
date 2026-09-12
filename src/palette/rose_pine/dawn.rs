//! Rosé Pine Dawn. See the family module for its upstream source.

define_palette! {
    Dawn, "Rosé Pine Dawn",
    colors {
        NC = 0xf8f0e7,
        BASE = 0xfaf4ed,
        SURFACE = 0xfffaf3,
        OVERLAY = 0xf2e9e1,
        MUTED = 0x9893a5,
        SUBTLE = 0x797593,
        TEXT = 0x464261,
        LOVE = 0xb4637a,
        GOLD = 0xea9d34,
        ROSE = 0xd7827e,
        PINE = 0x286983,
        FOAM = 0x56949f,
        IRIS = 0x907aa9,
        LEAF = 0x6d8f89,
        HIGHLIGHT_LOW = 0xf4ede8,
        HIGHLIGHT_MED = 0xdfdad9,
        HIGHLIGHT_HIGH = 0xcecacd,
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
