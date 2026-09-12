# Palette families

All factories return `ThemeVariant`. Choose an accent type from the variant's
module or use `NoAccent` for its default primary colour. `NoAccent` retains
`None` for both accent colour and accent name. Explicit transparent accents
remain transparent.

```rust
use ferriswatch::palette::{tokyo_night, rose_pine, gruvbox, kanagawa, everforest, NoAccent};
use ferriswatch::theme_variant::ThemePalette;

let themes = [
    tokyo_night::Night::variant::<tokyo_night::night::Blue>(),
    rose_pine::Dawn::variant::<rose_pine::dawn::Iris>(),
    gruvbox::DarkMedium::variant::<gruvbox::dark_medium::Orange>(),
    kanagawa::Wave::variant::<kanagawa::wave::CrystalBlue>(),
    everforest::LightMedium::variant::<everforest::light_medium::Green>(),
];
assert_eq!(themes[3].name(), "Kanagawa Wave");
assert_eq!(themes[3].accent_name(), Some("CrystalBlue"));
assert_eq!(rose_pine::Moon::variant::<NoAccent>().accent_name(), None);
```

Types are re-exported from each family module. Constants use uppercase snake
case, including numbered shades such as `BG_0`, `BLUE_2`, and `SUMI_INK_3`.
Kanagawa exposes its shared raw colour palette on all three variant types.
Accent names retain the Rust type spelling, such as `CrystalBlue`.

## Variants and defaults

| Module | Type | Default accent |
| --- | --- | --- |
| `tokyo_night::night` | `Night` | Blue |
| `tokyo_night::storm` | `Storm` | Blue |
| `tokyo_night::moon` | `Moon` | Blue |
| `tokyo_night::day` | `Day` | Blue |
| `rose_pine::main` | `Main` | Iris |
| `rose_pine::moon` | `Moon` | Iris |
| `rose_pine::dawn` | `Dawn` | Iris |
| `gruvbox::dark_hard` | `DarkHard` | Orange |
| `gruvbox::dark_medium` | `DarkMedium` | Orange |
| `gruvbox::dark_soft` | `DarkSoft` | Orange |
| `gruvbox::light_hard` | `LightHard` | Orange |
| `gruvbox::light_medium` | `LightMedium` | Orange |
| `gruvbox::light_soft` | `LightSoft` | Orange |
| `kanagawa::wave` | `Wave` | CrystalBlue |
| `kanagawa::dragon` | `Dragon` | DragonBlue2 |
| `kanagawa::lotus` | `Lotus` | LotusBlue4 |
| `everforest::dark_hard` | `DarkHard` | Green |
| `everforest::light_hard` | `LightHard` | Green |
| `everforest::dark_medium` | `DarkMedium` | Green |
| `everforest::light_medium` | `LightMedium` | Green |
| `everforest::dark_soft` | `DarkSoft` | Green |
| `everforest::light_soft` | `LightSoft` | Green |

## Colour sources

Ferriswatch stores literal colours from the following upstream definitions, so
builds do not fetch palettes or depend on editor plugins.

- Tokyo Night uses [Storm](https://github.com/folke/tokyonight.nvim/blob/main/lua/tokyonight/colors/storm.lua), [Night overrides](https://github.com/folke/tokyonight.nvim/blob/main/lua/tokyonight/colors/night.lua), [Moon](https://github.com/folke/tokyonight.nvim/blob/main/lua/tokyonight/colors/moon.lua), and the [generated Day palette](https://github.com/folke/tokyonight.nvim/blob/main/extras/lua/tokyonight_day.lua). Day's values already include upstream's colour inversion.
- Rosé Pine uses its [Neovim palette](https://github.com/rose-pine/neovim/blob/main/lua/rose-pine/palette.lua), including the Leaf accent.
- Gruvbox uses the [original palette and mode mappings](https://github.com/morhetz/gruvbox/blob/master/colors/gruvbox.vim). Dark uses bright accents and light uses faded accents. Medium is the default background contrast; hard and soft change `BG_0`.
- Kanagawa uses its [raw palette](https://github.com/rebelot/kanagawa.nvim/blob/master/lua/kanagawa/colors.lua) and [theme roles](https://github.com/rebelot/kanagawa.nvim/blob/master/lua/kanagawa/themes.lua).
- Everforest combines the [contrast backgrounds and dark/light foregrounds](https://github.com/sainnhe/everforest/blob/master/autoload/everforest.vim).

## Semantic mappings

Raw colours come from upstream. The selection of defaults and assignment to
Ferriswatch's semantic roles are library policy, rather than an exact reproduction
of every editor highlight. Each variant's `roles` block records the mapping.

Primary and focus use the selected accent. Hover scales linear RGB by 0.85 and
preserves alpha, consistently with Catppuccin. Status colours stay fixed when
switching accents. Kanagawa uses upstream diagnostic colours for success,
warning, error, and info. Rosé Pine uses Leaf, Gold, Love, and Foam for those roles.

No contrast ratio is implied by a role name; applications still need to choose
foreground/background pairings appropriate to their controls.

## Resolved colour groups

`ThemeVariantColors` contains concrete `Color` values grouped into `surface`,
`surface_alt`, `text`, `primary`, `secondary`, and `status`, alongside `border`,
`border_muted`, and `focus`. `surface` and `surface_alt` are independent
`SurfaceColors` values, each containing `background`, `surface`, `raised`,
`overlay`, and `hover`. There are no alternate fields inside `SurfaceColors`.

`TextColors` contains `normal`, `muted`, `subtle`, `on_primary`, and
`on_secondary`. `ActionColors` contains `normal`, `hover`, `pressed`, and `muted`.
`StatusColors` contains `success`, `warning`, `error`, `critical`, `info`, and
`trace`, with no alternates.

```rust
use ferriswatch::palette::catppuccin::mocha::{Mocha, Mauve};
use ferriswatch::theme_variant::{ThemePalette, ThemeVariant};

let theme = Mocha::variant::<Mauve>();
let mut colors = *theme.colors();
colors.surface_alt.background = Mocha::MANTLE;
colors.surface_alt.raised = Mocha::SURFACE_2;
colors.status.critical = Mocha::MAROON;
let custom = ThemeVariant::new("Custom Mocha", colors, theme.accent(),
    theme.accent_name().map(str::to_owned));
assert_eq!(custom.colors().surface_alt.background, Mocha::MANTLE);
assert_eq!(custom.background(), Mocha::BASE);
```

Initial built-in mappings repeat the normal surfaces in `surface_alt`, use the
existing surface hover for `raised`, use `error` for `critical`, and subtle text
for `trace`. Pressed actions repeat their normal colour; secondary muted repeats
secondary normal. These are explicit assignments that each palette can change.
Existing getters still return their original roles; `colors()` exposes all groups.

Factories select black or white action text using the action's linear RGB
luminance. Transparent fills depend on the final composited background, so those
foreground choices must be reviewed in the application. Colour fields never use
`None` or resolve fallbacks during rendering. Optional accent metadata still
represents whether an accent was explicitly selected.
