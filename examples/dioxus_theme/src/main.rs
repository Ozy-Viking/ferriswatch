use dioxus::logger::tracing::Level;
use dioxus::prelude::*;
use ferriswatch::{
    ThemePicker,
    css::ThemeScope,
    dioxus::{DEFAULT_STYLESHEET, LocalStorage, ThemeProvider, use_theme},
    palette::{
        NoAccent,
        catppuccin::{Latte, Mocha, latte::Blue, mocha::Mauve},
    },
    theme::{Appearance, Theme, ThemeSelection, config::ThemeConfig},
    theme_variant::ThemePalette,
};

const FAVICON: Asset = asset!(
    "/assets/ferriswatch_icon_3.png",
    AssetOptions::image()
        .with_size(ImageSize::Manual {
            width: 64,
            height: 64,
        })
        .with_png()
);

fn main() {
    dioxus::logger::init(Level::WARN).expect("failed to initialize logger");
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
        .override_dx_components_theme(true)
        .build()
        .expect("valid theme configuration")
    });
    rsx! {
        document::Title { "Ferriswatch Demo" }
        document::Link {
            rel: "icon",
            r#type: "image/png",
            href: FAVICON,
        }
        document::Stylesheet { href: DEFAULT_STYLESHEET }
        style { {include_str!("style.css")} }
        script { src: asset!("/assets/scrollbar.js") }
        ThemeProvider { config, scope: ThemeScope::Root, Workbench {} }
    }
}

