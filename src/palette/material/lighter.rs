//! Material Lighter. See `REGISTRATION` for pinned upstream sources.
//!
//! Default style branch with high_visibility disabled. Upstream editor active/highlight/selection colours supply placement and hover. Oceanic is the named Material style, independent of Oceanic Next.

define_palette! {
    Lighter, "Material Lighter",
    identity("material/lighter", "material", "Material", "lighter", "Lighter", Light, None);
    support(Light);
    sources: super::SOURCE_LIGHTER;
    colors {
        MAIN_WHITE = crate::color::Color::hex(0xeeffff),
        MAIN_GRAY = crate::color::Color::hex(0x717cb4),
        MAIN_BLACK = crate::color::Color::hex(0x000000),
        MAIN_RED = crate::color::Color::hex(0xe53935),
        MAIN_GREEN = crate::color::Color::hex(0x91b859),
        MAIN_YELLOW = crate::color::Color::hex(0xf6a434),
        MAIN_BLUE = crate::color::Color::hex(0x6182b8),
        MAIN_PALEBLUE = crate::color::Color::hex(0x8796b0),
        MAIN_CYAN = crate::color::Color::hex(0x39adb5),
        MAIN_PURPLE = crate::color::Color::hex(0x7c4dff),
        MAIN_ORANGE = crate::color::Color::hex(0xf76d47),
        MAIN_DARKRED = crate::color::Color::hex(0xdc6068),
        MAIN_DARKGREEN = crate::color::Color::hex(0xabcf76),
        MAIN_DARKYELLOW = crate::color::Color::hex(0xe6b455),
        MAIN_DARKBLUE = crate::color::Color::hex(0x6e98eb),
        MAIN_DARKCYAN = crate::color::Color::hex(0x71c6e7),
        MAIN_DARKPURPLE = crate::color::Color::hex(0xb480d6),
        MAIN_DARKORANGE = crate::color::Color::hex(0xe2795b),
        EDITOR_FG = crate::color::Color::hex(0x546e7a),
        EDITOR_FG_DARK = crate::color::Color::hex(0x94a7b0),
        EDITOR_SELECTION = crate::color::Color::hex(0x80cbc4),
        EDITOR_LINE_NUMBERS = crate::color::Color::hex(0xcfd8dc),
        EDITOR_ACCENT = crate::color::Color::hex(0x00bcd4),
        SYNTAX_COMMENTS = crate::color::Color::hex(0xaabfc9),
        MAIN_PINK = crate::color::Color::hex(0xff5370),
        EDITOR_BG = crate::color::Color::hex(0xfafafa),
        EDITOR_BG_ALT = crate::color::Color::hex(0xffffff),
        EDITOR_CONTRAST = crate::color::Color::hex(0xeeeeee),
        EDITOR_ACTIVE = crate::color::Color::hex(0xe7e7e8),
        EDITOR_BORDER = crate::color::Color::hex(0xd3e1e8),
        EDITOR_HIGHLIGHT = crate::color::Color::hex(0xe7e7e8),
        EDITOR_DISABLED = crate::color::Color::hex(0xd2d4d5),
        EDITOR_CURSOR = crate::color::Color::hex(0x272727),
        EDITOR_WHITE = crate::color::Color::hex(0xffffff),
        EDITOR_GRAY = crate::color::Color::hex(0x717cb4),
        LSP_ERROR = crate::color::Color::hex(0xff5370),
    }
    accents {
        Red = MAIN_RED => ("red", "Red"),
        Green = MAIN_GREEN => ("green", "Green"),
        Yellow = MAIN_YELLOW => ("yellow", "Yellow"),
        Blue = MAIN_BLUE => ("blue", "Blue"),
        Cyan = MAIN_CYAN => ("cyan", "Cyan"),
        Purple = MAIN_PURPLE => ("purple", "Purple"),
        Orange = MAIN_ORANGE => ("orange", "Orange"),
        Accent = EDITOR_ACCENT => ("accent", "Accent"),
    }
    default EDITOR_ACCENT => "accent";
    roles(primary) {
        surface: crate::theme_variant::SurfaceColors {
            background: Self::EDITOR_BG,
            surface: Self::EDITOR_ACTIVE,
            raised: Self::EDITOR_HIGHLIGHT,
            overlay: Self::EDITOR_BG,
            hover: Self::EDITOR_ACTIVE,
        },
        surface_alt: crate::theme_variant::SurfaceColors {
            background: Self::EDITOR_BG_ALT,
            surface: Self::EDITOR_BG,
            raised: Self::EDITOR_ACTIVE,
            overlay: Self::EDITOR_BG,
            hover: Self::EDITOR_ACTIVE,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::EDITOR_FG,
            muted: Self::EDITOR_FG_DARK,
            subtle: Self::SYNTAX_COMMENTS,
            on_primary: crate::palette::action_text(primary),
            on_secondary: crate::palette::action_text(Self::MAIN_PURPLE),
        },
        primary: crate::theme_variant::ActionColors {
            normal: primary,
            hover: crate::palette::primary_hover(primary),
            pressed: crate::palette::action_pressed(primary),
            muted: Self::EDITOR_ACTIVE,
        },
        secondary: crate::theme_variant::ActionColors {
            normal: Self::MAIN_PURPLE,
            hover: crate::palette::primary_hover(Self::MAIN_PURPLE),
            pressed: crate::palette::action_pressed(Self::MAIN_PURPLE),
            muted: Self::EDITOR_ACTIVE,
        },
        status: crate::theme_variant::StatusColors {
            success: Self::MAIN_GREEN,
            warning: Self::MAIN_YELLOW,
            error: Self::LSP_ERROR,
            critical: Self::MAIN_RED,
            info: Self::MAIN_PALEBLUE,
            trace: Self::EDITOR_FG,
        },
        border: Self::EDITOR_BORDER,
        border_muted: Self::EDITOR_ACTIVE,
        focus: primary,
    }
}
