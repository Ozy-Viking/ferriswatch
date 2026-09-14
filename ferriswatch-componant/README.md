# ferriswatch-componant

Dioxus widgets for Ferriswatch that depend on `dioxus-primitives` and
`dioxus-attributes`.

`ThemeProvider`, `ThemeConfig`, and `use_theme` stay in `ferriswatch` with the
`dioxus` feature. This crate adds `ThemePicker`, `ThemeCombobox`, and a
Ferriswatch-styled `Combobox`.

This crate is published only to the OneDev registry. `ferriswatch` itself is
also on crates.io.

```toml
# .cargo/config.toml
[registries.onedev]
index = "sparse+https://onedev.hankin.io/rust/~cargo/"
credential-provider = "cargo:token"
```

```toml
[dependencies]
ferriswatch = { version = "0.3.0", features = ["dioxus"] }
ferriswatch-componant = { version = "0.3.0", registry = "onedev" }
```

Mount `ThemePicker` or `ThemeCombobox` inside `ThemeProvider`, and build
component assets with `dx`.
