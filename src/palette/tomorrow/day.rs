//! Tomorrow. See `REGISTRATION` for pinned upstream sources.
//!
//! Original TextMate settings and named scopes. Line highlight supplies hover and selection supplies borders; floating content repeats the canvas when upstream has no distinct popup fill.

define_palette! {
    Day, "Tomorrow",
    identity("tomorrow/day", "tomorrow", "Tomorrow", "day", "Day", Light, None);
    sources: super::SOURCE_DAY;
    colors {
        DEFAULT_BACKGROUND = crate::color::Color::hex(0xffffff),
        DEFAULT_CARET = crate::color::Color::hex(0xaeafad),
        DEFAULT_FOREGROUND = crate::color::Color::hex(0x4d4d4c),
        DEFAULT_INVISIBLES = crate::color::Color::hex(0xd1d1d1),
        DEFAULT_LINE_HIGHLIGHT = crate::color::Color::hex(0xefefef),
        DEFAULT_SELECTION = crate::color::Color::hex(0xd6d6d6),
        COMMENT_FOREGROUND = crate::color::Color::hex(0x8e908c),
        FOREGROUND_FOREGROUND = crate::color::Color::hex(0x666969),
        VARIABLE_STRING_LINK_REGULAR_EXPRESSION_TAG_NAME_GIT_GUTTER_DELETED_FOREGROUND = crate::color::Color::hex(0xc82829),
        NUMBER_CONSTANT_FUNCTION_ARGUMENT_TAG_ATTRIBUTE_EMBEDDED_FOREGROUND = crate::color::Color::hex(0xf5871f),
        CLASS_SUPPORT_FOREGROUND = crate::color::Color::hex(0xc99e00),
        STRING_SYMBOLS_INHERITED_CLASS_MARKUP_HEADING_GIT_GUTTER_INSERTED_FOREGROUND = crate::color::Color::hex(0x718c00),
        OPERATOR_MISC_FOREGROUND = crate::color::Color::hex(0x3e999f),
        FUNCTION_SPECIAL_METHOD_BLOCK_LEVEL_GIT_GUTTER_CHANGED_FOREGROUND = crate::color::Color::hex(0x4271ae),
        KEYWORD_STORAGE_FOREGROUND = crate::color::Color::hex(0x8959a8),
        INVALID_BACKGROUND = crate::color::Color::hex(0xc82829),
        INVALID_FOREGROUND = crate::color::Color::hex(0xffffff),
        SEPARATOR_BACKGROUND = crate::color::Color::hex(0x4271ae),
        SEPARATOR_FOREGROUND = crate::color::Color::hex(0xffffff),
        DEPRECATED_BACKGROUND = crate::color::Color::hex(0x8959a8),
        DEPRECATED_FOREGROUND = crate::color::Color::hex(0xffffff),
        DIFF_FOREGROUND_FOREGROUND = crate::color::Color::hex(0xffffff),
        DIFF_INSERTION_BACKGROUND = crate::color::Color::hex(0x718c00),
        DIFF_DELETION_BACKGROUND = crate::color::Color::hex(0xc82829),
        DIFF_HEADER_FOREGROUND = crate::color::Color::hex(0xffffff),
        DIFF_HEADER_BACKGROUND = crate::color::Color::hex(0x4271ae),
        DIFF_RANGE_FOREGROUND = crate::color::Color::hex(0x3e999f),
    }
    accents {
        Red = VARIABLE_STRING_LINK_REGULAR_EXPRESSION_TAG_NAME_GIT_GUTTER_DELETED_FOREGROUND => ("red", "Red"),
        Orange = NUMBER_CONSTANT_FUNCTION_ARGUMENT_TAG_ATTRIBUTE_EMBEDDED_FOREGROUND => ("orange", "Orange"),
        Yellow = CLASS_SUPPORT_FOREGROUND => ("yellow", "Yellow"),
        Green = STRING_SYMBOLS_INHERITED_CLASS_MARKUP_HEADING_GIT_GUTTER_INSERTED_FOREGROUND => ("green", "Green"),
        Cyan = OPERATOR_MISC_FOREGROUND => ("cyan", "Cyan"),
        Blue = FUNCTION_SPECIAL_METHOD_BLOCK_LEVEL_GIT_GUTTER_CHANGED_FOREGROUND => ("blue", "Blue"),
        Purple = KEYWORD_STORAGE_FOREGROUND => ("purple", "Purple"),
    }
    default FUNCTION_SPECIAL_METHOD_BLOCK_LEVEL_GIT_GUTTER_CHANGED_FOREGROUND => "blue";
    roles(primary) {
        surface: crate::theme_variant::SurfaceColors {
            background: Self::DEFAULT_BACKGROUND,
            surface: Self::DEFAULT_LINE_HIGHLIGHT,
            raised: Self::DEFAULT_LINE_HIGHLIGHT,
            overlay: Self::DEFAULT_BACKGROUND,
            hover: Self::DEFAULT_LINE_HIGHLIGHT,
        },
        surface_alt: crate::theme_variant::SurfaceColors {
            background: Self::DEFAULT_BACKGROUND,
            surface: Self::DEFAULT_BACKGROUND,
            raised: Self::DEFAULT_LINE_HIGHLIGHT,
            overlay: Self::DEFAULT_BACKGROUND,
            hover: Self::DEFAULT_LINE_HIGHLIGHT,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::DEFAULT_FOREGROUND,
            muted: Self::DEFAULT_FOREGROUND,
            subtle: Self::COMMENT_FOREGROUND,
            on_primary: crate::palette::action_text(primary),
            on_secondary: crate::palette::action_text(Self::KEYWORD_STORAGE_FOREGROUND),
        },
        primary: crate::theme_variant::ActionColors {
            normal: primary,
            hover: crate::palette::primary_hover(primary),
            pressed: crate::palette::action_pressed(primary),
            muted: Self::DEFAULT_LINE_HIGHLIGHT,
        },
        secondary: crate::theme_variant::ActionColors {
            normal: Self::KEYWORD_STORAGE_FOREGROUND,
            hover: crate::palette::primary_hover(Self::KEYWORD_STORAGE_FOREGROUND),
            pressed: crate::palette::action_pressed(Self::KEYWORD_STORAGE_FOREGROUND),
            muted: Self::DEFAULT_LINE_HIGHLIGHT,
        },
        status: crate::theme_variant::StatusColors {
            success: Self::STRING_SYMBOLS_INHERITED_CLASS_MARKUP_HEADING_GIT_GUTTER_INSERTED_FOREGROUND,
            warning: Self::CLASS_SUPPORT_FOREGROUND,
            error: Self::VARIABLE_STRING_LINK_REGULAR_EXPRESSION_TAG_NAME_GIT_GUTTER_DELETED_FOREGROUND,
            critical: Self::INVALID_BACKGROUND,
            info: Self::FUNCTION_SPECIAL_METHOD_BLOCK_LEVEL_GIT_GUTTER_CHANGED_FOREGROUND,
            trace: Self::COMMENT_FOREGROUND,
        },
        border: Self::DEFAULT_SELECTION,
        border_muted: Self::DEFAULT_LINE_HIGHLIGHT,
        focus: primary,
    }
}
