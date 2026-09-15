use dioxus::prelude::*;
use ferriswatch::dioxus::{Memory, use_theme};

use crate::ThemeCombobox;

#[css_module("/src/theme_combobox.css")]

struct Layout;

#[css_module("/src/combobox/style.css")]

struct Combo;

/// Basic accessible selectors for available palettes and their supported accents.
///
/// Supplies the joined layout and combobox chrome that [`ThemeCombobox`] leaves
/// unstyled.
#[component]

pub fn ThemePicker() -> Element {

    let mut state = use_theme::<Memory>();

    rsx! {
        div { class: "fs-theme-picker",
            div { class: "fs-mode-control",
                span { "Light" }
                button {
                    class: "fs-mode-toggle", r#type: "button", role: "switch",
                    aria_label: "Dark mode", aria_checked: state.is_dark(),
                    onclick: move |_| state.toggle_mode(),
                    span { class: "fs-mode-thumb", aria_hidden: "true" }
                }
                span { "Dark" }
            }
            ThemeCombobox {
                class: Layout::fs_theme_combobox,
                theme_class: Layout::fs_theme_half.to_string(),
                accent_class: Layout::fs_accent_half.to_string(),
                combobox_class: Combo::dx_combobox.to_string(),
                input_class: Combo::dx_combobox_input.to_string(),
                list_class: Combo::dx_combobox_list.to_string(),
                option_class: Combo::dx_combobox_option.to_string(),
                empty_class: Combo::dx_combobox_empty.to_string(),
            }
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
