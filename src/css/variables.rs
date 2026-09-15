use crate::theme_variant::ThemeVariant;
use std::fmt::Write;

/// Exports all semantic colours as prefixed CSS variables, preserving alpha.

pub fn theme_css(theme: &ThemeVariant) -> String {

    let c = theme.colors();

    let mut out = String::new();

    for (prefix, group) in [("", c.surfaces), ("alt-", c.surfaces_alt)] {

        for (name, value) in [
            ("background", group.background),
            ("surface", group.base),
            ("raised", group.raised),
            ("overlay", group.overlay),
            ("hover", group.hover),
        ] {

            write!(out, "--fs-{prefix}{name}:{value};").unwrap();
        }
    }

    for (name, value) in [
        ("text", c.text.normal),
        ("muted", c.text.muted),
        ("subtle", c.text.subtle),
        ("on-primary", c.primary.normal.foreground),
        ("on-secondary", c.secondary.normal.foreground),
        ("primary", c.primary.normal.background),
        ("primary-hover", c.primary.hover.background),
        ("primary-pressed", c.primary.pressed.background),
        ("primary-muted", c.primary.muted.background),
        ("secondary", c.secondary.normal.background),
        ("secondary-hover", c.secondary.hover.background),
        ("secondary-pressed", c.secondary.pressed.background),
        ("secondary-muted", c.secondary.muted.background),
        ("border", c.border),
        ("border-muted", c.border_muted),
        ("focus", c.focus),
        ("success", c.status.success),
        ("warning", c.status.warning),
        ("error", c.status.error),
        ("critical", c.status.critical),
        ("info", c.status.info),
        ("debug", c.status.debug),
        ("trace", c.status.trace),
    ] {

        write!(out, "--fs-{name}:{value};").unwrap();
    }

    out
}
