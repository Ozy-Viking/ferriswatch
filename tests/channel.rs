use ferriswatch::color::{Channel, ChannelError, Clamp};
use rstest::rstest;
use std::ops::Bound::{Excluded, Included, Unbounded};

#[rstest]
#[case(-1.0, 0.0)]
#[case(0.0, 0.0)]
#[case(0.5, 0.5)]
#[case(1.0, 1.0)]
#[case(2.0, 1.0)]
#[case(f32::NEG_INFINITY, 0.0)]
#[case(f32::INFINITY, 1.0)]
fn bounded_channels(#[case] value: f32, #[case] expected: f32) {
    let channel = Channel::new("red", value, 0.0..=1.0).unwrap().clamp();
    assert_eq!(*channel.value(), expected);
    assert!(channel.in_bounds());
}

#[rstest]
#[case(-720.0, 0.0)]
#[case(-30.0, 330.0)]
#[case(360.0, 0.0)]
#[case(750.0, 30.0)]
#[case(-f32::EPSILON, 0.0)]
fn circular_channels(#[case] value: f32, #[case] expected: f32) {
    let channel = Channel::new("hue", value, 0.0..360.0)
        .unwrap()
        .with_wrapping()
        .unwrap()
        .clamp();
    assert_eq!(*channel.value(), expected);
    assert!(channel.in_bounds());
    assert_eq!(channel.clamp(), channel);
}

#[test]
fn shifted_wrapping_and_large_values() {
    for (value, expected) in [(190.0, -170.0), (-190.0, 170.0), (180.0, -180.0)] {
        let channel = Channel::new("angle", value, -180.0..180.0)
            .unwrap()
            .with_wrapping()
            .unwrap()
            .clamp();
        assert_eq!(*channel.value(), expected);
    }
    // f32::MAX is an integer multiple of 360. Subtracting 20 before modulo
    // would lose the offset even in f64, yielding the wrong result of 20.
    let channel = Channel::new("angle", f32::MAX, 20.0..380.0)
        .unwrap()
        .with_wrapping()
        .unwrap()
        .clamp();
    assert_eq!(*channel.value(), 360.0);
    // The width must now be representable in T, without implicit promotion.
    assert!(matches!(
        Channel::new("angle", f32::MAX, -f32::MAX..f32::MAX)
            .unwrap()
            .with_wrapping(),
        Err(ChannelError::InvalidWrappingRange("angle", (Included(start), Excluded(end)), _))
            if start == -f32::MAX && end == f32::MAX
    ));
}

#[test]
fn excluded_and_one_sided_bounds() {
    let channel = Channel::new("fraction", 1.0, 0.0..1.0).unwrap().clamp();
    assert_eq!(*channel.value(), 1.0_f32.next_down());
    assert!(channel.in_bounds());
    let channel = Channel::new("positive", 0.0, (Excluded(0.0), Unbounded))
        .unwrap()
        .clamp();
    assert_eq!(*channel.value(), 0.0_f32.next_up());
    assert!(channel.in_bounds());
    assert_eq!(
        Channel::new("chroma", 500.0, 0.0..)
            .unwrap()
            .clamp()
            .into_value(),
        500.0
    );
    assert_eq!(
        Channel::new("ceiling", 2.0, ..=1.0)
            .unwrap()
            .clamp()
            .into_value(),
        1.0
    );
}

#[test]
fn unbounded_channels_preserve_bits() {
    for value in [
        f32::MAX,
        -f32::MAX,
        -0.0,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::from_bits(0x7fc01234),
    ] {
        let channel = Channel::new("lms", value, ..).unwrap();
        assert_eq!(channel.in_bounds(), !value.is_nan());
        assert_eq!(channel.clamp().into_value().to_bits(), value.to_bits());
    }
}

#[test]
fn nonfinite_wrapping_values_remain_invalid() {
    for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let channel = Channel::new("hue", value, 0.0..360.0)
            .unwrap()
            .with_wrapping()
            .unwrap()
            .clamp();
        assert!(!channel.in_bounds());
        assert_eq!(channel.into_value().to_bits(), value.to_bits());
    }
}

