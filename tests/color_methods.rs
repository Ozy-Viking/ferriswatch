use ferriswatch::color::*;

#[test]

fn finite_linear_inputs_have_infallible_named_conversions() {

    // Matrix-only D50 XYZ and LMS transforms are linear. The eight extreme
    // corners bound each output coordinate. Other transforms compress the
    // magnitudes using signed powers/cube roots, or compute bounded hue ratios.
    let values = [
        -f32::MAX,
        -1.0,
        -f32::MIN_POSITIVE,
        -f32::from_bits(1),
        -0.0,
        0.0,
        f32::from_bits(1),
        f32::MIN_POSITIVE,
        0.5,
        1.0,
        f32::MAX,
    ];

    for r in values {

        for g in values {

            for b in values {

                let color = Color::new(r, g, b, 0.25).unwrap();

                let srgb = color.srgb();

                let a98 = color.a98_rgb();

                let p3 = color.display_p3();

                let prophoto = color.prophoto_rgb();

                let rec2020 = color.rec2020();

                let hwb = color.hwb();

                let oklab = color.oklab();

                let oklch = color.oklch();

                let lms = color.lms();

                let prime = color.lms_prime();

                let xyz = color.xyz_d50();

                for coordinates in [
                    [srgb.r(), srgb.g(), srgb.b()],
                    [*a98.r, *a98.g, *a98.b],
                    [*p3.r, *p3.g, *p3.b],
                    [*prophoto.r, *prophoto.g, *prophoto.b],
                    [*rec2020.r, *rec2020.g, *rec2020.b],
                    [*hwb.h, *hwb.w, *hwb.b],
                    [*oklab.l, *oklab.a, *oklab.b],
                    [*oklch.l, *oklch.c, *oklch.h],
                    [*lms.l, *lms.m, *lms.s],
                    [*prime.l, *prime.m, *prime.s],
                    [*xyz.x, *xyz.y, *xyz.z],
                ] {

                    assert!(
                        coordinates.into_iter().all(f32::is_finite),
                        "input: {r}, {g}, {b}; output: {coordinates:?}"
                    );
                }
            }
        }
    }
}

#[test]

fn fallible_named_conversions_preserve_real_failure_cases() {

    assert!(Color::new(2.0, 0.0, 0.0, 1.0).unwrap().rgb().is_err());

    assert!(Color::new(-1.0, 1.0, 0.0, 1.0).unwrap().hsl().is_err());

    assert!(Color::new(0.0, -1.0, -0.5, 1.0).unwrap().hsv().is_err());

    let negative = Color::new(-f32::MAX, -f32::MAX, -f32::MAX, 1.0).unwrap();

    assert!(negative.lab().is_err());

    assert!(negative.lch().is_err());

    let positive = Color::new(f32::MAX, f32::MAX, f32::MAX, 1.0).unwrap();

    assert!(positive.xyz().is_err());

    assert!(positive.xyz_d65().is_err());
}

#[test]

fn clamped_conversion_handles_bytes_and_target_bounds() {

    let color = Color::new(2.0, -0.5, 0.0, 0.25).unwrap();

    assert!(color.to_colorspace::<Rgb>().is_err());

    assert_eq!(color.to_clamped::<Rgb>().unwrap(), Rgb::new(255, 0, 0));

    assert_eq!(
        color.to_clamped::<Srgb>().unwrap(),
        Srgb::new(1.0, 0.0, 0.0).unwrap()
    );

    assert_eq!(
        color.to_clamped::<LinearSrgb>().unwrap(),
        LinearSrgb::new(1.0, 0.0, 0.0).unwrap()
    );

    assert_eq!(color.to_clamped::<Oklch>().unwrap(), color.oklch().clamp());

    assert_eq!(color.to_clamped::<Lms>().unwrap(), color.lms());

    assert_eq!(color.r(), 2.0);

    assert_eq!(color.a(), 0.25);

    let singular = Color::new(0.0, -1.0, -0.5, 1.0).unwrap();

    assert!(singular.to_clamped::<Hsv>().is_err());
}

#[test]

