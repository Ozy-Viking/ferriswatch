use ferriswatch::color::*;
use rstest::rstest;
use std::ops::Bound::{Included, Unbounded};

#[test]
fn every_space_configures_channel_names() {
    macro_rules! check {
        ($ty:ident, [$a:ident, $b:ident, $c:ident]) => {
            let color = $ty::new(0.25, 0.5, 0.75);
            assert_eq!(
                [color.$a.name(), color.$b.name(), color.$c.name()],
                [stringify!($a), stringify!($b), stringify!($c)]
            );
            let converted = $ty::try_from(LinearSrgb::new(0.25, 0.5, 0.75).unwrap()).unwrap();
            for (original, converted) in [
                (color.$a, converted.$a),
                (color.$b, converted.$b),
                (color.$c, converted.$c),
            ] {
                assert_eq!(original.name(), converted.name());
                assert_eq!(original.range(), converted.range());
                assert_eq!(original.is_wrapping(), converted.is_wrapping());
            }
        };
    }
    check!(A98Rgb, [r, g, b]);
    check!(DisplayP3, [r, g, b]);
    check!(ProPhotoRgb, [r, g, b]);
    check!(Rec2020, [r, g, b]);
    check!(Hsl, [h, s, l]);
    check!(Hsv, [h, s, v]);
    check!(Hwb, [h, w, b]);
    check!(Lab, [l, a, b]);
    check!(Lch, [l, c, h]);
    check!(Oklab, [l, a, b]);
    check!(Oklch, [l, c, h]);
    check!(Lms, [l, m, s]);
    check!(LmsPrime, [l, m, s]);
    check!(Xyz, [x, y, z]);
    check!(XyzD50, [x, y, z]);
    check!(XyzD65, [x, y, z]);

    let srgb = Srgb::new(-0.25, 0.5, 1.5).unwrap();
    let linear = LinearSrgb::new(-0.25, 0.5, 1.5).unwrap();
    for channels in [
        [srgb.r_channel(), srgb.g_channel(), srgb.b_channel()],
        [linear.r_channel(), linear.g_channel(), linear.b_channel()],
    ] {
        for (channel, name) in channels.into_iter().zip(["r", "g", "b"]) {
            assert_eq!(channel.name(), name);
            assert_eq!(*channel.range(), (Included(0.0), Included(1.0)));
        }
        assert!(!channels[0].in_bounds());
        assert!(channels[1].in_bounds());
        assert!(!channels[2].in_bounds());
    }
    let rgb = Rgb::new(0, 128, 255);
    for (channel, name) in [rgb.0, rgb.1, rgb.2].into_iter().zip(["r", "g", "b"]) {
        assert_eq!(channel.name(), name);
        assert_eq!(*channel.range(), (Included(0), Included(255)));
        assert!(channel.in_bounds());
    }
}

#[rstest]
#[case(Hsl::new(0.0, 0.5, 0.5).h)]
#[case(Hsv::new(0.0, 0.5, 0.5).h)]
#[case(Hwb::new(0.0, 0.25, 0.25).h)]
#[case(Lch::new(50.0, 10.0, 0.0).h)]
#[case(Oklch::new(0.5, 0.1, 0.0).h)]
fn hue_channels_validate_and_wrap_mutations(#[case] mut hue: Channel<f32>) {
    hue.set_value(-30.0);
    assert_eq!(
        hue.value_in_range(),
        Err(ChannelError::OutsideRange("h", -30.0))
    );
    assert_eq!(hue.clamp().value_in_range(), Ok(330.0));
    hue.set_value(360.0);
    assert!(!hue.in_bounds());
    assert_eq!(hue.clamp().value_in_range(), Ok(0.0));
}

#[test]
fn unbounded_axes_and_nonnegative_channels_keep_distinct_bounds() {
    let lab = Lab::new(50.0, -200.0, 200.0);
    let xyz = Xyz::new(-1.0, 2.0, 3.0);
    let polar = Oklch::new(0.5, 500.0, 20.0);
    for axis in [lab.a, lab.b, Lms::new(-1.0, 0.0, 1.0).l] {
        assert_eq!(*axis.range(), (Unbounded, Unbounded));
        assert!(axis.in_bounds());
    }
    for channel in [xyz.x, xyz.y, xyz.z, polar.c] {
        assert_eq!(*channel.range(), (Included(0.0), Unbounded));
    }
    assert!(!xyz.x.in_bounds());
    assert!(xyz.y.in_bounds());
    assert!(polar.c.in_bounds());
}

#[test]
fn alpha_setters_preserve_bounds_and_reject_invalid_values() {
    let mut color = Alpha::new(Rgb::new(10, 20, 30), 0.5).unwrap();
    for invalid in [-0.1, 1.1, f32::NAN, f32::INFINITY] {
        assert!(matches!(
            color.set_alpha(invalid),
            Err(ColorError::InvalidAlpha(_))
        ));
        assert_eq!(color.alpha(), 0.5);
    }
    color.set_alpha_u8(255);
    assert_eq!(color.alpha(), 1.0);
    assert_eq!(color.alpha_channel().name(), "alpha");
    assert_eq!(
        *color.alpha_channel().range(),
        (Included(0.0), Included(1.0))
    );

    let mut color = Color::new(-0.25, 0.5, 1.5, 0.5).unwrap();
    assert!(color.set_a(2.0).is_err());
    assert_eq!(color.a(), 0.5);
    color.set_a(0.25).unwrap();
    assert_eq!(color.a_channel().name(), "alpha");
    assert_eq!(color.linear_srgba().alpha(), 0.25);
    let clamped = color.clamp_to_srgb_gamut();
    assert_eq!(
        [clamped.r(), clamped.g(), clamped.b(), clamped.a()],
        [0.0, 0.5, 1.0, 0.25]
    );
}
