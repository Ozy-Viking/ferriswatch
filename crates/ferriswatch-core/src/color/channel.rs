use std::fmt;
use std::ops::{Bound, Deref, DerefMut, RangeBounds, RangeFull, Rem};

use crate::color::Clamp;

/// A named value with channel bounds and optional circular wrapping.
///
/// Construction uses [`AdjacentValue`] to resolve exclusive bounds. Wrapping
/// additionally requires [`WrappingValue`]. Both traits support floats and integers
/// and can be implemented for custom types. Values may start outside their bounds;
/// construction validates the range, not the value.
///
/// ```
/// use ferriswatch_core::color::{Channel, Clamp};
///
/// let hue = Channel::new("hue", -30.0_f32, 0.0..360.0)?.with_wrapping()?;
/// assert_eq!(*hue.clamp().value(), 330.0);
/// let red = Channel::new("red", 1.5_f32, 0.0..=1.0)?;
/// assert_eq!(*red.clamp().value(), 1.0);
/// # Ok::<(), ferriswatch_core::color::ChannelError>(())
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]

pub struct Channel<T> {
    name: &'static str,
    value: T,
    range: (Bound<T>, Bound<T>),
    limits: (Option<T>, Option<T>),
    wrapping: Option<Wrapping<T>>,
}

/// An invalid channel value, range, or wrapping configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]

pub enum ChannelError<T = f32> {
    /// The current value is outside the channel's range or is unordered.
    #[error("value outside range for channel {0}: {1}")]
    OutsideRange(&'static str, T),
    /// No representable value satisfies the supplied bounds, or a bound is NaN.
    #[error("invalid range for channel {0}: {bounds}: {2}", bounds = format_bounds(.1))]
    InvalidRange(&'static str, (Bound<T>, Bound<T>), RangeErrorReason),
    /// The range is valid for clamping but unsupported for wrapping.
    #[error(
        "invalid wrapping range for channel {0}: {bounds}: {2}", bounds = format_bounds(.1)
    )]
    InvalidWrappingRange(&'static str, (Bound<T>, Bound<T>), WrappingRangeErrorReason),
}

/// Why the supplied bounds cannot describe a channel range.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]

pub enum RangeErrorReason {
    /// At least one bound or comparison is unordered.
    #[error("bounds must be ordered and cannot be NaN")]
    UnorderedBounds,
    /// The supplied lower endpoint exceeds the upper endpoint.
    #[error("the lower bound is greater than the upper bound")]
    ReversedBounds,
    /// Excluding the endpoints leaves no representable value.
    #[error("no representable value lies within these bounds")]
    EmptyRange,
    /// The excluded lower endpoint has no successor.
    #[error("the excluded lower bound has no representable successor")]
    NoSuccessor,
    /// The excluded upper endpoint has no predecessor.
    #[error("the excluded upper bound has no representable predecessor")]
    NoPredecessor,
}

/// Why a valid channel range cannot be used for wrapping.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]

pub enum WrappingRangeErrorReason {
    /// The range does not have an included start and excluded end.
    #[error("wrapping requires an included start and excluded end (start..end)")]
    UnsupportedBounds,
    /// Integer wrapping needs two explicit finite endpoints.
    #[error("integer wrapping requires both a lower and an upper bound")]
    UnboundedRange,
    /// At least one endpoint is infinite.
    #[error("wrapping requires finite endpoints")]
    NonFiniteBounds,
    /// Subtracting the endpoints overflows the value type.
    #[error("the wrapping width (end - start) is not representable as a finite {0}")]
    UnrepresentableWidth(&'static str),
    /// The arithmetic implementation did not produce a positive width.
    #[error("the wrapping width (end - start) must be positive")]
    NonPositiveWidth,
}

/// Builds a named channel, validating its range when `build()` is called.
///
/// Supply a value with [`Self::with_value`]. The default range is unbounded and
/// wrapping is disabled. Configuration methods may be called in any order.
/// Building preserves the value; use [`Clamp::clamp`] to clamp or wrap it.
///
/// ```
/// use ferriswatch_core::color::{Channel, Clamp};
/// let hue = Channel::with_name("hue")
///     .with_range(0.0..360.0)
///     .with_wrapping()
///     .with_value(-30.0_f32)
///     .build()?;
/// assert_eq!(hue.clamp().into_value(), 330.0);
/// # Ok::<(), ferriswatch_core::color::ChannelError>(())
/// ```
#[derive(Clone, Copy, Debug)]
#[must_use = "call build() to validate and construct the channel"]

pub struct ChannelBuilder<T = (), R = RangeFull, const WRAPPING: bool = false> {
    name: &'static str,
    value: T,
    range: R,
}

impl Channel<()> {
    /// Starts a builder with a name. Supply a value before calling `build()`.
    ///
    /// ```compile_fail
    /// use ferriswatch_core::color::Channel;
    /// Channel::with_name("R").with_range(0_u8..=255).build();
    /// ```

