use crate::ferriswatch::dioxus::{Memory, use_theme};
use dioxus::prelude::*;
use dioxus_primitives::combobox::{
    Combobox, ComboboxEmpty, ComboboxInput, ComboboxList, ComboboxOption,
};

use crate::progressively_render::INITIAL_OPTIONS;

/// Searchable theme selector connected to the nearest theme provider.
///
/// Lists palettes for the active appearance by default. Selecting a palette
/// preserves the current accent when that palette supports it. Supply CSS
/// classes directly, or use [`crate::ThemePicker`] for the joined default UI.
#[component]
pub fn ThemeSelect(
    /// Restrict the menu to palettes supporting the active appearance.
    #[props(default = true)]
    matching_mode_only: bool,
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
    let mut render_count = use_signal(|| INITIAL_OPTIONS);

    let name = use_memo(move || state.selected_theme().metadata.name.to_string());

    let palettes = use_memo(use_reactive!(|(matching_mode_only,)| {
        state
            .listed_palettes(matching_mode_only)
            .into_iter()
            .map(|palette| (palette.metadata.id.as_ref(), palette.metadata.name.as_ref()))
            .collect::<Vec<_>>()
    }));

    let on_value_change_fn = move |id: Option<String>| {
        if let Some(id) = id {
            let _ = state.select_theme(&id);
        }
    };

    rsx! {
        div { title: name(), ..attributes,

            Combobox::<String> {
                class: "{combobox_class}",
                value: Some(state.theme_value().into()),
                on_open_change: move |open| {
                    if open {
                        let total = palettes.read().len();

                        render_count.set(INITIAL_OPTIONS.min(total));

                        #[cfg(target_arch = "wasm32")]
                        crate::progressively_render::progressively_render_options(render_count, total);
                    }
                },
                on_value_change: on_value_change_fn,

                ComboboxInput {
                    class: "{input_class}",
                    placeholder: name(),
                    aria_label: "Theme",
                }

                ComboboxList { class: "{list_class}", aria_label: "Themes",

                    ComboboxEmpty { class: "{empty_class}", "No themes found" }

                    for (index, palette) in palettes.read().iter().take(render_count()).enumerate() {
                        ComboboxOption::<String> {
                            key: "{palette.0}",
                            class: "{option_class}",
                            index,
                            value: palette.0.to_string(),
                            text_value: palette.1.to_string(),
                            "data-value": palette.0,
                            "{palette.1}"
                        }
                    }
                }
            }
        }
    }
}
