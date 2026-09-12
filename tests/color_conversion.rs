use ferriswatch::color::*;
use rstest::rstest;

fn close(actual: f32, expected: f32, tolerance: f32) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{actual} != {expected} (tolerance {tolerance})"
    );
}

fn round_trip<C>(input: [f32; 3], tolerance: f32)
where
    C: ColorSpace,
    LinearSrgb: TryFrom<C>,
{
    let linear = LinearSrgb::new(input[0], input[1], input[2]).unwrap();
    let space = C::try_from_linear_srgb_raw(linear).unwrap();
    let restored = space.try_into_linear_srgb_raw().unwrap();
    for (actual, expected) in [restored.r(), restored.g(), restored.b()]
        .into_iter()
        .zip(input)
    {
        close(actual, expected, tolerance);
    }
}

#[rstest]
#[case([0.0, 0.0, 0.0])]
#[case([1.0, 1.0, 1.0])]
#[case([1.0, 0.0, 0.0])]
#[case([0.0, 1.0, 0.0])]
#[case([0.0, 0.0, 1.0])]
#[case([0.25, 0.5, 0.75])]
#[case([0.003, 0.001, 0.0001])]
fn every_space_round_trips_srgb(#[case] rgb: [f32; 3]) {
    round_trip::<LinearSrgb>(rgb, 0.00001);
    round_trip::<Srgb>(rgb, 0.00001);
    round_trip::<Rgb>(rgb, 0.0045); // 8-bit quantization, measured in linear light.
    round_trip::<A98Rgb>(rgb, 0.00001);
    round_trip::<DisplayP3>(rgb, 0.00001);
    round_trip::<ProPhotoRgb>(rgb, 0.00001);
    round_trip::<Rec2020>(rgb, 0.00001);
    round_trip::<Hsl>(rgb, 0.00001);
    round_trip::<Hsv>(rgb, 0.00001);
    round_trip::<Hwb>(rgb, 0.00001);
    round_trip::<Lab>(rgb, 0.00001);
    round_trip::<Lch>(rgb, 0.00001);
    round_trip::<Oklab>(rgb, 0.00001);
    round_trip::<Oklch>(rgb, 0.00001);
    round_trip::<Lms>(rgb, 0.00001);
    round_trip::<LmsPrime>(rgb, 0.00001);
    round_trip::<Xyz>(rgb, 0.00001);
    round_trip::<XyzD50>(rgb, 0.00001);
    round_trip::<XyzD65>(rgb, 0.00001);
}

#[rstest]
#[case([-0.2, 0.4, 1.3])]
#[case([-0.1, -0.2, -0.3])]
#[case([2.0, 3.0, 4.0])]
fn raw_float_conversions_preserve_extended_colors(#[case] rgb: [f32; 3]) {
    round_trip::<A98Rgb>(rgb, 0.00002);
    round_trip::<DisplayP3>(rgb, 0.00002);
    round_trip::<ProPhotoRgb>(rgb, 0.00002);
    round_trip::<Rec2020>(rgb, 0.00002);
    round_trip::<Hsl>(rgb, 0.00002);
    round_trip::<Hsv>(rgb, 0.00002);
    round_trip::<Hwb>(rgb, 0.00002);
    round_trip::<Lab>(rgb, 0.00002);
    round_trip::<Lch>(rgb, 0.00002);
    round_trip::<Oklab>(rgb, 0.00002);
    round_trip::<Oklch>(rgb, 0.00002);
    round_trip::<Lms>(rgb, 0.00002);
    round_trip::<LmsPrime>(rgb, 0.00002);
    round_trip::<XyzD50>(rgb, 0.00002);
    round_trip::<XyzD65>(rgb, 0.00002);
}

