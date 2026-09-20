# ferriswatch-components

Dioxus widgets for Ferriswatch that depend on `dioxus-primitives` and
`dioxus-attributes`.

`ThemeProvider`, `ThemeConfig`, and `use_theme` stay in `ferriswatch` with the
`dioxus` feature. This crate adds `ThemePicker`, `ThemeCombobox`, and a
Ferriswatch-styled `Combobox`.

See [OneDev setup](../../docs/Onedev.md#cargo-registry) for registry configuration
and dependency declarations.

Mount `ThemePicker` or `ThemeCombobox` inside `ThemeProvider`, and build
component assets with `dx`. `ThemeCombobox` is unstyled: pass classes for the
root and each part, and set `matching_mode_only` to `false` to list every
configured palette instead of only those that match the active appearance.
Call `use_theme::<LocalStorage>()` in the host to persist the selection.

`ThemeSelect` and `AccentSelect` can also be mounted separately inside the
provider. Each accepts `class`, `combobox_class`, `input_class`, `list_class`,
`option_class`, and `empty_class`. `ThemeSelect` additionally accepts
`matching_mode_only`; `AccentSelect` follows the selected theme's accents.
`ThemeCombobox` joins these two components and displays selection errors.

See the [components guide](../../docs/Components.md) for setup, examples, and styling properties.
