use dioxus::prelude::*;
use ferriswatch::{
    dioxus::{ThemeConfig, ThemePicker, ThemeProvider, ThemeSelection, use_theme},
    palette::catppuccin::{Latte, Mocha, latte::Blue, mocha::Mauve},
    theme::{Appearance, Theme},
    theme_variant::ThemePalette,
};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let config = use_hook(|| {
        ThemeConfig::with_default(
            Theme::new(Latte::variant::<Blue>(), Mocha::variant::<Mauve>()),
            Appearance::Dark,
        )
        .available(ThemeSelection::all())
        .build()
        .expect("valid theme configuration")
    });
    rsx! {
        style { {include_str!("style.css")} }
        script { src: asset!("/assets/scrollbar.js") }
        ThemeProvider { config, Workbench {} }
    }
}

#[component]
fn Workbench() -> Element {
    let mut theme = use_theme();
    let active = theme.current();
    let mut tokens = use_signal(|| false);
    let scheme = match active.metadata().appearance {
        ferriswatch::theme_variant::Appearance::Dark => "dark",
        ferriswatch::theme_variant::Appearance::Light => "light",
    };
    rsx! {
        style { "html {{ color-scheme: {scheme}; }}" }
        div { class: "workbench", id: "workbench-content",
            overlay-scrollbar {}
            header { class: "masthead",
                a { class: "wordmark", href: "#", span { class: "mark", "f" } "ferriswatch" }
                span { class: "edition", "DIOXUS / THEME WORKBENCH" }
                span { class: "live", span {} "Live preview" }
            }
            main {
                div { class: "intro",
                    div {
                        h1 { "Theme workbench" }
                        p { class: "lede", "Choose a palette and accent to preview components and CSS variables." }
                    }
                    div { class: "palette-art", aria_hidden: "true",
                        for role in ["primary", "secondary", "info", "success", "warning"] {
                            span { style: "background-color:var(--fw-{role})" }
                        }
                    }
                }
                div { class: "layout",
                    aside { class: "settings",
                        h2 { "Appearance" }
                        ThemePicker {}
                        div { class: "selection-note",
                            span { class: "dot" }
                            div { strong { "{active.name()}" } small { "Accent: {active.accent_id().unwrap_or(\"palette default\")}" } }
                        }
                        button { class: "secondary reset", onclick: move |_| theme.reset(), "Reset theme" }
                        div { class: "aside-footer", span { "{theme.config().palettes_for(theme.mode()).count()} palettes" } span { "32 semantic colors" } }
                    }
                    div { class: "stage",
                        div { class: "stage-bar",
                            div { class: "view-switch", role: "group", aria_label: "Preview view",
                                button { aria_pressed: !tokens(), onclick: move |_| tokens.set(false), "Interface" }
                                button { aria_pressed: tokens(), onclick: move |_| tokens.set(true), "Color reference" }
                            }
                        }
                        div { hidden: tokens(), ProjectPreview {} }
                        if tokens() { ColorReference {} }
                    }
                }
                footer { class: "page-footer", span { "Built with Ferriswatch + Dioxus" } }
            }
        }
    }
}