#[test]
fn invalid_public_channels_are_rejected() {
    macro_rules! check {
        ($ty:ident, $a:ident, $b:ident, $c:ident) => {
            for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
                for index in 0..3 {
                    let mut values = [0.5; 3];
                    values[index] = invalid;
                    let value = $ty::new(values[0], values[1], values[2]);
                    let error = value.try_into_linear_srgb_raw().unwrap_err();
                    match error {
                        ColorError::InvalidColorChannel(name, _) => {
                            assert_eq!(
                                name,
                                [stringify!($a), stringify!($b), stringify!($c)][index]
                            );
                        }
                        other => panic!("unexpected error: {other:?}"),
                    }
                    assert!(LinearSrgb::try_from(value).is_err());
                }
            }
        };
    }
    check!(A98Rgb, r, g, b);
    check!(DisplayP3, r, g, b);
    check!(ProPhotoRgb, r, g, b);
    check!(Rec2020, r, g, b);
    check!(Hsl, h, s, l);
    check!(Hsv, h, s, v);
    check!(Hwb, h, w, b);
    check!(Lab, l, a, b);
    check!(Lch, l, c, h);
    check!(Oklab, l, a, b);
    check!(Oklch, l, c, h);
    check!(Lms, l, m, s);
    check!(LmsPrime, l, m, s);
    check!(Xyz, x, y, z);
    check!(XyzD50, x, y, z);
    check!(XyzD65, x, y, z);
}

#[test]
fn overflowing_conversion_is_an_error() {
    assert!(
        Lab::new(f32::MAX, 0.0, 0.0)
            .try_into_linear_srgb_raw()
            .is_err()
    );
    assert!(
        Rec2020::new(f32::MAX, 0.0, 0.0)
            .try_into_linear_srgb_raw()
            .is_err()
    );
    assert!(
        LmsPrime::new(f32::MAX, 0.0, 0.0)
            .try_into_linear_srgb_raw()
            .is_err()
    );
    assert!(Hsv::try_from(LinearSrgb::new(0.0, -1.0, -0.5).unwrap()).is_err());
    assert!(Hsl::try_from(LinearSrgb::new(-1.0, 1.0, 0.0).unwrap()).is_err());
}

#[test]
fn clamp_respects_each_spaces_bounds() {
    assert_eq!(
        DisplayP3::new(-1.0, 0.5, 2.0).clamp(),
        DisplayP3::new(0.0, 0.5, 1.0)
    );
    assert_eq!(
        Hsl::new(-60.0, 2.0, -1.0).clamp(),
        Hsl::new(300.0, 1.0, 0.0)
    );
    assert_eq!(Hwb::new(420.0, 1.0, 1.0).clamp(), Hwb::new(60.0, 0.5, 0.5));
    assert_eq!(
        Lab::new(150.0, -200.0, 200.0).clamp(),
        Lab::new(100.0, -200.0, 200.0)
    );
    assert_eq!(
        Oklch::new(2.0, -1.0, 400.0).clamp(),
        Oklch::new(1.0, 0.0, 40.0)
    );
    assert_eq!(Xyz::new(-1.0, 2.0, 3.0).clamp(), Xyz::new(0.0, 2.0, 3.0));
    let lms = Lms::new(-1.0, 2.0, 3.0);
    assert_eq!(lms.clamp(), lms);
}

#[test]
fn normalized_hwb_and_wrapped_hue() {
    let gray = Hwb::new(123.0, 0.75, 0.75)
        .try_into_linear_srgb_raw()
        .unwrap();
    let expected = Srgb::new(0.5, 0.5, 0.5)
        .unwrap()
        .try_into_linear_srgb_raw()
        .unwrap();
    close(gray.r(), expected.r(), 0.000001);
    close(gray.g(), expected.g(), 0.000001);
    close(gray.b(), expected.b(), 0.000001);
    let rgb = Hsv::new(-120.0, 1.0, 1.0)
        .try_into_linear_srgb_raw()
        .unwrap();
    close(rgb.r(), 0.0, 0.000001);
    close(rgb.g(), 0.0, 0.000001);
    close(rgb.b(), 1.0, 0.000001);
}

