# Ferriswatch

Shared themes and colour palettes for consistent styling across applications.

The goal is to define themes in one place so applications can use the same colours
and styling conventions.

## Status

Early development. Catppuccin, Tokyo Night, Rosé Pine, Gruvbox, Kanagawa, and
Everforest provide raw palette constants and semantic colours for application styling.

## Usage

```rust
use ferriswatch::palette::catppuccin::mocha::{Blue, Mauve, Mocha};
use ferriswatch::palette::NoAccent;
use ferriswatch::theme_variant::ThemePalette;

let mut theme = Mocha::variant::<Mauve>();
assert_eq!(theme.primary(), Mocha::MAUVE);
assert_eq!(theme.name(), "Catppuccin Mocha");
assert_eq!(theme.accent(), Some(Mocha::MAUVE));
assert_eq!(theme.accent_name(), Some("Mauve"));

// Every factory returns ThemeVariant, so settings can switch themes at runtime.
theme = Mocha::variant::<Blue>();
assert_eq!(theme.primary(), Mocha::BLUE);

theme = Mocha::variant::<NoAccent>();
assert_eq!(theme.accent(), None);
assert_eq!(theme.accent_name(), None);
assert_eq!(theme.primary(), Mocha::MAUVE);
```

`Palette` identifies raw colour collections. `ThemePalette: Palette` maps those
colours into semantic roles. `Accent<P>` restricts accent choices to their palette
and exposes `Option<Color>`; an explicit transparent colour remains `Some`.

Catppuccin flavours default to their own mauve when no accent is selected. Primary hover darkens the
resolved primary colour by 15% in linear RGB and preserves its alpha.

`ThemeVariant` stores `ThemeVariantColors` directly alongside the optional accent.
Custom mappings can use `ThemeVariant::new(name, colors, accent, accent_name)`. Theme and accent names are owned strings, so custom
themes can use names loaded at runtime. Variants implement `Clone`, not `Copy`.

The module is named `palette`. The earlier `pallet` spelling and generic
`ThemeVariant<A, P>` API have been replaced; unused named/scaled palette traits
have been removed.

## License

MIT. See [LICENSE](LICENSE).

## Catppuccin flavours

Each flavour has 26 colour constants and 14 named accents, using colours from
[Catppuccin palette 1.8.0](https://github.com/catppuccin/palette).

| Module | Palette type | Display name |
| --- | --- | --- |
| `catppuccin::latte` | `Latte` | Catppuccin Latte |
| `catppuccin::frappe` | `Frappe` | Catppuccin Frappé |
| `catppuccin::macchiato` | `Macchiato` | Catppuccin Macchiato |
| `catppuccin::mocha` | `Mocha` | Catppuccin Mocha |

Palette types are also re-exported directly from `palette::catppuccin`.
Accent types belong to their flavour, for example
`Latte::variant::<latte::Blue>()` or `Frappe::variant::<frappe::Mauve>()`.

## Other theme families

| Family module | Variants |
| --- | --- |
| `tokyo_night` | Night, Storm, Moon, Day |
| `rose_pine` | Main, Moon, Dawn |
| `gruvbox` | Dark and light, each with hard, medium, and soft contrast |
| `kanagawa` | Wave, Dragon, Lotus |
| `everforest` | Dark and light, each with hard, medium, and soft contrast |

See [palette families](docs/Palettes.md) for imports, named accents, default colours,
semantic mapping policy, and upstream sources.