#[component]
fn ProjectPreview() -> Element {
    let mut project_name = use_signal(|| "Website refresh".to_owned());
    let mut name = use_signal(|| "Website refresh".to_owned());
    let mut draft = use_signal(String::new);
    let mut tasks = use_signal(|| {
        vec![
            ("Map the content".to_owned(), true),
            ("Build the component library".to_owned(), false),
            ("Review keyboard navigation".to_owned(), false),
        ]
    });
    let mut saved = use_signal(|| false);
    let done = tasks.read().iter().filter(|(_, done)| *done).count();
    let total = tasks.read().len();
    rsx! {
        section { class: "project",
            div { class: "project-top", span { class: "eyebrow", "STUDIO / PROJECT 024" } span { class: "badge", "In progress" } }
            h2 { "{project_name}" }
            div { class: "progress-heading", span { "Project progress" } strong { "{done} of {total} complete" } }
            progress { value: done as f64, max: total as f64, aria_label: "Project progress" }
            div { class: "tasks",
                for (index, (title, complete)) in tasks.read().iter().enumerate() {
                    label { class: if *complete { "task complete" } else { "task" },
                        input { r#type: "checkbox", checked: *complete, onchange: move |event| tasks.write()[index].1 = event.checked() }
                        span { "{title}" }
                        span { class: "task-state", if *complete { "Done" } else { "To do" } }
                    }
                }
            }
            form { class: "add-task", onsubmit: move |event| {
                event.prevent_default();
                let title = draft().trim().to_owned();
                if !title.is_empty() { tasks.write().push((title, false)); draft.set(String::new()); }
            },
                input { aria_label: "New task", placeholder: "New task", value: draft(), oninput: move |event| draft.set(event.value()), maxlength: 100 }
                button { class: "secondary", r#type: "submit", disabled: draft().trim().is_empty(), "Add task" }
            }
            div { class: "project-settings",
                label { r#for: "project-name", "Project name" }
                div { class: "save-row",
                    input { id: "project-name",  value: name(), maxlength: 80, oninput: move |event| { name.set(event.value()); saved.set(false); } }
                    button { class: "primary", disabled: name().trim().is_empty(), onclick: move |_| { project_name.set(name().trim().to_owned()); saved.set(true); }, "Save changes" }
                }
                p { class: "save-status", role: "status", if saved() { "Saved for this session." } }
            }
            div { class: "feedback-grid",
                div { class: "feedback success", strong { "Ready to review" } p { "Your latest changes are in place." } }
                div { class: "feedback warning", strong { "Review needed" } p { "Test your colors in both light and dark." } }
            }
            details { class: "error-example", open: true, summary { "Preview an error message" } p { "We couldn't publish this project. This is a sample error state; your edits are still here." } }
        }
    }
}

#[component]
fn ColorReference() -> Element {
    let mut copy_status = use_signal(String::new);
    let mut copying = use_signal(|| false);
    rsx! {
        section { class: "reference",
            div { class: "reference-heading",
                h2 { "Color reference" }
                p { class: "copy-status", role: "status", "{copy_status}" }
            }
            p { class: "muted", "Click a swatch to copy its CSS variable name." }
            for (group, roles) in [
                ("Surfaces", vec!["background", "surface", "raised", "overlay", "hover", "alt-background", "alt-surface", "alt-raised", "alt-overlay", "alt-hover"]),
                ("Text", vec!["text", "muted", "subtle", "on-primary", "on-secondary"]),
                ("Actions", vec!["primary", "primary-hover", "primary-pressed", "primary-muted", "secondary", "secondary-hover", "secondary-pressed", "secondary-muted"]),
                ("Borders & focus", vec!["border", "border-muted", "focus"]),
                ("Status", vec!["success", "warning", "error", "critical", "info", "trace"]),
            ] {
                h3 { "{group}" }
                div { class: "swatches",
                    for role in roles {
                        button {
                            class: "swatch", r#type: "button",
                            aria_label: "Copy --fw-{role}", disabled: copying(),
                            onclick: move |_| async move {
                                copying.set(true);
                                let copied = if let Some(window) = web_sys::window().filter(|window| window.is_secure_context()) {
                                    wasm_bindgen_futures::JsFuture::from(
                                        window.navigator().clipboard().write_text(&format!("--fw-{role}"))
                                    ).await.is_ok()
                                } else {
                                    false
                                };
                                copy_status.set(if copied {
                                    format!("Copied --fw-{role}")
                                } else {
                                    format!("Couldn't copy. Select and copy --fw-{role} manually.")
                                });
                                copying.set(false);
                            },
                            span { style: "background-color:var(--fw-{role})" }
                            code { "--fw-{role}" }
                        }
                    }
                }
            }
        }
    }
}
