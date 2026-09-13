//! Oceanic Next. See `REGISTRATION` for pinned upstream sources.
//!
//! Original TextMate settings and named scopes. Line highlight supplies hover and selection supplies borders; floating content repeats the canvas when upstream has no distinct popup fill.

define_palette! {
    Main, "Oceanic Next",
    identity("oceanic_next/main", "oceanic_next", "Oceanic Next", "main", "Main", Dark, None);
    support(Dark);
    sources: super::SOURCE;
    colors {
        DEFAULT_ACTIVE_GUIDE = crate::color::Color::hex(0xfbc95a),
        DEFAULT_BACKGROUND = crate::color::Color::hex(0x1b2b34),
        DEFAULT_CARET = crate::color::Color::hex(0xc0c5ce),
        DEFAULT_BLOCK_CARET = crate::color::Color::hex(0xc0c5ce),
        DEFAULT_FOREGROUND = crate::color::Color::hex(0xcdd3de),
        DEFAULT_GUIDE = crate::color::Color::hex(0x65737f),
        DEFAULT_INVISIBLES = crate::color::Color::hex(0x65737e),
        DEFAULT_LINE_HIGHLIGHT = crate::color::Color::hex_alpha(0x65737e55),
        DEFAULT_SELECTION = crate::color::Color::hex(0x4f5b66),
        DEFAULT_STACK_GUIDE = crate::color::Color::hex(0x4f5b66),
        COMMENTS_FOREGROUND = crate::color::Color::hex(0x65737e),
        VARIABLE_FOREGROUND = crate::color::Color::hex(0xcdd3de),
        KEYWORD_STORAGE_FOREGROUND = crate::color::Color::hex(0xc594c5),
        OPERATOR_MISC_FOREGROUND = crate::color::Color::hex(0x5fb3b3),
        TAG_FOREGROUND = crate::color::Color::hex(0xeb606b),
        FUNCTION_SPECIAL_METHOD_BLOCK_LEVEL_FOREGROUND = crate::color::Color::hex(0x6699cc),
        OTHER_VARIABLE_STRING_LINK_FOREGROUND = crate::color::Color::hex(0xf2777a),
        NUMBER_CONSTANT_FUNCTION_ARGUMENT_TAG_ATTRIBUTE_EMBEDDED_FOREGROUND = crate::color::Color::hex(0xf99157),
        STRING_SYMBOLS_INHERITED_CLASS_MARKUP_HEADING_FOREGROUND = crate::color::Color::hex(0x99c794),
        CLASS_SUPPORT_FOREGROUND = crate::color::Color::hex(0xfac863),
        SUB_METHODS_FOREGROUND = crate::color::Color::hex(0xec5f67),
        LANGUAGE_METHODS_FOREGROUND = crate::color::Color::hex(0xec5f67),
        OBJECT_KEYS_FOREGROUND = crate::color::Color::hex(0xcdd3de),
        CLASS_METHODS_FOREGROUND = crate::color::Color::hex(0xd8dee9),
        ATTRIBUTES_FOREGROUND = crate::color::Color::hex(0xbb80b3),
        INSERTED_FOREGROUND = crate::color::Color::hex(0x99c794),
        DELETED_FOREGROUND = crate::color::Color::hex(0xec5f67),
        CHANGED_FOREGROUND = crate::color::Color::hex(0xbb80b3),
        REGULAR_EXPRESSIONS_FOREGROUND = crate::color::Color::hex(0x5fb3b3),
        ESCAPE_CHARACTERS_FOREGROUND = crate::color::Color::hex(0x5fb3b3),
        SEARCH_RESULTS_NUMS_FOREGROUND = crate::color::Color::hex(0xab7967),
        SEARCH_RESULTS_LINES_FOREGROUND = crate::color::Color::hex(0x99c794),
        DECORATORS_FOREGROUND = crate::color::Color::hex(0x6699cc),
        ES_7_BIND_OPERATOR_FOREGROUND = crate::color::Color::hex(0xec5f67),
        JSON_KEY_LEVEL_8_FOREGROUND = crate::color::Color::hex(0xfac863),
        JSON_KEY_LEVEL_7_FOREGROUND = crate::color::Color::hex(0xc594c5),
        JSON_KEY_LEVEL_6_FOREGROUND = crate::color::Color::hex(0xd8dee9),
        JSON_KEY_LEVEL_5_FOREGROUND = crate::color::Color::hex(0x6699cc),
        JSON_KEY_LEVEL_4_FOREGROUND = crate::color::Color::hex(0xab7967),
        JSON_KEY_LEVEL_3_FOREGROUND = crate::color::Color::hex(0xec5f67),
        JSON_KEY_LEVEL_2_FOREGROUND = crate::color::Color::hex(0xf99157),
        JSON_KEY_LEVEL_1_FOREGROUND = crate::color::Color::hex(0xfac863),
        JSON_KEY_LEVEL_0_FOREGROUND = crate::color::Color::hex(0xc594c5),
    }
    accents {
        Red = TAG_FOREGROUND => ("red", "Red"),
        Orange = NUMBER_CONSTANT_FUNCTION_ARGUMENT_TAG_ATTRIBUTE_EMBEDDED_FOREGROUND => ("orange", "Orange"),
        Yellow = CLASS_SUPPORT_FOREGROUND => ("yellow", "Yellow"),
        Green = STRING_SYMBOLS_INHERITED_CLASS_MARKUP_HEADING_FOREGROUND => ("green", "Green"),
        Cyan = OPERATOR_MISC_FOREGROUND => ("cyan", "Cyan"),
        Blue = FUNCTION_SPECIAL_METHOD_BLOCK_LEVEL_FOREGROUND => ("blue", "Blue"),
        Purple = KEYWORD_STORAGE_FOREGROUND => ("purple", "Purple"),
    }
    default FUNCTION_SPECIAL_METHOD_BLOCK_LEVEL_FOREGROUND => "blue";
    roles(primary) {
        surface: crate::theme_variant::SurfaceColors {
            background: Self::DEFAULT_BACKGROUND,
            surface: Self::DEFAULT_BACKGROUND,
            raised: Self::DEFAULT_BACKGROUND,
            overlay: Self::DEFAULT_BACKGROUND,
            hover: Self::DEFAULT_LINE_HIGHLIGHT,
        },
        surface_alt: crate::theme_variant::SurfaceColors {
            background: Self::DEFAULT_BACKGROUND,
            surface: Self::DEFAULT_BACKGROUND,
            raised: Self::DEFAULT_BACKGROUND,
            overlay: Self::DEFAULT_BACKGROUND,
            hover: Self::DEFAULT_LINE_HIGHLIGHT,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::DEFAULT_FOREGROUND,
            muted: Self::DEFAULT_FOREGROUND,
            subtle: Self::COMMENTS_FOREGROUND,
            on_primary: crate::palette::action_text(primary),
            on_secondary: crate::palette::action_text(Self::KEYWORD_STORAGE_FOREGROUND),
        },
        primary: crate::theme_variant::ActionColors {
            normal: primary,
            hover: crate::palette::primary_hover(primary),
            pressed: crate::palette::action_pressed(primary),
            muted: Self::DEFAULT_BACKGROUND,
        },
        secondary: crate::theme_variant::ActionColors {
            normal: Self::KEYWORD_STORAGE_FOREGROUND,
            hover: crate::palette::primary_hover(Self::KEYWORD_STORAGE_FOREGROUND),
            pressed: crate::palette::action_pressed(Self::KEYWORD_STORAGE_FOREGROUND),
            muted: Self::DEFAULT_BACKGROUND,
        },
        status: crate::theme_variant::StatusColors {
            success: Self::STRING_SYMBOLS_INHERITED_CLASS_MARKUP_HEADING_FOREGROUND,
            warning: Self::CLASS_SUPPORT_FOREGROUND,
            error: Self::TAG_FOREGROUND,
            critical: Self::DELETED_FOREGROUND,
            info: Self::FUNCTION_SPECIAL_METHOD_BLOCK_LEVEL_FOREGROUND,
            trace: Self::COMMENTS_FOREGROUND,
        },
        border: Self::DEFAULT_SELECTION,
        border_muted: Self::DEFAULT_BACKGROUND,
        focus: primary,
    }
}
