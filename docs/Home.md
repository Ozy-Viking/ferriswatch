# Ferriswatch

Ferriswatch provides colour palettes, resolved application themes, and CSS for
styling applications. Choose a palette and accent with typed Rust factories or
look them up at runtime using stable IDs. Both produce a
[`ThemeVariant`](crate::theme_variant::ThemeVariant) containing the colours an
application needs.

The colour, palette, catalogue, and CSS APIs work without a UI framework.
Optional Dioxus features add shared theme state, persistence, and selection
widgets. Each section below links to its dedicated guide and API reference.

## Colours

[`Color`](crate::color::Color) stores linear-sRGB colour with alpha. Create colours
from hex values or supported colour spaces, then convert them for calculations,
CSS output, or rendering. Conversions distinguish between preserving extended
colour values and clamping them to the destination's supported range.

The [colour guide](crate::color) covers constructors, colour-space conversions,
alpha, channel bounds, and wrapping. Use these APIs independently of the palette
system when an application needs colour conversion or manipulation.

```rust
use ferriswatch::color::{Color, Rgb};

let colour = Color::hex(0x89b4fa);
let rgb: Rgb = colour.into();
assert_eq!(rgb, Rgb::new(137, 180, 250));
```

## Palettes

The [palette guide](crate::palette) covers built-in families, typed accents, and
custom themes. Each palette supplies raw colour constants and implements
[`ThemePalette`](crate::theme_variant::ThemePalette) to create a resolved theme.
Choose a named accent or [`NoAccent`](crate::palette::NoAccent) for the palette's
default colours.

A resolved theme contains
[`ThemeVariantColors`](crate::theme_variant::ThemeVariantColors): surfaces, text,
actions, statuses, syntax colours, and chromatic hues. These shared roles let an
application change palettes without changing how it assigns colours to controls.
The guide also includes upstream source attribution and mapping validation,
including limitations that matter when choosing foreground/background pairs.

```rust
use ferriswatch::{
    palette::catppuccin::mocha::{Mocha, Mauve},
    theme_variant::ThemePalette,
};

let theme = Mocha::variant::<Mauve>();
assert_eq!(theme.id(), "catppuccin/mocha");
assert_eq!(theme.accent_id(), Some("mauve"));
```

## Catalogue

The [catalogue](crate::catalogue) lists the available palettes, stable theme IDs,
accents, and defaults. Use it when a selection comes from configuration, saved
preferences, or a menu rather than a concrete Rust palette type.

[`resolve`](crate::catalogue::resolve) turns a theme ID and optional accent ID
into the same [`ThemeVariant`](crate::theme_variant::ThemeVariant) returned by a
typed factory. [`get`](crate::catalogue::get) returns the registration for a theme,
including its metadata and available accents. Persist stable IDs; display names
are labels for people.

```rust
use ferriswatch::catalogue;

let theme = catalogue::resolve("catppuccin/mocha", Some("mauve"))?;
assert_eq!(theme.id(), "catppuccin/mocha");

// Omit the accent to use the palette default.
let default = catalogue::resolve("catppuccin/mocha", None)?;
assert!(default.selected_accent().is_none());
# Ok::<(), catalogue::ResolveError>(())
```

## Theme variants

[`ThemeVariant`](crate::theme_variant::ThemeVariant) is one resolved palette and
accent choice, such as Catppuccin Mocha with Mauve. It carries the theme's
identity, appearance, selected accent, and concrete semantic colours. This is
the value a renderer or CSS exporter consumes.

The [`theme_variant`](crate::theme_variant) API describes those colour groups.
[`ThemeVariantColors`](crate::theme_variant::ThemeVariantColors) contains the full
set, while accessors such as
[`primary`](crate::theme_variant::ThemeVariant::primary) expose individual groups.
An action's normal, hover, pressed, muted, and disabled states each provide a
[`ColorPair`](crate::theme_variant::ColorPair) with foreground and background.

```rust
use ferriswatch::{catalogue, theme_variant::Appearance};

let variant = catalogue::resolve("catppuccin/mocha", Some("mauve"))?;
let button = variant.primary().normal;
let canvas = variant.colors().surfaces.background;

assert_eq!(variant.metadata().appearance, Appearance::Dark);
assert_eq!(variant.accent_id(), Some("mauve"));
assert_ne!(button.foreground, button.background);
# Ok::<(), catalogue::ResolveError>(())
```

## Themes

[`Theme`](crate::theme::Theme) stores two resolved variants: one for light mode
and one for dark mode. Each choice retains its own palette and accent. The
[`theme`](crate::theme) API lets you read a slot with
[`variant`](crate::theme::Theme::variant) or replace it with
[`set`](crate::theme::Theme::set) without changing the other slot.

