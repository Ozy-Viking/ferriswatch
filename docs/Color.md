# Color

Color spaces store named `Channel` values with bounds. Construct them with `new(...)` so each component gets the correct range and hue wrapping configuration.

```rust
use ferriswatch::color::{Clamp, Hsl, Rgb};

let mut color = Hsl::new(120.0, 0.5, 0.25);
color.h.set_value(-30.0);
assert!(!color.h.in_bounds());
let color = color.clamp();
assert_eq!(*color.h, 330.0);

let rgb = Rgb::new(255, 128, 0);
assert_eq!(rgb.r(), 255);
assert_eq!(rgb.0.name(), "r");
```

`ColorChannel<T>` is an alias for `Channel<T>`, which stores its range as `(Bound<T>, Bound<T>)`. The bounds preserve inclusive, exclusive, and unbounded endpoints while allowing color types to remain `Copy`.

RGB fractions use `0..=1`, byte RGB uses `0..=255`, and hue wraps within `0..360`. Lab lightness uses `0..=100`; Oklab lightness uses `0..=1`. Chroma and XYZ coordinates have a zero lower bound and no upper bound. Lab opponent axes and LMS components are unbounded. HWB clamping also normalizes whiteness plus blackness to at most one.

Constructors for spaces with public channels preserve the supplied values, including values outside their bounds. Raw conversions reject non-finite values but preserve finite extended colors. Call `clamp()` explicitly to apply the configured bounds.

`Srgb`, `LinearSrgb`, and `Color` retain their fallible constructors and numeric getters. Their `r_channel()`, `g_channel()`, and `b_channel()` methods borrow the underlying channels. `Color::a_channel()` and `Alpha::alpha_channel()` expose opacity bounds; alpha setters still reject values outside `0..=1`.

Public scalar struct literals must become constructor calls, such as `Hsl::new(h, s, l)`. Byte RGB construction is now `Rgb::new(r, g, b)`. `Rgb` and `Color` no longer provide a packed C layout; use numeric getters when exporting component values.

## Building a channel

Use `Channel::with_name` when assembling the configuration in steps:

```rust
use ferriswatch::color::{Channel, Clamp};

let hue = Channel::with_name("hue")
    .with_range(0.0..360.0)
    .with_wrapping()
    .with_value(-30.0_f32)
    .build()?;
assert_eq!(hue.clamp().into_value(), 330.0);
```

The configuration methods can be called in any order. A value is required before `build()`; the range defaults to unbounded and wrapping defaults to disabled. Use `.with_min_value(0).with_max_value(255)` as an alternative to `.with_range(0..=255)`. Each setter accepts a value or `Some(value)` for an inclusive bound, or `None` to remove that bound. It preserves the other endpoint, so either can also be used alone. A later `with_range(...)` replaces both bounds. Building validates the configuration and preserves the supplied value.

Range errors retain the channel name, original bounds, and a typed reason. They distinguish unordered or reversed bounds, an empty range, and excluded endpoints without a successor or predecessor. Integer wrapping supports every bounded, nonempty range, including inclusive endpoints, singletons, and the full type range. Widths larger than the integer type are supported, including `0..=255` for `u8` and `i128::MIN..=i128::MAX`. Unbounded integer ranges are rejected. Floating-point wrapping requires an included start, excluded end, finite endpoints, and a positive width representable in the float type.

Existing channels support `set_min_value(...)` and `set_max_value(...)` with the same value, `Some(value)`, and `None` inputs. Each returns `Result<&mut Self, ChannelError<T>>`. Setters preserve the current value and opposite endpoint, rebuild any wrapping configuration, and apply the change only if validation succeeds. For example:

```rust
let mut red = Channel::new("R", 200_u8, 0..=255)?;
red.set_max_value(Some(128))?;
assert_eq!(red.max_value(), Some(128));
assert_eq!(*red.value(), 200); // bounds changes do not clamp the stored value
red.set_max_value(None)?;
assert_eq!(red.max_value(), None);
```

`Channel<T>` stores a bound pair regardless of the constructor's range syntax, so either bound can be removed later. Removing a bound from a wrapping channel is rejected. Setting an inclusive maximum on a floating-point wrapping channel is also rejected, because floating-point wrapping requires an excluded upper endpoint.

## Named conversions

The infallible named conversions return the color directly: `srgb()`, `a98_rgb()`, `display_p3()`, `prophoto_rgb()`, `rec2020()`, `hwb()`, `oklab()`, `oklch()`, `lms()`, `lms_prime()`, and `xyz_d50()`. These target types also implement `From<LinearSrgb>`.

`rgb()`, `hsl()`, `hsv()`, `lab()`, `lch()`, `xyz()`, and `xyz_d65()` return `ColorResult<Space>` because some finite extended inputs are outside the target gamut, singular, or too large to represent. For example, `let hsl = color.hsl()?;` while `let oklch = color.oklch();`.

`to_colorspace::<T>()` and the existing `to::<T>()` return `ColorResult<T>` for a generic target. `to_clamped::<T>()` uses the target's clamping rules and also returns `ColorResult<T>`. Byte RGB clamps before quantization, so `Color::new(2.0, -0.5, 0.0, 1.0)?.to_clamped::<Rgb>()?` produces `Rgb::new(255, 0, 0)`. Conversion errors that occur before clamping are still reported.

All these conversions ignore alpha. The existing `linear_srgb()` remains infallible because `Color` already stores linear-sRGB components. `linear_srgba()` also preserves alpha.
