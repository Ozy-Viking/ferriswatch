use std::rc::Rc;

use crate::{
    catalogue::{PaletteRegistration, ThemeLists},
    theme::{Appearance, Theme, ThemeError, ThemeSelection},
    theme_variant::ThemeVariant,
};

/// Validated provider configuration. Construct through `with_default(...).build()`.
#[derive(Clone, Debug)]

pub struct ThemeConfig(Rc<ConfigData>);

impl ThemeConfig {
    /// Starts the builder with a mandatory default and all built-in palettes available.

    pub fn with_default(default: Theme, mode: Appearance) -> ThemeConfigBuilder {
        ThemeConfigBuilder {
            default,
            mode,
            available: ThemeSelection::all(),
            override_dx_components_theme: false,
        }
    }

    /// Whether the provider overrides Dioxus Components' color variables.

    pub fn override_dx_components_theme(&self) -> bool {
        self.0.override_dx_components_theme
    }

    pub fn default_theme(&self) -> &Theme {
        &self.0.default
    }
    pub fn theme_lists(&self) -> &ThemeLists {
        &self.0.theme_lists
    }
    pub fn palettes(&self) -> &[&'static PaletteRegistration] {
        self.0.available.palettes()
    }

    /// Initial mode, also restored by reset.

    pub fn default_mode(&self) -> Appearance {
        self.0.mode
    }

    /// Available registrations filtered by advisory support, including `Both`.

    pub fn palettes_for(
        &self,
        mode: Appearance,
    ) -> impl Iterator<Item = &'static PaletteRegistration> + '_ {
        self.palettes()
            .iter()
            .copied()
            .filter(move |entry| entry.metadata.support.supports(mode))
    }

    /// Default variants for the requested mode, including `Both`.

    pub fn variants_for(&self, mode: Appearance) -> impl Iterator<Item = ThemeVariant> + '_ {
        self.palettes_for(mode).map(|entry| (entry.factory)())
    }

    /// Resolves a runtime selection only within this configuration.

    pub fn resolve(&self, id: &str, accent: Option<&str>) -> Result<ThemeVariant, ThemeError> {
        let entry = self
            .palettes()
            .iter()
            .find(|p| p.metadata.id == id)
            .ok_or_else(|| ThemeError::Unavailable(id.into()))?;

        Ok(entry.resolve(accent)?)
    }
}
impl PartialEq for ThemeConfig {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Debug)]

pub struct ConfigData {
    default: Theme,
    mode: Appearance,
    available: ThemeSelection,
    override_dx_components_theme: bool,
    theme_lists: ThemeLists,
}

/// Builder that always contains an explicit default theme.
#[derive(Clone, Debug)]

pub struct ThemeConfigBuilder {
    default: Theme,
    mode: Appearance,
    available: ThemeSelection,
    override_dx_components_theme: bool,
}

impl ThemeConfigBuilder {
    /// Map dx-components-theme.css colors and mode switches to this provider.
    /// Disabled by default. Uses the provider's configured CSS scope.

    pub fn override_dx_components_theme(mut self, enabled: bool) -> Self {
        self.override_dx_components_theme = enabled;

        self
    }

    /// Sets the allowed selection. The selection's `with_*` methods are additive.

    pub fn available(mut self, available: ThemeSelection) -> Self {
        self.available = available;

        self
    }

    /// Validates registrations and checks that the exact default is available.

    pub fn build(self) -> Result<ThemeConfig, ThemeError> {
        if self.available.entries.is_empty() {
            return Err(ThemeError::EmptySelection);
        }

        if let Some(id) = self.available.conflicts.first() {
            return Err(ThemeError::ConflictingRegistration(id.clone()));
        }

        for entry in &self.available.entries {
            let invalid = || ThemeError::InvalidRegistration(entry.metadata.id.to_string());

            let default = entry.resolve(None)?;

            if default.metadata() != &entry.metadata
                || default.selected_accent().is_some()
                || !entry.accents.iter().any(|a| a.id == entry.default_accent)
            {
                return Err(invalid());
            }

            let mut ids = std::collections::HashSet::new();

            for accent in entry.accents {
                let theme = (accent.factory)();

                if !ids.insert(accent.id)
                    || theme.metadata() != &entry.metadata
                    || theme.accent_id() != Some(accent.id)
                    || theme.accent_name() != Some(accent.name)
                    || theme.accent() != Some(accent.color)
                {
                    return Err(invalid());
                }
            }

            if entry.resolve(Some(entry.default_accent))?.colors() != default.colors() {
                return Err(invalid());
            }
        }
        let theme_lists = ThemeLists::new(&self.available.entries);

        let config = ThemeConfig(Rc::new(ConfigData {
            default: self.default,
            mode: self.mode,
            available: self.available,
            override_dx_components_theme: self.override_dx_components_theme,
            theme_lists,
        }));

        for default in [&config.default_theme().light, &config.default_theme().dark] {
            if config.resolve(default.id(), default.accent_id())? != *default {
                return Err(ThemeError::InvalidDefault);
            }
        }

        Ok(config)
    }
}
