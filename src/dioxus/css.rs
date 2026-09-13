use crate::theme_variant::ThemeVariant;
use std::fmt::Write;

/// Exports all semantic colours as prefixed CSS variables, preserving alpha.
pub fn theme_css(theme: &ThemeVariant) -> String {
    let c = theme.colors();
    let mut out = String::new();
    for (prefix, group) in [("", c.surface), ("alt-", c.surface_alt)] {
        for (name, value) in [
            ("background", group.background),
            ("surface", group.surface),
            ("raised", group.raised),
            ("overlay", group.overlay),
            ("hover", group.hover),
        ] {
            write!(out, "--fw-{prefix}{name}:{value};").unwrap();
        }
    }
    for (name, value) in [
        ("text", c.text.normal),
        ("muted", c.text.muted),
        ("subtle", c.text.subtle),
        ("on-primary", c.text.on_primary),
        ("on-secondary", c.text.on_secondary),
        ("primary", c.primary.normal),
        ("primary-hover", c.primary.hover),
        ("primary-pressed", c.primary.pressed),
        ("primary-muted", c.primary.muted),
        ("secondary", c.secondary.normal),
        ("secondary-hover", c.secondary.hover),
        ("secondary-pressed", c.secondary.pressed),
        ("secondary-muted", c.secondary.muted),
        ("border", c.border),
        ("border-muted", c.border_muted),
        ("focus", c.focus),
        ("success", c.status.success),
        ("warning", c.status.warning),
        ("error", c.status.error),
        ("critical", c.status.critical),
        ("info", c.status.info),
        ("trace", c.status.trace),
    ] {
        write!(out, "--fw-{name}:{value};").unwrap();
    }
    out.push_str(match theme.metadata().appearance {
        crate::theme_variant::Appearance::Dark => "color-scheme:dark;",
        crate::theme_variant::Appearance::Light => "color-scheme:light;",
    });
    // A variable in the background shorthand leaves empty longhand values that
    // Dioxus's style preservation can restore, clearing the background on updates.
    out.push_str("background-color:var(--fw-background);color:var(--fw-text);");
    out
}
