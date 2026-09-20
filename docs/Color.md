# Color

`Color` is Ferriswatch's common representation for storing a color.
Create it from any supported color space, keep it as a `Color`, and obtain
another color-space representation when needed.

Internally, `Color` stores linear-sRGB red, green, and blue values with separate
alpha. It does not retain the source color space or switch its stored
representation when you request a conversion. The methods return new values,
leaving the stored color unchanged.

```rust
# extern crate ferriswatch_core as ferriswatch;
use ferriswatch::color::{Color, ColorError, Hsl, Rgb};

fn main() -> Result<(), ColorError> {
    // Create the stored color from HSL.
    let color = Color::try_from(Hsl::new(120.0, 0.5, 0.25))?;

    // Obtain representations for calculations or output as needed.
    let oklch = color.oklch();
    let rgb: Rgb = color.to_clamped::<Rgb>()?;
    assert!(oklch.l.is_finite());
    assert_eq!(rgb, Rgb::new(32, 96, 32));
    Ok(())
}
```

Creating `Color` from a color-space value sets alpha to `1.0`. Converting out to
a color space ignores alpha; `linear_srgba()` preserves it. The alpha section
describes how to carry opacity with other representations.

`Color::new(r, g, b, a)` creates the stored representation directly from
linear-sRGB values. RGB values must be finite but may extend outside `0..=1`,
preserving colors outside the sRGB gamut. Alpha must be finite and within
`0..=1`. A conversion can still fail if its result cannot be represented in the
target space.

## Creating colors

[`Color::TRANSPARENT`] is fully transparent black: red, green, blue, and alpha
are all `0.0`. It is a constant, so it can be used in other constant definitions
without a constructor or error handling.

```rust
# extern crate ferriswatch_core as ferriswatch;
use ferriswatch::color::Color;

const BACKGROUND: Color = Color::TRANSPARENT;
assert_eq!(BACKGROUND.a(), 0.0);
assert_eq!((BACKGROUND.r(), BACKGROUND.g(), BACKGROUND.b()), (0.0, 0.0, 0.0));
```

Use `Color::hex(...)` to define palette constants from encoded sRGB hex. It
returns `Color` directly and decodes RGB bytes to linear light at compile time.
`Color::hex(...)` accepts only packed `0xRRGGBB` values up to `0xFFFFFF` and
always supplies opaque alpha. Larger values panic. Integers do not retain digit
counts, so shorter literals are interpreted with leading zeros.
`Color::hex_alpha(...)` takes packed `0xRRGGBBAA`, including leading-zero values
such as `0x00000080`. The final byte supplies alpha. Every `u32` is interpreted
as eight hex digits, padded with leading zeros.
`Color::from_hex(...)` is also const, retains its RGB/RGBA dispatch, and returns
`ColorResult<Color>` for compatibility. For explicit alpha, including leading-zero RGBA colors, use
`Color::from_rgba8(r, g, b, a)`.

```rust
# extern crate ferriswatch_core as ferriswatch;
use ferriswatch::color::Color;

const ROSEWATER: Color = Color::hex(0xf5e0dc);
const TRANSLUCENT_BLACK: Color = Color::hex_alpha(0x00000080);
assert_eq!(ROSEWATER.a(), 1.0);
assert_eq!(TRANSLUCENT_BLACK.a(), 128.0 / 255.0);
```

Each color-space type represents coordinates in that space. For example, `Srgb`
holds encoded sRGB values, while `Hsl` holds hue, saturation, and lightness.
`Color::try_from(value)` creates the common stored representation from any
supported color space. Linear sRGB also supports `Color::from(value)` because
that conversion cannot fail.

```rust
# extern crate ferriswatch_core as ferriswatch;
use ferriswatch::color::{Color, ColorError, Hsl, Srgb};

fn main() -> Result<(), ColorError> {
    let encoded = Srgb::new(1.0, 0.0, 0.0)?;
    let color = Color::try_from(encoded)?;
    let hsl: Hsl = color.hsl()?;

    assert_eq!((*hsl.h, *hsl.s, *hsl.l), (0.0, 1.0, 0.5));
    assert_eq!(color.a(), 1.0);
    Ok(())
}
```

