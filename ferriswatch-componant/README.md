# ferriswatch-componant

Dioxus widgets for Ferriswatch that depend on `dioxus-primitives` and
`dioxus-attributes`.

`ThemeProvider`, `ThemeConfig`, and `use_theme` stay in `ferriswatch` with the
`dioxus` feature. This crate adds `ThemePicker`, `ThemeCombobox`, and a
Ferriswatch-styled `Combobox`.

```toml
ferriswatch = { version = "0.3.0", features = ["dioxus"] }
ferriswatch-componant = "0.1.0"
```

Mount `ThemePicker` or `ThemeCombobox` inside `ThemeProvider`, and build
component assets with `dx`.
