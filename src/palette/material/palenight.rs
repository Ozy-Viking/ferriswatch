//! Material Palenight. See `REGISTRATION` for pinned upstream sources.
//!
//! Default style branch with high_visibility disabled. Upstream editor active/highlight/selection colours supply placement and hover. Oceanic is the named Material style, independent of Oceanic Next.

define_palette! {
    Palenight, "Material Palenight",
    identity("material/palenight", "material", "Material", "palenight", "Palenight", Dark, None);
    sources: super::SOURCE_PALENIGHT;
    colors {
        MAIN_WHITE = crate::color::Color::hex(0xeeffff),
        MAIN_GRAY = crate::color::Color::hex(0x717cb4),
        MAIN_BLACK = crate::color::Color::hex(0x000000),
        MAIN_RED = crate::color::Color::hex(0xf07178),
        MAIN_GREEN = crate::color::Color::hex(0xc3e88d),
        MAIN_YELLOW = crate::color::Color::hex(0xffcb6b),
        MAIN_BLUE = crate::color::Color::hex(0x82aaff),
        MAIN_PALEBLUE = crate::color::Color::hex(0xb0c9ff),
        MAIN_CYAN = crate::color::Color::hex(0x89ddff),
        MAIN_PURPLE = crate::color::Color::hex(0xc792ea),
        MAIN_ORANGE = crate::color::Color::hex(0xf78c6c),
        MAIN_DARKRED = crate::color::Color::hex(0xdc6068),
        MAIN_DARKGREEN = crate::color::Color::hex(0xabcf76),
        MAIN_DARKYELLOW = crate::color::Color::hex(0xe6b455),
        MAIN_DARKBLUE = crate::color::Color::hex(0x6e98eb),
        MAIN_DARKCYAN = crate::color::Color::hex(0x71c6e7),
        MAIN_DARKPURPLE = crate::color::Color::hex(0xb480d6),
        MAIN_DARKORANGE = crate::color::Color::hex(0xe2795b),
        EDITOR_BG = crate::color::Color::hex(0x292d3e),
        EDITOR_BG_ALT = crate::color::Color::hex(0x1b1e2b),
        EDITOR_FG = crate::color::Color::hex(0xa6accd),
        EDITOR_FG_DARK = crate::color::Color::hex(0x717cb4),
        EDITOR_SELECTION = crate::color::Color::hex(0x444267),
        EDITOR_CONTRAST = crate::color::Color::hex(0x202331),
        EDITOR_ACTIVE = crate::color::Color::hex(0x414863),
        EDITOR_BORDER = crate::color::Color::hex(0x364367),
        EDITOR_LINE_NUMBERS = crate::color::Color::hex(0x3a3f58),
        EDITOR_HIGHLIGHT = crate::color::Color::hex(0x444267),
        EDITOR_DISABLED = crate::color::Color::hex(0x515772),
        EDITOR_ACCENT = crate::color::Color::hex(0xab47bc),
        SYNTAX_COMMENTS = crate::color::Color::hex(0x676e95),
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
            hover: Self::EDITOR_SELECTION,
        },
        surface_alt: crate::theme_variant::SurfaceColors {
            background: Self::EDITOR_BG_ALT,
            surface: Self::EDITOR_BG,
            raised: Self::EDITOR_ACTIVE,
            overlay: Self::EDITOR_BG,
            hover: Self::EDITOR_SELECTION,
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
            trace: Self::SYNTAX_COMMENTS,
        },
        border: Self::EDITOR_BORDER,
        border_muted: Self::EDITOR_ACTIVE,
        focus: primary,
    }
}