`Rgb::new(r, g, b)` accepts three `u8` values. Floating-point constructors
accept three `f32` values, except `Color::new(r, g, b, a)`, which also requires
alpha.

Public components, such as `Hsl::h`, are `Channel` values. Dereferencing reads
the number; `set_value(...)` replaces it without validation. `Rgb`, `Srgb`,
`LinearSrgb`, and `Color` expose numeric `r()`, `g()`, and `b()` getters and
immutable `r_channel()`, `g_channel()`, and `b_channel()` accessors.

## Color spaces and named conversions

These methods borrow a `Color`, convert its RGB components, and ignore alpha. A
direct return type means every valid `Color` can be converted. A
`ColorResult<T>` return type requires error handling.

| Color space | Components | Method on `Color` | Return type |
|---|---|---|---|
| Byte sRGB | `r`, `g`, `b` as `u8` | `rgb()` | `ColorResult<Rgb>` |
| Encoded sRGB | `r`, `g`, `b` | `srgb()` | `Srgb` |
| Linear sRGB | `r`, `g`, `b` | `linear_srgb()` | `LinearSrgb` |
| Adobe RGB 1998 | `r`, `g`, `b` | `a98_rgb()` | `A98Rgb` |
| Display P3 | `r`, `g`, `b` | `display_p3()` | `DisplayP3` |
| ProPhoto RGB | `r`, `g`, `b` | `prophoto_rgb()` | `ProPhotoRgb` |
| Rec. 2020 | `r`, `g`, `b` | `rec2020()` | `Rec2020` |
| HSL | `h`, `s`, `l` | `hsl()` | `ColorResult<Hsl>` |
| HSV | `h`, `s`, `v` | `hsv()` | `ColorResult<Hsv>` |
| HWB | `h`, `w`, `b` | `hwb()` | `Hwb` |
| CIELAB, D50 | `l`, `a`, `b` | `lab()` | `ColorResult<Lab>` |
| CIELCh, D50 | `l`, `c`, `h` | `lch()` | `ColorResult<Lch>` |
| Oklab | `l`, `a`, `b` | `oklab()` | `Oklab` |
| OkLCh | `l`, `c`, `h` | `oklch()` | `Oklch` |
| LMS | `l`, `m`, `s` | `lms()` | `Lms` |
| Cube-root LMS | `l`, `m`, `s` | `lms_prime()` | `LmsPrime` |
| XYZ, D65 | `x`, `y`, `z` | `xyz()` | `ColorResult<Xyz>` |
| XYZ, D50 | `x`, `y`, `z` | `xyz_d50()` | `XyzD50` |
| XYZ, D65 | `x`, `y`, `z` | `xyz_d65()` | `ColorResult<XyzD65>` |

The named `rgb()` method rejects out-of-gamut input. HSL and HSV can encounter singular
coordinates for extended colors. Lab, LCh, and D65 XYZ can overflow for large
finite inputs. These failures remain errors rather than silently clamping the
result.

The infallible target spaces also implement `From<LinearSrgb>`. Other targets
implement `TryFrom<LinearSrgb>`. Conversion back to linear sRGB remains fallible
for other color spaces.

## From and Into conversions

Infallible conversions to and from `Color` implement `From`, which also provides
`Into`. They clamp destination channels to their declared bounds, wrap hue, and
leave unbounded channels unchanged. Byte RGB quantizes encoded channels to the
nearest byte. These conversions can lose out-of-gamut values and precision.

`Rgb`, `Srgb`, and `LinearSrgb` support both directions, including their alpha
wrappers. `Color` also converts infallibly to A98 RGB, Display P3, ProPhoto RGB,
Rec.2020, HWB, Oklab, OkLCh, LMS, cube-root LMS, and D50 XYZ, with or without
alpha. Reverse directions that can reject non-finite values or overflow retain
`TryFrom`. Bare spaces discard alpha or supply opaque alpha; wrappers preserve it.

