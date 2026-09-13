# Ferriswatch

Shared colour palettes and resolved themes for application styling.

Ferriswatch provides 64 palettes across 28 families, typed accent factories, and a
runtime catalogue with stable theme and accent IDs. See the [complete catalogue](docs/Catalogue.md).

## Usage

```rust
use ferriswatch::palette::catppuccin::mocha::{Mauve, Mocha};
use ferriswatch::theme_variant::ThemePalette;
use ferriswatch::catalogue;

let typed = Mocha::variant::<Mauve>();
assert_eq!(typed.primary(), Mocha::MAUVE);
assert_eq!(typed.id(), "catppuccin/mocha");
assert_eq!(typed.accent_id(), Some("mauve"));

// Settings can choose the same theme without retaining palette types.
let selected = catalogue::resolve("catppuccin/mocha", Some("mauve"))?;
assert_eq!(typed, selected);

let default = catalogue::resolve("everforest/light/soft", None)?;
assert!(default.selected_accent().is_none());
# Ok::<(), ferriswatch::catalogue::ResolveError>(())
```

`ThemeVariantColors` groups the two independent surface sets, text, actions and
statuses. Every colour is concrete, including transparent colours and action
foregrounds. `ThemeVariant` adds identity and optional explicit accent metadata;
all built-in factories return this same runtime type.

Read [palette usage and migration](docs/Palettes.md) for custom themes, metadata,
source policy and the constructor changes. [Mapping validation](docs/PaletteValidation.md)
documents visual decisions and remaining pairing limitations.

## Dioxus

Framework-independent CSS variables and optional semantic color classes are
available through `ferriswatch::css`. See the [CSS guide](docs/Css.md) for
`DEFAULT_CSS`, generation, and commit-hook setup.

Enable the optional `dioxus` feature for typed theme selection, a required-default
configuration builder, shared theme state, a provider and a basic theme picker.
See [Dioxus integration](docs/Dioxus.md) for setup and custom palettes.

## Palette inspector

```sh
cargo run --example catalogue -- html > target/palette-inspector.html
```

Open the generated file in a browser. Search by family or ID and switch accents
to compare panels, raised content, popups, text tiers, actions and statuses.

## Development

```sh
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets
cargo doc --locked --no-deps
python tools/check_palette_sources.py
```

Builds use checked-in literals and never download themes. Catalogue documentation
is generated from the same registrations used by runtime lookup.

## License

MIT. See [LICENSE](LICENSE). Imported palette data retains its
[upstream attribution and licenses](docs/PaletteSources.md).
