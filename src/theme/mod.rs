//! Independent light and dark theme selections and catalogue resolution.

#[cfg(feature = "dioxus")]
use dioxus::stores as dioxus_stores;

pub use crate::catalogue::{ResolveError, get, resolve};
pub use crate::theme_variant::{
    Appearance, ChromaticHue, ColorPair, Contrast, ThemeMetadata, ThemeSupport, ThemeVariant,
};

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