```rust
# extern crate ferriswatch_core as ferriswatch;
use ferriswatch::color::{Color, Rgb, Rgba};
let source = Color::new(-0.25, 0.5, 2.0, 0.375)?;
let rgb: Rgb = source.into();
assert_eq!(rgb, Rgb::new(0, 188, 255));
let rgba: Rgba = source.into();
assert_eq!(rgba.alpha(), 0.375);
let restored: Color = rgba.into();
assert_eq!(restored.a(), 0.375);
# Ok::<(), ferriswatch::color::ColorError>(())
```

Where `From` exists, Rust's blanket `TryFrom` performs the same clamping
conversion. Use `ColorSpace::try_into_color()` and the explicit raw methods for
unclamped conversion. Existing conversions directly between spaces through
`LinearSrgb` retain their raw behavior.

## Generic and clamped conversions

`Color::to_colorspace::<T>()` and `Color::to::<T>()` return `ColorResult<T>` for
a target implementing `ColorSpace`. Both preserve representable extended values
without clamping.

`Color::to_clamped::<T>()` applies the target's channel constraints. For byte
RGB, it clamps before quantization so an out-of-gamut color can become a valid
byte color. Conversion errors that occur before clamping still propagate.

```rust
# extern crate ferriswatch_core as ferriswatch;
use ferriswatch::color::{Color, ColorError, Oklch, Rgb};

fn main() -> Result<(), ColorError> {
    let color = Color::new(2.0, -0.5, 0.0, 0.5)?;

    let polar = color.to_colorspace::<Oklch>()?;
    assert_eq!(polar, color.oklch());
    assert!(color.rgb().is_err());
    assert_eq!(color.to_clamped::<Rgb>()?, Rgb::new(255, 0, 0));
    assert_eq!(color.a(), 0.5);
    Ok(())
}
```

`ColorSpace::try_into_color()` converts a color-space value into `Color` with
alpha set to one. The raw methods `try_into_linear_srgb_raw()` and
`try_from_linear_srgb_raw(...)` convert without clamping. `ClampedInto` and
`ClampedFrom` provide clamped conversions between color spaces.

`Color::clamped_from(value)` creates a stored color and clamps its linear-sRGB
components to `0..=1`, with alpha set to one. It converts the source first;
conversion errors still propagate. Hue wraps during HSL conversion, so 480
degrees represents the same hue as 120 degrees.

```rust
# extern crate ferriswatch_core as ferriswatch;
use ferriswatch::color::{Color, ColorError, DisplayP3, Hsl};

fn main() -> Result<(), ColorError> {
    let color = Color::clamped_from(Hsl::new(480.0, 0.5, 0.25))?;
    let expected = Color::try_from(Hsl::new(120.0, 0.5, 0.25))?;
    assert_eq!(color, expected);
    assert_eq!(color.a(), 1.0);

    let wide_gamut = DisplayP3::new(1.0, 0.0, 0.0);
    assert!(!Color::try_from(wide_gamut)?.is_in_srgb_gamut());
    let clamped = Color::clamped_from(wide_gamut)?;
    assert!(clamped.is_in_srgb_gamut());
    Ok(())
}
```

The same operation is available through `ClampedFrom` and `ClampedInto`, such
as `let color: Color = source.clamped_into()?` with `ClampedInto` in scope.

Clamping occurs only in the destination space of a clamped conversion. Source
channels and intermediate linear-sRGB coordinates retain their extended values.
Byte RGB applies its final bounds immediately before quantization.

For a sequence of conversions, use `ColorSpace::try_into_color()` and unclamped
conversions for intermediate results, then call `to_clamped::<T>()` for the
final target. `Color::clamped_from(...)` treats `Color` as the destination and
clamps immediately; a later conversion cannot recover values clipped there.

