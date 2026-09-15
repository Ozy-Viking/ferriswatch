# Palette families

Ferriswatch includes 64 concrete palettes across 28 families. All factories return
`ThemeVariant`. The [generated catalogue](Catalogue.md) lists every stable theme ID,
accent ID, default accent and pinned upstream source. [Source attribution](PaletteSources.md)
links the retained upstream licenses.

## Typed factories and runtime selection

```rust
use ferriswatch::palette::{kanagawa, everforest, NoAccent};
use ferriswatch::theme_variant::ThemePalette;
use ferriswatch::catalogue;

let typed = kanagawa::Wave::variant::<kanagawa::wave::CrystalBlue>();
assert_eq!(typed.id(), "kanagawa/wave");
assert_eq!(typed.accent_id(), Some("crystal_blue"));
assert_eq!(typed.accent_name(), Some("Crystal Blue"));
let selected = catalogue::resolve("kanagawa/wave", Some("crystal_blue"))?;
assert_eq!(typed, selected);

let default = everforest::LightSoft::variant::<NoAccent>();
assert_eq!(default.id(), "everforest/light/soft");
assert!(default.selected_accent().is_none());
# Ok::<(), ferriswatch::catalogue::ResolveError>(())
```

Palette types are re-exported from their family modules. Single-variant families
use `main::Main`. Typed accents remain local to each concrete palette module and implement
`FromStr` / `Display` for their persisted snake_case ID:

```rust
use ferriswatch::palette::catppuccin::mocha::Blue;
let accent: Blue = "blue".parse()?;
assert_eq!(accent.to_string(), "blue");
# Ok::<(), ferriswatch::palette::ParseAccentError>(())
```
Raw constants preserve source vocabulary, such as `SUMI_INK_3`, `NORD_0`,
`BASE_0A`, and editor keys normalized to uppercase snake_case.

`catalogue::PALETTES` is the single runtime registration list. Each entry owns its
metadata, supported accents, default factory, raw colours and source references.
`catalogue::get(id)` exposes that entry for menus and discovery;
`catalogue::resolve(id, accent_id)` creates the selected theme. Unknown theme IDs,
unsupported contrasts and unknown accents return distinct typed errors.
An empty accent ID is an error. Omit the accent ID to choose `NoAccent`.

## Identity

Persist `family/variant[/contrast]` plus an optional accent ID. Gruvbox and
Everforest always include contrast, including `medium`. Appearance is independent
metadata: Ayu Mirage is variant `mirage` with dark appearance. Display labels never
act as identifiers. `ThemeVariant::name()` returns the full label; `metadata()`
provides family, variant, appearance and contrast separately.

`selected_accent()` returns one optional `ResolvedAccent`, keeping its stable ID,
display label and colour together. `accent()`, `accent_name()` and `accent_id()`
remain convenience accessors. Explicit transparent accents retain all metadata.
`NoAccent` uses the palette-default colours and does not record a
`ResolvedAccent`; its typed id is `none` and display name is `None`.
Custom `Accent<P>` implementations always supply `ID` and `NAME`. `ACCENT` is
`None` only for `NoAccent`. Factories panic if the id or name is invalid.

## Resolved colour groups

`ThemeVariantColors` groups the palette mappings:

| Group | Fields |
| --- | --- |
| `surfaces`, `surfaces_alt` | `background`, `base`, `raised`, `overlay`, `hover` |
| `text`, optional `text_alt` | `normal`, `muted`, `subtle` |
| `primary`, `secondary` | `normal`, `hover`, `pressed`, `muted`, `disabled`, each a `ColorPair` with `foreground` and `background` |
| `status` | `success`, `warning`, `error`, `critical`, `info`, `debug`, `trace` |
| `syntax` | highlighting roles including `macro_name` |
| `chromatic` | `red`, `orange`, `yellow`, `green`, `cyan`, `blue`, `purple`, `pink` |
| Ungrouped | `border`, `border_muted`, `focus` |

The two surface groups are independent. Placement does not imply brightness order.
Factories supply every colour; simple palettes deliberately repeat source colours.
Built-in palettes set `text_alt` to `None`, so `text_alt()` resolves to the normal
text group. Transparency is never replaced with a default. Group getters return
their full structures; `colors()` exposes every field.

Muted actions pair normal text with the existing muted fill. Disabled actions pair
subtle text with that fill. Debug uses muted text, and syntax `macro_name` uses the
function colour. These mappings reuse existing palette colours.

Primary and focus use the selected accent. Derived hover scales linear RGB by
0.85, and pressed scales it by 0.70, preserving alpha. Secondary states use explicit
upstream interaction colours where mapped, including JetBrains controls.
Foreground selection chooses the black or white colour with the best minimum
contrast over the derived normal/hover/pressed range. A fixed secondary action
keeps its foreground when only the primary accent changes.

This is a mapping policy, not a claim that every role can be used as small text on
every background. The [mapping review](PaletteValidation.md) records repeated
assignments, source-specific decisions and measured limitations. Rendering an
application with transparent fills requires checking its actual composited canvas.

## Custom themes

Custom themes require a stable, caller-supplied `custom/identifier`. Labels may be
owned strings loaded at runtime. They need not be registry entries.

```rust
use ferriswatch::{catalogue, color::Color};
use ferriswatch::theme_variant::{ThemeVariant, ResolvedAccent};

let original = catalogue::resolve("catppuccin/mocha", None)?;
let mut colors = *original.colors();
colors.surfaces_alt.background = Color::TRANSPARENT;
let accent = ResolvedAccent::new("clear", "Clear", Color::TRANSPARENT)?;
let custom = ThemeVariant::new(
    "custom/my_workspace", "My workspace", original.metadata().appearance,
    colors, Some(accent),
)?;
assert_eq!(custom.colors().surfaces, original.colors().surfaces);
assert_eq!(custom.colors().surfaces_alt.background, Color::TRANSPARENT);
assert_eq!(custom.accent_id(), Some("clear"));
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Migration

The incomplete `ThemeName` enum has been removed. Use stable catalogue IDs and
`ThemeMetadata`; `theme` re-exports the new lookup and identity API. No persisted
legacy ID format was present in this repository, so no guessed aliases are added.

`ThemeVariant::new` now accepts an explicit custom ID, name, appearance, colours,
and optional `ResolvedAccent`, and returns a `Result`. Typed built-in factory paths
are unchanged. The display labels `Tokyo Night Night`, `Rosé Pine Main`, and joined
accent labels such as `CrystalBlue` become `Tokyo Night`, `Rosé Pine`, and
`Crystal Blue`. Persist IDs rather than these labels.

## Verification and visual inspection

```sh
cargo test --locked
cargo test --locked --doc
cargo clippy --locked --all-targets
cargo doc --locked --no-deps --all-features
cargo run --locked --example catalogue -- docs > docs/Catalogue.md
cargo run --locked --example catalogue -- html > target/palette-inspector.html
cargo run --locked --example catalogue -- contrast > target/palette-contrast.tsv
python tools/check_palette_sources.py
```

Open the standalone inspector in a browser to compare all palettes or search for
one family. Its accent menus are generated from registrations. The TSV reports
text tiers over both surface groups, action foregrounds over normal/hover/pressed
fills, and all status colours over the canvas, for every registered accent and
`NoAccent`. It composites alpha in sRGB before measuring relative luminance.

Pinned source snapshots and independently imported expected literals live under
`tests/fixtures`. Source code, snapshots and fixtures are checked in together.
Normal builds and tests do not fetch upstream resources.
