//! Gruvbox Light Hard. See the family module for its upstream source.

define_palette! {
    LightHard, "Gruvbox Light Hard",
    colors {
        BG_0 = 0xf9f5d7,
        BG_1 = 0xebdbb2,
        BG_2 = 0xd5c4a1,
        BG_3 = 0xbdae93,
        BG_4 = 0xa89984,
        FG_0 = 0x282828,
        FG_1 = 0x3c3836,
        FG_2 = 0x504945,
        FG_3 = 0x665c54,
        FG_4 = 0x7c6f64,
        GRAY = 0x928374,
        RED = 0x9d0006,
        GREEN = 0x79740e,
        YELLOW = 0xb57614,
        BLUE = 0x076678,
        PURPLE = 0x8f3f71,
        AQUA = 0x427b58,
        ORANGE = 0xaf3a03,
    }
    accents {
        Red = RED,
        Green = GREEN,
        Yellow = YELLOW,
        Blue = BLUE,
        Purple = PURPLE,
        Aqua = AQUA,
        Orange = ORANGE,
    }
    default ORANGE;
    roles(primary) {
        surface: crate::theme_variant::SurfaceColors {
            background: Self::BG_0,
            surface: Self::BG_1,
            raised: Self::BG_2,
            overlay: Self::BG_3,
            hover: Self::BG_2,
        },
        surface_alt: crate::theme_variant::SurfaceColors {
            background: Self::BG_0,
            surface: Self::BG_1,
            raised: Self::BG_2,
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
            pressed: primary,
            muted: Self::BG_2,
        },
        secondary: crate::theme_variant::ActionColors {
            normal: Self::BLUE,
            hover: crate::palette::primary_hover(Self::BLUE),
            pressed: Self::BLUE,
            muted: Self::BLUE,
        },
        status: crate::theme_variant::StatusColors {
            success: Self::GREEN,
            warning: Self::YELLOW,
            error: Self::RED,
            critical: Self::RED,
            info: Self::AQUA,
            trace: Self::GRAY,
        },
        border: Self::BG_4,
        border_muted: Self::BG_2,
        focus: primary,
    }
}
