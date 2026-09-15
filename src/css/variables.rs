use crate::theme_variant::{ActionColors, ColorPair, ThemeVariant};
use std::fmt::Write;

const SYNTAX: &[(
    &str,
    fn(&crate::theme_variant::SyntaxColors) -> crate::color::Color,
)] = &[
    ("attribute", |s| s.attribute),
    ("boolean", |s| s.boolean),
    ("builtin", |s| s.builtin),
    ("builtin-function", |s| s.builtin_function),
    ("builtin-type", |s| s.builtin_type),
    ("comment", |s| s.comment),
    ("constant", |s| s.constant),
    ("control-keyword", |s| s.control_keyword),
    ("deleted", |s| s.deleted),
    ("deprecated", |s| s.deprecated),
    ("documentation", |s| s.documentation),
    ("escape", |s| s.escape),
    ("foreground", |s| s.foreground),
    ("function", |s| s.function),
    ("heading", |s| s.heading),
    ("inserted", |s| s.inserted),
    ("invalid", |s| s.invalid),
    ("keyword", |s| s.keyword),
    ("link", |s| s.link),
    ("macro-name", |s| s.macro_name),
    ("markup-bold", |s| s.markup_bold),
    ("markup-italic", |s| s.markup_italic),
    ("modifier", |s| s.modifier),
    ("namespace", |s| s.namespace),
    ("number", |s| s.number),
    ("operator", |s| s.operator),
    ("parameter", |s| s.parameter),
    ("property", |s| s.property),
    ("punctuation", |s| s.punctuation),
    ("string", |s| s.string),
    ("tag", |s| s.tag),
    ("type-keyword", |s| s.type_keyword),
    ("type-name", |s| s.type_name),
    ("variable", |s| s.variable),
];

fn write_var(out: &mut String, name: &str, value: crate::color::Color) {

    write!(out, "--fs-{name}:{value};").unwrap();
}

fn write_pair(out: &mut String, role: &str, state: &str, pair: ColorPair) {

    if state.is_empty() {

        write_var(out, &format!("on-{role}"), pair.foreground);

        write_var(out, role, pair.background);
    } else {

        write_var(out, &format!("on-{role}-{state}"), pair.foreground);

        write_var(out, &format!("{role}-{state}"), pair.background);
    }
}

fn write_action(out: &mut String, role: &str, action: ActionColors) {

    write_pair(out, role, "", action.normal);

    write_pair(out, role, "hover", action.hover);

    write_pair(out, role, "pressed", action.pressed);

    write_pair(out, role, "muted", action.muted);

    write_pair(out, role, "disabled", action.disabled);
}

/// Exports every colour on [`ThemeVariant`] as a prefixed CSS variable, preserving alpha.

pub fn theme_css(theme: &ThemeVariant) -> String {

    let c = theme.colors();

    let mut out = String::new();

    for (prefix, group) in [("", c.surfaces), ("alt-", c.surfaces_alt)] {

        write_var(&mut out, &format!("{prefix}background"), group.background);

        write_var(&mut out, &format!("{prefix}surface"), group.base);

        write_var(&mut out, &format!("{prefix}raised"), group.raised);

        write_var(&mut out, &format!("{prefix}overlay"), group.overlay);

        write_var(&mut out, &format!("{prefix}hover"), group.hover);
    }

    let text_alt = c.resolved_text_alt();

    write_var(&mut out, "text", c.text.normal);

    write_var(&mut out, "muted", c.text.muted);

    write_var(&mut out, "subtle", c.text.subtle);

    write_var(&mut out, "alt-text", text_alt.normal);

    write_var(&mut out, "alt-muted", text_alt.muted);

    write_var(&mut out, "alt-subtle", text_alt.subtle);

    write_action(&mut out, "primary", c.primary);

    write_action(&mut out, "secondary", c.secondary);

    write_var(&mut out, "border", c.border);

    write_var(&mut out, "border-muted", c.border_muted);

    write_var(&mut out, "focus", c.focus);

    write_var(&mut out, "success", c.status.success);

    write_var(&mut out, "warning", c.status.warning);

    write_var(&mut out, "error", c.status.error);

    write_var(&mut out, "critical", c.status.critical);

    write_var(&mut out, "info", c.status.info);

    write_var(&mut out, "debug", c.status.debug);

    write_var(&mut out, "trace", c.status.trace);

    for (name, value) in [
        ("red", c.chromatic.red),
        ("orange", c.chromatic.orange),
        ("yellow", c.chromatic.yellow),
        ("green", c.chromatic.green),
        ("cyan", c.chromatic.cyan),
        ("blue", c.chromatic.blue),
        ("purple", c.chromatic.purple),
        ("pink", c.chromatic.pink),
    ] {

        write_var(&mut out, &format!("chromatic-{name}"), value);
    }

    for (name, getter) in SYNTAX {

        write_var(&mut out, &format!("syntax-{name}"), getter(&c.syntax));
    }

    out
}