#[test]
fn invalid_ranges_are_rejected() {
    for bounds in [
        (Included(2.0), Included(1.0)),
        (Included(f32::NAN), Unbounded),
        (Unbounded, Excluded(f32::NAN)),
        (Included(0.0), Excluded(0.0)),
        (Excluded(0.0), Excluded(0.0_f32.next_up())),
        (Excluded(f32::INFINITY), Unbounded),
        (Unbounded, Excluded(f32::NEG_INFINITY)),
    ] {
        assert!(matches!(
            Channel::new("test", 0.0, bounds),
            Err(ChannelError::InvalidRange("test", actual, _))
                if format!("{actual:?}") == format!("{bounds:?}")
        ));
    }
    assert!(
        Channel::new("singleton", 0.0, 1.0..=1.0)
            .unwrap()
            .clamp()
            .in_bounds()
    );
}

#[test]
fn wrapping_requires_finite_half_open_bounds() {
    for bounds in [
        (Unbounded, Unbounded),
        (Included(0.0), Included(360.0)),
        (Excluded(0.0), Excluded(360.0)),
        (Included(f32::NEG_INFINITY), Excluded(360.0)),
        (Included(0.0), Excluded(f32::INFINITY)),
    ] {
        assert!(matches!(
            Channel::new("hue", 0.0, bounds).unwrap().with_wrapping(),
            Err(ChannelError::InvalidWrappingRange("hue", actual, _)) if actual == bounds
        ));
    }
}

#[test]
fn accessors_and_replacement() {
    let mut channel = Channel::new("red", 0.5, 0.0..=1.0).unwrap();
    assert_eq!(channel.name(), "red");
    assert_eq!(channel.range(), &(Included(0.0), Included(1.0)));
    assert!(!channel.is_wrapping());
    channel.set_value(2.0);
    assert!(!channel.in_bounds());
    assert_eq!(channel.clamp().into_value(), 1.0);
}

#[rstest]
#[case(0.0, Ok(0.0))]
#[case(0.5, Ok(0.5))]
#[case(1.0, Ok(1.0))]
#[case(-0.1, Err(ChannelError::OutsideRange("red", -0.1)))]
#[case(1.1, Err(ChannelError::OutsideRange("red", 1.1)))]
fn value_in_range_checks_bounds(#[case] value: f32, #[case] expected: Result<f32, ChannelError>) {
    let channel = Channel::new("red", value, 0.0..=1.0).unwrap();
    assert_eq!(channel.value_in_range(), expected);
    assert_eq!(channel.into_value().to_bits(), value.to_bits());
}

#[test]
fn value_in_range_does_not_wrap() {
    let hue = Channel::new("hue", 360.0, 0.0..360.0)
        .unwrap()
        .with_wrapping()
        .unwrap();
    assert_eq!(
        hue.value_in_range(),
        Err(ChannelError::OutsideRange("hue", 360.0))
    );
    assert_eq!(hue.clamp().value_in_range(), Ok(0.0));
    let unbounded = Channel::new("lms", -500.0, ..).unwrap();
    assert_eq!(unbounded.value_in_range(), Ok(-500.0));
}

#[test]
fn outside_range_error_retains_name_and_value() {
    let value = f32::from_bits(0x7fc01234);
    let channel = Channel::new("red", value, 0.0..=1.0).unwrap();
    match channel.value_in_range() {
        Err(ChannelError::OutsideRange(name, rejected)) => {
            assert_eq!(name, "red");
            assert_eq!(rejected.to_bits(), value.to_bits());
        }
        other => panic!("unexpected result: {other:?}"),
    }
    assert_eq!(
        ChannelError::OutsideRange("hue", 360.0).to_string(),
        "value outside range for channel hue: 360"
    );
}