#[test]
fn generic_color_and_clamped_conversion_paths() {
    let input = DisplayP3::new(1.0, 0.0, 0.0);
    let color = input.try_into_color().unwrap();
    assert!(color.r() > 1.0 && color.g() < 0.0);
    let polar: Oklch = color.to().unwrap();
    let restored = polar.try_into_color().unwrap();
    close(restored.r(), color.r(), 0.00001);
    let clamped: Rgb = input.clamped_into().unwrap();
    assert_eq!(clamped, Rgb::new(255, 0, 0));
    assert!(Rgb::try_from_color(color).is_err());
    let srgb: Srgb = input.clamped_into().unwrap();
    assert!(srgb.is_in_srgb_gamut());
}

#[test]
fn d50_and_d65_white_points_are_adapted() {
    let white = LinearSrgb::new(1.0, 1.0, 1.0).unwrap();
    let d50 = XyzD50::from(white);
    let d65 = XyzD65::try_from(white).unwrap();
    close(*d50.x, 0.9642957, 0.000001);
    close(*d50.z, 0.8251046, 0.000001);
    close(*d65.x, 0.9504559, 0.000001);
    close(*d65.z, 1.0890578, 0.000001);
    let lab = Lab::try_from(white).unwrap();
    close(*lab.l, 100.0, 0.00001);
    close(*lab.a, 0.0, 0.00001);
    close(*lab.b, 0.0, 0.00001);
}

// Golden values evaluated independently with the W3C CSS Color 4 JavaScript
// sample code on 2026-09-12, for linear sRGB [0.25, 0.5, 0.75].
// https://www.w3.org/TR/css-color-4/#color-conversion-code
#[test]
fn matches_css_reference_values() {
    let linear = LinearSrgb::new(0.25, 0.5, 0.75).unwrap();
    let value = Xyz::try_from(linear).unwrap();
    reference_close(*value.x, 0.41725046, 0.00002);
    reference_close(*value.y, 0.46488833, 0.00002);
    reference_close(*value.z, 0.77732921, 0.00002);
    let value = XyzD65::try_from(linear).unwrap();
    reference_close(*value.x, 0.41725046, 0.00002);
    reference_close(*value.y, 0.46488833, 0.00002);
    reference_close(*value.z, 0.77732921, 0.00002);
    let value = XyzD50::from(linear);
    reference_close(*value.x, 0.40890101, 0.00002);
    reference_close(*value.y, 0.45953166, 0.00002);
    reference_close(*value.z, 0.58759616, 0.00002);
    let value = DisplayP3::from(linear);
    reference_close(*value.r, 0.57882158, 0.00002);
    reference_close(*value.g, 0.72986461, 0.00002);
    reference_close(*value.b, 0.86682821, 0.00002);
    let value = A98Rgb::from(linear);
    reference_close(*value.r, 0.59667639, 0.00002);
    reference_close(*value.g, 0.72965838, 0.00002);
    reference_close(*value.b, 0.87188905, 0.00002);
    let value = ProPhotoRgb::from(linear);
    reference_close(*value.r, 0.60341898, 0.00002);
    reference_close(*value.g, 0.66702156, 0.00002);
    reference_close(*value.b, 0.82812217, 0.00002);
    let value = Rec2020::from(linear);
    reference_close(*value.r, 0.64874360, 0.00002);
    reference_close(*value.g, 0.74006558, 0.00002);
    reference_close(*value.b, 0.87197729, 0.00002);
    let value = Lab::try_from(linear).unwrap();
    reference_close(*value.l, 73.51513438, 0.00002);
    reference_close(*value.a, -10.20038222, 0.00002);
    reference_close(*value.b, -24.26569476, 0.00002);
    let value = Lch::try_from(linear).unwrap();
    reference_close(*value.l, 73.51513438, 0.00002);
    reference_close(*value.c, 26.32245695, 0.00002);
    reference_close(*value.h, 247.19995307, 0.00002);
    let value = Oklab::from(linear);
    reference_close(*value.l, 0.77154682, 0.00002);
    reference_close(*value.a, -0.03680201, 0.00002);
    reference_close(*value.b, -0.06571721, 0.00002);
    let value = Oklch::from(linear);
    reference_close(*value.l, 0.77154682, 0.00002);
    reference_close(*value.c, 0.07532024, 0.00002);
    reference_close(*value.h, 240.75092280, 0.00002);
}

