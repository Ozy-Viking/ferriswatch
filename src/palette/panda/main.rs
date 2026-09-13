//! Panda. See `REGISTRATION` for pinned upstream sources.
//!
//! Panda uses white for ordinary text; its variable named foreground is pink. The single opaque background repeats; orange serves warnings and error repeats as critical.

define_palette! {
    Main, "Panda",
    identity("panda/main", "panda", "Panda", "main", "Main", Dark, None);
    sources: super::SOURCE;
    colors {
        BACKGROUND = crate::color::Color::hex(0x292a2b),
        FOREGROUND = crate::color::Color::hex(0xff2c6d),
        COMMENT = crate::color::Color::hex(0x676b79),
        ERROR = crate::color::Color::hex(0xff2c6d),
        KEYWORD = crate::color::Color::hex(0xff75b5),
        WHITE = crate::color::Color::hex(0xf3f3f3),
        LIGHT = crate::color::Color::hex(0xe6e6e6),
        BLUE = crate::color::Color::hex(0x45a9f9),
        LIGHT_BLUE = crate::color::Color::hex(0x6fc1ff),
        PURPLE = crate::color::Color::hex(0xb084eb),
        GREEN = crate::color::Color::hex(0x19f9d8),
        RED = crate::color::Color::hex(0xff2c6d),
        ORANGE = crate::color::Color::hex(0xffb86c),
        PINK = crate::color::Color::hex(0xff75b5),
        LIGHT_PINK = crate::color::Color::hex(0xff9ac1),
    }
    accents {
        Blue = BLUE => ("blue", "Blue"),
        Purple = PURPLE => ("purple", "Purple"),
        Green = GREEN => ("green", "Green"),
        Red = RED => ("red", "Red"),
        Orange = ORANGE => ("orange", "Orange"),
        Pink = PINK => ("pink", "Pink"),
    }
    default PURPLE => "purple";
    roles(primary) {
        surface: crate::theme_variant::SurfaceColors {
            background: Self::BACKGROUND,
            surface: Self::BACKGROUND,
            raised: Self::BACKGROUND,
            overlay: Self::BACKGROUND,
            hover: Self::BACKGROUND,
        },
        surface_alt: crate::theme_variant::SurfaceColors {
            background: Self::BACKGROUND,
            surface: Self::BACKGROUND,
            raised: Self::BACKGROUND,
            overlay: Self::BACKGROUND,
            hover: Self::BACKGROUND,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::WHITE,
            muted: Self::LIGHT,
            subtle: Self::COMMENT,
            on_primary: crate::palette::action_text(primary),
            on_secondary: crate::palette::action_text(Self::PURPLE),
        },
        primary: crate::theme_variant::ActionColors {
            normal: primary,
            hover: crate::palette::primary_hover(primary),
            pressed: crate::palette::action_pressed(primary),
            muted: Self::BACKGROUND,
        },
        secondary: crate::theme_variant::ActionColors {
            normal: Self::PURPLE,
            hover: crate::palette::primary_hover(Self::PURPLE),
            pressed: crate::palette::action_pressed(Self::PURPLE),
            muted: Self::BACKGROUND,
        },
        status: crate::theme_variant::StatusColors {
            success: Self::GREEN,
            warning: Self::ORANGE,
            error: Self::ERROR,
            critical: Self::ERROR,
            info: Self::BLUE,
            trace: Self::COMMENT,
        },
        border: Self::COMMENT,
        border_muted: Self::BACKGROUND,
        focus: primary,
    }
}