```rust
# extern crate ferriswatch_core as ferriswatch;
use ferriswatch::color::{Color, ColorError, DisplayP3, Oklch};

fn main() -> Result<(), ColorError> {
    let stored = Color::try_from(DisplayP3::new(1.0, 0.0, 0.0))?;
    assert!(!stored.is_in_srgb_gamut());

    let intermediate = stored.to_colorspace::<Oklch>()?;
    let restored = Color::try_from(intermediate)?;
    let output = restored.to_clamped::<DisplayP3>()?;
    assert!((*output.r - 1.0).abs() < 0.00001);
    assert!(output.g.abs() < 0.00001);
    assert!(output.b.abs() < 0.00001);
    Ok(())
}
```

## Channel bounds and clamping

`Channel<T>` stores a name, a value, a lower bound, an upper bound, and optional
wrapping. Color components use `Channel<f32>` or `Channel<u8>`. All color-space
types remain `Copy`.

Color-space constructors assign each component its name, bounds, and wrapping
behavior. Constructors for spaces with public channels preserve the supplied
values. `Color::new`, `Srgb::new`, and `LinearSrgb::new` reject non-finite RGB
values.

The built-in constructors configure these bounds:

| Components | Bounds and clamping behavior |
|---|---|
| Floating-point RGB | `0.0..=1.0` |
| Byte RGB | `0..=255` |
| Hue | `0.0..360.0`, with wrapping |
| HSL saturation and lightness; HSV saturation and value | `0.0..=1.0` |
| HWB whiteness and blackness | `0.0..=1.0`, then normalize their sum to at most one |
| Lab and LCh lightness | `0.0..=100.0` |
| Oklab and OkLCh lightness | `0.0..=1.0` |
| LCh and OkLCh chroma | `0.0..`, with no upper bound |
| XYZ coordinates | `0.0..`, with no upper bound |
| Lab and Oklab opponent axes; LMS and cube-root LMS | Unbounded |
| Alpha | `0.0..=1.0` |

`Clamp::clamp()` consumes a value and returns the clamped copy. It does not
perform perceptual gamut mapping or validate non-finite values. Non-finite
inputs to color-space conversions are rejected.

```rust
# extern crate ferriswatch_core as ferriswatch;
use ferriswatch::color::{Clamp, Hsl};

let mut color = Hsl::new(120.0, 0.5, 0.25);
color.h.set_value(-30.0);
assert!(!color.h.in_bounds());

let clamped = color.clamp();
assert_eq!(*clamped.h, 330.0);
assert_eq!(*color.h, -30.0);
```

`in_bounds()` returns a boolean. `value_in_range()` returns a copied value or
`ChannelError::OutsideRange`; neither method clamps or wraps. NaN is outside
every range, including an unbounded range.

`range()` borrows the original bound pair as `(Bound<T>, Bound<T>)`.
`min_value()` and `max_value()` return effective inclusive limits as
`Option<T>`. An excluded endpoint resolves to its adjacent representable value;
an unbounded endpoint returns `None`.

## Building a channel

`Channel::new(name, value, range)` constructs a channel directly.
`Channel::with_name(name)` starts a builder. Its default range is unbounded,
wrapping is disabled, and a value must be supplied before `build()`.

```rust
# extern crate ferriswatch_core as ferriswatch;
use ferriswatch::color::{Channel, ChannelError, Clamp};

fn main() -> Result<(), ChannelError<u16>> {
    let channel = Channel::with_name("R")
        .with_min_value(0)
        .with_max_value(Some(255))
        .with_value(260_u16)
        .with_wrapping()
        .build()?;

    assert_eq!(channel.min_value(), Some(0));
    assert_eq!(channel.max_value(), Some(255));
    assert_eq!(channel.clamp().into_value(), 4);
    Ok(())
}
```

Builder methods can be called in any order:

| Method | Effect |
|---|---|
| `with_value(value)` | Supplies or replaces the value |
| `with_range(range)` | Replaces both bounds, preserving inclusive or exclusive endpoints |
| `with_min_value(value)` | Sets an inclusive minimum and preserves the upper bound |
| `with_max_value(value)` | Sets an inclusive maximum and preserves the lower bound |
| `with_wrapping()` | Enables wrapping |
| `without_wrapping()` | Disables wrapping |
| `build()` | Validates the configuration and returns `Result<Channel<T>, ChannelError<T>>` |

Both bound methods accept a plain value, `Some(value)`, or `None`. `None`
removes that bound. If the range type is not yet known, an explicit type such as
`.with_max_value::<u8>(None)` resolves inference. Building validates the bounds
and wrapping configuration, not the stored value.

## Wrapping

Integer wrapping supports every bounded, nonempty range, including inclusive
endpoints, singletons, and the full type range. This includes `0..=255` for `u8`
and `i128::MIN..=i128::MAX`, even though their widths cannot fit in the value
type. Every representable value is already in bounds for a full-type range.

Floating-point wrapping requires finite `start..end` bounds with an included
start and excluded end. The width must be positive and representable in the
float type. Non-finite values remain unchanged when clamped with wrapping
enabled.

```rust
# extern crate ferriswatch_core as ferriswatch;
use ferriswatch::color::{Channel, ChannelError, Clamp};

fn main() -> Result<(), ChannelError> {
    let hue = Channel::with_name("hue")
        .with_value(-30.0_f32)
        .with_range(0.0..360.0)
        .with_wrapping()
        .build()?;

    assert_eq!(hue.clamp().into_value(), 330.0);
    assert_eq!(hue.without_wrapping().clamp().into_value(), 0.0);
    Ok(())
}
```

On an existing channel, `with_wrapping()` validates the configuration and
returns `Result<Self, ChannelError<T>>`. `without_wrapping()` returns `Self`
directly. Both preserve the value and bounds.

## Changing bounds

`set_min_value(...)` and `set_max_value(...)` accept a plain value,
`Some(value)`, or `None` and return `Result<&mut Self, ChannelError<T>>`. They
preserve the opposite endpoint and stored value. An invalid update leaves the
entire channel unchanged.

```rust
# extern crate ferriswatch_core as ferriswatch;
use ferriswatch::color::{Channel, ChannelError};

fn main() -> Result<(), ChannelError<u8>> {
    let mut red = Channel::new("R", 200_u8, 0..=255)?;
    red.set_max_value(Some(128))?;
    assert_eq!(red.max_value(), Some(128));
    assert_eq!(*red.value(), 200);
    assert!(!red.in_bounds());

    red.set_max_value(None)?;
    assert_eq!(red.max_value(), None);
    assert!(red.in_bounds());
    Ok(())
}
```

Setters rebuild and validate active wrapping. Removing either bound from a
wrapping channel is rejected. Setting an inclusive maximum on a floating-point
wrapping channel is also rejected; disable wrapping first if ordinary clamping
is intended.

## Channel errors and custom value types

Channel errors include the configured channel name:

| Error | Additional information |
|---|---|
| `OutsideRange` | Rejected value |
| `InvalidRange` | Supplied bounds and a `RangeErrorReason` |
| `InvalidWrappingRange` | Supplied bounds and a `WrappingRangeErrorReason` |

Range reasons distinguish unordered bounds such as NaN, reversed bounds, empty
ranges, and excluded endpoints without a representable successor or predecessor.
Wrapping reasons identify unsupported bounds, unbounded integer ranges,
non-finite endpoints, an unrepresentable width, or a nonpositive width.

`ChannelError` defaults to `ChannelError<f32>`. `ColorError` wraps that default
type and preserves its display message. Errors for other channel value types,
such as `ChannelError<u8>`, remain separately typed.

Construction requires `AdjacentValue`, which defines the immediate predecessor
and successor. Wrapping also requires `WrappingValue`. All standard integer
types, `f32`, and `f64` implement both traits. Custom types can implement
`AdjacentValue` for ordinary clamping without implementing arithmetic or
wrapping.

