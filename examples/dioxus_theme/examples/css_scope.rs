//! Browser fixture: dx serve --example css_scope --web --port 8081
use dioxus::prelude::*;
use ferriswatch::{
    dioxus::{DefaultStyles, ThemeConfig, ThemeProvider, ThemeScope, use_theme},
    palette::{
        NoAccent,
        catppuccin::{Latte, Mocha},
    },
    theme::{Appearance, Theme},
    theme_variant::ThemePalette,
};

fn main() {
    dioxus::launch(App);
}

fn config(mode: Appearance) -> ThemeConfig {
    ThemeConfig::with_default(
        Theme::new(Latte::variant::<NoAccent>(), Mocha::variant::<NoAccent>()),
        mode,
    )
    .build()
    .unwrap()
}

#[component]
fn App() -> Element {
    let mut scope = use_signal(|| ThemeScope::Root);
    let mut mounted = use_signal(|| true);
    let mut styles = use_signal(|| false);
    rsx! {
        // These declarations must be revealed again when the root provider leaves.
        style { ":root {{ --fs-background: rgb(1, 2, 3); --fs-text: rgb(4, 5, 6); color-scheme: light; }}" }
        if styles() { DefaultStyles {} DefaultStyles {} }
        button { id: "styles", onclick: move |_| styles.set(true), "Load classes" }
        button { id: "scope", onclick: move |_| scope.set(if scope() == ThemeScope::Root { ThemeScope::Scoped } else { ThemeScope::Root }), "Toggle scope" }
        button { id: "mount", onclick: move |_| mounted.toggle(), "Toggle provider" }
        div { id: "outside", class: "fs-page", "Outside provider" }
        if mounted() {
            ThemeProvider { config: config(Appearance::Dark), scope: scope(),
                Controls {}
                div { id: "inside", class: "fs-page", "Inside provider" }
                ThemeProvider { config: config(Appearance::Light),
                    div { id: "nested", class: "fs-page", "Nested light" }
                }
            }
        }
    }
}

#[component]
fn Controls() -> Element {
    let mut theme = use_theme();
    let mut count = use_signal(|| 0);
    rsx! {
        button { id: "mode", onclick: move |_| {
            theme.set_mode(if theme.mode() == Appearance::Dark { Appearance::Light } else { Appearance::Dark });
        }, "Toggle mode" }
        button { id: "counter", onclick: move |_| count += 1, "{count}" }
    }
}
