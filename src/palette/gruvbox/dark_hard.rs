//! Gruvbox Dark Hard. See the family module for its upstream source.

define_palette! {
    DarkHard, "Gruvbox Dark Hard",
    identity("gruvbox/dark/hard", "gruvbox", "Gruvbox", "dark", "Dark", Dark, Some(crate::theme_variant::Contrast::Hard));
    sources: crate::palette::sources::GRUVBOX;
    colors {
        BG_0 = crate::color::Color::hex(0x1d2021),
        BG_1 = crate::color::Color::hex(0x3c3836),
        BG_2 = crate::color::Color::hex(0x504945),
        BG_3 = crate::color::Color::hex(0x665c54),
        BG_4 = crate::color::Color::hex(0x7c6f64),
        FG_0 = crate::color::Color::hex(0xfbf1c7),
        FG_1 = crate::color::Color::hex(0xebdbb2),
        FG_2 = crate::color::Color::hex(0xd5c4a1),
        FG_3 = crate::color::Color::hex(0xbdae93),
        FG_4 = crate::color::Color::hex(0xa89984),
        GRAY = crate::color::Color::hex(0x928374),
        RED = crate::color::Color::hex(0xfb4934),
        GREEN = crate::color::Color::hex(0xb8bb26),
        YELLOW = crate::color::Color::hex(0xfabd2f),
        BLUE = crate::color::Color::hex(0x83a598),
        PURPLE = crate::color::Color::hex(0xd3869b),
        AQUA = crate::color::Color::hex(0x8ec07c),
        ORANGE = crate::color::Color::hex(0xfe8019),
    }
    accents {
        Red = RED => ("red", "Red"),
        Green = GREEN => ("green", "Green"),
        Yellow = YELLOW => ("yellow", "Yellow"),
        Blue = BLUE => ("blue", "Blue"),
        Purple = PURPLE => ("purple", "Purple"),
        Aqua = AQUA => ("aqua", "Aqua"),
        Orange = ORANGE => ("orange", "Orange"),
    }
    default ORANGE => "orange";
    roles(primary) {
        surface: crate::theme_variant::SurfaceColors {
            background: Self::BG_0,
            surface: Self::BG_1,
            raised: Self::BG_2,
            overlay: Self::BG_3,
            hover: Self::BG_2,
        },
        surface_alt: crate::theme_variant::SurfaceColors {
            background: Self::BG_1,
            surface: Self::BG_2,
            raised: Self::BG_3,
            overlay: Self::BG_3,
            hover: Self::BG_2,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::FG_1,
            muted: Self::FG_2,
            subtle: Self::GRAY,
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
            info: Self::AQUA,
            trace: Self::FG_4,
        },
        border: Self::BG_4,
        border_muted: Self::BG_2,
        focus: primary,
    }
}
