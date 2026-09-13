use super::{ThemeConfig, ThemeError, ThemeScope, theme_css};
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
/// Place at the application root for application-wide state. Scoped mode is
/// the default. Root mode publishes variables and scheme on `:root`, including
/// for portals in the same document; use one root provider per document.
/// Scope controls CSS inheritance, not where component context is available.
#[component]
pub fn ThemeProvider(
    config: ThemeConfig,
    children: Element,
    #[props(default)] scope: ThemeScope,
) -> Element {
    let data = use_store(|| ProviderData {
        theme: config.default_theme().clone(),
        mode: config.default_mode(),
    });
    let config = use_signal(|| config);
    let state = use_context_provider(|| ThemeState { data, config });
    let current = state.current();
    let scheme = match current.metadata().appearance {
        Appearance::Dark => "dark",
        Appearance::Light => "light",
    };
    let declarations = format!("{}color-scheme:{scheme};", theme_css(&current));
    let scoped = scope == ThemeScope::Scoped;
    // Keep longhand background-color: Dioxus style preservation can clear a
    // variable-based background shorthand on subsequent theme updates.
    let style = format!(
        "{}background-color:var(--fs-background);color:var(--fs-text);",
        if scoped { declarations.as_str() } else { "" }
    );
    rsx! {
        if !scoped {
            style { "data-fs-root": "", ":root {{{declarations}}}" }
        }
        div { class: "fs-theme", style, {children} }
    }
}

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
            super::ThemeCombobox {}
        }
    }
}