#[test]
fn range_error_messages_survive_color_error_conversion() {
    let invalid = Channel::new("red", 0.0_f32, 2.0..=1.0).unwrap_err();
    let wrapping = Channel::new("hue", 0.0_f32, 0.0..=360.0)
        .unwrap()
        .with_wrapping()
        .unwrap_err();
    for (error, expected) in [
        (
            invalid,
            "invalid range for channel red: (Included(2), Included(1)): the lower bound is greater than the upper bound",
        ),
        (
            wrapping,
            "invalid wrapping range for channel hue: (Included(0), Included(360)): wrapping requires an included start and excluded end (start..end)",
        ),
    ] {
        assert_eq!(error.to_string(), expected);
        let outer = ferriswatch::color::ColorError::from(error);
        assert_eq!(outer.to_string(), expected);
    }
}

#[test]
fn builder_preserves_values_and_accepts_configuration_in_any_order() {
    let red = Channel::with_name("R")
        .with_range(0..=255)
        .with_value(128_u8)
        .build()
        .unwrap();
    assert_eq!(red.name(), "R");
    assert_eq!(red.value_in_range(), Ok(128));
    assert_eq!(format!("{red:02X}"), "80");

    let hue = Channel::with_name("hue")
        .with_wrapping()
        .with_range(0.0..360.0)
        .with_value(-30.0_f32)
        .build()
        .unwrap();
    assert_eq!(*hue.value(), -30.0);
    assert_eq!(hue.clamp().into_value(), 330.0);
    let other_order = Channel::with_name("hue")
        .with_value(-30.0_f32)
        .with_range(0.0..360.0)
        .with_wrapping()
        .build()
        .unwrap();
    assert_eq!(other_order, hue);
}

#[test]
fn builder_defaults_to_unbounded_and_allows_reconfiguration() {
    let free = Channel::with_name("free")
        .with_value(-12_i32)
        .build()
        .unwrap();
    assert!(free.in_bounds());
    assert_eq!(free.min_value(), None);
    assert_eq!(free.max_value(), None);
    assert!(!free.is_wrapping());

    let changed = Channel::with_name("fraction")
        .with_value(5_u8)
        .with_range((Included(2), Included(1)))
        .with_value(1.5_f64)
        .with_range(0.0..=1.0)
        .build()
        .unwrap();
    assert_eq!(*changed.value(), 1.5);
    assert_eq!(changed.clamp().into_value(), 1.0);
}

#[test]
fn builder_does_not_require_arithmetic_without_wrapping() {
    use ferriswatch::color::AdjacentValue;
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    struct Level(u8);
    impl AdjacentValue for Level {
        fn next_value(self) -> Option<Self> {
            self.0.checked_add(1).map(Self)
        }
        fn previous_value(self) -> Option<Self> {
            self.0.checked_sub(1).map(Self)
        }
    }
    let channel = Channel::with_name("level")
        .with_range(Level(1)..=Level(3))
        .with_value(Level(5))
        .build()
        .unwrap();
    let mut channel = channel;
    channel.set_max_value(Some(Level(2))).unwrap();
    assert_eq!(channel.clamp().into_value(), Level(2));
    channel.set_min_value(None).unwrap();
    assert_eq!(channel.min_value(), None);
}

