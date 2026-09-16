use ferriswatch::color::*;
use rstest::rstest;
use std::fmt::{self, Display, Write};

#[rstest]
#[case(Srgb::new(0.25, 0.5, 0.75).unwrap(), "color(srgb 0.250 0.500 0.750)")]
#[case(LinearSrgb::new(0.25, 0.5, 0.75).unwrap(), "color(srgb-linear 0.250 0.500 0.750)")]
#[case(Rgb::new(10, 128, 255), "rgb(10 128 255)")]
#[case(A98Rgb::new(0.25, 0.5, 0.75), "color(a98-rgb 0.250 0.500 0.750)")]
#[case(DisplayP3::new(0.25, 0.5, 0.75), "color(display-p3 0.250 0.500 0.750)")]
#[case(
    ProPhotoRgb::new(0.25, 0.5, 0.75),
    "color(prophoto-rgb 0.250 0.500 0.750)"
)]
#[case(Rec2020::new(0.25, 0.5, 0.75), "color(rec2020 0.250 0.500 0.750)")]
#[case(Hsl::new(120.0, 0.5, 0.25), "hsl(120.000 50.000% 25.000%)")]
#[case(Hsv::new(120.0, 0.5, 0.75), "hwb(120.000 37.500% 25.000%)")]
#[case(Hwb::new(120.0, 0.25, 0.5), "hwb(120.000 25.000% 50.000%)")]
#[case(Lab::new(50.0, -25.0, 40.0), "lab(50.000 -25.000 40.000)")]
#[case(Lch::new(50.0, 40.0, 120.0), "lch(50.000 40.000 120.000)")]
#[case(Oklab::new(0.5, -0.125, 0.25), "oklab(0.500 -0.125 0.250)")]
#[case(Oklch::new(0.5, 0.25, 120.0), "oklch(0.500 0.250 120.000)")]
#[case(Xyz::new(0.25, 0.5, 1.25), "color(xyz 0.250 0.500 1.250)")]
#[case(XyzD50::new(0.25, 0.5, 1.25), "color(xyz-d50 0.250 0.500 1.250)")]
#[case(XyzD65::new(0.25, 0.5, 1.25), "color(xyz-d65 0.250 0.500 1.250)")]
#[case(Lms::new(0.25, 0.5, -0.75), "lms(0.250 0.500 -0.750)")]
#[case(LmsPrime::new(0.25, 0.5, -0.75), "lms-prime(0.250 0.500 -0.750)")]

fn color_and_alpha_display<C>(#[case] color: C, #[case] expected: &str)
where
    C: Display,
    Alpha<C>: Display,
{
    assert_eq!(color.to_string(), expected);

    let alpha = Alpha::new(color, 0.5).unwrap();

    let expected_alpha = format!("{} / 0.500)", expected.strip_suffix(')').unwrap());

    assert_eq!(alpha.to_string(), expected_alpha);
}

#[test]

fn precision_and_extended_values() {
    let color = Alpha::new(Srgb::new(-0.125, 1.25, 0.375).unwrap(), 0.5).unwrap();

    assert_eq!(format!("{color:.2}"), "color(srgb -0.12 1.25 0.38 / 0.50)");

    assert_eq!(format!("{color:.0}"), "color(srgb -0 1 0 / 0)");

    let hsl = Hsl::new(12.345, 0.125, 0.375);

    assert_eq!(format!("{hsl:.1}"), "hsl(12.3 12.5% 37.5%)");
}

#[rstest]
#[case(0.0, 0.0, "hwb(240.000 0.000% 100.000%)")]
#[case(1.0, 0.0, "hwb(240.000 0.000% 100.000%)")]
#[case(0.0, 1.0, "hwb(240.000 100.000% 0.000%)")]
#[case(0.0, 0.5, "hwb(240.000 50.000% 50.000%)")]
#[case(1.0, 1.0, "hwb(240.000 0.000% 0.000%)")]

fn hsv_achromatic_and_saturated_colors(#[case] s: f32, #[case] v: f32, #[case] expected: &str) {
    assert_eq!(Hsv::new(240.0, s, v).to_string(), expected);
}

#[test]

fn non_finite_public_channels_are_missing_css_components() {
    let color = Hsl::new(f32::NAN, f32::INFINITY, f32::NEG_INFINITY);

    assert_eq!(color.to_string(), "hsl(none none none)");
}

#[test]

fn rgb_hex_case_prefix_padding_and_alpha() {
    let rgb = Rgb::new(0, 10, 255);

    assert_eq!(
        format!("{rgb:x} {rgb:X} {rgb:#x} {rgb:#X}"),
        "000aff 000AFF #000aff #000AFF"
    );

    let rgba = Alpha::new(rgb, 0.5).unwrap();

    assert_eq!(
        format!("{rgba:x} {rgba:X} {rgba:#x} {rgba:#X}"),
        "000aff80 000AFF80 #000aff80 #000AFF80"
    );

    assert_eq!(Alpha::opaque(rgb).to_string(), "rgb(0 10 255 / 1.000)");

    assert_eq!(format!("{:#x}", Alpha::new(rgb, 0.0).unwrap()), "#000aff00");

    assert_eq!(format!("{:#X}", Alpha::opaque(rgb)), "#000AFFFF");
}

#[test]

fn srgb_hex_clamps_and_quantizes() {
    let rgb = Srgb::new(-0.25, 0.5, 1.25).unwrap();

    assert_eq!(
        format!("{rgb:x} {rgb:X} {rgb:#x} {rgb:#X}"),
        "0080ff 0080FF #0080ff #0080FF"
    );

    let rgba = Alpha::new(rgb, 0.5).unwrap();

    assert_eq!(
        format!("{rgba:x} {rgba:X} {rgba:#x} {rgba:#X}"),
        "0080ff80 0080FF80 #0080ff80 #0080FF80"
    );
}

#[test]

fn color_retains_linear_encoding_and_alpha() {
    let color = Color::new(0.25, 0.5, 0.75, 0.5).unwrap();

    assert_eq!(
        format!("{color:.2}"),
        "color(srgb-linear 0.25 0.50 0.75 / 0.50)"
    );
}

#[test]

fn propagates_writer_errors() {
    struct FailingWriter;

    impl Write for FailingWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    assert!(write!(FailingWriter, "{}", Oklch::new(0.5, 0.2, 120.0)).is_err());
}
