//! Browser fixture: dx serve --example dx_theme --web --port 8081
use dioxus::prelude::*;
use ferriswatch::{
    dioxus::{ThemeConfig, ThemeProvider, ThemeScope},
    palette::{
        catppuccin::{Latte, Mocha},
        NoAccent,
    },
    theme::{Appearance, Theme},
    theme_variant::ThemePalette,
};

fn main() {
    dioxus::launch(App);
}

fn config(mode: Appearance, adapter: bool) -> ThemeConfig {
    ThemeConfig::with_default(
        Theme::new(Latte::variant::<NoAccent>(), Mocha::variant::<NoAccent>()),
        mode,
    )
    .override_dx_components_theme(adapter)
    .build()
    .unwrap()
}

#[component]
fn App() -> Element {
    let mut adapter = use_signal(|| false);
    let mut scope = use_signal(|| ThemeScope::Root);
    let mut mounted = use_signal(|| true);
    rsx! {
        div { id: "controls",
            button { id: "adapter", onclick: move |_| adapter.toggle(), "Toggle adapter" }
            button {
                id: "scope",
                onclick: move |_| {
                    scope
                        .set(
                            if scope() == ThemeScope::Root {
                                ThemeScope::Scoped
                            } else {
                                ThemeScope::Root
                            },
                        )
                },
                "Toggle scope"
            }
            button { id: "mount", onclick: move |_| mounted.toggle(), "Toggle root provider" }
            span { id: "state",
                "adapter: {adapter()} / "
                if scope() == ThemeScope::Root {
                    "root"
                } else {
                    "scoped"
                }
            }
        }
        div {
            id: "outside-probe",
            class: "dxc-system",
            style: "background-color:var(--primary-color);color:var(--secondary-color)",
        }
        if mounted() {
            // ThemeConfig is captured on mount; the adapter flag is part of this key.
            ThemeProvider {
                key: "adapter-{adapter()}",
                config: config(Appearance::Dark, adapter()),
                scope: scope(),
                div { id: "root-probe", class: "probe",
                    Controls {}
                    ProbeColors { prefix: "root" }
                    ThemeProvider {
                        config: config(Appearance::Light, true),
                        scope: ThemeScope::Scoped,
                        div { id: "nested-probe", class: "probe",
                            ProbeColors { prefix: "nested" }
                        }
                    }
                }
            }
        }
        // Deliberately after provider rules: this is the actual upstream stylesheet.
        style { {include_str!("../fixtures/dx-components-theme.css")} }
    }
}

#[component]
fn Controls() -> Element {
    let mut theme = ferriswatch::dioxus::use_theme::<ferriswatch::dioxus::Memory>();
    rsx! {
        button { id: "mode", onclick: move |_| theme.toggle_mode(), "Toggle mode" }
    }
}

#[component]
fn ProbeColors(prefix: &'static str) -> Element {
    rsx! {
        div {
            id: "{prefix}-primary",
            style: "background-color:var(--primary-color)",
        }
        div { id: "{prefix}-secondary", style: "color:var(--secondary-color)" }
        div {
            id: "{prefix}-focused",
            style: "border:1px solid var(--focused-border-color)",
        }
        div {
            id: "{prefix}-success",
            style: "color:var(--secondary-success-color)",
        }
        div {
            id: "{prefix}-warning",
            style: "color:var(--secondary-warning-color)",
        }
        div { id: "{prefix}-error", style: "color:var(--secondary-error-color)" }
        div { id: "{prefix}-info", style: "color:var(--secondary-info-color)" }
        div { id: "{prefix}-flags", class: "dxc-system", "flags" }
        div {
            id: "{prefix}-dark-flag",
            class: "dxc-system",
            style: "background-color:var(--dxc-dark-on, rgb(1, 2, 3))",
        }
        div {
            id: "{prefix}-light-flag",
            class: "dxc-system",
            style: "background-color:var(--dxc-light-on, rgb(4, 5, 6))",
        }
    }
}
