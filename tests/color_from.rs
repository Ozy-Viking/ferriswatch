use ferriswatch::color::*;

fn check_from<C>(source: Color)
where
    C: ColorSpace + From<Color> + From<LinearSrgb> + PartialEq + std::fmt::Debug,
    LinearSrgb: TryFrom<C>,
{
    let expected = C::from(source.linear_srgb()).clamp();
    let converted: C = source.into();
    assert_eq!(converted, expected);
    let with_alpha: Alpha<C> = source.into();
    assert_eq!(with_alpha.color(), &expected);
    assert_eq!(with_alpha.alpha(), source.a());
}

#[test]
fn infallible_destinations_support_from_and_into() {
    for source in [
        Color::new(0.25, 0.5, 0.75, 0.25).unwrap(),
        Color::new(-0.25, 2.0, 0.5, 0.0).unwrap(),
        Color::new(f32::MAX, -f32::MAX, f32::MAX, 1.0).unwrap(),
    ] {
        check_from::<LinearSrgb>(source);
        check_from::<Srgb>(source);
        check_from::<A98Rgb>(source);
        check_from::<DisplayP3>(source);
        check_from::<ProPhotoRgb>(source);
        check_from::<Rec2020>(source);
        check_from::<Hwb>(source);
        check_from::<Oklab>(source);
        check_from::<Oklch>(source);
        check_from::<Lms>(source);
        check_from::<LmsPrime>(source);
        check_from::<XyzD50>(source);
    }
}

#[test]
fn byte_rgb_survives_conversion_to_color() {
    for byte in 0..=255_u8 {
        let source = Rgb::new(byte, 255 - byte, byte / 2);
        let color: Color = source.into();
        assert_eq!(color.a(), 1.0);
        assert_eq!(Rgb::from(color), source);
        let linear: LinearSrgb = source.into();
        assert_eq!(linear, color.linear_srgb());
    }
}

#[test]
fn alpha_linear_conversion_clamps_and_preserves_alpha() {
    let source = Color::new(-0.25, 2.0, 0.5, 0.375).unwrap();
    let linear: LinearSrgb = source.into();
    assert_eq!(Color::from(linear).a(), 1.0);
    let with_alpha: LinearSrgba = source.into();
    assert_eq!(Color::from(with_alpha), source.clamp_to_srgb_gamut());
}

#[test]
fn fallible_directions_keep_errors() {
    let outside = Color::new(2.0, 0.5, 0.5, 1.0).unwrap();
    assert!(matches!(
        outside.rgb(),
        Err(ColorError::OutOfSrgbGamut("r", _))
    ));
    let singular = Color::new(0.0, -1.0, -0.5, 1.0).unwrap();
    assert!(Hsv::try_from(singular).is_err());
    assert!(
        Srgb::new(f32::MAX, 0.0, 0.0)
            .unwrap()
            .try_into_color()
            .is_err()
    );
    assert!(Color::try_from(Oklab::new(f32::NAN, 0.0, 0.0)).is_err());
}

#[test]
fn byte_conversion_clamps_quantizes_and_preserves_wrapper_alpha() {
    let source = Color::new(-0.25, 0.5, 2.0, 0.375).unwrap();
    let rgb: Rgb = source.into();
    assert_eq!(rgb, Rgb::new(0, 188, 255));
    let rgba: Rgba = source.into();
    assert_eq!(*rgba.color(), rgb);
    assert_eq!(rgba.alpha(), source.a());
    let restored: Color = rgba.into();
    assert_eq!(restored.a(), source.a());
    assert_eq!(Rgb::from(restored), rgb);
}

#[test]
fn encoded_srgb_conversion_clamps_even_extreme_finite_values() {
    let source = Srgb::new(-f32::MAX, 0.5, f32::MAX).unwrap();
    let color: Color = source.into();
    assert_eq!(color.r(), 0.0);
    assert!((color.g() - 0.21404114).abs() < 0.000001);
    assert_eq!(color.b(), 1.0);
    let alpha: Color = Alpha::new(source, 0.375).unwrap().into();
    assert_eq!(alpha.a(), 0.375);
    assert_eq!(alpha.linear_srgb(), color.linear_srgb());
}
