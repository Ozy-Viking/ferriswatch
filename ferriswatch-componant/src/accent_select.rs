use dioxus::prelude::*;
use dioxus_primitives::combobox::{
    Combobox, ComboboxEmpty, ComboboxInput, ComboboxList, ComboboxOption,
};
use ferriswatch::dioxus::{Memory, use_theme};

/// Searchable accent selector connected to the nearest theme provider.
///
/// Lists only accents supported by the selected theme, plus its default.
/// Supply CSS classes directly, or use [`crate::ThemePicker`] for the joined UI.
#[component]
pub fn AccentSelect(
    /// CSS class for the primitives combobox root.
    #[props(default)]
    combobox_class: String,
    /// CSS class for the search input.
    #[props(default)]
    input_class: String,
    /// CSS class for the popup list.
    #[props(default)]
    list_class: String,
    /// CSS class for each option.
    #[props(default)]
    option_class: String,
    /// CSS class for the empty search result.
    #[props(default)]
    empty_class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut state = use_theme::<Memory>();
    let current = state.current();
    let accents = state.available_accents();

    rsx! {
        div {
            title: current
                .accent_name()
                .unwrap_or("Default")
                .to_owned(),
            ..attributes,

            Combobox::<String> {
                class: "{combobox_class}",
                value: Some(state.accent_value().into()),

                on_value_change: move |accent: Option<String>| {
                    if let Some(accent) = accent {
                        let accent = (!accent.is_empty()).then_some(accent.as_str());

                        let _ = state.select_accent(accent);
                    }
                },

                ComboboxInput {
                    class: "{input_class}",
                    placeholder: current
                        .accent_name()
                        .unwrap_or("Default")
                        .to_owned(),
                    aria_label: "Accent",
                }

                ComboboxList {
                    class: "{list_class}",
                    aria_label: "Accents",

                    ComboboxEmpty {
                        class: "{empty_class}",
                        "No accents found"
                    }

                    ComboboxOption::<String> {
                        class: "{option_class}",
                        index: 0usize,
                        value: String::new(),
                        text_value: "Default",
                        "data-value": "",
                        "Default"
                    }

                    for (index, accent) in accents.iter().enumerate() {
                        ComboboxOption::<String> {
                            key: "{accent.id}",
                            class: "{option_class}",
                            index: index + 1,
                            value: accent.id.to_owned(),
                            text_value: accent.name.to_owned(),
                            "data-value": accent.id,
                            "{accent.name}"
                        }
                    }
                }
            }
        }
    }
}
