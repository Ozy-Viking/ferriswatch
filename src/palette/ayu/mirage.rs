//! Ayu Mirage. See `REGISTRATION` for pinned upstream sources.
//!
//! Ayu v7.0.1 literals preserve common/syntax/VCS vocabulary. Background roles repeat the source common background; no editor-specific generated shades are claimed.

define_palette! {
    Mirage, "Ayu Mirage",
    identity("ayu/mirage", "ayu", "Ayu", "mirage", "Mirage", Dark, None);
    sources: super::SOURCE_MIRAGE;
    colors {
        COMMON_ACCENT = crate::color::Color::hex(0xffcc66),
        COMMON_BG = crate::color::Color::hex(0x212733),
        COMMON_FG = crate::color::Color::hex(0xcbccc6),
        COMMON_UI = crate::color::Color::hex(0x707a8c),
        SYNTAX_TAG = crate::color::Color::hex(0x5ccfe6),
        SYNTAX_FUNC = crate::color::Color::hex(0xffd580),
        SYNTAX_ENTITY = crate::color::Color::hex(0x73d0ff),
        SYNTAX_STRING = crate::color::Color::hex(0xbae67e),
        SYNTAX_REGEXP = crate::color::Color::hex(0x95e6cb),
        SYNTAX_MARKUP = crate::color::Color::hex(0xf28779),
        SYNTAX_KEYWORD = crate::color::Color::hex(0xffa759),
        SYNTAX_SPECIAL = crate::color::Color::hex(0xffdd99),
        SYNTAX_COMMENT = crate::color::Color::hex(0x5c6773),
        SYNTAX_CONSTANT = crate::color::Color::hex(0xd4bfff),
        SYNTAX_OPERATOR = crate::color::Color::hex(0xf29e74),
        SYNTAX_ERROR = crate::color::Color::hex(0xff3333),
        VCS_ADDED = crate::color::Color::hex(0xa6cc70),
        VCS_MODIFIED = crate::color::Color::hex(0x77a8d9),
        VCS_REMOVED = crate::color::Color::hex(0xf27983),
    }
    accents {
        Tag = SYNTAX_TAG => ("tag", "Tag"),
        Func = SYNTAX_FUNC => ("func", "Func"),
        Entity = SYNTAX_ENTITY => ("entity", "Entity"),
        String = SYNTAX_STRING => ("string", "String"),
        Regexp = SYNTAX_REGEXP => ("regexp", "Regexp"),
        Markup = SYNTAX_MARKUP => ("markup", "Markup"),
        Keyword = SYNTAX_KEYWORD => ("keyword", "Keyword"),
        Special = SYNTAX_SPECIAL => ("special", "Special"),
        Constant = SYNTAX_CONSTANT => ("constant", "Constant"),
        Operator = SYNTAX_OPERATOR => ("operator", "Operator"),
        Accent = COMMON_ACCENT => ("accent", "Accent"),
    }
    default COMMON_ACCENT => "accent";
    roles(primary) {
        surface: crate::theme_variant::SurfaceColors {
            background: Self::COMMON_BG,
            surface: Self::COMMON_BG,
            raised: Self::COMMON_BG,
            overlay: Self::COMMON_BG,
            hover: Self::COMMON_BG,
        },
        surface_alt: crate::theme_variant::SurfaceColors {
            background: Self::COMMON_BG,
            surface: Self::COMMON_BG,
            raised: Self::COMMON_BG,
            overlay: Self::COMMON_BG,
            hover: Self::COMMON_BG,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::COMMON_FG,
            muted: Self::SYNTAX_COMMENT,
            subtle: Self::COMMON_UI,
            on_primary: crate::palette::action_text(primary),
            on_secondary: crate::palette::action_text(Self::SYNTAX_ENTITY),
        },
        primary: crate::theme_variant::ActionColors {
            normal: primary,
            hover: crate::palette::primary_hover(primary),
            pressed: crate::palette::action_pressed(primary),
            muted: Self::COMMON_BG,
        },
        secondary: crate::theme_variant::ActionColors {
            normal: Self::SYNTAX_ENTITY,
            hover: crate::palette::primary_hover(Self::SYNTAX_ENTITY),
            pressed: crate::palette::action_pressed(Self::SYNTAX_ENTITY),
            muted: Self::COMMON_BG,
        },
        status: crate::theme_variant::StatusColors {
            success: Self::VCS_ADDED,
            warning: Self::SYNTAX_FUNC,
            error: Self::SYNTAX_ERROR,
            critical: Self::VCS_REMOVED,
            info: Self::SYNTAX_TAG,
            trace: Self::COMMON_UI,
        },
        border: Self::COMMON_UI,
        border_muted: Self::COMMON_BG,
        focus: primary,
    }
}
