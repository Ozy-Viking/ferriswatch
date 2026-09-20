use crate::ferriswatch::dioxus::{Memory, use_theme};
use dioxus::prelude::*;

use crate::components::ThemeCombobox;

const THEME_COMBOBOX_CSS: Asset = asset!("/src/theme_combobox.css");
const COMBOBOX_CSS: Asset = asset!("/src/combobox/style.css");

/// Basic accessible selectors for available palettes and their supported accents.
///
/// Supplies the joined layout and combobox chrome that [`crate::components::ThemeCombobox`] leaves
/// unstyled.
#[component]
pub fn ThemePicker() -> Element {
    let mut state = use_theme::<Memory>();

    rsx! {
        document::Stylesheet { href: THEME_COMBOBOX_CSS }
        document::Stylesheet { href: COMBOBOX_CSS }
        div { class: "fs-theme-picker",
            div { class: "fs-mode-control",
                span { "Light" }
                button {
                    class: "fs-mode-toggle",
                    r#type: "button",
                    role: "switch",
                    aria_label: "Dark mode",
                    aria_checked: state.is_dark(),
                    onclick: move |_| state.toggle_mode(),
                    span { class: "fs-mode-thumb", aria_hidden: "true" }
                }
                span { "Dark" }
            }
            ThemeCombobox {
                class: "fs-theme-combobox",
                theme_class: "fs-theme-half",
                accent_class: "fs-accent-half",
                combobox_class: "dx-combobox",
                input_class: "dx-combobox-input",
                list_class: "dx-combobox-list",
                option_class: "dx-combobox-option",
                empty_class: "dx-combobox-empty",
            }
        }
    }
}

#[cfg(test)]

mod tests {

    use super::*;
    use crate::ferriswatch::{
        dioxus::ThemeProvider,
        palette::catppuccin::{Mocha, mocha::Mauve},
        theme::{Appearance, Theme, config::ThemeConfig},
        theme_variant::ThemePalette,
    };

    fn app() -> Element {
        let config = ThemeConfig::with_default(
            Theme::new(Mocha::variant::<Mauve>(), Mocha::variant::<Mauve>()),
            Appearance::Dark,
        )
        .build()
        .unwrap();

        rsx! {
            ThemeProvider { config, ThemePicker {} }

        }
    }

    #[test]

    fn picker_mounts_inside_provider() {
        let mut dom = VirtualDom::new(app);

        dom.rebuild_in_place();
    }
}