fn try_from_colorspace_preserves_extended_values_and_reports_errors() {

    let color = Color::try_from(Hsl::new(120.0, 0.5, 0.25)).unwrap();

    assert_eq!(color.to_clamped::<Rgb>().unwrap(), Rgb::new(32, 96, 32));

    assert_eq!(color.a(), 1.0);

    let extended = Color::try_from(DisplayP3::new(1.0, 0.0, 0.0)).unwrap();

    assert!(extended.r() > 1.0);

    assert!(extended.g() < 0.0);

    assert!(matches!(
        Color::try_from(Hsl::new(f32::INFINITY, 0.5, 0.25)),
        Err(ColorError::InvalidColorChannel("h", _))
    ));

    assert!(Color::try_from(Lab::new(f32::MAX, 0.0, 0.0)).is_err());

    let linear = LinearSrgb::new(-0.5, 0.0, 1.5).unwrap();

    let stored = Color::from(linear);

    assert_eq!(stored.linear_srgb(), linear.clamp());

    assert_eq!(stored.a(), 1.0);
}

#[test]

fn clamped_from_color_matches_trait_paths_and_rejects_invalid_input() {

    let source = DisplayP3::new(1.0, 0.0, 0.0);

    let color = Color::clamped_from(source).unwrap();

    assert!(color.is_in_srgb_gamut());

    assert_eq!(color.a(), 1.0);

    assert_eq!(
        color,
        Color::try_from(source).unwrap().clamp_to_srgb_gamut()
    );

    let via_into: Color = source.clamped_into().unwrap();

    assert_eq!(via_into, color);

    assert_eq!(
        <Color as ClampedFrom<DisplayP3>>::clamped_from(source).unwrap(),
        color
    );

    assert!(Color::clamped_from(Hsl::new(f32::NAN, 0.5, 0.25)).is_err());

    assert!(Color::clamped_from(Lab::new(f32::MAX, 0.0, 0.0)).is_err());
}

#[test]

fn wide_gamut_intermediates_are_not_clamped_to_srgb() {

    let source = DisplayP3::new(1.0, 0.0, 0.0);

    let stored = Color::try_from(source).unwrap();

    assert!(stored.r() > 1.0 && stored.g() < 0.0 && stored.b() < 0.0);

    let via_into: DisplayP3 = source.clamped_into().unwrap();

    let via_from = DisplayP3::clamped_from(source).unwrap();

    let via_stored = stored.to_clamped::<DisplayP3>().unwrap();

    let via_linear = DisplayP3::try_from_linear_srgb_clamped(stored.linear_srgb()).unwrap();

    for restored in [via_into, via_from, via_stored, via_linear] {

        assert!((*restored.r - 1.0).abs() < 0.000001);

        assert!(restored.g.abs() < 0.000001);

        assert!(restored.b.abs() < 0.000001);
    }

    // Clipping the intermediate produces visibly different P3 coordinates.
    let prematurely_clamped = stored.clamp_to_srgb_gamut().display_p3();

    assert!((*prematurely_clamped.r - 1.0).abs() > 0.01);

    assert!(*prematurely_clamped.g > 0.01);

    let polar = stored.to_colorspace::<Oklch>().unwrap();

    let restored = Color::try_from(polar).unwrap();

    let output = restored.to_clamped::<DisplayP3>().unwrap();

    assert!((*output.r - 1.0).abs() < 0.00001);

    assert!(output.g.abs() < 0.00001);

    assert!(output.b.abs() < 0.00001);
}

#[test]

fn clamped_from_does_not_clamp_source_channels() {

    let source = Hsl::new(20.0, 2.0, 0.5);

    assert!(!source.s.in_bounds());

    // Extended HSL gives encoded RGB (1.5, 1/6, -0.5). Only the
    // destination red and blue should be clipped; green stays at 1/6.
    let expected_green = ((1.0_f64 / 6.0 + 0.055) / 1.055).powf(2.4) as f32;

    let stored = Color::clamped_from(source).unwrap();

    assert_eq!(stored.r(), 1.0);

    assert_eq!(stored.b(), 0.0);

    assert!((stored.g() - expected_green).abs() < 0.000001);

    let converted: LinearSrgb = source.clamped_into().unwrap();

    assert!((converted.g() - expected_green).abs() < 0.000001);

    let prematurely_clamped = Color::clamped_from(source.clamp()).unwrap();

    assert!((prematurely_clamped.g() - stored.g()).abs() > 0.05);
}