    pub fn with_name(name: &'static str) -> ChannelBuilder {
        ChannelBuilder {
            name,
            value: (),
            range: ..,
        }
    }
}

impl<T, R, const WRAPPING: bool> ChannelBuilder<T, R, WRAPPING> {
    /// Supplies or replaces the value without validating it.

    pub fn with_value<U>(self, value: U) -> ChannelBuilder<U, R, WRAPPING> {
        ChannelBuilder {
            name: self.name,
            value,
            range: self.range,
        }
    }

    /// Supplies or replaces the range; validation happens in `build()`.

    pub fn with_range<S>(self, range: S) -> ChannelBuilder<T, S, WRAPPING> {
        ChannelBuilder {
            name: self.name,
            value: self.value,
            range,
        }
    }

    /// Sets an inclusive minimum, or removes it with `None`, preserving the upper bound.
    ///
    /// May be called before `with_value`. When the range type is not yet known,
    /// use an explicit type, such as `with_min_value::<u8>(None)`.
    /// Validation happens in `build()`.
    /// A later `with_range` replaces both bounds.

    pub fn with_min_value<U: Copy>(
        self,
        minimum: impl Into<Option<U>>,
    ) -> ChannelBuilder<T, (Bound<U>, Bound<U>), WRAPPING>
    where
        R: RangeBounds<U>,
    {
        let upper = self.range.end_bound().map(|value| *value);

        self.with_range((
            minimum.into().map_or(Bound::Unbounded, Bound::Included),
            upper,
        ))
    }

    /// Sets an inclusive maximum, or removes it with `None`, preserving the lower bound.
    ///
    /// May be called before `with_value`. When the range type is not yet known,
    /// use an explicit type, such as `with_max_value::<u8>(None)`.
    /// Validation happens in `build()`.
    /// A later `with_range` replaces both bounds.

    pub fn with_max_value<U: Copy>(
        self,
        maximum: impl Into<Option<U>>,
    ) -> ChannelBuilder<T, (Bound<U>, Bound<U>), WRAPPING>
    where
        R: RangeBounds<U>,
    {
        let lower = self.range.start_bound().map(|value| *value);

        self.with_range((
            lower,
            maximum.into().map_or(Bound::Unbounded, Bound::Included),
        ))
    }

    /// Requests wrapping. Building then also requires [`WrappingValue`].

    pub fn with_wrapping(self) -> ChannelBuilder<T, R, true> {
        ChannelBuilder {
            name: self.name,
            value: self.value,
            range: self.range,
        }
    }

    /// Disables wrapping, preserving the configured value and range.

    pub fn without_wrapping(self) -> ChannelBuilder<T, R, false> {
        ChannelBuilder {
            name: self.name,
            value: self.value,
            range: self.range,
        }
    }
}

impl<T: AdjacentValue, R: RangeBounds<T>> ChannelBuilder<T, R, false> {
    /// Builds a channel without clamping its value.
    ///
    /// # Errors
    /// Returns [`ChannelError::InvalidRange`] with the bounds and reason if invalid.

