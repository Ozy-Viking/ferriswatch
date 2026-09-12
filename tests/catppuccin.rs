use ferriswatch::color::Color;
use ferriswatch::palette::NoAccent;
use ferriswatch::palette::catppuccin::{frappe, latte, macchiato, mocha};
use ferriswatch::theme_variant::ThemePalette;

// Expected values from Catppuccin palette 1.8.0, independent of the semantic mapping.
#[test]
fn latte_matches_official_colours() {
    use latte::Latte as P;
    let colours = [
        (P::ROSEWATER, 0xdc8a78),
        (P::FLAMINGO, 0xdd7878),
        (P::PINK, 0xea76cb),
        (P::MAUVE, 0x8839ef),
        (P::RED, 0xd20f39),
        (P::MAROON, 0xe64553),
        (P::PEACH, 0xfe640b),
        (P::YELLOW, 0xdf8e1d),
        (P::GREEN, 0x40a02b),
        (P::TEAL, 0x179299),
        (P::SKY, 0x04a5e5),
        (P::SAPPHIRE, 0x209fb5),
        (P::BLUE, 0x1e66f5),
        (P::LAVENDER, 0x7287fd),
        (P::TEXT, 0x4c4f69),
        (P::SUBTEXT_1, 0x5c5f77),
        (P::SUBTEXT_0, 0x6c6f85),
        (P::OVERLAY_2, 0x7c7f93),
        (P::OVERLAY_1, 0x8c8fa1),
        (P::OVERLAY_0, 0x9ca0b0),
        (P::SURFACE_2, 0xacb0be),
        (P::SURFACE_1, 0xbcc0cc),
        (P::SURFACE_0, 0xccd0da),
        (P::BASE, 0xeff1f5),
        (P::MANTLE, 0xe6e9ef),
        (P::CRUST, 0xdce0e8),
    ];
    for (actual, expected) in colours {
        assert_eq!(actual, Color::hex(expected));
    }
}

#[test]
fn frappe_matches_official_colours() {
    use frappe::Frappe as P;
    let colours = [
        (P::ROSEWATER, 0xf2d5cf),
        (P::FLAMINGO, 0xeebebe),
        (P::PINK, 0xf4b8e4),
        (P::MAUVE, 0xca9ee6),
        (P::RED, 0xe78284),
        (P::MAROON, 0xea999c),
        (P::PEACH, 0xef9f76),
        (P::YELLOW, 0xe5c890),
        (P::GREEN, 0xa6d189),
        (P::TEAL, 0x81c8be),
        (P::SKY, 0x99d1db),
        (P::SAPPHIRE, 0x85c1dc),
        (P::BLUE, 0x8caaee),
        (P::LAVENDER, 0xbabbf1),
        (P::TEXT, 0xc6d0f5),
        (P::SUBTEXT_1, 0xb5bfe2),
        (P::SUBTEXT_0, 0xa5adce),
        (P::OVERLAY_2, 0x949cbb),
        (P::OVERLAY_1, 0x838ba7),
        (P::OVERLAY_0, 0x737994),
        (P::SURFACE_2, 0x626880),
        (P::SURFACE_1, 0x51576d),
        (P::SURFACE_0, 0x414559),
        (P::BASE, 0x303446),
        (P::MANTLE, 0x292c3c),
        (P::CRUST, 0x232634),
    ];
    for (actual, expected) in colours {
        assert_eq!(actual, Color::hex(expected));
    }
}

#[test]
fn macchiato_matches_official_colours() {
    use macchiato::Macchiato as P;
    let colours = [
        (P::ROSEWATER, 0xf4dbd6),
        (P::FLAMINGO, 0xf0c6c6),
        (P::PINK, 0xf5bde6),
        (P::MAUVE, 0xc6a0f6),
        (P::RED, 0xed8796),
        (P::MAROON, 0xee99a0),
        (P::PEACH, 0xf5a97f),
        (P::YELLOW, 0xeed49f),
        (P::GREEN, 0xa6da95),
        (P::TEAL, 0x8bd5ca),
        (P::SKY, 0x91d7e3),
        (P::SAPPHIRE, 0x7dc4e4),
        (P::BLUE, 0x8aadf4),
        (P::LAVENDER, 0xb7bdf8),
        (P::TEXT, 0xcad3f5),
        (P::SUBTEXT_1, 0xb8c0e0),
        (P::SUBTEXT_0, 0xa5adcb),
        (P::OVERLAY_2, 0x939ab7),
        (P::OVERLAY_1, 0x8087a2),
        (P::OVERLAY_0, 0x6e738d),
        (P::SURFACE_2, 0x5b6078),
        (P::SURFACE_1, 0x494d64),
        (P::SURFACE_0, 0x363a4f),
        (P::BASE, 0x24273a),
        (P::MANTLE, 0x1e2030),
        (P::CRUST, 0x181926),
    ];
    for (actual, expected) in colours {
        assert_eq!(actual, Color::hex(expected));
    }
}