## Alpha

`Alpha<C>` pairs a color with separate, unpremultiplied opacity.
`Alpha::new(color, alpha)` and `set_alpha(...)` reject non-finite values and
values outside `0..=1`. `opaque(color)` creates alpha one. Byte accessors
convert between `0..=255` and the normalized fraction.

```rust
# extern crate ferriswatch_core as ferriswatch;
use ferriswatch::color::{Alpha, ColorError, Rgb};

fn main() -> Result<(), ColorError> {
    let mut rgba = Alpha::new(Rgb::new(255, 128, 0), 0.5)?;
    assert_eq!(rgba.alpha_u8(), 128);
    assert_eq!(format!("{rgba:#X}"), "#FF800080");

    rgba.set_alpha_u8(255);
    assert_eq!(rgba.alpha(), 1.0);
    Ok(())
}
```

Aliases such as `Rgba`, `SRgba`, `Hsla`, and `Oklcha` name `Alpha<Rgb>`,
`Alpha<Srgb>`, `Alpha<Hsl>`, and `Alpha<Oklch>`. `color()` and `color_mut()`
access the wrapped color. `alpha_channel()` exposes its opacity channel
immutably.

`Color::a()` reads opacity, `set_a(...)` validates a replacement, and
`a_channel()` borrows its channel. `Color::linear_srgba()` preserves alpha.
Other named and generic color-space conversions ignore alpha; it can be carried
explicitly with `Alpha::new(converted, color.a())`.

## Formatting

Color-space `Display` implementations, used by `{}` and `.to_string()`, produce
CSS color syntax except for LMS and
cube-root LMS, which produce descriptive text. HSV displays as equivalent HWB.
Lab and LCh use D50, while `Xyz` uses D65.

Floating-point formatting defaults to three decimal places. `{:.N}` chooses
precision, and non-finite components display as `none`. Display formatting does
not clamp finite values or guarantee a lossless round trip.

`Rgb` and `Srgb` support lowercase and uppercase hex. The alternate `#` flag
adds a CSS `#` prefix. sRGB hex formatting clamps to `0..=1` and rounds to
bytes; alpha wrappers append a rounded alpha byte.

`Rgb::to_hex()` and `Rgb::to_upper_hex()` return uppercase `#RRGGBB` strings.
`Rgb::to_lower_hex()` returns lowercase `#rrggbb`. All three include the `#`
prefix and two digits per channel.

```rust
# extern crate ferriswatch_core as ferriswatch;
use ferriswatch::color::{Channel, ChannelError, Hsl, Rgb};

fn main() -> Result<(), ChannelError<u8>> {
    assert_eq!(format!("{:.1}", Hsl::new(120.0, 0.5, 0.25)), "hsl(120.0 50.0% 25.0%)");
    let rgb = Rgb::new(255, 128, 0);
    assert_eq!(rgb.r_channel().name(), "r");
    assert_eq!(format!("{rgb:#X}"), "#FF8000");
    assert_eq!(rgb.to_hex(), "#FF8000");
    assert_eq!(rgb.to_upper_hex(), "#FF8000");
    assert_eq!(rgb.to_lower_hex(), "#ff8000");

    let byte = Channel::new("R", 10_u8, 0..=255)?;
    assert_eq!(format!("{byte:02x}"), "0a");
    assert_eq!(format!("{byte:#X}"), "0xA");
    Ok(())
}
```

`Channel<u8>` delegates hex formatting to its byte value, including padding and
the standard `0x` alternate prefix. Its formatting uses the stored value without
clamping.

## Storage and compatibility

`Channel<T>` stores bounds as a pair regardless of the range syntax passed to
its constructor. This lets setters add or remove endpoints without changing the
channel's type.

Color-space fields now contain channels. Construction uses methods such as
`Hsl::new(h, s, l)` and `Rgb::new(r, g, b)`. `Rgb` and `Color` do not expose the
former C-compatible scalar layout. Numeric getters provide component values for
external formats.
