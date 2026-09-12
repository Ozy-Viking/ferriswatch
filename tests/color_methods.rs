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
