# Dioxus integration

Enable Ferriswatch's `dioxus` feature alongside the consuming application's Dioxus
0.7 dependency. Ferriswatch supplies components and hooks; your application
chooses the web, desktop or other renderer.

```toml
[dependencies]
ferriswatch = { path = "../ferriswatch", features = ["dioxus"] }
dioxus = { version = "0.7.10", features = ["web"] }
```

## Configure the application

```rust
use dioxus::prelude::*;
use ferriswatch::{
    dioxus::{ThemeConfig, ThemePicker, ThemeProvider, ThemeSelection},
    palette::{
        catppuccin::{Latte, Mocha, latte::Blue, mocha::Mauve},
        families::{Catppuccin, RosePine},
    },
    theme_variant::ThemePalette,
    theme::{Theme, Appearance},
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
            ThemePicker {}
            button {
                style: "background:var(--fw-primary);color:var(--fw-on-primary);border:1px solid var(--fw-border)",
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

Put `ThemeProvider` at the application root. Descendants call `use_theme()` to
read the active theme or select another allowed theme. State belongs to the
provider, so separate application instances and SSR requests do not share it.
Nested providers have independent state and CSS scopes.

```rust
use dioxus::prelude::*;
use ferriswatch::dioxus::use_theme;

#[component]
fn ResetTheme() -> Element {
    let mut theme = use_theme();
    let active = theme.current();
    rsx! {
        button { onclick: move |_| theme.reset(), "Reset {active.name()}" }
    }
}
```

`ThemeState::select(theme_id, accent_id)` updates only the active mode's saved
variant. `select_for(mode, theme_id, accent_id)` updates a specified slot without
switching modes. It returns errors without changing the current
theme. `None` selects the palette default without recording an explicit accent;
selecting that same accent by name records its ID. The picker retains a supported
accent when switching palettes and otherwise uses the new palette's default.
`set_mode(Appearance::Light)` switches to the saved light selection without
changing either slot. `variant(mode)` reads one slot; `theme()` reads the pair.
`reset()` restores both defaults and the initial mode. A Dioxus store tracks the
pair and mode, with field subscriptions so inactive-slot changes do not notify
components that read only `current()`.

Applications own persistence and handling of invalid saved settings.

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

`Theme::new(light, dark)` holds independent selections, including accents.
`variant(mode)` reads a selection and `set(mode, variant)` replaces just that slot.
Support is advisory: these methods and provider selection do not reject a
variant because its support differs from the target mode.

`ThemePicker` includes a Light/Dark switch and filters palette options for that mode.
If an application explicitly selects an out-of-mode palette, the picker includes
it as the current selection rather than displaying an unrelated value.

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

## Styling and current scope

The provider renders a `div.fw-theme` with all 32 semantic colour variables,
background/text styles and `color-scheme`. Variables use the `--fw-` prefix:
`--fw-background`, `--fw-alt-surface`, `--fw-primary-hover`, `--fw-on-primary`,
`--fw-border`, and so on. `theme_css` exposes the same declarations for custom
wrappers. Colours retain alpha through Ferriswatch's CSS colour formatter.

The initial `ThemePicker` uses labelled HTML selects. Applications can replace
these with Dioxus Components controls using `use_theme()` and
`theme.config().palettes()`. A stylesheet adapter for Dioxus Components is not
included in this pass.

CSS variables inherit through the rendered DOM subtree. Dialogs or portals
mounted outside the provider wrapper need a wrapper using `theme_css` and the
same active theme. The provider does not change document-wide styles or layout.

## Run the example

From `examples/dioxus_theme`, run `dx serve --web`. The standalone example
offers all built-in palettes filtered by mode and presents a responsive theme workbench. It includes
an editable project board, task progress, feedback states, a reset button, and a
live reference for all 32 semantic colours. Project state survives theme and view
changes; light and dark retain their own palettes and accents. Reloading resets
the session. See `examples/dioxus_theme/README.md` for a walkthrough.
Its renderer dependencies stay separate from the library's feature set.

With the example running on port 8080, run its browser regression checks with
`uv run --with playwright python test_browser.py` from the example directory.
Install Chromium first with `uv run --with playwright playwright install chromium`.
Set `FERRISWATCH_EXAMPLE_URL` for another port or
`PLAYWRIGHT_CHROMIUM_EXECUTABLE` to use an existing Chromium installation.