fn reference_close(actual: f32, expected: f64, tolerance: f64) {
    assert!(
        (f64::from(actual) - expected).abs() <= tolerance,
        "{actual} != {expected}"
    );
}

#[test]
fn lms_is_the_oklab_intermediate() {
    let red = LinearSrgb::new(1.0, 0.0, 0.0).unwrap();
    let lms = Lms::from(red);
    // Ottosson's published linear-sRGB -> LMS example matrix.
    // https://bottosson.github.io/posts/oklab/
    close(*lms.l, 0.41222146, 0.000001);
    close(*lms.m, 0.2119035, 0.000001);
    close(*lms.s, 0.08830246, 0.000001);
    let prime = LmsPrime::from(red);
    close(*prime.l, lms.l.cbrt(), 0.000001);
    close(*prime.m, lms.m.cbrt(), 0.000001);
    close(*prime.s, lms.s.cbrt(), 0.000001);
}

#[rstest]
#[case(0.0, [1.0, 0.0, 0.0])]
#[case(60.0, [1.0, 1.0, 0.0])]
#[case(120.0, [0.0, 1.0, 0.0])]
#[case(180.0, [0.0, 1.0, 1.0])]
#[case(240.0, [0.0, 0.0, 1.0])]
#[case(300.0, [1.0, 0.0, 1.0])]
fn hue_sectors_match_rgb_primaries(#[case] h: f32, #[case] expected: [f32; 3]) {
    for color in [
        Hsl::new(h, 1.0, 0.5).try_into_linear_srgb_raw().unwrap(),
        Hsv::new(h, 1.0, 1.0).try_into_linear_srgb_raw().unwrap(),
        Hwb::new(h, 0.0, 0.0).try_into_linear_srgb_raw().unwrap(),
    ] {
        for (actual, expected) in [color.r(), color.g(), color.b()].into_iter().zip(expected) {
            close(actual, expected, 0.000001);
        }
    }
}

#[rstest]
#[case(-720.0, 0.0)]
#[case(-420.0, 300.0)]
#[case(-60.0, 300.0)]
#[case(0.0, 0.0)]
#[case(120.0, 120.0)]
#[case(360.0, 0.0)]
#[case(780.0, 60.0)]
#[case(-f32::EPSILON, 0.0)]
fn clamp_wraps_every_hue_channel(#[case] hue: f32, #[case] expected: f32) {
    let hues = [
        Hsl::new(hue, 0.5, 0.5).clamp().h,
        Hsv::new(hue, 0.5, 0.5).clamp().h,
        Hwb::new(hue, 0.25, 0.25).clamp().h,
        Lch::new(50.0, 200.0, hue).clamp().h,
        Oklch::new(0.5, 2.0, hue).clamp().h,
    ];
    for actual in hues {
        assert_eq!(*actual, expected);
        assert!((0.0..360.0).contains(&*actual));
    }
}

#[test]
fn clamp_leaves_unbounded_channels_unchanged() {
    // Include signed zero and a NaN payload to catch unnecessary conversions.
    for value in [-f32::MAX, f32::MAX, -0.0, f32::from_bits(0x7fc01234)] {
        let lms = Lms::new(value, value, value).clamp();
        let prime = LmsPrime::new(value, value, value).clamp();
        let lab = Lab::new(50.0, value, value).clamp();
        let oklab = Oklab::new(0.5, value, value).clamp();
        for actual in [
            lms.l, lms.m, lms.s, prime.l, prime.m, prime.s, lab.a, lab.b, oklab.a, oklab.b,
        ] {
            assert_eq!(actual.to_bits(), value.to_bits());
        }
    }
    assert_eq!(Lch::new(50.0, 500.0, 20.0).clamp().c.into_value(), 500.0);
    assert_eq!(Oklch::new(0.5, 5.0, 20.0).clamp().c.into_value(), 5.0);
    assert_eq!(Rgb::new(0, 128, 255).clamp(), Rgb::new(0, 128, 255));
}
