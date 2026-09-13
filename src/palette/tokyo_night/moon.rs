//! Tokyo Night Moon. See the family module for its upstream source.

define_palette! {
    Moon, "Tokyo Night Moon",
    identity("tokyo_night/moon", "tokyo_night", "Tokyo Night", "moon", "Moon", Dark, None);
    sources: super::SOURCE;
    colors {
        BG = crate::color::Color::hex(0x222436),
        BG_DARK = crate::color::Color::hex(0x1e2030),
        BG_DARK_1 = crate::color::Color::hex(0x191b29),
        BG_HIGHLIGHT = crate::color::Color::hex(0x2f334d),
        BLUE = crate::color::Color::hex(0x82aaff),
        BLUE_0 = crate::color::Color::hex(0x3e68d7),
        BLUE_1 = crate::color::Color::hex(0x65bcff),
        BLUE_2 = crate::color::Color::hex(0x0db9d7),
        BLUE_5 = crate::color::Color::hex(0x89ddff),
        BLUE_6 = crate::color::Color::hex(0xb4f9f8),
        BLUE_7 = crate::color::Color::hex(0x394b70),
        COMMENT = crate::color::Color::hex(0x636da6),
        CYAN = crate::color::Color::hex(0x86e1fc),
        DARK_3 = crate::color::Color::hex(0x545c7e),
        DARK_5 = crate::color::Color::hex(0x737aa2),
        FG = crate::color::Color::hex(0xc8d3f5),
        FG_DARK = crate::color::Color::hex(0x828bb8),
        FG_GUTTER = crate::color::Color::hex(0x3b4261),
        GREEN = crate::color::Color::hex(0xc3e88d),
        GREEN_1 = crate::color::Color::hex(0x4fd6be),
        GREEN_2 = crate::color::Color::hex(0x41a6b5),
        MAGENTA = crate::color::Color::hex(0xc099ff),
        MAGENTA_2 = crate::color::Color::hex(0xff007c),
        ORANGE = crate::color::Color::hex(0xff966c),
        PURPLE = crate::color::Color::hex(0xfca7ea),
        RED = crate::color::Color::hex(0xff757f),
        RED_1 = crate::color::Color::hex(0xc53b53),
        TEAL = crate::color::Color::hex(0x4fd6be),
        TERMINAL_BLACK = crate::color::Color::hex(0x444a73),
        YELLOW = crate::color::Color::hex(0xffc777),
    }
    accents {
        Blue = BLUE => ("blue", "Blue"),
        Cyan = CYAN => ("cyan", "Cyan"),
        Green = GREEN => ("green", "Green"),
        Magenta = MAGENTA => ("magenta", "Magenta"),
        Orange = ORANGE => ("orange", "Orange"),
        Purple = PURPLE => ("purple", "Purple"),
        Red = RED => ("red", "Red"),
        Teal = TEAL => ("teal", "Teal"),
        Yellow = YELLOW => ("yellow", "Yellow"),
    }
    default BLUE => "blue";
    roles(primary) {
        surface: crate::theme_variant::SurfaceColors {
            background: Self::BG,
            surface: Self::BG_DARK,
            raised: Self::BG_HIGHLIGHT,
            overlay: Self::BG_DARK_1,
            hover: Self::BG_HIGHLIGHT,
        },
        surface_alt: crate::theme_variant::SurfaceColors {
            background: Self::BG_DARK_1,
            surface: Self::BG_DARK,
            raised: Self::BG,
            overlay: Self::BG_DARK_1,
            hover: Self::BG_HIGHLIGHT,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::FG,
            muted: Self::FG_DARK,
            subtle: Self::COMMENT,
            on_primary: crate::palette::action_text(primary),
            on_secondary: crate::palette::action_text(Self::MAGENTA),
        },
        primary: crate::theme_variant::ActionColors {
            normal: primary,
            hover: crate::palette::primary_hover(primary),
            pressed: crate::palette::action_pressed(primary),
            muted: Self::BG_HIGHLIGHT,
        },
        secondary: crate::theme_variant::ActionColors {
            normal: Self::MAGENTA,
            hover: crate::palette::primary_hover(Self::MAGENTA),
            pressed: crate::palette::action_pressed(Self::MAGENTA),
            muted: Self::BG_DARK,
        },
        status: crate::theme_variant::StatusColors {
            success: Self::GREEN,
            warning: Self::YELLOW,
            error: Self::RED_1,
            critical: Self::RED,
            info: Self::BLUE_2,
            trace: Self::FG_DARK,
        },
        border: Self::FG_GUTTER,
        border_muted: Self::BG_HIGHLIGHT,
        focus: primary,
    }
}
