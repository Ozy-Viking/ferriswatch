use dioxus::prelude::*;
use dioxus_primitives::combobox::{
    Combobox, ComboboxEmpty, ComboboxInput, ComboboxList, ComboboxOption,
};
use ferriswatch::dioxus::{Memory, use_theme};

#[derive(Clone, Copy, PartialEq, Eq)]

enum PickerSide {
    Theme,
    Accent,
}

/// Connected, searchable theme and accent controls for the nearest provider.
///
/// This widget ships no layout or chrome CSS. Pass classes for each part and
/// mount it inside [`ferriswatch::dioxus::ThemeProvider`].
///
/// When `matching_mode_only` is `true` (the default), the theme list includes
/// palettes that support the active appearance. An explicitly selected
/// out-of-mode theme stays visible because support is advisory. When `false`,
/// every configured palette is listed. Changing a theme preserves a compatible
/// accent.
///
/// # Examples
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use ferriswatch_componant::ThemeCombobox;
///
/// fn picker() -> Element {
///     rsx! {
///         ThemeCombobox {
///             class: "theme-combobox",
///             theme_class: "theme-half",
///             accent_class: "accent-half",
///             input_class: "combobox-input",
///             list_class: "combobox-list",
///             option_class: "combobox-option",
///             empty_class: "combobox-empty",
///             matching_mode_only: false,
///         }
///     }
/// }
/// ```
#[component]

pub fn ThemeCombobox(
    /// When true, list only palettes that support the active appearance.
    #[props(default = true)]
    matching_mode_only: bool,
    /// Class for the theme half of the joined control.
    #[props(default)]
    theme_class: String,
    /// Class for the accent half of the joined control.
    #[props(default)]
    accent_class: String,
    /// Class for each primitives combobox root.
    #[props(default)]
    combobox_class: String,
    /// Class for each combobox text input.
    #[props(default)]
    input_class: String,
    /// Class for each popup list.
    #[props(default)]
    list_class: String,
    /// Class for each option.
    #[props(default)]
    option_class: String,
    /// Class for the empty-list placeholder.
    #[props(default)]
    empty_class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {

    let mut state = use_theme::<Memory>();

    let current = state.current();

    let mode = state.mode();

    let palettes = state.listed_palettes(matching_mode_only);

    let accents = state.available_accents();

    let mut open = use_signal(|| None::<PickerSide>);

    use_effect(move || {

        let _ = (state.mode(), state.current());

        open.set(None);
    });

    rsx! {
        div {
            role: "group",
            aria_label: "Theme and accent",
            ..attributes,
            div { class: "{theme_class}", title: current.name().to_owned(),
                Combobox::<String> {
                    key: "theme-{mode:?}",
                    class: "{combobox_class}",
                    value: Some(state.theme_value().into()),
                    open: Some(open() == Some(PickerSide::Theme)),
                    on_open_change: move |next| {
                        if next { open.set(Some(PickerSide::Theme)); } else if open() == Some(PickerSide::Theme) { open.set(None); }
                    },
                    on_value_change: move |id: Option<String>| {
                        if let Some(id) = id {
                            let _ = state.select_theme(&id);
                        }
                    },
                    ComboboxInput {
                        class: "{input_class}",
                        placeholder: current.name().to_owned(),
                        aria_label: "Theme",
                    }
                    ComboboxList {
                        class: "{list_class}",
                        aria_label: "Themes",
                        ComboboxEmpty { class: "{empty_class}", "No themes found" }
                        for (index, palette) in palettes.iter().enumerate() {
                            ComboboxOption::<String> {
                                key: "{palette.metadata.id}",
                                class: "{option_class}",
                                index,
                                value: palette.metadata.id.to_string(),
                                text_value: palette.metadata.name.to_string(),
                                "data-value": palette.metadata.id.to_string(),
                                "{palette.metadata.name}"
                            }
                        }
                    }
                }
            }
            div { class: "{accent_class}", title: current.accent_name().unwrap_or("Default").to_owned(),
                Combobox::<String> {
                    key: "accent-{mode:?}-{current.id()}",
                    class: "{combobox_class}",
                    value: Some(state.accent_value().into()),
                    open: Some(open() == Some(PickerSide::Accent)),
                    on_open_change: move |next| {
                        if next { open.set(Some(PickerSide::Accent)); } else if open() == Some(PickerSide::Accent) { open.set(None); }
                    },
                    on_value_change: move |accent: Option<String>| {
                        if let Some(accent) = accent {
                            let _ = state.select_accent((!accent.is_empty()).then_some(accent.as_str()));
                        }
                    },
                    ComboboxInput {
                        class: "{input_class}",
                        placeholder: current.accent_name().unwrap_or("Default").to_owned(),
                        aria_label: "Accent",
                    }
                    ComboboxList {
                        class: "{list_class}",
                        aria_label: "Accents",
                        ComboboxEmpty { class: "{empty_class}", "No accents found" }
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
        if let Some(message) = state.last_error() { p { role: "alert", "{message}" } }
    }
}

#[cfg(test)]

mod tests {

    use super::*;
    use ferriswatch::{
        dioxus::{ThemeConfig, ThemeProvider, ThemeSelection},
        palette::catppuccin::{Latte, Mocha, latte::Blue, mocha::Mauve},
        theme::{Appearance, Theme},
        theme_variant::ThemePalette,
    };

    fn mixed_config() -> ThemeConfig {

        ThemeConfig::with_default(
            Theme::new(Latte::variant::<Blue>(), Mocha::variant::<Mauve>()),
            Appearance::Dark,
        )
        .available(
            ThemeSelection::new()
                .with_palette::<Latte>()
                .with_palette::<Mocha>(),
        )
        .build()
        .unwrap()
    }

    fn app() -> Element {

        let config = mixed_config();

        rsx! {
            ThemeProvider { config,
                ThemeCombobox {
                    matching_mode_only: false,
                    class: "root",
                    theme_class: "theme",
                    accent_class: "accent",
                }
            }
        }
    }

    #[test]

    fn combobox_mounts_inside_provider() {

        let mut dom = VirtualDom::new(app);

        dom.rebuild_in_place();
    }
}
