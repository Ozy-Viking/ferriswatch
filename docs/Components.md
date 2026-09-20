# Dioxus components

Ferriswatch provides theme and accent controls through the `dioxus-components`
feature. Import them from `ferriswatch::components`. The application chooses its
Dioxus renderer and builds component assets with `dx`.

```toml
[dependencies]
ferriswatch = { version = "0.4", features = ["dioxus-components"] }
dioxus = { version = "0.7.10", features = ["web"] }
```

## Choosing a control

| Component | Contents | Styling |
| --- | --- | --- |
| [`ThemePicker`](crate::components::ThemePicker) | Light/dark switch and joined theme/accent selectors | Ferriswatch classes and stylesheets |
| [`ThemeCombobox`](crate::components::ThemeCombobox) | Joined theme/accent selectors and selection errors | Application-supplied classes |
| [`ThemeSelect`](crate::components::ThemeSelect) | Searchable theme selector | Application-supplied classes |
| [`AccentSelect`](crate::components::AccentSelect) | Searchable accent selector, including the palette default | Application-supplied classes |

`ThemePicker` contains `ThemeCombobox`, which composes `ThemeSelect` and
`AccentSelect`. Use the smaller controls when the application needs its own
layout or already has a mode switch.

## Provider setup

Mount the theme controls inside a
[`ThemeProvider`](fn@ferriswatch_core::dioxus::ThemeProvider). They read its configured
palette choices and update its shared state. Each provider keeps independent
light and dark selections, including accents.

```rust,no_run
# mod ferriswatch { pub use ferriswatch_core::*; pub use ferriswatch_components::components; }
use dioxus::prelude::*;
use ferriswatch::{
    components::ThemePicker,
    dioxus::ThemeProvider,
    palette::{NoAccent, catppuccin::{Latte, Mocha}},
    theme::{Appearance, Theme, config::ThemeConfig},
    theme_variant::ThemePalette,
};

#[component]
fn App() -> Element {
    let config = ThemeConfig::with_default(
        Theme::new(Latte::variant::<NoAccent>(), Mocha::variant::<NoAccent>()),
        Appearance::Dark,
    )
    .use_config()
    .expect("built-in defaults are available");

    rsx! {
        ThemeProvider { config,
            ThemePicker {}
        }
    }
}
```

[`use_config`](ferriswatch_core::theme::config::ThemeConfigBuilder::use_config)
retains the first build result for this component's lifetime. Call it
unconditionally; later builder values do not update the configuration. Use
`.build()` when constructing configuration outside a Dioxus component.

See the [Dioxus guide](ferriswatch_core::dioxus) for configuration, custom palettes,
CSS scope, and provider lifecycle.

## Complete picker

[`ThemePicker`](crate::components::ThemePicker) supplies the light/dark switch, joined dropdown layout, and
combobox styling. Its styles use Ferriswatch's `--fs-*` colour variables and load
through Dioxus stylesheet assets. Build with `dx` so those assets are available.

The picker lists palettes matching the active mode. Switching mode restores that
mode's saved palette and accent. Application layout around the picker remains
under the host's control.

## Joined selectors

[`ThemeCombobox`](crate::components::ThemeCombobox) supplies both selectors without a mode switch or default
layout/chrome CSS. Pass classes for the outer wrapper, each half, and the
combobox parts. The class names below are examples for your stylesheet to define.

```rust,no_run
# mod ferriswatch { pub use ferriswatch_core::*; pub use ferriswatch_components::components; }
use dioxus::prelude::*;
use ferriswatch::{components::ThemeCombobox, dioxus::ThemeProvider, theme::config::ThemeConfig};

#[component]
fn ThemeSettings(config: ThemeConfig) -> Element {
    rsx! {
        ThemeProvider { config,
            ThemeCombobox {
                class: "theme-settings",
                theme_class: "theme-field",
                accent_class: "accent-field",
                combobox_class: "combobox",
                input_class: "combobox-input",
                list_class: "combobox-list",
                option_class: "combobox-option",
                empty_class: "combobox-empty",
                matching_mode_only: false,
            }
        }
    }
}
```

`matching_mode_only` defaults to `true`. Set it to `false` to list all configured
palettes. A selected out-of-mode palette stays visible even with filtering enabled;
mode support is advisory. Changing palette keeps the current accent when
supported, otherwise it uses the new palette's default.

The joined control displays the provider's last selection error in an alert.

## Separate selectors

[`ThemeSelect`](crate::components::ThemeSelect) and [`AccentSelect`](crate::components::AccentSelect) can live in different parts of the provider's
subtree. Both stay connected to the same active selection. The accent selector
updates its choices when the theme changes. Choosing `Default` removes the
explicit accent selection and uses the palette default.

