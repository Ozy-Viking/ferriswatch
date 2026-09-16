# Dioxus integration

Enable Ferriswatch's `dioxus` feature alongside the consuming application's Dioxus
0.7 dependency. Ferriswatch supplies theme configuration, provider state, and
hooks. Widgets that wrap `dioxus-primitives` live in `ferriswatch-componant`.
Your application chooses the web, desktop or other renderer.

```toml
[dependencies]
ferriswatch = { version = "0.3.0", features = ["dioxus"] }
dioxus = { version = "0.7.10", features = ["web"] }
```

To add `ferriswatch-componant`, follow the
[registry and dependency setup in the source repository](https://github.com/Ozy-Viking/ferriswatch/blob/main/docs/Onedev.md#cargo-registry).

## Configure the application

```rust
use dioxus::prelude::*;
use ferriswatch::{
    dioxus::ThemeProvider,
    palette::{
        catppuccin::{Latte, Mocha, latte::Blue, mocha::Mauve},
        families::{Catppuccin, RosePine},
    },
    theme_variant::ThemePalette,
    theme::{Appearance, Theme, ThemeSelection, config::ThemeConfig},
};

#[component]
fn App() -> Element {
    let config = use_hook(|| {
        ThemeConfig::with_default(
            Theme::new(Latte::variant::<Blue>(), Mocha::variant::<Mauve>()),
            Appearance::Dark,
        )
            .available(
                ThemeSelection::all()
                    .without_family::<RosePine>()
                    .without_family::<Catppuccin>()
                    .with_palette::<Mocha>()
                    .with_palette::<Latte>(),
            )
            .build()
            .expect("the configured default is available")
    });

    rsx! {
        ThemeProvider { config,
            button {
                style: "background:var(--fs-primary);color:var(--fs-on-primary);border:1px solid var(--fs-border)",
                "Save"
            }
        }
    }
}
```

`ThemeConfig::with_default(theme, mode)` requires both saved variants and an
initial mode. The builder starts with all built-in palettes available.
`available` replaces that selection. `build` rejects empty selections,
conflicting registrations, inconsistent accent factories, and either default that
does not exactly match an allowed registered theme and accent. Mode support does
not restrict defaults or programmatic selection.

`ThemeSelection::new()` starts with no palettes. `with_family`, `with_palette`
and `with_custom` **add** to the existing selection; they do not replace it.
`without_family` and `without_palette` remove palettes. Calls apply in order,
so a later addition restores a removed palette at the end. Repeated additions
have no effect. Families expand in catalogue order. Conflicting registrations
with the same ID are configuration errors.

Family markers live in `palette::families`, for example `Catppuccin`, `RosePine`
and `TokyoNight`. Palette types remain in their existing family modules.

## Shared state

Put `ThemeProvider` at the application root. Descendants call `use_theme::<S>()`
to read the active theme or select another allowed theme. State belongs to the
provider, so separate application instances and SSR requests do not share it.
Nested providers have independent state and CSS scopes.

```rust
use dioxus::prelude::*;
use ferriswatch::dioxus::{LocalStorage, use_theme};

#[component]
fn ResetTheme() -> Element {
    let mut theme = use_theme::<LocalStorage>();
    let active = theme.current();
    rsx! {
        button { onclick: move |_| theme.reset(), "Reset {active.name()}" }
    }
}
```

`ThemeState::select(theme_id, accent_id)` updates only the active mode's saved
variant. `select_theme(id)` keeps a compatible accent or the palette default.
`select_accent(None)` (or `""`) records the palette default. `select_for(mode,
theme_id, accent_id)` updates a specified slot without switching modes. Failed
selects set `last_error()` and leave the current theme unchanged. `toggle_mode()`
and `set_mode(Appearance::Light)` switch to the saved selection for that
appearance without changing either slot. `variant(mode)` reads one slot;
`theme()` reads the pair. `reset()` restores both defaults and the initial mode.
`theme_value()` and `accent_value()` are memos for combobox bindings (`""` is the
default accent). A Dioxus store tracks the pair and mode, with field
subscriptions so inactive-slot changes do not notify components that read only
`current()`.

`use_theme::<Memory>()` does not persist. `use_theme::<LocalStorage>()` and
`use_theme::<SessionStorage>()` load and save a [`ThemeSnapshot`] through
[`dioxus_sdk_storage::StorageBacking`] under `ferriswatch.theme.v1`. Invalid
saved ids are ignored and leave the configured default in place. Any type that
implements [`dioxus_sdk_storage::StorageBacking`] with a `String` key gets
`load`/`save` from [`ThemeStorage`].

## Mode support and filtering

Each built-in palette explicitly declares `support(Light)` or `support(Dark)`.
`ThemeSupport::Both` includes a palette in either list. Support is stored in
`ThemeMetadata` and accessible through `ThemeVariant::support()` and
`supports(mode)`. It is separate from `Appearance`, which describes the palette's
actual background and controls CSS `color-scheme`.

For application-defined variants, `ThemeVariant::new` initially derives support
from appearance. Use `.with_support(ThemeSupport::Both)` in the custom factory
to override it, and use matching metadata in its registration.

`catalogue::palettes_for(mode)` returns eligible built-in registrations.
`catalogue::variants_for(mode)` returns their default `ThemeVariant` values.
`ThemeConfig` exposes the same methods scoped to its allowed selection, including
custom palettes. Both methods include `Both` entries and retain catalogue order.
Accent choices are applied through the usual resolution methods.

`ThemeState::listed_palettes(matching_mode_only)` is the menu helper: `true`
lists the active mode, `false` lists every configured palette. An explicitly
selected out-of-mode theme stays visible because support is advisory.
`selected_theme` is the current palette registration and `available_accents`
are its named accents.

`Theme::new(light, dark)` holds independent selections, including accents.
`variant(mode)` reads a selection and `set(mode, variant)` replaces just that slot.
Support is advisory: these methods and provider selection do not reject a
variant because its support differs from the target mode.

`ferriswatch_componant::ThemePicker` includes a Light/Dark switch and filters
palette options for that mode. If an application explicitly selects an
out-of-mode palette, the picker includes it as the current selection rather than
displaying an unrelated value.

The provider captures configuration on mount. Remount it with a new Dioxus key
to replace the configuration and reset the subtree. For SSR, supply matching
initial configuration on server and client.

## Custom palettes

Implement the existing `Palette` and `ThemePalette` traits, and implement
`Accent<YourPalette>` for each supported accent. `Palette::registration()` now
returns a static `PaletteRegistration` reference. Built-in palettes return their
existing `REGISTRATION`; custom palettes can use a static `LazyLock` to construct
metadata through `ThemeVariant::new` with a `custom/identifier`.

Registration includes metadata, supported accent labels/IDs/colours and factories,
the default accent ID, the default factory, raw colours and upstream sources.
The default factory must return a theme with no explicit accent. Its colours
must match those from the declared default accent factory. Every accent factory
must return matching palette metadata and accent metadata.

Factory fields on `PaletteRegistration` and `AccentRegistration` are public so
external applications can construct registrations. Register a custom palette
with `ThemeSelection::new().with_custom::<YourPalette>()`; it can also supply
the mandatory default through `YourPalette::variant::<YourAccent>()`.

Existing custom `Palette` implementations must add `registration()`.

## Styling and scope

The provider renders a `div.fs-theme`. By default, `ThemeScope::Scoped` publishes
all `--fs-*` variables and `color-scheme` on that wrapper. Its background and
text colors retain the existing defaults. `scope: ThemeScope::Root` publishes
variables and scheme on `:root`, including for portals in the same document.
Use one root provider per document; nested scoped providers override inherited
values. Provider state remains scoped through Dioxus context in either mode.

`theme_css` now exports only variable declarations. Custom wrappers must set
background/text and `color-scheme` themselves. Colors preserve alpha.

Use `DEFAULT_STYLESHEET` to load the optional semantic color classes through
Dioxus's stylesheet asset integration. It is independent of provider scope and
is never loaded automatically. `DefaultStyles {}` remains a convenience wrapper.
For example:

```rust,no_run
use dioxus::prelude::*;
use ferriswatch::{
    dioxus::{DEFAULT_STYLESHEET, ThemeProvider, ThemeScope},
    theme::config::ThemeConfig,
};

#[component]
fn ThemedApp(config: ThemeConfig) -> Element {
    rsx! {
        document::Stylesheet { href: DEFAULT_STYLESHEET }
        ThemeProvider { config, scope: ThemeScope::Root,
            section { class: "fs-card", "Application-owned layout" }
        }
    }
}
```

See [`crate::css`] for class mappings, overrides, generation, and commit checks.
`ferriswatch-componant` provides `ThemePicker` and `ThemeCombobox`. The picker
combines the light/dark slider with `ThemeCombobox`, a joined theme/accent
control built from `dx components add combobox`. Click the left half to browse
or search themes; use the right half to browse or search its accents. Arrow
keys navigate, Enter selects, and Escape closes without changing the
selection. Only one dropdown opens at a time.

Use `ThemeCombobox {}` directly within a provider when your application has its
own mode control. The widget ships no layout or chrome CSS: pass `class` and
the `*_class` props for each part. `matching_mode_only` (default `true`) lists
only palettes that support the active appearance; set it to `false` to list
every configured palette. Changing theme retains a compatible accent, otherwise
it uses the palette default. Each mode keeps its own saved selection.
Explicitly selected out-of-mode themes remain visible because support is
advisory.

`ThemePicker` passes Ferriswatch's joined layout and combobox classes into
`ThemeCombobox`. Those styles use `--fs-*` colors and do not extend
`DEFAULT_CSS` or require the Dioxus Components global theme stylesheet.

## Override Dioxus Components colors

Opt in when constructing the provider config:

```rust
use ferriswatch::{
    palette::{NoAccent, catppuccin::{Latte, Mocha}},
    theme::{Appearance, Theme, ThemeError, config::ThemeConfig},
    theme_variant::ThemePalette,
};

let config = ThemeConfig::with_default(
    Theme::new(Latte::variant::<NoAccent>(), Mocha::variant::<NoAccent>()),
    Appearance::Dark,
)
.override_dx_components_theme(true)
.build()?;
# Ok::<(), ThemeError>(())
```

This overrides the color variables from `dx-components-theme.css` in the
provider's `ThemeScope`, without rewriting the file. Continue loading the
upstream stylesheet when other components need it. The setting defaults to
false and is captured on provider mount, like the rest of `ThemeConfig`.

The adapter targets [Dioxus Components revision 9a758255](https://github.com/DioxusLabs/components/blob/9a758255ea26e2b20c8cecf4c1c946feb9e71da7/preview/assets/dx-components-theme.css).
Upstream `primary-color*` variables describe neutral backgrounds and borders;
`secondary-color*` describe foregrounds and borders. They do not map to
Ferriswatch's primary and secondary action colors.

| Upstream variables | Ferriswatch roles |
| --- | --- |
| `primary-color`, `primary-color-2` | background, alternate background |
| `primary-color-1`, `primary-color-3` | surface |
| `primary-color-4`, `primary-color-5` | hover, raised |
| `primary-color-6`, `primary-color-7` | muted border, border |
| `secondary-color` through `secondary-color-4` | text |
| `secondary-color-5`, `secondary-color-6` | muted text, border |
| `focused-border-color` | focus |
| `primary-success/warning/info-color` | 15% status color mixed with surface |
| `secondary-success/warning/info-color` | status foreground |
| `primary-error-color` | error |
| `secondary-error-color` | 85% error mixed with text |
| `contrast-error-color` | background |

The `--light`/`--dark` and `--dxc-light-on`/`--dxc-dark-on` switches follow the
variant's actual appearance. Root mode outranks upstream `:root` and
`html[data-theme]` rules even when the upstream stylesheet loads later. Scoped
mode sets aliases on the provider wrapper. Nested providers should enable the
adapter too when their DX components need the nested palette; disabling the
adapter does not erase CSS inherited from an ancestor. Layout, animations,
typography, and Ferriswatch's default semantic stylesheet are unchanged.

## Run the example

From `examples/dioxus_theme`, run `dx serve --web`. The standalone example
offers all built-in palettes filtered by mode and presents a responsive theme workbench. It includes
an editable project board, task progress, feedback states, a reset button, and a
live reference for every exported `--fs-*` colour. Project state survives theme and view
changes; light and dark retain their own palettes and accents. Reloading resets
the session. See `examples/dioxus_theme/README.md` for a walkthrough.
Its renderer dependencies stay separate from the library's feature set.

With the example running on port 8080, run its browser regression checks with
`uv run --with playwright python test_browser.py` from the example directory.
Install Chromium first with `uv run --with playwright playwright install chromium`.
Set `FERRISWATCH_EXAMPLE_URL` for another port or
`PLAYWRIGHT_CHROMIUM_EXECUTABLE` to use an existing Chromium installation.
