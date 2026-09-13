//! Everforest Light Hard. See the family module for its upstream source.

define_palette! {
    LightHard, "Everforest Light Hard",
    identity("everforest/light/hard", "everforest", "Everforest", "light", "Light", Light, Some(crate::theme_variant::Contrast::Hard));
    sources: super::SOURCE;
    colors {
        BG_DIM = crate::color::Color::hex(0xf2efdf),
        BG_0 = crate::color::Color::hex(0xfffbef),
        BG_1 = crate::color::Color::hex(0xf8f5e4),
        BG_2 = crate::color::Color::hex(0xf2efdf),
        BG_3 = crate::color::Color::hex(0xedeada),
        BG_4 = crate::color::Color::hex(0xe8e5d5),
        BG_5 = crate::color::Color::hex(0xbec5b2),
        BG_VISUAL = crate::color::Color::hex(0xf0f2d4),
        BG_RED = crate::color::Color::hex(0xffe7de),
        BG_YELLOW = crate::color::Color::hex(0xfef2d5),
        BG_GREEN = crate::color::Color::hex(0xf3f5d9),
        BG_BLUE = crate::color::Color::hex(0xecf5ed),
        BG_PURPLE = crate::color::Color::hex(0xfceced),
        FG = crate::color::Color::hex(0x5c6a72),
        RED = crate::color::Color::hex(0xf85552),
        ORANGE = crate::color::Color::hex(0xf57d26),
        YELLOW = crate::color::Color::hex(0xdfa000),
        GREEN = crate::color::Color::hex(0x8da101),
        AQUA = crate::color::Color::hex(0x35a77c),
        BLUE = crate::color::Color::hex(0x3a94c5),
        PURPLE = crate::color::Color::hex(0xdf69ba),
        GREY_0 = crate::color::Color::hex(0xa6b0a0),
        GREY_1 = crate::color::Color::hex(0x939f91),
        GREY_2 = crate::color::Color::hex(0x829181),
        STATUSLINE_1 = crate::color::Color::hex(0x93b259),
        STATUSLINE_2 = crate::color::Color::hex(0x708089),
        STATUSLINE_3 = crate::color::Color::hex(0xe66868),
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
            trace: Self::FG,
        },
        border: Self::BG_4,
        border_muted: Self::BG_3,
        focus: primary,
    }
}