#[component]
fn Workbench() -> Element {
    let mut theme = use_theme::<LocalStorage>();
    let active = theme.current();
    let saved = theme.theme();
    let accent_count = theme.available_accents().len();
    let palette_count = theme.listed_palettes(true).len();
    let mut tokens = use_signal(|| false);
    rsx! {
        div { class: "workbench fs-page", id: "workbench-content",
            overlay-scrollbar {}
            main {
                div { class: "intro",
                    div {
                        class: "palette-art",
                        aria_hidden: "true",
                        dangerous_inner_html: include_str!("../assets/ferris_swatch_with_eyes.svg"),
                    }
                    div {
                        h1 { "Ferriswatch" }
                        p { class: "lede",
                            "Ferris-swatch ... rust based swatches ... rust ba ... you get it"
                        }
                    }
                }
                div { class: "layout",
                    aside { class: "settings",
                        h2 { "Appearance" }
                        ThemePicker {}
                        div { class: "selection-note",
                            span { class: "dot" }
                            div {
                                strong { "{active.name()}" }
                                small {
                                    "Accent: {theme.selected_accent_name().unwrap_or_else(|| NoAccent::NAME.to_owned())}"
                                }
                            }
                        }
                        button {
                            class: "secondary fs-panel reset",
                            onclick: move |_| theme.reset(),
                            "Reset theme"
                        }
                        dl { class: "saved-themes",
                            div { class: "saved-dark",
                                dt { "Dark theme" }
                                dd {
                                    "{saved.dark.name()}"
                                    small { {saved.dark.accent_name().unwrap_or("Default")} }
                                }
                            }
                            div { class: "saved-light",
                                dt { "Light theme" }
                                dd {
                                    "{saved.light.name()}"
                                    small { {saved.light.accent_name().unwrap_or("Default")} }
                                }
                            }
                        }
                        div { class: "aside-footer",
                            span { "{palette_count} palettes" }
                            span { "{active.name()} has {accent_count} accent colors" }
                        }
                    }
                    div { class: "stage",
                        div { class: "stage-bar",
                            div {
                                class: "view-switch",
                                role: "group",
                                aria_label: "Preview view",
                                button {
                                    aria_pressed: !tokens(),
                                    onclick: move |_| tokens.set(false),
                                    "Interface"
                                }
                                button {
                                    aria_pressed: tokens(),
                                    onclick: move |_| tokens.set(true),
                                    "Color reference"
                                }
                            }
                        }
                        div { hidden: tokens(), ProjectPreview {} }
                        if tokens() {
                            ColorReference {}
                        }
                    }
                }
                footer { class: "page-footer",
                    span { "Built with Ferriswatch + Dioxus" }
                }
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
    let completed = total > 0 && done == total;
    rsx! {
        section { class: "project fs-card fs-border",
            div { class: "project-top",
                span { class: "eyebrow", "STUDIO / PROJECT 024" }
                span {
                    class: if completed { "badge completed fs-success" } else { "badge" },
                    aria_live: "polite",
                    if completed {
                        "Completed"
                    } else {
                        "In progress"
                    }
                }
            }
            h2 { "{project_name}" }
            div { class: "progress-heading",
                span { "Project progress" }
                strong { "{done} of {total} complete" }
            }
            progress {
                value: done as f64,
                max: total as f64,
                aria_label: "Project progress",
            }
            div { class: "tasks",
                for (index, (title, complete)) in tasks.read().iter().enumerate() {
                    label { class: if *complete { "task complete" } else { "task" },
                        input {
                            r#type: "checkbox",
                            checked: *complete,
                            onchange: move |event| tasks.write()[index].1 = event.checked(),
                        }
                        span { "{title}" }
                        span { class: "task-state",
                            if *complete {
                                "Done"
                            } else {
                                "To do"
                            }
                        }
                    }
                }
            }
            form {
                class: "add-task",
                onsubmit: move |event| {
                    event.prevent_default();
                    let title = draft().trim().to_owned();
                    if !title.is_empty() {
                        tasks.write().push((title, false));
                        draft.set(String::new());
                    }
                },
                input {
                    aria_label: "New task",
                    placeholder: "New task",
                    value: draft(),
                    oninput: move |event| draft.set(event.value()),
                    maxlength: 100,
                }
                button {
                    class: "secondary fs-panel",
                    r#type: "submit",
                    disabled: draft().trim().is_empty(),
                    "Add task"
                }
            }
            div { class: "project-settings",
                label { r#for: "project-name", "Project name" }
                div { class: "save-row",
                    input {
                        id: "project-name",
                        value: name(),
                        maxlength: 80,
                        oninput: move |event| {
                            name.set(event.value());
                            saved.set(false);
                        },
                    }
                    button {
                        class: "primary fs-primary fs-focus",
                        disabled: name().trim().is_empty(),
                        onclick: move |_| {
                            project_name.set(name().trim().to_owned());
                            saved.set(true);
                        },
                        "Save changes"
                    }
                }
                p { class: "save-status fs-text-muted", role: "status",
                    if saved() {
                        "Updated project name"
                    }
                }
            }
            div { class: "feedback-grid",
                div { class: "feedback success fs-success",
                    strong { "Ready to review" }
                    p { class: "fs-text-muted", "Your latest changes are in place." }
                }
                div { class: "feedback warning fs-warning",
                    strong { "Review needed" }
                    p { class: "fs-text-muted", "Test your colors in both light and dark." }
                }
            }
            details { class: "error-example", open: true,
                summary { "Preview an error message" }
                p { class: "fs-error",
                    "We couldn't publish this project. This is a sample error state; your edits are still here."
                }
            }
        }
    }
}

#[component]
fn ColorReference() -> Element {
    let mut copy_status = use_signal(String::new);
    let mut copying = use_signal(|| false);
    rsx! {
        section { class: "reference fs-card fs-border",
            div { class: "reference-heading",
                h2 { "Color reference" }
                p { class: "copy-status fs-text-muted", role: "status", "{copy_status}" }
            }
            p { class: "muted fs-text-muted", "Click a swatch to copy its CSS variable name." }
            for (group, roles) in [
                (
                    "Surfaces",
                    vec![
                        "background",
                        "surface",
                        "raised",
                        "overlay",
                        "hover",
                        "alt-background",
                        "alt-surface",
                        "alt-raised",
                        "alt-overlay",
                        "alt-hover",
                    ],
                ),
                ("Text", vec!["text", "muted", "subtle", "alt-text", "alt-muted", "alt-subtle"]),
                (
                    "Actions",
                    vec![
                        "primary",
                        "on-primary",
                        "primary-hover",
                        "on-primary-hover",
                        "primary-pressed",
                        "on-primary-pressed",
                        "primary-muted",
                        "on-primary-muted",
                        "primary-disabled",
                        "on-primary-disabled",
                        "secondary",
                        "on-secondary",
                        "secondary-hover",
                        "on-secondary-hover",
                        "secondary-pressed",
                        "on-secondary-pressed",
                        "secondary-muted",
                        "on-secondary-muted",
                        "secondary-disabled",
                        "on-secondary-disabled",
                    ],
                ),
                ("Borders & focus", vec!["border", "border-muted", "focus"]),
                (
                    "Status",
                    vec!["success", "warning", "error", "critical", "info", "debug", "trace"],
                ),
                (
                    "Chromatic",
                    vec![
                        "chromatic-red",
                        "chromatic-orange",
                        "chromatic-yellow",
                        "chromatic-green",
                        "chromatic-cyan",
                        "chromatic-blue",
                        "chromatic-purple",
                        "chromatic-pink",
                    ],
                ),
                (
                    "Syntax",
                    vec![
                        "syntax-attribute",
                        "syntax-boolean",
                        "syntax-builtin",
                        "syntax-builtin-function",
                        "syntax-builtin-type",
                        "syntax-comment",
                        "syntax-constant",
                        "syntax-control-keyword",
                        "syntax-deleted",
                        "syntax-deprecated",
                        "syntax-documentation",
                        "syntax-escape",
                        "syntax-foreground",
                        "syntax-function",
                        "syntax-heading",
                        "syntax-inserted",
                        "syntax-invalid",
                        "syntax-keyword",
                        "syntax-link",
                        "syntax-macro-name",
                        "syntax-markup-bold",
                        "syntax-markup-italic",
                        "syntax-modifier",
                        "syntax-namespace",
                        "syntax-number",
                        "syntax-operator",
                        "syntax-parameter",
                        "syntax-property",
                        "syntax-punctuation",
                        "syntax-string",
                        "syntax-tag",
                        "syntax-type-keyword",
                        "syntax-type-name",
                        "syntax-variable",
                    ],
                ),
            ]
            {
                h3 { "{group}" }
                div { class: "swatches",
                    for role in roles {
                        button {
                            class: "swatch",
                            r#type: "button",
                            aria_label: "Copy --fs-{role}",
                            disabled: copying(),
                            onclick: move |_| async move {
                                copying.set(true);
                                let copied = if let Some(window) = web_sys::window()
                                    .filter(|window| window.is_secure_context())
                                {
                                    wasm_bindgen_futures::JsFuture::from(
                                            window.navigator().clipboard().write_text(&format!("--fs-{role}")),
                                        )
                                        .await
                                        .is_ok()
                                } else {
                                    false
                                };
                                copy_status
                                    .set(
                                        if copied {
                                            format!("Copied --fs-{role}")
                                        } else {
                                            format!("Couldn't copy. Select and copy --fs-{role} manually.")
                                        },
                                    );
                                copying.set(false);
                            },
                            span { style: "background-color:var(--fs-{role})" }
                            code { "--fs-{role}" }
                        }
                    }
                }
            }
        }
    }
}