A `Theme` does not track which mode is active. Your application chooses which
slot to use; the Dioxus provider manages that active mode when using the
integration. The slots also do not enforce palette appearance or support, so
choose defaults appropriate for your application.

```rust
use ferriswatch::{catalogue, theme::{Appearance, Theme}};

let light = catalogue::resolve("catppuccin/latte", Some("blue"))?;
let dark = catalogue::resolve("catppuccin/mocha", Some("mauve"))?;
let mut theme = Theme::new(light, dark);

// Change the dark choice while preserving the light choice.
theme.set(Appearance::Dark, catalogue::resolve("kanagawa/wave", None)?);
assert_eq!(theme.variant(Appearance::Dark).id(), "kanagawa/wave");
assert_eq!(theme.variant(Appearance::Light).accent_id(), Some("blue"));
# Ok::<(), catalogue::ResolveError>(())
```

[`ThemeSelection`](crate::theme::ThemeSelection) defines the available palette
choices. [`ThemeConfig`](crate::theme::config::ThemeConfig) combines those choices
with a default `Theme` and initial mode for the provider.

## CSS

The [CSS guide](crate::css) explains how to publish a resolved theme as custom
properties with [`theme_css`](crate::css::theme_css). Applications can use these
variables in their own styles or load the optional semantic classes supplied by
[`DEFAULT_CSS`](crate::css::DEFAULT_CSS).

The classes cover colours for surfaces, text, actions, borders, and other theme
roles. Applications own layout, spacing, and typography. Importing Ferriswatch
does not inject a stylesheet. The guide covers loading, cascade behaviour,
stylesheet regeneration, and the checks that keep generated CSS current.

```rust
use ferriswatch::{catalogue, css::{theme_css, DEFAULT_CSS}};

let theme = catalogue::resolve("catppuccin/mocha", None)?;
let variables = theme_css(&theme);
let stylesheet = format!(":root {{ {variables} }}\n{DEFAULT_CSS}");
assert!(stylesheet.contains("--fs-primary:"));
# Ok::<(), catalogue::ResolveError>(())
```

Load the resulting stylesheet in your application, then use its classes:

```html
<article class="fs-card">
  <p class="fs-text-muted">Ready to save.</p>
  <button class="fs-primary">Save</button>
</article>
```

## Dioxus integration

Enable `dioxus` for the provider and hooks described in the
[Dioxus integration guide][dioxus-guide]. The provider shares theme state with
its descendants and publishes CSS variables for the active selection. The
consuming application chooses its Dioxus renderer.

[`Theme`](crate::theme::Theme) holds independent light and dark selections,
including their accents. [`ThemeConfig`](crate::theme::config::ThemeConfig)
defines the initial mode, defaults, and available choices. The guide explains
configuration, state updates, storage-backed persistence, custom palettes, and
scoping CSS variables to a provider subtree or the document root.

```rust,no_run
# #[cfg(feature = "dioxus")]
# mod example {
use dioxus::prelude::*;
use ferriswatch::{
    dioxus::{Memory, ThemeProvider, use_theme},
    theme::config::ThemeConfig,
};

#[component]
fn App(config: ThemeConfig) -> Element {
    rsx! { ThemeProvider { config, ModeButton {} } }
}

#[component]
fn ModeButton() -> Element {
    let mut theme = use_theme::<Memory>();
    rsx! {
        button {
            onclick: move |_| theme.toggle_mode(),
            "Toggle light/dark"
        }
    }
}
# }
```

## Dioxus components

Enable `dioxus-components` for the [components guide][components-guide] as well
as the provider and hooks. Mount the widgets inside a theme provider so they use
the same configured choices and shared state as the rest of the application.

`ThemePicker` combines a light/dark switch, a searchable theme dropdown, and an
accent dropdown with Ferriswatch styling. `ThemeCombobox` joins the theme and
accent dropdowns for applications with their own mode control. `ThemeSelect`
and `AccentSelect` can be mounted separately. The component reference documents
the styling properties available when composing a custom interface.

```rust,no_run
# #[cfg(feature = "dioxus-components")]
# mod example {
use dioxus::prelude::*;
use ferriswatch::{
    components::ThemePicker,
    dioxus::ThemeProvider,
    theme::config::ThemeConfig,
};

#[component]
fn Settings(config: ThemeConfig) -> Element {
    rsx! {
        ThemeProvider { config,
            ThemePicker {}
        }
    }
}
# }
```
