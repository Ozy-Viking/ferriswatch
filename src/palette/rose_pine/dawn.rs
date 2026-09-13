//! Rosé Pine Dawn. See the family module for its upstream source.

define_palette! {
    Dawn, "Rosé Pine Dawn",
    identity("rose_pine/dawn", "rose_pine", "Rosé Pine", "dawn", "Dawn", Light, None);
    support(Light);
    sources: super::SOURCE;
    colors {
        NC = crate::color::Color::hex(0xf8f0e7),
        BASE = crate::color::Color::hex(0xfaf4ed),
        SURFACE = crate::color::Color::hex(0xfffaf3),
        OVERLAY = crate::color::Color::hex(0xf2e9e1),
        MUTED = crate::color::Color::hex(0x9893a5),
        SUBTLE = crate::color::Color::hex(0x797593),
        TEXT = crate::color::Color::hex(0x464261),
        LOVE = crate::color::Color::hex(0xb4637a),
        GOLD = crate::color::Color::hex(0xea9d34),
        ROSE = crate::color::Color::hex(0xd7827e),
        PINE = crate::color::Color::hex(0x286983),
        FOAM = crate::color::Color::hex(0x56949f),
        IRIS = crate::color::Color::hex(0x907aa9),
        LEAF = crate::color::Color::hex(0x6d8f89),
        HIGHLIGHT_LOW = crate::color::Color::hex(0xf4ede8),
        HIGHLIGHT_MED = crate::color::Color::hex(0xdfdad9),
        HIGHLIGHT_HIGH = crate::color::Color::hex(0xcecacd),
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
