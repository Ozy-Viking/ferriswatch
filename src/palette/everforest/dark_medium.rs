//! Everforest Dark Medium. See the family module for its upstream source.

define_palette! {
    DarkMedium, "Everforest Dark Medium",
    identity("everforest/dark/medium", "everforest", "Everforest", "dark", "Dark", Dark, Some(crate::theme_variant::Contrast::Medium));
    support(Dark);
    sources: super::SOURCE;
    colors {
        BG_DIM = crate::color::Color::hex(0x232a2e),
        BG_0 = crate::color::Color::hex(0x2d353b),
        BG_1 = crate::color::Color::hex(0x343f44),
        BG_2 = crate::color::Color::hex(0x3d484d),
        BG_3 = crate::color::Color::hex(0x475258),
        BG_4 = crate::color::Color::hex(0x4f585e),
        BG_5 = crate::color::Color::hex(0x56635f),
        BG_VISUAL = crate::color::Color::hex(0x543a48),
        BG_RED = crate::color::Color::hex(0x514045),
        BG_YELLOW = crate::color::Color::hex(0x4d4c43),
        BG_GREEN = crate::color::Color::hex(0x425047),
        BG_BLUE = crate::color::Color::hex(0x3a515d),
        BG_PURPLE = crate::color::Color::hex(0x4a444e),
        FG = crate::color::Color::hex(0xd3c6aa),
        RED = crate::color::Color::hex(0xe67e80),
        ORANGE = crate::color::Color::hex(0xe69875),
        YELLOW = crate::color::Color::hex(0xdbbc7f),
        GREEN = crate::color::Color::hex(0xa7c080),
        AQUA = crate::color::Color::hex(0x83c092),
        BLUE = crate::color::Color::hex(0x7fbbb3),
        PURPLE = crate::color::Color::hex(0xd699b6),
        GREY_0 = crate::color::Color::hex(0x7a8478),
        GREY_1 = crate::color::Color::hex(0x859289),
        GREY_2 = crate::color::Color::hex(0x9da9a0),
        STATUSLINE_1 = crate::color::Color::hex(0xa7c080),
        STATUSLINE_2 = crate::color::Color::hex(0xd3c6aa),
        STATUSLINE_3 = crate::color::Color::hex(0xe67e80),
    }
    accents {
        Red = RED => ("red", "Red"),
        Orange = ORANGE => ("orange", "Orange"),
        Yellow = YELLOW => ("yellow", "Yellow"),
        Green = GREEN => ("green", "Green"),
        Aqua = AQUA => ("aqua", "Aqua"),
        Blue = BLUE => ("blue", "Blue"),
        Purple = PURPLE => ("purple", "Purple"),
    }
    default GREEN => "green";
    roles(primary) {
        surface: crate::theme_variant::SurfaceColors {
            background: Self::BG_0,
            surface: Self::BG_1,
            raised: Self::BG_2,
            overlay: Self::BG_DIM,
            hover: Self::BG_2,
        },
        surface_alt: crate::theme_variant::SurfaceColors {
            background: Self::BG_DIM,
            surface: Self::BG_0,
            raised: Self::BG_1,
            overlay: Self::BG_DIM,
            hover: Self::BG_2,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::FG,
            muted: Self::GREY_2,
            subtle: Self::GREY_0,
            on_primary: crate::palette::action_text(primary),
            on_secondary: crate::palette::action_text(Self::BLUE),
        },
        primary: crate::theme_variant::ActionColors {
            normal: primary,
            hover: crate::palette::primary_hover(primary),
            pressed: crate::palette::action_pressed(primary),
            muted: Self::BG_2,
        },
        secondary: crate::theme_variant::ActionColors {
            normal: Self::BLUE,
            hover: crate::palette::primary_hover(Self::BLUE),
            pressed: crate::palette::action_pressed(Self::BLUE),
            muted: Self::BG_1,
        },
        status: crate::theme_variant::StatusColors {
            success: Self::GREEN,
            warning: Self::YELLOW,
            error: Self::RED,
            critical: Self::RED,
            info: Self::BLUE,
            trace: Self::GREY_2,
        },
        border: Self::BG_4,
        border_muted: Self::BG_3,
        focus: primary,
    }
}
