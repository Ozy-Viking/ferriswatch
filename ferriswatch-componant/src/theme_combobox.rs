use crate::combobox::*;
use dioxus::prelude::*;
use ferriswatch::dioxus::use_theme;

#[css_module("/src/theme_combobox.css")]
struct Styles;

#[derive(Clone, Copy, PartialEq, Eq)]
enum PickerSide {
    Theme,
    Accent,
}

/// Connected, searchable theme and accent controls for the nearest provider.
/// Themes are filtered by the active mode; changing a theme preserves a compatible
/// accent. Mount within `ThemeProvider` and build component assets with `dx`.
#[component]
pub fn ThemeCombobox() -> Element {
    let mut state = use_theme();
    let current = state.current();
    let mode = state.mode();
    let config = state.config();
    let mut palettes: Vec<_> = config.palettes_for(mode).collect();
    let entry = config
        .palettes()
        .iter()
        .find(|p| p.metadata.id == current.id())
        .copied()
        .expect("current theme is registered");
    if !current.supports(mode) {
        palettes.push(entry);
    }
    let selected_theme = use_memo(move || Some(state.current().id().to_owned()));
    let selected_accent =
        use_memo(move || Some(state.current().accent_id().unwrap_or("").to_owned()));
    let mut open = use_signal(|| None::<PickerSide>);
    let mut error = use_signal(|| None::<String>);
    use_effect(move || {
        let _ = (state.mode(), state.current());
        open.set(None);
    });
    rsx! {
        div { class: Styles::fs_theme_combobox, role: "group", aria_label: "Theme and accent",
            div { class: Styles::fs_theme_half, title: current.name().to_owned(),
                Combobox::<String> {
                    key: "theme-{mode:?}",
                    value: Some(selected_theme.into()),
                    open: Some(open() == Some(PickerSide::Theme)),
                    on_open_change: move |next| {
                        if next { open.set(Some(PickerSide::Theme)); } else if open() == Some(PickerSide::Theme) { open.set(None); }
                    },
                    aria_label: "Theme", list_aria_label: "Themes",
                    placeholder: current.name().to_owned(),
                    on_value_change: move |id: Option<String>| {
                        if let Some(id) = id {
                            let config = state.config();
                            let current = state.current();
                            let accent = current.accent_id().filter(|a| config.resolve(&id, Some(a)).is_ok());
                            error.set(state.select(&id, accent).err().map(|e| e.to_string()));
                        }
                    },
                    ComboboxEmpty { "No themes found" }
                    for (index, palette) in palettes.iter().enumerate() {
                        ComboboxOption::<String> {
                            key: "{palette.metadata.id}", index,
                            value: palette.metadata.id.to_string(),
                            text_value: palette.metadata.name.to_string(),
                            "data-value": palette.metadata.id.to_string(),
                            "{palette.metadata.name}"
                        }
                    }
                }
            }
            div { class: Styles::fs_accent_half, title: current.accent_name().unwrap_or("Default").to_owned(),
                Combobox::<String> {
                    key: "accent-{mode:?}-{current.id()}",
                    value: Some(selected_accent.into()),
                    open: Some(open() == Some(PickerSide::Accent)),
                    on_open_change: move |next| {
                        if next { open.set(Some(PickerSide::Accent)); } else if open() == Some(PickerSide::Accent) { open.set(None); }
                    },
                    aria_label: "Accent", list_aria_label: "Accents",
                    placeholder: current.accent_name().unwrap_or("Default").to_owned(),
                    on_value_change: move |accent: Option<String>| {
                        if let Some(accent) = accent {
                            let id = state.current().id().to_owned();
                            error.set(state.select(&id, (!accent.is_empty()).then_some(accent.as_str())).err().map(|e| e.to_string()));
                        }
                    },
                    ComboboxEmpty { "No accents found" }
                    ComboboxOption::<String> {
                        index: 0usize, value: String::new(), text_value: "Default",
                        "data-value": "", "Default"
                    }
                    for (index, accent) in entry.accents.iter().enumerate() {
                        ComboboxOption::<String> {
                            key: "{accent.id}", index: index + 1,
                            value: accent.id.to_owned(), text_value: accent.name.to_owned(),
                            "data-value": accent.id, "{accent.name}"
                        }
                    }
                }
            }
        }
        if let Some(message) = error() { p { role: "alert", "{message}" } }
    }
}
