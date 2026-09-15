use ferriswatch::color::{ColorError, ColorSpace, LinearSrgb, Rgb};
use rstest::rstest;

#[rstest]
#[case("#f5e0dc", 0xf5e0dc)]
#[case("F5e0Dc", 0xf5e0dc)]
#[case("#abc", 0xaabbcc)]
#[case("0Af", 0x00aaff)]
#[case("000000", 0)]
#[case("#ffffff", 0xffffff)]

fn parses_hex(#[case] input: &str, #[case] packed: u32) {

    let expected = Rgb::from_hex(packed);

    assert_eq!(Rgb::from_hex_str(input).unwrap(), expected);

    assert_eq!(input.parse::<Rgb>().unwrap(), expected);

    assert_eq!(Rgb::hex(packed), expected);

    assert_eq!(expected.to_hex(), format!("#{packed:06X}"));
}

#[rstest]
#[case("", ColorError::InvalidLength(0))]
#[case("#abcd", ColorError::InvalidLength(4))]
#[case("#12345678", ColorError::InvalidLength(8))]
#[case("#gggggg", ColorError::InvalidHex)]
#[case(" 12345", ColorError::InvalidHex)]
#[case("0x1234", ColorError::InvalidHex)]
#[case("ééé", ColorError::InvalidHex)]

fn rejects_invalid_hex(#[case] input: &str, #[case] error: ColorError) {

    assert_eq!(Rgb::from_hex_str(input), Err(error));
}

#[test]
#[should_panic(expected = "RGB hex value must fit in 24 bits")]

fn rejects_alpha_in_packed_rgb() {

    Rgb::from_hex(0xff112233);
}

#[test]

fn const_hex_and_colorspace_conversion() {

    const COLOR: Rgb = Rgb::from_hex(0x804020);

    let linear = COLOR.try_into_linear_srgb_raw().unwrap();

    // Encoded sRGB bytes must be decoded to linear light.
    assert!((linear.r() - 0.2158605).abs() < 0.000001);

    assert_eq!(Rgb::try_from_linear_srgb_raw(linear).unwrap(), COLOR);

    let extended = LinearSrgb::new(-0.5, 0.0, 2.0).unwrap();

    assert!(Rgb::try_from_linear_srgb_raw(extended).is_err());

    assert_eq!(
        Rgb::try_from_linear_srgb_clamped(extended).unwrap(),
        Rgb::from_hex(0x0000ff)
    );
}

#[rstest]
#[case("#f5e0dc80", 0xf5e0dc80)]
#[case("F5E0Dc80", 0xf5e0dc80)]
#[case("#abc8", 0xaabbcc88)]
#[case("0Af0", 0x00aaff00)]
#[case("#abc", 0xaabbccff)]
#[case("123456", 0x123456ff)]
#[case("#00000000", 0x00000000)]
#[case("#000000ff", 0x000000ff)]
#[case("#ffffffff", 0xffffffff)]

fn parses_rgba_hex(#[case] input: &str, #[case] packed: u32) {

    use ferriswatch::color::Rgba;

    let expected = Rgba::from_hex(packed);

    assert_eq!(Rgba::from_hex_str(input).unwrap(), expected);

    assert_eq!(input.parse::<Rgba>().unwrap(), expected);

    assert_eq!(Rgba::hex(packed), expected);

    assert_eq!(format!("{expected:#X}"), format!("#{packed:08X}"));
}

#[rstest]
#[case("#", ColorError::InvalidLength(0))]
#[case("12345", ColorError::InvalidLength(5))]
#[case("1234567", ColorError::InvalidLength(7))]
#[case("#1234567g", ColorError::InvalidHex)]
#[case("#abcg", ColorError::InvalidHex)]
#[case("0x123456", ColorError::InvalidHex)]
#[case("éé", ColorError::InvalidHex)]

fn rejects_invalid_rgba_hex(#[case] input: &str, #[case] error: ColorError) {

    assert_eq!(ferriswatch::color::Rgba::from_hex_str(input), Err(error));
}

#[test]

fn rgba_const_constructor_preserves_every_alpha_byte() {

    use ferriswatch::color::Rgba;

    const COLOR: Rgba = Rgba::from_hex(0x12345680);

    assert_eq!(*COLOR.color(), Rgb::from_hex(0x123456));

    assert_eq!(COLOR.alpha_u8(), 128);

    for alpha in 0..=255_u32 {

        let color = Rgba::from_hex(0x12345600 | alpha);

        assert_eq!(color.alpha_u8(), alpha as u8);

        assert!(color.alpha_channel().in_bounds());
    }
}

