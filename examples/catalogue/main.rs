//! Generate catalogue documentation, a standalone visual inspector, or a pairing report.

use ferriswatch::{
    catalogue::{self, PALETTES},
    color::Color,
    theme_variant::ThemeVariant,
};
use std::fmt::Write;

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn css(theme: &ThemeVariant) -> String {
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
            write!(out, "--{prefix}{name}:{value};").unwrap();
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
        ("trace", c.status.trace),
    ] {
        write!(out, "--{name}:{value};").unwrap();
    }

    out
}

fn docs() {
    println!(
        "# Built-in catalogue\n\nGenerated with `cargo run --example catalogue -- docs`. The registry is the source\nof this table. Omit the accent ID for `NoAccent`; the listed default colour is\nstill used without recording an explicit accent selection.\n"
    );

    println!(
        "| Theme ID | Display name | Appearance | Default accent ID | Supported accent IDs |\n| --- | --- | --- | --- | --- |"
    );

    for p in PALETTES {
        println!(
            "| `{}` | {} | {:?} | `{}` | {} |",
            p.metadata.id,
            p.metadata.name,
            p.metadata.appearance,
            p.default_accent,
            p.accents
                .iter()
                .map(|a| format!("`{}`", a.id))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    println!("\n## Pinned sources\n");

    for p in PALETTES {
        println!(
            "- `{}`: {}",
            p.metadata.id,
            p.sources
                .iter()
                .map(|s| format!("[{}]({})", s.path, s.permalink()))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}

fn html() {
    let families = PALETTES
        .iter()
        .map(|p| p.metadata.family_id.as_ref())
        .collect::<std::collections::HashSet<_>>()
        .len();

    print!(
        "{}",
        include_str!("palette_inspector.html")
            .replace("{{palette_count}}", &PALETTES.len().to_string())
            .replace("{{family_count}}", &families.to_string())
    );

    for (i, p) in PALETTES.iter().enumerate() {
        let theme = p.resolve(None).unwrap();

        println!(
            "<article class=theme data-id='{}' style='{}'><header><div><span class=eyebrow>{:02} / {:?}</span><h2>{}</h2><code>{}</code></div><label>Accent<select aria-label='Accent for {}'><option value='' data-css='{}'>Default</option>",
            escape(theme.id()),
            escape(&css(&theme)),
            i + 1,
            p.metadata.appearance,
            escape(theme.name()),
            escape(theme.id()),
            escape(theme.name()),
            escape(&css(&theme))
        );

        for accent in p.accents {
            let selected = p.resolve(Some(accent.id)).unwrap();

            println!(
                "<option value='{}' data-css='{}'>{}</option>",
                accent.id,
                escape(&css(&selected)),
                escape(accent.name)
            );
        }

        println!("</select></label></header><div class=groups>");

        for (class, label) in [
            ("normal", "Main surfaces"),
            ("alternate", "Alternate surfaces"),
        ] {
            println!(
                "<section class='{class}'><h3>{label}</h3><div class=panel><strong>Workspace panel</strong><p>Normal text for reading and navigation.</p><p class=muted>Muted text for supporting details.</p><p class=subtle>Subtle text for quiet labels.</p><div class=raised>Raised content <span class=muted>Nested detail</span></div><div class=surface-hover>Surface hover</div><div class=popup><strong>Popup overlay</strong><p>Floating content on this surface group.</p></div></div></section>"
            );
        }

        println!("</div><div class=actions>");

        for kind in ["primary", "secondary"] {
            println!(
                "<div><span class=eyebrow>{kind} action</span><div class=states><button class='{kind}'>Normal</button><button class='{kind} hover'>Hover</button><button class='{kind} pressed'>Pressed</button><button class='{kind} focus'>Focus</button><button class='{kind} disabled' disabled>Disabled</button></div></div>"
            );
        }

        println!("</div><div class=statuses>");

        for status in ["success", "warning", "error", "critical", "info", "trace"] {
            println!("<span style='color:var(--{status})'>● {status}</span>");
        }

        println!(
            "</div><footer><span class=transparent>Transparent fill over the canvas</span><span class=muted>Labels preserve meaning when hues repeat.</span></footer></article>"
        );
    }

    println!("{}", include_str!("palette_inspector.js.html"));
}

// CSS compositing occurs in encoded sRGB here, followed by linear-light luminance.
fn encoded(v: f32) -> f32 {
    if v <= 0.0031308 {
        12.92 * v
    } else {
        1.055 * v.powf(1.0 / 2.4) - 0.055
    }
}

fn linear(v: f32) -> f32 {
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

fn composite(fg: Color, bg: Color) -> Color {
    let channel = |f, b| linear(encoded(f) * fg.a() + encoded(b) * (1.0 - fg.a()));

    Color::new(
        channel(fg.r(), bg.r()),
        channel(fg.g(), bg.g()),
        channel(fg.b(), bg.b()),
        1.0,
    )
    .unwrap()
}

fn luminance(c: Color) -> f32 {
    0.2126 * c.r() + 0.7152 * c.g() + 0.0722 * c.b()
}

fn ratio(fg: Color, bg: Color, canvas: Color) -> f32 {
    let bg = composite(bg, canvas);

    let fg = composite(fg, bg);

    let a = luminance(fg);

    let b = luminance(bg);

    (a.max(b) + 0.05) / (a.min(b) + 0.05)
}

fn contrast() {
    println!("theme_id\taccent_id\tpairing\tratio\tbelow_4_5");

    for p in PALETTES {
        for accent in std::iter::once(None).chain(p.accents.iter().map(|a| Some(a.id))) {
            let t = p.resolve(accent).unwrap();

            let c = t.colors();

            let mut pairs = Vec::new();

            for (prefix, group) in [("surface", c.surfaces), ("surface_alt", c.surfaces_alt)] {
                for (name, bg) in [
                    ("background", group.background),
                    ("surface", group.base),
                    ("raised", group.raised),
                    ("overlay", group.overlay),
                    ("hover", group.hover),
                ] {
                    for (tier, fg) in [
                        ("normal", c.text.normal),
                        ("muted", c.text.muted),
                        ("subtle", c.text.subtle),
                    ] {
                        pairs.push((format!("{tier}/{prefix}.{name}"), fg, bg));
                    }
                }
            }

            for (prefix, action, fg) in [
                ("primary", c.primary, c.primary.normal.foreground),
                ("secondary", c.secondary, c.secondary.normal.foreground),
            ] {
                for (name, bg) in [
                    ("normal", action.normal.background),
                    ("hover", action.hover.background),
                    ("pressed", action.pressed.background),
                ] {
                    pairs.push((format!("on_{prefix}/{prefix}.{name}"), fg, bg));
                }
            }

            for (status, fg) in [
                ("success", c.status.success),
                ("warning", c.status.warning),
                ("error", c.status.error),
                ("critical", c.status.critical),
                ("info", c.status.info),
                ("trace", c.status.trace),
            ] {
                pairs.push((format!("{status}/background"), fg, c.surfaces.background));
            }

            for (name, fg, bg) in pairs {
                let r = ratio(fg, bg, c.surfaces.background);

                println!(
                    "{}\t{}\t{name}\t{r:.3}\t{}",
                    t.id(),
                    accent.unwrap_or(""),
                    r < 4.5
                );
            }
        }
    }
}

fn main() -> Result<(), String> {
    match std::env::args().nth(1).as_deref() {
        Some("docs") => docs(),
        Some("html") => html(),
        Some("contrast") => contrast(),
        _ => return Err("usage: cargo run --example catalogue -- <docs|html|contrast>".into()),
    }

    // Keep lookup reachable in this example as well as iteration.
    catalogue::get("catppuccin/mocha").map_err(|e| e.to_string())?;

    Ok(())
}
