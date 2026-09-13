use std::rc::Rc;

use crate::{
    catalogue::{PALETTES, PaletteRegistration, ResolveError},
    palette::families::PaletteFamily,
    theme::{Appearance, Theme},
    theme_variant::{ThemePalette, ThemeVariant},
};

/// An ordered set of palette registrations.
/// `with_*` adds to the selection, never replaces it. Operations apply in call
/// order; repeated additions have no effect and removed palettes can be re-added.
#[derive(Clone, Debug, Default)]
pub struct ThemeSelection {
    entries: Vec<&'static PaletteRegistration>,
    conflicts: Vec<String>,
}

impl ThemeSelection {
    /// Starts with no palettes selected.
    pub fn new() -> Self {
        Self::default()
    }
    /// Selects all built-in palettes in catalogue order.
    pub fn all() -> Self {
        Self {
            entries: PALETTES.to_vec(),
            conflicts: Vec::new(),
        }
    }
    /// Adds every built-in palette in the family, retaining existing selections.
    pub fn with_family<F: PaletteFamily>(mut self) -> Self {
        for entry in PALETTES.iter().filter(|p| p.metadata.family_id == F::ID) {
            self.add(entry);
        }
        self
    }
    /// Removes all selected palettes in this family.
    pub fn without_family<F: PaletteFamily>(mut self) -> Self {
        self.entries.retain(|p| p.metadata.family_id != F::ID);
        self.conflicts
            .retain(|id| self.entries.iter().any(|p| p.metadata.id == *id));
        self
    }
    /// Adds one palette without replacing existing selections.
    pub fn with_palette<P: ThemePalette>(mut self) -> Self {
        self.add(P::registration());
        self
    }
    /// Adds an application-defined palette using the same registration contract.
    pub fn with_custom<P: ThemePalette>(self) -> Self {
        self.with_palette::<P>()
    }
    /// Removes one palette by its registered identity.
    pub fn without_palette<P: ThemePalette>(mut self) -> Self {
        let id = &P::registration().metadata.id;
        self.entries.retain(|p| p.metadata.id != *id);
        self.conflicts.retain(|conflict| conflict != id);
        self
    }
    /// Selected palettes in display order, including accents and defaults.
    pub fn palettes(&self) -> &[&'static PaletteRegistration] {
        &self.entries
    }
    fn add(&mut self, entry: &'static PaletteRegistration) {
        if let Some(existing) = self
            .entries
            .iter()
            .find(|p| p.metadata.id == entry.metadata.id)
        {
            if existing.metadata != entry.metadata
                || existing.sources != entry.sources
                || existing.raw_colors != entry.raw_colors
                || existing.default_accent != entry.default_accent
                || existing.resolve(None) != entry.resolve(None)
                || existing.accents.len() != entry.accents.len()
                || existing.accents.iter().zip(entry.accents).any(|(a, b)| {
                    a.id != b.id
                        || a.name != b.name
                        || a.color != b.color
                        || (a.factory)() != (b.factory)()
                })
            {
                self.conflicts.push(entry.metadata.id.to_string());
            }
        } else {
            self.entries.push(entry);
        }
    }
}

/// Invalid configuration or a theme selection outside the allowed catalogue.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ThemeError {
    #[error("at least one palette must be available")]
    EmptySelection,
    #[error("conflicting palette registration: {0}")]
    ConflictingRegistration(String),
    #[error("invalid palette registration: {0}")]
    InvalidRegistration(String),
    #[error("theme is not available: {0}")]
    Unavailable(String),
    #[error("default theme does not match its registered palette and accent")]
    InvalidDefault,
    #[error(transparent)]
    Resolve(#[from] ResolveError),
}

#[derive(Debug)]
struct ConfigData {
    default: Theme,
    mode: Appearance,
    available: ThemeSelection,
    override_dx_components_theme: bool,
}

/// Validated provider configuration. Construct through `with_default(...).build()`.
#[derive(Clone, Debug)]
pub struct ThemeConfig(Rc<ConfigData>);
impl PartialEq for ThemeConfig {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

/// Builder that always contains an explicit default theme.
#[derive(Clone, Debug)]
pub struct ThemeConfigBuilder {
    default: Theme,
    mode: Appearance,
    available: ThemeSelection,
    override_dx_components_theme: bool,
}
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
        let config = ThemeConfig(Rc::new(ConfigData {
            default: self.default,
            mode: self.mode,
            available: self.available,
            override_dx_components_theme: self.override_dx_components_theme,
        }));
        for default in [&config.default_theme().light, &config.default_theme().dark] {
            if config.resolve(default.id(), default.accent_id())? != *default {
                return Err(ThemeError::InvalidDefault);
            }
        }
        Ok(config)
    }
}
