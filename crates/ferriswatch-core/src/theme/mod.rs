//! Independent light and dark theme selections and catalogue resolution.

#[cfg(feature = "dioxus")]
use dioxus::stores as dioxus_stores;

pub mod config;

pub use crate::catalogue::{ResolveError, get, resolve};
pub use crate::theme_variant::{
    Appearance, ChromaticHue, ColorPair, Contrast, ThemeMetadata, ThemeSupport, ThemeVariant,
};
use crate::{catalogue::PALETTES, palette::families::PaletteFamily};
use crate::{catalogue::PaletteRegistration, theme_variant::ThemePalette};

/// A saved selection for each mode, including each variant's accent.
/// Support metadata is advisory: either slot may contain any variant.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "dioxus", derive(dioxus::prelude::Store))]

pub struct Theme {
    /// Variant used in light mode.
    pub light: ThemeVariant,
    /// Variant used in dark mode.
    pub dark: ThemeVariant,
}

impl Theme {
    /// Stores both choices without enforcing their support metadata.

    pub const fn new(light: ThemeVariant, dark: ThemeVariant) -> Self {
        Self { light, dark }
    }

    /// Returns the saved choice for a mode.

    pub const fn variant(&self, mode: Appearance) -> &ThemeVariant {
        match mode {
            Appearance::Light => &self.light,
            Appearance::Dark => &self.dark,
        }
    }

    /// Updates one mode without changing the other selection.

    pub fn set(&mut self, mode: Appearance, variant: ThemeVariant) {
        match mode {
            Appearance::Light => self.light = variant,
            Appearance::Dark => self.dark = variant,
        }
    }
}

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
