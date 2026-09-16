use super::{persistence::ThemeSnapshot, theme_css};
use crate::{
    catalogue::{AccentRegistration, PaletteRegistration},
    css::ThemeScope,
    theme::{Theme, ThemeError, ThemeStoreExt, config::ThemeConfig},
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
    theme_value: Memo<Option<String>>,
    accent_value: Memo<Option<String>>,
    last_error: Signal<Option<String>>,
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

    /// Whether the active appearance is dark.

    pub fn is_dark(&self) -> bool {
        self.mode() == Appearance::Dark
    }

    /// Switches to the other appearance without changing either saved selection.

    pub fn toggle_mode(&mut self) {
        self.set_mode(if self.is_dark() {
            Appearance::Light
        } else {
            Appearance::Dark
        });
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

    /// Registered palette for the active mode's current selection.
    ///
    /// # Panics
    ///
    /// Panics if the current theme is not in this provider's configuration.

    pub fn selected_theme(&self) -> &'static PaletteRegistration {
        let current = self.current();

        self.config
            .read()
            .palettes()
            .iter()
            .copied()
            .find(|palette| palette.metadata.id == current.id())
            .expect("current theme is registered")
    }

    /// Accents declared by [`Self::selected_theme`], excluding the palette default.

    pub fn available_accents(&self) -> &'static [AccentRegistration] {
        self.selected_theme().accents
    }

    /// Canonical id of the active palette.

    pub fn selected_theme_id(&self) -> String {
        self.current().id().to_owned()
    }

    /// Explicit accent id of the active selection, if any.

    pub fn selected_accent_id(&self) -> Option<String> {
        self.current().accent_id().map(str::to_owned)
    }

    /// Display name of the explicit accent, if any.

    pub fn selected_accent_name(&self) -> Option<String> {
        self.current().accent_name().map(str::to_owned)
    }

    /// Combobox binding for the active palette id.

    pub fn theme_value(&self) -> Memo<Option<String>> {
        self.theme_value
    }

    /// Combobox binding for the active accent; `Some("")` means the palette default.

    pub fn accent_value(&self) -> Memo<Option<String>> {
        self.accent_value
    }

    /// Message from the last failed select, restore, or equivalent.

    pub fn last_error(&self) -> Option<String> {
        self.last_error.read().clone()
    }

    /// Palettes to show in a theme menu.
    ///
    /// When `matching_mode_only` is `true`, this is [`ThemeConfig::palettes_for`]
    /// for the active mode. An explicitly selected out-of-mode theme stays in
    /// the list because support is advisory. When `false`, this is every
    /// configured palette.

    pub fn listed_palettes(&self, matching_mode_only: bool) -> Vec<&'static PaletteRegistration> {
        let config = self.config.read();

        if !matching_mode_only {
            return config.palettes().to_vec();
        }

        let mode = self.mode();
        let current = self.current();
        let mut palettes = config.theme_lists().get(mode).to_vec();

        if !current.supports(mode) {
            let selected = self.selected_theme();

            if palettes
                .iter()
                .all(|palette| palette.metadata.id != selected.metadata.id)
            {
                palettes.push(selected);
            }
        }

        palettes
    }

    /// Changes only the active mode's palette and accent.
    /// Availability is enforced, but mode support remains advisory.

    pub fn select(&mut self, id: &str, accent: Option<&str>) -> Result<(), ThemeError> {
        self.select_for(self.mode(), id, accent)
    }

    /// Selects a palette, keeping a compatible accent or the palette default.

    pub fn select_theme(&mut self, id: &str) -> Result<(), ThemeError> {
        let accent = self
            .current()
            .accent_id()
            .filter(|accent| self.config.peek().resolve(id, Some(accent)).is_ok())
            .map(str::to_owned);

        self.select(id, accent.as_deref())
    }

    /// Selects an accent on the active palette. `None` or `""` is the default.

    pub fn select_accent(&mut self, accent: Option<&str>) -> Result<(), ThemeError> {
        let id = self.current().id().to_owned();

        self.select(&id, accent.filter(|value| !value.is_empty()))
    }

    /// Changes one saved selection, without changing the active mode.

    pub fn select_for(
        &mut self,
        mode: Appearance,
        id: &str,
        accent: Option<&str>,
    ) -> Result<(), ThemeError> {
        match self.config.peek().resolve(id, accent) {
            Ok(theme) => {
                match mode {
                    Appearance::Light => self.data.theme().light().set(theme),
                    Appearance::Dark => self.data.theme().dark().set(theme),
                }

                self.last_error.set(None);

                Ok(())
            }
            Err(error) => {
                self.last_error.set(Some(error.to_string()));

                Err(error)
            }
        }
    }

    /// Restores both configured variants, their accents, and the initial mode.

    pub fn reset(&mut self) {
        let config = self.config.peek();

        self.data.set(ProviderData {
            theme: config.default_theme().clone(),
            mode: config.default_mode(),
        });

        self.last_error.set(None);
    }

    /// Active mode, palette id, and accent for persistence backends.

    pub fn snapshot(&self) -> ThemeSnapshot {
        let current = self.current();

        ThemeSnapshot {
            mode: self.mode(),
            id: current.id().to_owned(),
            accent: current.accent_id().map(str::to_owned),
        }
    }

    /// Applies a snapshot to the matching mode slot, then switches to that mode.
    ///
    /// # Errors
    ///
    /// Returns [`ThemeError::Unavailable`] or a resolve error without changing
    /// mode when the palette or accent is not in this configuration.

    pub fn restore(&mut self, snapshot: ThemeSnapshot) -> Result<(), ThemeError> {
        self.select_for(snapshot.mode, &snapshot.id, snapshot.accent.as_deref())?;

        self.set_mode(snapshot.mode);

        Ok(())
    }
}

