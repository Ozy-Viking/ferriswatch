#![cfg(feature = "syntect")]

use ferriswatch::{catalogue::PALETTES, color::Color, integrations::syntect::SyntectTheme};
use rstest::rstest;

#[rstest]
#[case(0x12345600, 0)]
#[case(0x12345680, 128)]
#[case(0x123456ff, 255)]

fn conversion_preserves_rgba_bytes(#[case] rgba: u32, #[case] alpha: u8) {
    let converted: syntect::highlighting::Color = Color::hex_alpha(rgba).into();

    assert_eq!(
        (converted.r, converted.g, converted.b, converted.a),
        (0x12, 0x34, 0x56, alpha)
    );
}

#[test]

fn every_palette_builds_a_syntect_theme() {
    for palette in PALETTES {
        let variant = palette.resolve(None).unwrap();

        let theme = variant.syntect();

        assert_eq!(theme.name.as_deref(), Some(variant.name()));

        assert_eq!(
            theme.settings.foreground,
            Some(variant.colors().syntax.foreground.into())
        );

        assert!(!theme.scopes.is_empty());
    }
}
