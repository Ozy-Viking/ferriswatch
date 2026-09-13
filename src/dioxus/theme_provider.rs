use super::{ThemeConfig, ThemeError, theme_css};
use crate::{
    theme::{Theme, ThemeStoreExt},
    theme_variant::{Appearance, ThemeVariant},
};
use dioxus::prelude::*;

#[derive(Clone, Store)]
struct ProviderData {
    theme: Theme,
    mode: Appearance,
}

/// Application-wide theme state shared by the nearest provider.
#[derive(Clone, Copy)]
pub struct ThemeState {
    data: Store<ProviderData>,
    config: Signal<ThemeConfig>,
}
impl ThemeState {
    /// Reads and subscribes only to the active mode's selection.
    pub fn current(&self) -> ThemeVariant {
        self.variant(self.mode())
    }
    /// Reads and subscribes to the active mode.
    pub fn mode(&self) -> Appearance {
        *self.data.mode().read()
    }
    /// Switches modes without modifying either saved selection.
    pub fn set_mode(&mut self, mode: Appearance) {
        self.data.mode().set(mode);
    }
    /// Reads one saved variant without subscribing to the other slot.
    pub fn variant(&self, mode: Appearance) -> ThemeVariant {
        match mode {
            Appearance::Light => self.data.theme().light().read().clone(),
            Appearance::Dark => self.data.theme().dark().read().clone(),
        }
    }
    /// Reads both selections.
    pub fn theme(&self) -> Theme {
        self.data.theme().read().clone()
    }
    pub fn config(&self) -> ThemeConfig {
        self.config.read().clone()
    }
    /// Changes only the active mode's palette and accent.
    /// Availability is enforced, but mode support remains advisory.
    pub fn select(&mut self, id: &str, accent: Option<&str>) -> Result<(), ThemeError> {
        self.select_for(self.mode(), id, accent)
    }
    /// Changes one saved selection, without changing the active mode.
    pub fn select_for(
        &mut self,
        mode: Appearance,
        id: &str,
        accent: Option<&str>,
    ) -> Result<(), ThemeError> {
        let theme = self.config.peek().resolve(id, accent)?;
        match mode {
            Appearance::Light => self.data.theme().light().set(theme),
            Appearance::Dark => self.data.theme().dark().set(theme),
        }
        Ok(())
    }
    /// Restores both configured variants, their accents, and the initial mode.
    pub fn reset(&mut self) {
        let config = self.config.peek();
        self.data.set(ProviderData {
            theme: config.default_theme().clone(),
            mode: config.default_mode(),
        });
    }
}

/// Reads the nearest provider's state. Call unconditionally in a component or hook.
/// Panics when used outside `ThemeProvider`.
pub fn use_theme() -> ThemeState {
    use_context()
}

/// Provides theme state and scoped CSS variables to its descendants.
/// Configuration is captured on mount. Remount with a new key to replace it.
/// Place at the application root for application-wide state. Portals rendered
/// outside this DOM wrapper need their own themed wrapper.
#[component]
pub fn ThemeProvider(config: ThemeConfig, children: Element) -> Element {
    let data = use_store(|| ProviderData {
        theme: config.default_theme().clone(),
        mode: config.default_mode(),
    });
    let config = use_signal(|| config);
    let state = use_context_provider(|| ThemeState { data, config });
    let style = theme_css(&state.current());
    rsx! { div { class: "fw-theme", style, {children} } }
}

/// Basic accessible selectors for available palettes and their supported accents.
#[component]
pub fn ThemePicker() -> Element {
    let mut state = use_theme();
    let theme = state.current();
    let mode = state.mode();
    let config = state.config();
    let entry = config
        .palettes()
        .iter()
        .find(|p| p.metadata.id == theme.id())
        .copied()
        .expect("provider only permits configured themes");
    let mut error = use_signal(|| None::<String>);
    rsx! {
        div { class: "fw-theme-picker",
            div { class: "fw-mode-control",
                span { "Light" }
                button {
                    class: "fw-mode-toggle", r#type: "button", role: "switch",
                    aria_label: "Dark mode", aria_checked: mode == Appearance::Dark,
                    onclick: move |_| {
                        state.set_mode(if state.mode() == Appearance::Dark { Appearance::Light } else { Appearance::Dark });
                        error.set(None);
                    },
                    span { class: "fw-mode-thumb", aria_hidden: "true" }
                }
                span { "Dark" }
            }
            label { "Theme"
                select { aria_label: "Theme", value: theme.id().to_owned(),
                    onchange: move |event| {
                        let id = event.value();
                        let config = state.config();
                        let current = state.current();
                        let accent = current.accent_id().filter(|accent| config.resolve(&id, Some(accent)).is_ok());
                        error.set(state.select(&id, accent).err().map(|e| e.to_string()));
                    },
                    // Keep an explicitly selected out-of-mode palette visible; support is advisory.
                    if !theme.supports(mode) {
                        option { value: theme.id().to_owned(), selected: true, "{theme.name()} (current selection)" }
                    }
                    for palette in config.palettes_for(mode) {
                        option { value: palette.metadata.id.to_string(), selected: palette.metadata.id == theme.id(), "{palette.metadata.name}" }
                    }
                }
            }
            label { "Accent"
                select { aria_label: "Accent", value: theme.accent_id().unwrap_or("").to_owned(),
                    onchange: move |event| {
                        let accent = event.value();
                        let id = state.current().id().to_owned();
                        error.set(state.select(&id, (!accent.is_empty()).then_some(accent.as_str())).err().map(|e| e.to_string()));
                    },
                    option { value: "", selected: theme.accent_id().is_none(), "Palette default" }
                    for accent in entry.accents {
                        option { value: accent.id, selected: theme.accent_id() == Some(accent.id), "{accent.name}" }
                    }
                }
            }
            if let Some(message) = error() { p { role: "alert", "{message}" } }
        }
    }
}