    pub fn build(self) -> Result<Channel<T>, ChannelError<T>> {
        Channel::new(self.name, self.value, self.range)
    }
}

impl<T: AdjacentValue + WrappingValue, R: RangeBounds<T>> ChannelBuilder<T, R, true> {
    /// Builds a wrapping channel without wrapping its current value.
    ///
    /// # Errors
    /// Returns [`ChannelError::InvalidRange`] or [`ChannelError::InvalidWrappingRange`]
    /// with the bounds and reason if the configuration is invalid.

    pub fn build(self) -> Result<Channel<T>, ChannelError<T>> {
        Channel::new(self.name, self.value, self.range)?.with_wrapping()
    }
}

fn format_bounds<T: std::fmt::Display>(bounds: &(Bound<T>, Bound<T>)) -> String {
    let describe = |bound: &Bound<T>| match bound {
        Bound::Included(value) => format!("Included({value})"),
        Bound::Excluded(value) => format!("Excluded({value})"),
        Bound::Unbounded => "Unbounded".to_owned(),
    };

    format!("({}, {})", describe(&bounds.0), describe(&bounds.1))
}

impl<T> Channel<T> {
    /// Returns the channel's name.

    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Borrows the current value.

    pub fn value(&self) -> &T {
        &self.value
    }

    /// Replaces the value without clamping or validating it.

    pub fn set_value(&mut self, value: T) -> &mut Self {
        self.value = value;

        self
    }

    /// Returns the value, consuming the channel.

    pub fn into_value(self) -> T {
        self.value
    }

    /// Borrows the configured lower and upper bounds, preserving inclusion and exclusion.

    pub fn range(&self) -> &(Bound<T>, Bound<T>) {
        &self.range
    }

    /// Returns whether clamping wraps around the range.

    pub fn is_wrapping(&self) -> bool {
        self.wrapping.is_some()
    }

    /// Disables wrapping without changing the value or bounds.
    /// Subsequent clamping uses the effective minimum and maximum.

    pub fn without_wrapping(mut self) -> Self {
        self.wrapping = None;

        self
    }
}

impl<T> AsRef<T> for Channel<T> {
    /// Borrows the current channel value.

    fn as_ref(&self) -> &T {
        &self.value
    }
}

/// Formats the stored byte as lowercase hex, preserving the formatter's flags.

impl fmt::LowerHex for Channel<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.value, f)
    }
}

/// Formats the stored byte as uppercase hex, preserving the formatter's flags.

impl fmt::UpperHex for Channel<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.value, f)
    }
}