#[test]
fn mocha_matches_official_colours() {
    use mocha::Mocha as P;
    let colours = [
        (P::ROSEWATER, 0xf5e0dc),
        (P::FLAMINGO, 0xf2cdcd),
        (P::PINK, 0xf5c2e7),
        (P::MAUVE, 0xcba6f7),
        (P::RED, 0xf38ba8),
        (P::MAROON, 0xeba0ac),
        (P::PEACH, 0xfab387),
        (P::YELLOW, 0xf9e2af),
        (P::GREEN, 0xa6e3a1),
        (P::TEAL, 0x94e2d5),
        (P::SKY, 0x89dceb),
        (P::SAPPHIRE, 0x74c7ec),
        (P::BLUE, 0x89b4fa),
        (P::LAVENDER, 0xb4befe),
        (P::TEXT, 0xcdd6f4),
        (P::SUBTEXT_1, 0xbac2de),
        (P::SUBTEXT_0, 0xa6adc8),
        (P::OVERLAY_2, 0x9399b2),
        (P::OVERLAY_1, 0x7f849c),
        (P::OVERLAY_0, 0x6c7086),
        (P::SURFACE_2, 0x585b70),
        (P::SURFACE_1, 0x45475a),
        (P::SURFACE_0, 0x313244),
        (P::BASE, 0x1e1e2e),
        (P::MANTLE, 0x181825),
        (P::CRUST, 0x11111b),
    ];
    for (actual, expected) in colours {
        assert_eq!(actual, Color::hex(expected));
    }
}

macro_rules! check_flavour {
    ($test:ident, $module:ident, $palette:ident, $name:literal) => {
        #[test]
        fn $test() {
            use $module::*;
            let variants = [
                (
                    $palette::variant::<Rosewater>(),
                    $palette::ROSEWATER,
                    "Rosewater",
                ),
                (
                    $palette::variant::<Flamingo>(),
                    $palette::FLAMINGO,
                    "Flamingo",
                ),
                ($palette::variant::<Pink>(), $palette::PINK, "Pink"),
                ($palette::variant::<Mauve>(), $palette::MAUVE, "Mauve"),
                ($palette::variant::<Red>(), $palette::RED, "Red"),
                ($palette::variant::<Maroon>(), $palette::MAROON, "Maroon"),
                ($palette::variant::<Peach>(), $palette::PEACH, "Peach"),
                ($palette::variant::<Yellow>(), $palette::YELLOW, "Yellow"),
                ($palette::variant::<Green>(), $palette::GREEN, "Green"),
                ($palette::variant::<Teal>(), $palette::TEAL, "Teal"),
                ($palette::variant::<Sky>(), $palette::SKY, "Sky"),
                (
                    $palette::variant::<Sapphire>(),
                    $palette::SAPPHIRE,
                    "Sapphire",
                ),
                ($palette::variant::<Blue>(), $palette::BLUE, "Blue"),
                (
                    $palette::variant::<Lavender>(),
                    $palette::LAVENDER,
                    "Lavender",
                ),
            ];
            for (theme, colour, name) in variants {
                assert_eq!(theme.name(), $name);
                assert_eq!(theme.accent_name(), Some(name));
                assert_eq!(theme.accent(), Some(colour));
                assert_eq!(theme.primary(), colour);
                assert_eq!(theme.focus(), colour);
                assert_ne!(theme.primary_hover(), colour);
                assert_eq!(theme.primary_hover().a(), colour.a());
                assert_eq!(theme.background(), $palette::BASE);
                assert_eq!(theme.text(), $palette::TEXT);
                assert_eq!(theme.surface(), $palette::SURFACE_0);
                assert_eq!(theme.surface_hover(), $palette::SURFACE_1);
                assert_eq!(theme.success(), $palette::GREEN);
                assert_eq!(theme.warning(), $palette::YELLOW);
                assert_eq!(theme.error(), $palette::RED);
            }
            let default = $palette::variant::<NoAccent>();
            assert_eq!(default.name(), $name);
            assert_eq!(default.accent(), None);
            assert_eq!(default.accent_name(), None);
            assert_eq!(default.primary(), $palette::MAUVE);
            assert_eq!(default.focus(), $palette::MAUVE);
            assert_eq!(
                default.primary_hover(),
                $palette::variant::<Mauve>().primary_hover()
            );
        }
    };
}

check_flavour!(latte_variants, latte, Latte, "Catppuccin Latte");
check_flavour!(frappe_variants, frappe, Frappe, "Catppuccin Frappé");
check_flavour!(
    macchiato_variants,
    macchiato,
    Macchiato,
    "Catppuccin Macchiato"
);
check_flavour!(mocha_variants, mocha, Mocha, "Catppuccin Mocha");