#[rstest]
#[case("#804020", 0x804020, 255)]
#[case("80402080", 0x804020, 128)]
#[case("#abc", 0xaabbcc, 255)]
#[case("#abc8", 0xaabbcc, 136)]
#[case("#00000080", 0, 128)]
#[case("#00000000", 0, 0)]

fn color_parses_rgb_and_rgba(#[case] input: &str, #[case] rgb: u32, #[case] alpha: u8) {

    use ferriswatch::color::{Color, Rgba};

    let expected = Color::from(Rgba::from_hex((rgb << 8) | u32::from(alpha)));

    let actual = Color::from_hex_str(input).unwrap();

    assert_eq!(actual, expected);

    assert_eq!(actual.rgb().unwrap(), Rgb::from_hex(rgb));

    assert_eq!(actual.a(), f32::from(alpha) / 255.0);

    assert_eq!(Color::hex_str(input).unwrap(), actual);
}

#[test]

fn packed_color_hex_dispatches_by_value() {

    use ferriswatch::color::Color;

    for (packed, text) in [
        (0x804020, "#804020"),
        (0x80402080, "#80402080"),
        (0xffffff, "#ffffff"),
        (0x01000000, "#01000000"),
        (0, "#000000"),
    ] {

        assert_eq!(
            Color::from_hex(packed).unwrap(),
            Color::from_hex_str(text).unwrap()
        );
    }

    assert_ne!(
        Color::from_hex(0x80).unwrap(),
        Color::from_hex_str("#00000080").unwrap()
    );
}

#[rstest]
#[case("#12345", ColorError::InvalidLength(5))]
#[case("#ggg", ColorError::InvalidHex)]
#[case("#gggg", ColorError::InvalidHex)]

fn color_hex_propagates_parse_errors(#[case] input: &str, #[case] error: ColorError) {

    assert_eq!(ferriswatch::color::Color::from_hex_str(input), Err(error));
}

#[test]

fn const_color_construction_matches_runtime_decoding() {

    use ferriswatch::color::{Color, Rgba};

    const ROSEWATER: Color = Color::hex(0xf5e0dc);

    const PACKED: ferriswatch::color::ColorResult<Color> = Color::from_hex(0xf5e0dc80);

    const BLACK: Color = Color::from_rgba8(0, 0, 0, 128);

    assert_eq!(ROSEWATER, Color::from(Rgb::hex(0xf5e0dc)));

    assert_eq!(PACKED.unwrap(), Color::from(Rgba::hex(0xf5e0dc80)));

    assert_eq!(BLACK.a(), 128.0 / 255.0);

    assert_eq!(BLACK.r(), 0.0);

    for byte in 0..=255_u8 {

        let color = Color::from_rgba8(byte, 255 - byte, byte / 2, byte);

        let expected = LinearSrgb::from(Rgb::new(byte, 255 - byte, byte / 2));

        // powf may differ slightly across platforms; the table must agree to f32 precision.
        for (actual, expected) in [
            (color.r(), expected.r()),
            (color.g(), expected.g()),
            (color.b(), expected.b()),
        ] {

            assert!((actual - expected).abs() <= f32::EPSILON);
        }

        assert_eq!(color.a(), f32::from(byte) / 255.0);

        assert_eq!(Rgb::from(color), Rgb::new(byte, 255 - byte, byte / 2));
    }
}

#[test]
#[should_panic(expected = "RGB hex value must fit in 24 bits")]

fn color_hex_rejects_more_than_24_bits() {

    ferriswatch::color::Color::hex(0x01000000);
}

#[test]

fn color_hex_bounds_are_opaque() {

    use ferriswatch::color::Color;

    const BLACK: Color = Color::hex(0x000000);

    const WHITE: Color = Color::hex(0xffffff);

    assert_eq!(BLACK, Color::new(0.0, 0.0, 0.0, 1.0).unwrap());

    assert_eq!(WHITE, Color::new(1.0, 1.0, 1.0, 1.0).unwrap());
}

#[rstest]
#[case(0x00000000)]
#[case(0x00000080)]
#[case(0x00123456)]
#[case(0xf5e0dc80)]
#[case(0xffffffff)]

fn color_hex_alpha_decodes_rgba(#[case] packed: u32) {

    use ferriswatch::color::{Color, Rgba};

    assert_eq!(Color::hex_alpha(packed), Color::from(Rgba::hex(packed)));
}