/// Reads the nearest provider's state. Call unconditionally in a component or hook.
///
/// [`super::persistence::Memory`] does not persist. [`dioxus_sdk_storage::LocalStorage`]
/// and [`dioxus_sdk_storage::SessionStorage`] load a [`super::persistence::ThemeSnapshot`]
/// on first use and write it when the selection changes.
///
/// Panics when used outside `ThemeProvider`.

pub fn use_theme<S: super::persistence::ThemeStorage>() -> ThemeState {
    let mut state = use_context::<ThemeState>();

    use_hook(|| {
        if let Some(snapshot) = S::load() {
            let _ = state.restore(snapshot);
        }
    });

    use_effect(move || {
        S::save(&state);
    });

    state
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

    let last_error = use_signal(|| None::<String>);

    let theme_value = use_memo(move || {
        let mode = *data.mode().read();

        let variant = match mode {
            Appearance::Light => data.theme().light().read().clone(),
            Appearance::Dark => data.theme().dark().read().clone(),
        };

        Some(variant.id().to_owned())
    });

    let accent_value = use_memo(move || {
        let mode = *data.mode().read();

        let variant = match mode {
            Appearance::Light => data.theme().light().read().clone(),
            Appearance::Dark => data.theme().dark().read().clone(),
        };

        Some(variant.accent_id().unwrap_or("").to_owned())
    });

    let state = use_context_provider(|| ThemeState {
        data,
        config,
        theme_value,
        accent_value,
        last_error,
    });

    let current = state.current();

    let scheme = match current.metadata().appearance {
        Appearance::Dark => "dark",
        Appearance::Light => "light",
    };

    let mut declarations = format!("{}color-scheme:{scheme};", theme_css(&current));

    let override_dx = state.config().override_dx_components_theme();

    if override_dx {
        declarations.push_str(&super::dx_theme::declarations(
            current.metadata().appearance,
        ));
    }

    // Outrank upstream :root and html[data-theme] without !important or DOM mutation.
    let root_selector = if override_dx { ":root:root" } else { ":root" };

    let scoped = scope == ThemeScope::Scoped;

    // Upstream assigns these switches directly on .dxc-system descendants, so
    // inherited provider declarations alone cannot override them.
    let system_selector = if scoped {
        ".fs-theme[data-fs-dx-theme][data-fs-dx-theme] .dxc-system"
    } else {
        ":root:root .dxc-system"
    };

    // Keep longhand background-color: Dioxus style preservation can clear a
    // variable-based background shorthand on subsequent theme updates.
    let style = format!(
        "{}background-color:var(--fs-background);color:var(--fs-text);",
        if scoped { declarations.as_str() } else { "" }
    );

    rsx! {
        if !scoped {
            style { "data-fs-root": "", "{root_selector} {{{declarations}}}" }
        }
        if override_dx {
            style { "data-fs-dx-switches": "",
                "{system_selector} {{ --dxc-dark-on:var(--dark);--dxc-light-on:var(--light); }}"
            }
        }
        div {
            class: "fs-theme",
            "data-fs-dx-theme": override_dx.then_some("true"),
            style,
            {children}
        }
    }
}
