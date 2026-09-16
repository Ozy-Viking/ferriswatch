//! Gruvbox Dark Medium. See the family module for its upstream source.

define_palette! {
    DarkMedium, "Gruvbox Dark Medium",
    identity("gruvbox/dark/medium", "gruvbox", "Gruvbox", "dark", "Dark", Dark, Some(crate::theme_variant::Contrast::Medium));
    support(Dark);
    sources: super::SOURCE;
    colors {
        BG_0 = crate::color::Color::hex(0x282828),
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
        surfaces: crate::theme_variant::SurfaceColors {
            background: Self::BG_0,
            base: Self::BG_1,
            raised: Self::BG_2,
            overlay: Self::BG_3,
            hover: Self::BG_2,
        },
        surfaces_alt: crate::theme_variant::SurfaceColors {
            background: Self::BG_1,
            base: Self::BG_2,
            raised: Self::BG_3,
            overlay: Self::BG_3,
            hover: Self::BG_2,
        },
        text: crate::theme_variant::TextColors {
            normal: Self::FG_1,
            muted: Self::FG_2,
            subtle: Self::GRAY,
        },
        text_alt: None,
primary: crate::theme_variant::ActionColors {
    normal: crate::theme_variant::ColorPair::new(
        Self::BG_0,
        primary,
    ),
    hover: crate::theme_variant::ColorPair::new(
        Self::BG_0,
        crate::palette::primary_hover(primary),
    ),
    pressed: crate::theme_variant::ColorPair::new(
        Self::BG_0,
        crate::palette::action_pressed(primary),
    ),
    muted: crate::theme_variant::ColorPair::new(
        Self::FG_1,
        Self::BG_2,
    ),
    disabled: crate::theme_variant::ColorPair::new(
        Self::GRAY,
        Self::BG_2,
    ),
},
secondary: crate::theme_variant::ActionColors {
    normal: crate::theme_variant::ColorPair::new(
        Self::BG_0,
        Self::BLUE,
    ),
    hover: crate::theme_variant::ColorPair::new(
        Self::BG_0,
        crate::palette::primary_hover(Self::BLUE),
    ),
    pressed: crate::theme_variant::ColorPair::new(
        Self::BG_0,
        crate::palette::action_pressed(Self::BLUE),
    ),
    muted: crate::theme_variant::ColorPair::new(
        Self::FG_1,
        Self::BG_1,
    ),
    disabled: crate::theme_variant::ColorPair::new(
        Self::GRAY,
        Self::BG_1,
    ),
},
        status: crate::theme_variant::StatusColors {
            success: Self::GREEN,
            warning: Self::YELLOW,
            error: Self::RED,
            critical: Self::RED,
            info: Self::AQUA,
            debug: Self::FG_2,
            trace: Self::FG_4,
        },
        border: Self::BG_4,
        border_muted: Self::BG_2,
        focus: primary,
        chromatic: crate::theme_variant::ChromaticColors {
            red: Self::RED,
            orange: Self::ORANGE,
            yellow: Self::YELLOW,
            green: Self::GREEN,
            cyan: Self::AQUA,
            blue: Self::BLUE,
            purple: Self::PURPLE,
            pink: Self::PURPLE,
        },
        syntax: crate::theme_variant::SyntaxColors {
            attribute: Self::AQUA,
            boolean: Self::PURPLE,
            builtin: Self::ORANGE,
            builtin_function: Self::ORANGE,
            builtin_type: Self::YELLOW,
            comment: Self::GRAY,
            constant: Self::PURPLE,
            control_keyword: Self::RED,
            deleted: Self::RED,
            deprecated: Self::RED,
            documentation: Self::GRAY,
            escape: Self::ORANGE,
            foreground: Self::FG_1,
            function: Self::GREEN,
            heading: Self::GREEN,
            inserted: Self::GREEN,
            invalid: Self::RED,
            keyword: Self::RED,
            link: Self::PURPLE,
            macro_name: Self::AQUA,
            markup_bold: Self::FG_1,
            markup_italic: Self::FG_1,
            modifier: Self::ORANGE,
            namespace: Self::YELLOW,
            number: Self::PURPLE,
            operator: Self::FG_1,
            parameter: Self::FG_1,
            property: Self::FG_1,
            punctuation: Self::FG_1,
            string: Self::GREEN,
            tag: Self::AQUA,
            type_keyword: Self::RED,
            type_name: Self::YELLOW,
            variable: Self::BLUE,
        },
    }
}
