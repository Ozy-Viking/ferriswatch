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
    }
    accents {
        Hue1 = HUE_1 => ("hue_1", "Hue 1"),
        Hue2 = HUE_2 => ("hue_2", "Hue 2"),
        Hue3 = HUE_3 => ("hue_3", "Hue 3"),
        Hue4 = HUE_4 => ("hue_4", "Hue 4"),
        Hue5 = HUE_5 => ("hue_5", "Hue 5"),
        Hue6 = HUE_6 => ("hue_6", "Hue 6"),
    }
    default HUE_2 => "hue_2";
    roles(primary) {
        surface: crate::theme_variant::SurfaceColors {
            background: Self::SYNTAX_BG,
            surface: Self::SYNTAX_BG,
            raised: Self::SYNTAX_BG,
            overlay: Self::SYNTAX_BG,
            hover: Self::SYNTAX_BG,
        },
        surface_alt: crate::theme_variant::SurfaceColors {
            background: Self::SYNTAX_BG,
            surface: Self::SYNTAX_BG,
            raised: Self::SYNTAX_BG,
            overlay: Self::SYNTAX_BG,
            hover: Self::SYNTAX_BG,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::MONO_1,
            muted: Self::MONO_2,
            subtle: Self::MONO_3,
            on_primary: crate::palette::action_text(primary),
            on_secondary: crate::palette::action_text(Self::HUE_3),
        },
        primary: crate::theme_variant::ActionColors {
            normal: primary,
            hover: crate::palette::primary_hover(primary),
            pressed: crate::palette::action_pressed(primary),
            muted: Self::SYNTAX_BG,
        },
        secondary: crate::theme_variant::ActionColors {
            normal: Self::HUE_3,
            hover: crate::palette::primary_hover(Self::HUE_3),
            pressed: crate::palette::action_pressed(Self::HUE_3),
            muted: Self::SYNTAX_BG,
        },
        status: crate::theme_variant::StatusColors {
            success: Self::HUE_4,
            warning: Self::HUE_6_2,
            error: Self::HUE_5,
            critical: Self::HUE_5_2,
            info: Self::HUE_2,
            trace: Self::MONO_3,
        },
        border: Self::MONO_3,
        border_muted: Self::SYNTAX_BG,
        focus: primary,
    }
}