impl<T> AsMut<T> for Channel<T> {
    /// Mutably borrows the value without clamping or validating changes.

    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> Deref for Channel<T> {
    type Target = T;

    /// Borrows the current channel value.

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Channel<T> {
    /// Mutably borrows the value without clamping or validating changes.

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: PartialOrd> Channel<T> {
    /// Reports whether the current value is ordered and within the range.
    ///
    /// NaN is outside every range, including an unbounded range.

    pub fn in_bounds(&self) -> bool {
        self.value.partial_cmp(&self.value).is_some() && self.range.contains(&self.value)
    }

    /// Returns a copy of the value if it is within the configured range.
    ///
    /// Does not clamp or wrap the value.
    ///
    /// # Errors
    /// Returns [`ChannelError::OutsideRange`] if the value is outside the range
    /// or unordered, such as NaN.

    pub fn value_in_range(&self) -> Result<T, ChannelError<T>>
    where
        T: Copy,
    {
        if self.in_bounds() {
            Ok(self.value)
        } else {
            Err(ChannelError::OutsideRange(self.name, self.value))
        }
    }
}

impl<T: AdjacentValue> Channel<T> {
    /// Creates a channel and resolves its effective minimum and maximum.
    ///
    /// Included bounds are used directly. Excluded bounds use the successor or
    /// predecessor; unbounded limits are `None`. The value is not validated.
    ///
    /// # Errors
    /// Returns [`ChannelError::InvalidRange`] for unordered or reversed bounds,
    /// missing adjacent values, or a range with no representable value.

    pub fn new<R: RangeBounds<T>>(
        name: &'static str,
        value: T,
        range: R,
    ) -> Result<Self, ChannelError<T>> {
        let bounds = (
            range.start_bound().map(|value| *value),
            range.end_bound().map(|value| *value),
        );

        let invalid = |reason| ChannelError::InvalidRange(name, bounds, reason);

        let endpoint = |bound: Bound<T>| match bound {
            Bound::Included(value) | Bound::Excluded(value) => Some(value),
            Bound::Unbounded => None,
        };

        let start = endpoint(bounds.0);

        let end = endpoint(bounds.1);

        for endpoint in [start, end].into_iter().flatten() {
            if endpoint.partial_cmp(&endpoint).is_none() {
                return Err(invalid(RangeErrorReason::UnorderedBounds));
            }
        }

        if let (Some(start), Some(end)) = (start, end) {
            match start.partial_cmp(&end) {
                None => return Err(invalid(RangeErrorReason::UnorderedBounds)),
                Some(std::cmp::Ordering::Greater) => {
                    return Err(invalid(RangeErrorReason::ReversedBounds));
                }
                _ => {}
            }
        }

        let lower = match range.start_bound() {
            Bound::Included(value) => Some(*value),
            Bound::Excluded(value) => Some(
                value
                    .next_value()
                    .ok_or_else(|| invalid(RangeErrorReason::NoSuccessor))?,
            ),
            Bound::Unbounded => None,
        };

        let upper = match range.end_bound() {
            Bound::Included(value) => Some(*value),
            Bound::Excluded(value) => Some(
                value
                    .previous_value()
                    .ok_or_else(|| invalid(RangeErrorReason::NoPredecessor))?,
            ),
            Bound::Unbounded => None,
        };

        if let (Some(lower), Some(upper)) = (lower, upper) {
            match lower.partial_cmp(&upper) {
                None => return Err(invalid(RangeErrorReason::UnorderedBounds)),
                Some(std::cmp::Ordering::Greater) => {
                    return Err(invalid(RangeErrorReason::EmptyRange));
                }
                _ => {}
            }
        }

        for limit in [lower, upper].into_iter().flatten() {
            if limit.partial_cmp(&limit).is_none() {
                return Err(invalid(RangeErrorReason::UnorderedBounds));
            }

            if !range.contains(&limit) {
                return Err(invalid(RangeErrorReason::EmptyRange));
            }
        }

        Ok(Self {
            name,
            value,
            range: bounds,
            limits: (lower, upper),
            wrapping: None,
        })
    }
}

impl<T: AdjacentValue> Channel<T> {
    /// Sets an inclusive minimum, or removes it with `None`.
    ///
    /// Preserves the upper bound and current value without clamping it.
    ///
    /// # Errors
    /// Returns the range or wrapping error if the new configuration is invalid.
    /// On error, the channel is unchanged.

    pub fn set_min_value(
        &mut self,
        minimum: impl Into<Option<T>>,
    ) -> Result<&mut Self, ChannelError<T>> {
        self.set_bounds((
            minimum.into().map_or(Bound::Unbounded, Bound::Included),
            self.range.1,
        ))
    }

    /// Sets an inclusive maximum, or removes it with `None`.
    ///
    /// Preserves the lower bound and current value without clamping it.
    ///
    /// # Errors
    /// Returns the range or wrapping error if the new configuration is invalid.
    /// On error, the channel is unchanged. An inclusive maximum is unsupported
    /// for floating-point wrapping.

    pub fn set_max_value(
        &mut self,
        maximum: impl Into<Option<T>>,
    ) -> Result<&mut Self, ChannelError<T>> {
        self.set_bounds((
            self.range.0,
            maximum.into().map_or(Bound::Unbounded, Bound::Included),
        ))
    }

    fn set_bounds(&mut self, bounds: (Bound<T>, Bound<T>)) -> Result<&mut Self, ChannelError<T>> {
        let mut updated = Self::new(self.name, self.value, bounds)?;

        if let Some(wrapping) = &self.wrapping {
            let rebuild = match wrapping {
                Wrapping::Continuous { rebuild, .. } | Wrapping::Discrete { rebuild, .. } => {
                    rebuild
                }
            };

            updated = rebuild(updated)?;
        }

        *self = updated;

        Ok(self)
    }
}

impl<T: Copy> Channel<T> {
    /// Returns the effective inclusive minimum, or `None` if unbounded.

    pub fn min_value(&self) -> Option<T> {
        self.limits.0
    }

    /// Returns the effective inclusive maximum, or `None` if unbounded.

    pub fn max_value(&self) -> Option<T> {
        self.limits.1
    }
}

impl<T: WrappingValue> Channel<T> {
    /// Enables modulo wrapping within the configured range.
    ///
    /// Integers support any bounded, nonempty range, including inclusive endpoints
    /// and the full type range. Floats require finite `start..end` bounds and a
    /// positive width representable in the float type. Non-finite values remain unchanged.
    ///
    /// # Errors
    /// Returns [`ChannelError::InvalidWrappingRange`] with the specific reason
    /// when the range cannot support wrapping for this value type.

    pub fn with_wrapping(mut self) -> Result<Self, ChannelError<T>> {
        let bounds = (
            self.range.start_bound().map(|value| *value),
            self.range.end_bound().map(|value| *value),
        );

        let invalid = |reason| ChannelError::InvalidWrappingRange(self.name, bounds, reason);

        if let Some(apply) = T::discrete_wrapping() {
            let (Some(lower), Some(upper)) = self.limits else {
                return Err(invalid(WrappingRangeErrorReason::UnboundedRange));
            };

            self.wrapping = Some(Wrapping::Discrete {
                lower,
                upper,
                apply,
                rebuild: Self::with_wrapping,
            });

            return Ok(self);
        }

        let (Bound::Included(start), Bound::Excluded(end)) =
            (self.range.start_bound(), self.range.end_bound())
        else {
            return Err(invalid(WrappingRangeErrorReason::UnsupportedBounds));
        };

        if !start.is_finite() || !end.is_finite() {
            return Err(invalid(WrappingRangeErrorReason::NonFiniteBounds));
        }

        let width = end
            .checked_sub(*start)
            .filter(|width| width.is_finite())
            .ok_or_else(|| {
                invalid(WrappingRangeErrorReason::UnrepresentableWidth(
                    std::any::type_name::<T>(),
                ))
            })?;

        if width.partial_cmp(&T::ZERO) != Some(std::cmp::Ordering::Greater) {
            return Err(invalid(WrappingRangeErrorReason::NonPositiveWidth));
        }

        self.wrapping = Some(Wrapping::Continuous {
            start: *start,
            end: *end,
            width,
            apply: wrap::<T>,
            rebuild: Self::with_wrapping,
        });

        Ok(self)
    }
}

impl<T: Copy + PartialOrd> Clamp for Channel<T> {
    fn clamp(mut self) -> Self {
        if let Some(wrapping) = &self.wrapping {
            self.value = match wrapping {
                Wrapping::Continuous {
                    start,
                    end,
                    width,
                    apply,
                    ..
                } => apply(self.value, *start, *end, *width),
                Wrapping::Discrete {
                    lower,
                    upper,
                    apply,
                    ..
                } => apply(self.value, *lower, *upper),
            };
        } else {
            if let Some(lower) = self.limits.0
                && self.value < lower
            {
                self.value = lower;
            }

            if let Some(upper) = self.limits.1
                && self.value > upper
            {
                self.value = upper;
            }
        }

        self
    }
}

type RebuildWrapping<T> = fn(Channel<T>) -> Result<Channel<T>, ChannelError<T>>;

// Store the wrapping operation when it is enabled so ordinary Clamp needs only
// comparisons, even for custom values that do not implement arithmetic.
#[derive(Clone, Copy, Debug)]

enum Wrapping<T> {
    Continuous {
        start: T,
        end: T,
        width: T,
        apply: fn(T, T, T, T) -> T,
        rebuild: RebuildWrapping<T>,
    },
    Discrete {
        lower: T,
        upper: T,
        apply: fn(T, T, T) -> T,
        rebuild: RebuildWrapping<T>,
    },
}

impl<T: PartialEq> PartialEq for Wrapping<T> {
    fn eq(&self, other: &Self) -> bool {
        // Operations are fixed by T; function addresses are not equality keys.
        match (self, other) {
            (
                Self::Continuous {
                    start, end, width, ..
                },
                Self::Continuous {
                    start: r_start,
                    end: r_end,
                    width: r_width,
                    ..
                },
            ) => start == r_start && end == r_end && width == r_width,
            (
                Self::Discrete { lower, upper, .. },
                Self::Discrete {
                    lower: r_lower,
                    upper: r_upper,
                    ..
                },
            ) => lower == r_lower && upper == r_upper,
            _ => false,
        }
    }
}

// Integer coordinates are mapped monotonically to u128, with the type's minimum
// at zero. Full-width ranges return before computing a possibly overflowing size.
fn wrap_integer(value: u128, lower: u128, upper: u128) -> u128 {
    if value >= lower && value <= upper {
        return value;
    }

    let width = upper - lower + 1;

    if value > upper {
        lower + (value - lower) % width
    } else {
        let remainder = (lower - value) % width;

        if remainder == 0 {
            lower
        } else {
            upper - (remainder - 1)
        }
    }
}

fn wrap<T: WrappingValue>(value: T, start: T, end: T, width: T) -> T {
    if !value.is_finite() || (value >= start && value < end) {
        return value;
    }

    // Reduce first, avoiding overflow and loss of small offsets in value - start.
    let residue = |v: T| {
        let r = v % width;

        if r < T::ZERO {
            r.checked_add(width)
        } else {
            Some(r)
        }
    };

    let result = (|| {
        let r = residue(value)?;

        let s = residue(start)?;

        let offset = if r >= s {
            r.checked_sub(s)?
        } else {
            width.checked_sub(s.checked_sub(r)?)?
        };

        start.checked_add(offset)
    })();

    match result {
        Some(result) if result >= start && result < end => result,
        // A floating-point remainder or sum can round up to the excluded end.
        Some(result) if result == end => start,
        _ => value,
    }
}

/// Ordered values with immediately adjacent representable values.
///
/// Implementations must return the least value strictly greater than `self` and
/// the greatest value strictly less than `self`, respectively. Return `None` at
/// the corresponding type boundary or for unordered values such as NaN.

pub trait AdjacentValue: Copy + PartialOrd {
    /// Returns the immediate successor, if one exists.

    fn next_value(self) -> Option<Self>;

    /// Returns the immediate predecessor, if one exists.

    fn previous_value(self) -> Option<Self>;
}

/// Arithmetic needed for circular channels, without requiring it for clamping.
///
/// Remainder must follow integer/float `%` semantics for a finite positive
/// divisor for the default continuous wrapping algorithm. Discrete types may
/// supply [`Self::discrete_wrapping`] to support inclusive ranges and larger widths.
/// Checked operations must return `None` on overflow or non-finite
/// results. `ZERO` is the additive identity. Implementations must preserve the
/// ordering and arithmetic laws used by modulo reduction.

pub trait WrappingValue: Copy + PartialOrd + Rem<Output = Self> {
    /// Optionally supplies exact wrapping for discrete values within inclusive bounds.
    ///
    /// The operation receives `(value, lower, upper)` with valid ordered bounds.
    /// It must preserve in-range values and wrap other values into `lower..=upper`,
    /// including when the range's size cannot be represented in `Self`.
    /// Return the same operation on every call. The default uses continuous wrapping.

    fn discrete_wrapping() -> Option<fn(Self, Self, Self) -> Self> {
        None
    }

    /// The additive identity.

    const ZERO: Self;

    /// Adds two values, rejecting overflow or a non-finite result.

    fn checked_add(self, rhs: Self) -> Option<Self>;

    /// Subtracts two values, rejecting overflow or a non-finite result.

    fn checked_sub(self, rhs: Self) -> Option<Self>;

    /// Reports whether this value can participate in modulo arithmetic.

    fn is_finite(self) -> bool;
}

macro_rules! integer_values {
    ($($ty:ty),+ $(,)?) => {$(
        impl AdjacentValue for $ty {
            fn next_value(self) -> Option<Self> { self.checked_add(1) }
            fn previous_value(self) -> Option<Self> { self.checked_sub(1) }
        }
        impl WrappingValue for $ty {
            fn discrete_wrapping() -> Option<fn(Self, Self, Self) -> Self> {
                Some(|value, lower, upper| {
                    let origin = <$ty>::MIN as u128;
                    let key = |v: $ty| (v as u128).wrapping_sub(origin);
                    wrap_integer(key(value), key(lower), key(upper)).wrapping_add(origin) as $ty
                })
            }
            const ZERO: Self = 0;
            fn checked_add(self, rhs: Self) -> Option<Self> { self.checked_add(rhs) }
            fn checked_sub(self, rhs: Self) -> Option<Self> { self.checked_sub(rhs) }
            fn is_finite(self) -> bool { true }
        }
    )+};
}

integer_values!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

macro_rules! float_values {
    ($($ty:ty),+ $(,)?) => {$(
        impl AdjacentValue for $ty {
            fn next_value(self) -> Option<Self> {
                let next = self.next_up();
                (next > self).then_some(next)
            }
            fn previous_value(self) -> Option<Self> {
                let previous = self.next_down();
                (previous < self).then_some(previous)
            }
        }
        impl WrappingValue for $ty {
            const ZERO: Self = 0.0;
            fn checked_add(self, rhs: Self) -> Option<Self> {
                let result = self + rhs;
                result.is_finite().then_some(result)
            }
            fn checked_sub(self, rhs: Self) -> Option<Self> {
                let result = self - rhs;
                result.is_finite().then_some(result)
            }
            fn is_finite(self) -> bool { self.is_finite() }
        }
    )+};
}

float_values!(f32, f64);

impl<T: AdjacentValue> Channel<T> {
    /// Creates a channel with the supplied bounds, preserving its value.
    ///
    /// Does not clamp or validate the value. Wrapping is disabled.
    ///
    /// # Panics
    /// Panics if the range has invalid bounds or contains no representable values.
    /// Use [`Self::new`] to receive a range error instead.

    pub fn color_channel<R: RangeBounds<T>>(name: &'static str, value: T, range: R) -> Self {
        match Self::new(name, value, range) {
            Ok(channel) => channel,
            Err(_) => panic!("invalid range for channel {name}"),
        }
    }
}

impl Channel<u8> {
    /// Creates a byte channel with inclusive bounds `0..=255` and no wrapping.

    pub const fn byte_color_channel(name: &'static str, value: u8) -> Self {
        Self {
            name,
            value,
            range: (Bound::Included(0), Bound::Included(255)),
            limits: (Some(0), Some(255)),
            wrapping: None,
        }
    }
}

impl Channel<f32> {
    /// Creates an `alpha` channel in `0..=1` by dividing a byte by 255.

    pub const fn alpha_byte_channel(value: u8) -> Self {
        Self::unit_color_channel("alpha", value as f32 / 255.0)
    }

    /// Creates a channel with inclusive bounds `0..=1` and no wrapping.
    ///
    /// Preserves the supplied value, including out-of-range or non-finite values.
    /// Use [`Self::value_in_range`] to validate it or [`Clamp::clamp`] to clamp it.
    ///
    /// ```
    /// use ferriswatch_core::color::Channel;
    /// const RED: Channel<f32> = Channel::unit_color_channel("r", 0.5);
    /// const ALPHA: Channel<f32> = Channel::alpha_byte_channel(128);
    /// const BYTE: Channel<u8> = Channel::byte_color_channel("r", 255);
    /// assert_eq!(*RED, 0.5);
    /// assert_eq!(*ALPHA, 128.0 / 255.0);
    /// assert_eq!(*BYTE, 255);
    /// ```

    pub const fn unit_color_channel(name: &'static str, value: f32) -> Self {
        Self {
            name,
            value,
            range: (Bound::Included(0.0), Bound::Included(1.0)),
            limits: (Some(0.0), Some(1.0)),
            wrapping: None,
        }
    }
}