#[rstest]
#[case((Included(2.0), Included(1.0)), ferriswatch::color::RangeErrorReason::ReversedBounds)]
#[case((Included(f32::NAN), Unbounded), ferriswatch::color::RangeErrorReason::UnorderedBounds)]
#[case((Unbounded, Excluded(f32::NAN)), ferriswatch::color::RangeErrorReason::UnorderedBounds)]
#[case((Included(0.0), Excluded(0.0)), ferriswatch::color::RangeErrorReason::EmptyRange)]
#[case((Excluded(0.0), Excluded(0.0_f32.next_up())), ferriswatch::color::RangeErrorReason::EmptyRange)]
#[case((Excluded(f32::INFINITY), Unbounded), ferriswatch::color::RangeErrorReason::NoSuccessor)]
#[case((Unbounded, Excluded(f32::NEG_INFINITY)), ferriswatch::color::RangeErrorReason::NoPredecessor)]
fn builder_reports_why_range_is_invalid(
    #[case] bounds: (std::ops::Bound<f32>, std::ops::Bound<f32>),
    #[case] expected: ferriswatch::color::RangeErrorReason,
) {
    let error = Channel::with_name("test")
        .with_value(0.0_f32)
        .with_range(bounds)
        .build()
        .unwrap_err();
    match error {
        ChannelError::InvalidRange(name, actual, reason) => {
            assert_eq!(name, "test");
            assert_eq!(format!("{actual:?}"), format!("{bounds:?}"));
            assert_eq!(reason, expected);
            assert!(error.to_string().contains(&reason.to_string()));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn builder_distinguishes_wrapping_shape_finiteness_and_width() {
    use ferriswatch::color::WrappingRangeErrorReason;
    let inclusive = Channel::with_name("R")
        .with_value(128.0_f32)
        .with_range(0.0..=255.0)
        .with_wrapping()
        .build()
        .unwrap_err();
    assert_eq!(
        inclusive,
        ChannelError::InvalidWrappingRange(
            "R",
            (Included(0.0), Included(255.0)),
            WrappingRangeErrorReason::UnsupportedBounds,
        )
    );
    let unbounded = Channel::with_name("angle")
        .with_value(0.0_f32)
        .with_wrapping()
        .build()
        .unwrap_err();
    assert!(matches!(
        unbounded,
        ChannelError::InvalidWrappingRange("angle", _, WrappingRangeErrorReason::UnsupportedBounds,)
    ));
    let infinite = Channel::with_name("angle")
        .with_range(0.0..f32::INFINITY)
        .with_value(0.0)
        .with_wrapping()
        .build()
        .unwrap_err();
    assert!(matches!(
        infinite,
        ChannelError::InvalidWrappingRange("angle", _, WrappingRangeErrorReason::NonFiniteBounds,)
    ));
    let overflow = Channel::with_name("position")
        .with_range(-f32::MAX..f32::MAX)
        .with_value(0.0_f32)
        .with_wrapping()
        .build()
        .unwrap_err();
    assert_eq!(
        overflow,
        ChannelError::InvalidWrappingRange(
            "position",
            (Included(-f32::MAX), Excluded(f32::MAX)),
            WrappingRangeErrorReason::UnrepresentableWidth("f32"),
        )
    );
    assert!(
        overflow
            .to_string()
            .contains("not representable as a finite f32")
    );
}

#[rstest]
#[case(-7, 5)]
#[case(4, 10)]
#[case(5, 5)]
#[case(10, 10)]
#[case(11, 5)]
#[case(17, 5)]
fn inclusive_integer_ranges_wrap_at_both_ends(#[case] value: i32, #[case] expected: i32) {
    let channel = Channel::with_name("whole")
        .with_range(5..=10)
        .with_wrapping()
        .with_value(value)
        .build()
        .unwrap()
        .clamp();
    assert_eq!(channel.value_in_range(), Ok(expected));
}

#[test]
fn integer_wrapping_supports_all_bound_shapes_and_singletons() {
    for bounds in [
        (Included(5), Included(10)),
        (Included(5), Excluded(11)),
        (Excluded(4), Included(10)),
        (Excluded(4), Excluded(11)),
    ] {
        assert_eq!(
            Channel::new("whole", 11_i8, bounds)
                .unwrap()
                .with_wrapping()
                .unwrap()
                .clamp()
                .into_value(),
            5
        );
    }
    assert_eq!(
        Channel::new("single", i8::MIN, 7_i8..=7)
            .unwrap()
            .with_wrapping()
            .unwrap()
            .clamp()
            .into_value(),
        7
    );
    assert!(matches!(
        Channel::new("whole", 0_i8, 0..).unwrap().with_wrapping(),
        Err(ChannelError::InvalidWrappingRange(
            "whole",
            _,
            ferriswatch::color::WrappingRangeErrorReason::UnboundedRange
        ))
    ));
}

#[test]
fn every_integer_type_supports_its_full_range() {
    macro_rules! check {
        ($($ty:ty),+) => {$(
            for value in [<$ty>::MIN, 0, <$ty>::MAX] {
                let channel = Channel::with_name("whole")
                    .with_range(<$ty>::MIN..=<$ty>::MAX)
                    .with_value(value)
                    .with_wrapping()
                    .build()
                    .unwrap();
                assert!(channel.is_wrapping());
                assert_eq!(channel.clamp().into_value(), value);
            }
        )+};
    }
    check!(
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
    );
}

#[test]
fn wrapping_handles_ranges_nearly_as_wide_as_u128() {
    assert_eq!(
        Channel::new("wide", 0_u128, 1..=u128::MAX)
            .unwrap()
            .with_wrapping()
            .unwrap()
            .clamp()
            .into_value(),
        u128::MAX
    );
    assert_eq!(
        Channel::new("wide", u128::MAX, 0..u128::MAX)
            .unwrap()
            .with_wrapping()
            .unwrap()
            .clamp()
            .into_value(),
        0
    );
    assert_eq!(
        Channel::new("signed", i128::MIN, (i128::MIN + 1)..=i128::MAX)
            .unwrap()
            .with_wrapping()
            .unwrap()
            .clamp()
            .into_value(),
        i128::MAX
    );
    assert_eq!(
        Channel::new("signed", i128::MAX, i128::MIN..i128::MAX)
            .unwrap()
            .with_wrapping()
            .unwrap()
            .clamp()
            .into_value(),
        i128::MIN
    );
}

#[test]
fn all_byte_ranges_match_wider_modulo_arithmetic() {
    for lower in 0..=255_i32 {
        for upper in lower..=255 {
            let width = upper - lower + 1;
            let mut unsigned = Channel::new("byte", 0_u8, lower as u8..=upper as u8)
                .unwrap()
                .with_wrapping()
                .unwrap();
            let mut signed =
                Channel::new("signed", 0_i8, (lower - 128) as i8..=(upper - 128) as i8)
                    .unwrap()
                    .with_wrapping()
                    .unwrap();
            for value in 0..=255_i32 {
                let expected = lower + (value - lower).rem_euclid(width);
                unsigned.set_value(value as u8);
                signed.set_value((value - 128) as i8);
                assert_eq!(unsigned.clamp().into_value() as i32, expected);
                assert_eq!(signed.clamp().into_value() as i32, expected - 128);
            }
        }
    }
}

#[test]
fn builder_min_max_are_inclusive_and_support_wrapping() {
    let channel = Channel::with_name("R")
        .with_min_value(0)
        .with_max_value(255)
        .with_value(260_u16)
        .with_wrapping()
        .build()
        .unwrap();
    assert_eq!(channel.min_value(), Some(0));
    assert_eq!(channel.max_value(), Some(255));
    assert_eq!(channel.clamp().into_value(), 4);
    let reversed_order = Channel::with_name("R")
        .with_wrapping()
        .with_max_value(255)
        .with_value(260_u16)
        .with_min_value(0)
        .build()
        .unwrap();
    assert_eq!(channel, reversed_order);
}

#[test]
fn builder_min_max_preserve_the_other_bound() {
    let lower_only = Channel::with_name("positive")
        .with_min_value(0)
        .with_value(-5)
        .build()
        .unwrap();
    assert_eq!(*lower_only.range(), (Included(0), Unbounded));
    assert_eq!(lower_only.clamp().into_value(), 0);
    let upper_only = Channel::with_name("ceiling")
        .with_max_value(10)
        .with_value(15)
        .build()
        .unwrap();
    assert_eq!(*upper_only.range(), (Unbounded, Included(10)));
    assert_eq!(upper_only.clamp().into_value(), 10);
    let lower_replaced = Channel::with_name("hue")
        .with_range(0.0..360.0)
        .with_min_value(20.0)
        .with_value(0.0_f32)
        .build()
        .unwrap();
    assert_eq!(*lower_replaced.range(), (Included(20.0), Excluded(360.0)));
    let upper_replaced = Channel::with_name("positive")
        .with_range((Excluded(0), Included(10)))
        .with_max_value(20)
        .with_value(0)
        .build()
        .unwrap();
    assert_eq!(*upper_replaced.range(), (Excluded(0), Included(20)));
    let replaced_range = Channel::with_name("whole")
        .with_min_value(10)
        .with_max_value(20)
        .with_range(0..=5)
        .with_value(8)
        .build()
        .unwrap();
    assert_eq!(replaced_range.clamp().into_value(), 5);
}

#[test]
fn builder_min_max_report_invalid_bounds_at_build() {
    let error = Channel::with_name("R")
        .with_min_value(255_u8)
        .with_max_value(0)
        .with_value(128)
        .build()
        .unwrap_err();
    assert_eq!(
        error,
        ChannelError::InvalidRange(
            "R",
            (Included(255), Included(0)),
            ferriswatch::color::RangeErrorReason::ReversedBounds
        )
    );
}

#[test]
fn builder_optional_bounds_can_be_set_and_removed() {
    let lower_only = Channel::with_name("positive")
        .with_range((Excluded(0_i32), Included(10)))
        .with_max_value(None)
        .with_value(20)
        .build()
        .unwrap();
    assert_eq!(*lower_only.range(), (Excluded(0), Unbounded));
    assert_eq!(lower_only.max_value(), None);
    assert_eq!(lower_only.clamp().into_value(), 20);

    let upper_only = Channel::with_name("ceiling")
        .with_range((Included(0_i32), Excluded(10)))
        .with_min_value(None)
        .with_value(-20)
        .build()
        .unwrap();
    assert_eq!(*upper_only.range(), (Unbounded, Excluded(10)));
    assert_eq!(upper_only.min_value(), None);
    assert_eq!(upper_only.clamp().into_value(), -20);

    let bounded = Channel::with_name("bounded")
        .with_min_value::<u16>(None)
        .with_max_value(None)
        .with_min_value(Some(0))
        .with_max_value(Some(255))
        .with_value(260_u16)
        .build()
        .unwrap();
    assert_eq!(bounded.min_value(), Some(0));
    assert_eq!(bounded.max_value(), Some(255));
    assert_eq!(bounded.clamp().into_value(), 255);
}

#[test]
fn setters_update_bounds_without_clamping_the_value() {
    let mut channel = Channel::new("level", 15_i32, 0..=20).unwrap();
    channel
        .set_min_value(Some(5))
        .unwrap()
        .set_max_value(10)
        .unwrap();
    assert_eq!(*channel.range(), (Included(5), Included(10)));
    assert_eq!(channel.min_value(), Some(5));
    assert_eq!(channel.max_value(), Some(10));
    assert_eq!(channel.into_value(), 15);
    assert!(!channel.in_bounds());
    assert_eq!(channel.clamp().into_value(), 10);
    channel.set_max_value(None).unwrap();
    assert!(channel.in_bounds());
    assert_eq!(channel.max_value(), None);
    channel.set_min_value(None).unwrap();
    assert_eq!(*channel.range(), (Unbounded, Unbounded));
    assert_eq!(channel.min_value(), None);
}

#[test]
fn setters_preserve_excluded_opposite_bounds() {
    let mut channel = Channel::new("level", 0_i32, (Excluded(0), Excluded(20))).unwrap();
    channel.set_min_value(5).unwrap();
    assert_eq!(*channel.range(), (Included(5), Excluded(20)));
    assert_eq!(channel.max_value(), Some(19));
    let mut channel = Channel::new("level", 0_i32, (Excluded(0), Excluded(20))).unwrap();
    channel.set_max_value(Some(10)).unwrap();
    assert_eq!(*channel.range(), (Excluded(0), Included(10)));
    assert_eq!(channel.min_value(), Some(1));
}

#[test]
fn setters_leave_channel_unchanged_on_invalid_range() {
    let mut channel = Channel::new("level", 0.5_f32, 0.0..=1.0).unwrap();
    let original = channel;
    assert!(matches!(
        channel.set_min_value(2.0),
        Err(ChannelError::InvalidRange(
            "level",
            (Included(2.0), Included(1.0)),
            ferriswatch::color::RangeErrorReason::ReversedBounds
        ))
    ));
    assert_eq!(channel, original);
    assert!(matches!(
        channel.set_max_value(f32::NAN),
        Err(ChannelError::InvalidRange(
            "level",
            _,
            ferriswatch::color::RangeErrorReason::UnorderedBounds
        ))
    ));
    assert_eq!(channel, original);
}

#[test]
fn setters_rebuild_integer_wrapping_and_reject_unbounded_wrapping() {
    let mut channel = Channel::new("whole", 12_i32, 0..=10)
        .unwrap()
        .with_wrapping()
        .unwrap();
    channel.set_min_value(5).unwrap();
    assert_eq!(channel.clamp().into_value(), 6);
    channel.set_max_value(Some(7)).unwrap();
    assert_eq!(channel.clamp().into_value(), 6);
    assert_eq!(channel.into_value(), 12);
    let original = channel;
    for error in [
        channel.set_min_value(None).unwrap_err(),
        channel.set_max_value(None).unwrap_err(),
    ] {
        assert!(matches!(
            error,
            ChannelError::InvalidWrappingRange(
                "whole",
                _,
                ferriswatch::color::WrappingRangeErrorReason::UnboundedRange
            )
        ));
    }
    assert_eq!(channel, original);
}

#[test]
fn setters_revalidate_float_wrapping_before_mutating() {
    let mut channel = Channel::new("hue", 370.0_f32, 0.0..360.0)
        .unwrap()
        .with_wrapping()
        .unwrap();
    channel.set_min_value(20.0).unwrap();
    assert_eq!(channel.clamp().into_value(), 30.0);
    let original = channel;
    assert!(matches!(
        channel.set_max_value(400.0),
        Err(ChannelError::InvalidWrappingRange(
            "hue",
            _,
            ferriswatch::color::WrappingRangeErrorReason::UnsupportedBounds
        ))
    ));
    assert_eq!(channel, original);
    assert!(channel.set_min_value(f32::NEG_INFINITY).is_err());
    assert_eq!(channel, original);
}

#[test]
fn disabling_wrapping_preserves_value_and_changes_clamping() {
    let wrapped = Channel::new("hue", 370.0_f32, 0.0..360.0)
        .unwrap()
        .with_wrapping()
        .unwrap();
    let mut bounded = wrapped.without_wrapping();
    assert!(!bounded.is_wrapping());
    assert_eq!(bounded.range(), wrapped.range());
    assert_eq!(*bounded.value(), 370.0);
    assert_eq!(wrapped.clamp().into_value(), 10.0);
    assert_eq!(bounded.clamp().into_value(), 360.0_f32.next_down());
    assert_eq!(bounded.without_wrapping(), bounded);
    bounded.set_max_value(None).unwrap();
    assert_eq!(bounded.clamp().into_value(), 370.0);
}

#[test]
fn builder_can_disable_and_reenable_wrapping() {
    let builder = Channel::with_name("whole")
        .with_value(12_i32)
        .with_range((Included(0), Included(10)))
        .with_wrapping()
        .without_wrapping();
    let bounded = builder.build().unwrap();
    assert!(!bounded.is_wrapping());
    assert_eq!(bounded.clamp().into_value(), 10);
    assert_eq!(
        builder
            .with_wrapping()
            .build()
            .unwrap()
            .clamp()
            .into_value(),
        1
    );
    assert!(
        Channel::with_name("free")
            .with_value(12_i32)
            .with_wrapping()
            .without_wrapping()
            .build()
            .is_ok()
    );
}