```rust,no_run
# mod ferriswatch { pub use ferriswatch_core::*; pub use ferriswatch_components::components; }
use dioxus::prelude::*;
use ferriswatch::{
    components::{AccentSelect, ThemeSelect},
    dioxus::ThemeProvider,
    theme::config::ThemeConfig,
};

#[component]
fn SeparateSettings(config: ThemeConfig) -> Element {
    rsx! {
        ThemeProvider { config,
            section { class: "theme-section",
                h2 { "Palette" }
                ThemeSelect { class: "theme-field", input_class: "settings-input" }
            }
            section { class: "accent-section",
                h2 { "Accent" }
                AccentSelect { class: "accent-field", input_class: "settings-input" }
            }
        }
    }
}
```

The standalone selectors do not render selection errors. A custom host can read
[`ThemeState::last_error`](ferriswatch_core::dioxus::ThemeState::last_error) and
present them alongside its other feedback.

## Styling properties

`ThemeCombobox`, `ThemeSelect`, and `AccentSelect` accept these classes:

| Property | Target |
| --- | --- |
| `class` | Outer wrapper, through Dioxus global attributes |
| `combobox_class` | Combobox root |
| `input_class` | Search input |
| `list_class` | Popup list |
| `option_class` | Each option |
| `empty_class` | Empty search result |

`ThemeCombobox` additionally accepts `theme_class` and `accent_class` for its two
selector wrappers. `ThemeCombobox` and `ThemeSelect` accept `matching_mode_only`.
Global attributes such as `id` and `style` apply to the outer wrapper.

The separate [`Combobox`](crate::components::Combobox) and its related parts provide Ferriswatch-styled
building blocks for other dropdowns. They do not select themes automatically;
the host supplies their options and state.

## Persistence

Every [`use_theme`](ferriswatch_core::dioxus::use_theme) call reads the state of
the nearest [`ThemeProvider`](fn@ferriswatch_core::dioxus::ThemeProvider). Its storage
type controls what that particular hook does; it does not configure persistence
for descendants. There is no rule that the highest hook's type wins.

Call `use_theme::<LocalStorage>()` or `use_theme::<SessionStorage>()` once per
provider, in a component that stays mounted. Use
`use_theme::<Memory>()` in the other components. They share the restored state,
and changes made through any of them are saved by the persistent hook.
[`Memory`](ferriswatch_core::dioxus::Memory) skips storage reads and writes; it
does not create separate theme state. The built-in theme controls already use it.

Each persistent hook independently restores on mount and saves changes. Repeating
it adds storage work and can restore a saved value again when another component
mounts. Mixing backends also causes independent restores and saves. Choose one
persistent hook for each provider. See the [Dioxus guide](ferriswatch_core::dioxus)
for custom storage implementations.

### Local storage

Keep the active selection across browser visits with `LocalStorage`. The
`PersistentPicker` component mounts inside the provider, so its persistence hook
and the picker use the same theme state. The hook restores the saved selection
on mount and saves subsequent changes.

```rust,no_run
# mod ferriswatch { pub use ferriswatch_core::*; pub use ferriswatch_components::components; }
use dioxus::prelude::*;
use ferriswatch::{
    components::ThemePicker,
    dioxus::{LocalStorage, Memory, ThemeProvider, use_theme},
    theme::config::ThemeConfig,
};

#[component]
fn App(config: ThemeConfig) -> Element {
    rsx! {
        ThemeProvider { config,
            PersistentPicker {}
        }
    }
}

#[component]
fn PersistentPicker() -> Element {
    let _theme = use_theme::<LocalStorage>();

    rsx! {
        ThemePicker {}
        ResetThemeButton {}
    }
}

#[component]
fn ResetThemeButton() -> Element {
    // The persistent hook saves changes made through this shared state too.
    let mut theme = use_theme::<Memory>();

    rsx! {
        button {
            onclick: move |_| theme.reset(),
            "Reset to defaults"
        }
    }
}
```

Call the persistence hook in a descendant such as `PersistentPicker`, rather
than in the component that creates the provider. Keep that descendant mounted
while changes should be saved. Other controls can continue using the same
provider without adding their own persistence hook.

### Session storage

Use `SessionStorage` for a selection that survives reloads within the browser
tab's session. Mount this component inside `ThemeProvider` in place of
`PersistentPicker` above:

```rust,no_run
# mod ferriswatch { pub use ferriswatch_core::*; pub use ferriswatch_components::components; }
use dioxus::prelude::*;
use ferriswatch::{
    components::ThemePicker,
    dioxus::{SessionStorage, use_theme},
};

#[component]
fn SessionPicker() -> Element {
    let _theme = use_theme::<SessionStorage>();
    rsx! { ThemePicker {} }
}
```

Choose one storage backend for a provider. Both examples store a
[`ThemeSnapshot`](ferriswatch_core::dioxus::ThemeSnapshot) under
`ferriswatch.theme.v1`: the active mode, palette ID, and optional accent ID.
Persistence saves the active selection; it does not save both light and dark
slots. Invalid saved choices leave the configured defaults in place.
