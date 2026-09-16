//! Ayu Dark. See `REGISTRATION` for pinned upstream sources.
//!
//! Ayu v7.0.1 literals preserve common/syntax/VCS vocabulary. Background roles repeat the source common background; no editor-specific generated shades are claimed.

define_palette! {
    Dark, "Ayu Dark",
    identity("ayu/dark", "ayu", "Ayu", "dark", "Dark", Dark, None);
    support(Dark);
    sources: super::SOURCE_DARK;
    colors {
        COMMON_ACCENT = crate::color::Color::hex(0xf29718),
        COMMON_BG = crate::color::Color::hex(0x0d131a),
        COMMON_FG = crate::color::Color::hex(0xbfbab0),
        COMMON_UI = crate::color::Color::hex(0x475059),
        SYNTAX_TAG = crate::color::Color::hex(0x39bae6),
        SYNTAX_FUNC = crate::color::Color::hex(0xffb454),
        SYNTAX_ENTITY = crate::color::Color::hex(0x59c2ff),
        SYNTAX_STRING = crate::color::Color::hex(0xc2d94c),
        SYNTAX_REGEXP = crate::color::Color::hex(0x95e6cb),
        SYNTAX_MARKUP = crate::color::Color::hex(0xf07178),
        SYNTAX_KEYWORD = crate::color::Color::hex(0xff7733),
        SYNTAX_SPECIAL = crate::color::Color::hex(0xe6b673),
        SYNTAX_COMMENT = crate::color::Color::hex(0x626a73),
        SYNTAX_CONSTANT = crate::color::Color::hex(0xffee99),
        SYNTAX_OPERATOR = crate::color::Color::hex(0xf29668),
        SYNTAX_ERROR = crate::color::Color::hex(0xff3333),
        VCS_ADDED = crate::color::Color::hex(0x91b362),
        VCS_MODIFIED = crate::color::Color::hex(0x6994bf),
        VCS_REMOVED = crate::color::Color::hex(0xd96c75),
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
        surfaces: crate::theme_variant::SurfaceColors {
            background: Self::COMMON_BG,
            base: Self::COMMON_BG,
            raised: Self::COMMON_BG,
            overlay: Self::COMMON_BG,
            hover: Self::COMMON_BG,
        },
        surfaces_alt: crate::theme_variant::SurfaceColors {
            background: Self::COMMON_BG,
            base: Self::COMMON_BG,
            raised: Self::COMMON_BG,
            overlay: Self::COMMON_BG,
            hover: Self::COMMON_BG,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::COMMON_FG,
            muted: Self::SYNTAX_COMMENT,
            subtle: Self::COMMON_UI,
        },
        text_alt: None,
primary: crate::theme_variant::ActionColors {
    normal: crate::theme_variant::ColorPair::new(
        crate::palette::action_text(
            primary,
            Self::COMMON_BG,
            Self::COMMON_FG,
        ),
        primary,
    ),
    hover: crate::theme_variant::ColorPair::new(
        crate::palette::action_text(
            primary,
            Self::COMMON_BG,
            Self::COMMON_FG,
        ),
        crate::palette::primary_hover(primary),
    ),
    pressed: crate::theme_variant::ColorPair::new(
        crate::palette::action_text(
            primary,
            Self::COMMON_BG,
            Self::COMMON_FG,
        ),
        crate::palette::action_pressed(primary),
    ),
    muted: crate::theme_variant::ColorPair::new(
        Self::COMMON_FG,
        Self::COMMON_BG,
    ),
    disabled: crate::theme_variant::ColorPair::new(
        Self::COMMON_UI,
        Self::COMMON_BG,
    ),
},
secondary: crate::theme_variant::ActionColors {
    normal: crate::theme_variant::ColorPair::new(
        crate::palette::action_text(
            Self::SYNTAX_ENTITY,
            Self::COMMON_BG,
            Self::COMMON_FG,
        ),
        Self::SYNTAX_ENTITY,
    ),
    hover: crate::theme_variant::ColorPair::new(
        crate::palette::action_text(
            Self::SYNTAX_ENTITY,
            Self::COMMON_BG,
            Self::COMMON_FG,
        ),
        crate::palette::primary_hover(Self::SYNTAX_ENTITY),
    ),
    pressed: crate::theme_variant::ColorPair::new(
        crate::palette::action_text(
            Self::SYNTAX_ENTITY,
            Self::COMMON_BG,
            Self::COMMON_FG,
        ),
        crate::palette::action_pressed(Self::SYNTAX_ENTITY),
    ),
    muted: crate::theme_variant::ColorPair::new(
        Self::COMMON_FG,
        Self::COMMON_BG,
    ),
    disabled: crate::theme_variant::ColorPair::new(
        Self::COMMON_UI,
        Self::COMMON_BG,
    ),
},
        status: crate::theme_variant::StatusColors {
            success: Self::VCS_ADDED,
            warning: Self::SYNTAX_FUNC,
            error: Self::SYNTAX_ERROR,
            critical: Self::VCS_REMOVED,
            info: Self::SYNTAX_TAG,
            debug: Self::SYNTAX_COMMENT,
            trace: Self::COMMON_UI,
        },
        border: Self::COMMON_UI,
        border_muted: Self::COMMON_BG,
        focus: primary,
        chromatic: crate::theme_variant::ChromaticColors {
            red: Self::SYNTAX_MARKUP,
            orange: Self::SYNTAX_KEYWORD,
            yellow: Self::SYNTAX_CONSTANT,
            green: Self::SYNTAX_STRING,
            cyan: Self::SYNTAX_REGEXP,
            blue: Self::SYNTAX_ENTITY,
            purple: Self::SYNTAX_ENTITY,
            pink: Self::SYNTAX_MARKUP,
        },
        syntax: crate::theme_variant::SyntaxColors {
            attribute: Self::SYNTAX_ENTITY,
            boolean: Self::SYNTAX_CONSTANT,
            builtin: Self::SYNTAX_ENTITY,
            builtin_function: Self::SYNTAX_FUNC,
            builtin_type: Self::SYNTAX_ENTITY,
            comment: Self::SYNTAX_COMMENT,
            constant: Self::SYNTAX_CONSTANT,
            control_keyword: Self::SYNTAX_KEYWORD,
            deleted: Self::VCS_REMOVED,
            deprecated: Self::SYNTAX_ERROR,
            documentation: Self::SYNTAX_COMMENT,
            escape: Self::SYNTAX_SPECIAL,
            foreground: Self::COMMON_FG,
            function: Self::SYNTAX_FUNC,
            heading: Self::SYNTAX_MARKUP,
            inserted: Self::VCS_ADDED,
            invalid: Self::SYNTAX_ERROR,
            keyword: Self::SYNTAX_KEYWORD,
            link: Self::SYNTAX_FUNC,
            macro_name: Self::SYNTAX_FUNC,
            markup_bold: Self::COMMON_FG,
            markup_italic: Self::COMMON_FG,
            modifier: Self::SYNTAX_KEYWORD,
            namespace: Self::SYNTAX_ENTITY,
            number: Self::SYNTAX_CONSTANT,
            operator: Self::SYNTAX_OPERATOR,
            parameter: Self::COMMON_FG,
            property: Self::COMMON_FG,
            punctuation: Self::COMMON_FG,
            string: Self::SYNTAX_STRING,
            tag: Self::SYNTAX_TAG,
            type_keyword: Self::SYNTAX_KEYWORD,
            type_name: Self::SYNTAX_ENTITY,
            variable: Self::COMMON_FG,
        },
    }
}
