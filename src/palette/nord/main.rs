//! Nord. See `REGISTRATION` for pinned upstream sources.
//!
//! Polar Night supplies surfaces. Frost supplies actions and trace; critical repeats the single Aurora error red.

define_palette! {
    Main, "Nord",
    identity("nord/main", "nord", "Nord", "main", "Main", Dark, None);
    support(Dark);
    sources: super::SOURCE;
    colors {
        NORD_0 = crate::color::Color::hex(0x2e3440),
        NORD_1 = crate::color::Color::hex(0x3b4252),
        NORD_2 = crate::color::Color::hex(0x434c5e),
        NORD_3 = crate::color::Color::hex(0x4c566a),
        NORD_4 = crate::color::Color::hex(0xd8dee9),
        NORD_5 = crate::color::Color::hex(0xe5e9f0),
        NORD_6 = crate::color::Color::hex(0xeceff4),
        NORD_7 = crate::color::Color::hex(0x8fbcbb),
        NORD_8 = crate::color::Color::hex(0x88c0d0),
        NORD_9 = crate::color::Color::hex(0x81a1c1),
        NORD_10 = crate::color::Color::hex(0x5e81ac),
        NORD_11 = crate::color::Color::hex(0xbf616a),
        NORD_12 = crate::color::Color::hex(0xd08770),
        NORD_13 = crate::color::Color::hex(0xebcb8b),
        NORD_14 = crate::color::Color::hex(0xa3be8c),
        NORD_15 = crate::color::Color::hex(0xb48ead),
    }
    accents {
        Nord7 = NORD_7 => ("nord_7", "Nord 7"),
        Nord8 = NORD_8 => ("nord_8", "Nord 8"),
        Nord9 = NORD_9 => ("nord_9", "Nord 9"),
        Nord10 = NORD_10 => ("nord_10", "Nord 10"),
        Nord11 = NORD_11 => ("nord_11", "Nord 11"),
        Nord12 = NORD_12 => ("nord_12", "Nord 12"),
        Nord13 = NORD_13 => ("nord_13", "Nord 13"),
        Nord14 = NORD_14 => ("nord_14", "Nord 14"),
        Nord15 = NORD_15 => ("nord_15", "Nord 15"),
    }
    default NORD_8 => "nord_8";
    roles(primary) {
        surface: crate::theme_variant::SurfaceColors {
            background: Self::NORD_0,
            surface: Self::NORD_1,
            raised: Self::NORD_2,
            overlay: Self::NORD_1,
            hover: Self::NORD_3,
        },
        surface_alt: crate::theme_variant::SurfaceColors {
            background: Self::NORD_1,
            surface: Self::NORD_0,
            raised: Self::NORD_1,
            overlay: Self::NORD_1,
            hover: Self::NORD_3,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::NORD_4,
            muted: Self::NORD_5,
            subtle: Self::NORD_3,
            on_primary: crate::palette::action_text(primary),
            on_secondary: crate::palette::action_text(Self::NORD_9),
        },
        primary: crate::theme_variant::ActionColors {
            normal: primary,
            hover: crate::palette::primary_hover(primary),
            pressed: crate::palette::action_pressed(primary),
            muted: Self::NORD_1,
        },
        secondary: crate::theme_variant::ActionColors {
            normal: Self::NORD_9,
            hover: crate::palette::primary_hover(Self::NORD_9),
            pressed: crate::palette::action_pressed(Self::NORD_9),
            muted: Self::NORD_1,
        },
        status: crate::theme_variant::StatusColors {
            success: Self::NORD_14,
            warning: Self::NORD_13,
            error: Self::NORD_11,
            critical: Self::NORD_11,
            info: Self::NORD_8,
            trace: Self::NORD_10,
        },
        border: Self::NORD_3,
        border_muted: Self::NORD_1,
        focus: primary,
    }
}
