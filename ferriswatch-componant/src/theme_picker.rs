use dioxus::prelude::*;
use ferriswatch::{dioxus::use_theme, theme::Appearance};

use crate::ThemeCombobox;

/// Basic accessible selectors for available palettes and their supported accents.
#[component]
pub fn ThemePicker() -> Element {
    let mut state = use_theme();
    let mode = state.mode();
    rsx! {
        div { class: "fs-theme-picker",
            div { class: "fs-mode-control",
                span { "Light" }
                button {
                    class: "fs-mode-toggle", r#type: "button", role: "switch",
                    aria_label: "Dark mode", aria_checked: mode == Appearance::Dark,
                    onclick: move |_| {
                        state.set_mode(if state.mode() == Appearance::Dark { Appearance::Light } else { Appearance::Dark });
                    },
                    span { class: "fs-mode-thumb", aria_hidden: "true" }
                }
                span { "Dark" }
            }
            ThemeCombobox {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ferriswatch::{
        dioxus::{ThemeConfig, ThemeProvider},
        palette::catppuccin::{Mocha, mocha::Mauve},
        theme::{Appearance, Theme},
        theme_variant::ThemePalette,
    };

    fn app() -> Element {
        let config = ThemeConfig::with_default(
            Theme::new(Mocha::variant::<Mauve>(), Mocha::variant::<Mauve>()),
            Appearance::Dark,
        )
        .build()
        .unwrap();
        rsx! { ThemeProvider { config, ThemePicker {} } }
    }

    #[test]
    fn picker_mounts_inside_provider() {
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
    }
}
