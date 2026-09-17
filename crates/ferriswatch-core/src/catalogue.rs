//! Built-in theme discovery and resolution from stable IDs.
//!
//! ```
//! use ferriswatch_core::catalogue;
//! let theme = catalogue::resolve("everforest/light/soft", Some("green"))?;
//! assert_eq!(theme.id(), "everforest/light/soft");
//! assert_eq!(theme.accent_id(), Some("green"));
//! # Ok::<(), catalogue::ResolveError>(())
//! ```

use std::borrow::Cow;

use crate::{
    color::Color,
    theme::ThemeSelection,
    theme_variant::{Appearance, ThemeMetadata, ThemeVariant},
};

/// A pinned upstream resource used to import a palette or its UI mappings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub struct PaletteSource {
    /// Upstream repository URL.
    pub repository: &'static str,
    /// Full commit SHA or upstream release tag.
    pub revision: &'static str,
    /// Exact path relative to the repository root.
    pub path: &'static str,
    /// Upstream licence identifier or descriptive name at the pinned revision.
    /// `None` means unverified, not permission to use the source.
    /// This records the upstream notice, not a legal compatibility assessment.
    pub licence: Option<&'static str>,
}

impl PaletteSource {
    /// Permanent source link for inspecting the imported revision.

    pub fn permalink(&self) -> String {
        use std::fmt::Write;

        let mut path = String::new();

        for byte in self.path.bytes() {
            if byte.is_ascii_alphanumeric() || b"-._~/".contains(&byte) {
                path.push(char::from(byte));
            } else {
                write!(path, "%{byte:02X}").expect("writing to a String cannot fail");
            }
        }

        format!("{}/blob/{}/{}", self.repository, self.revision, path)
    }
}

/// A curated named accent and its typed factory erased for runtime selection.
#[derive(Clone, Copy, Debug)]

pub struct AccentRegistration {
    /// Persisted ID.
    pub id: &'static str,
    /// Display label.
    pub name: &'static str,
    /// Raw accent colour.
    pub color: Color,
    /// Factory producing the registered theme selection.
    pub factory: fn() -> ThemeVariant,
}

/// One valid variant/contrast combination. This drives menus and resolution.
#[derive(Debug, Clone)]

pub struct PaletteRegistration {
    /// Stable identity, appearance and labels.
    pub metadata: ThemeMetadata,
    /// Eligible accents, excluding `NoAccent`.
    pub accents: &'static [AccentRegistration],
    /// Palette-default accent ID used when the selection is absent.
    pub default_accent: &'static str,
    /// Pinned upstream resources.
    pub sources: &'static [PaletteSource],
    /// Raw source colours, independently available from semantic roles.
    pub raw_colors: &'static [(&'static str, Color)],
    /// Factory producing the registered theme selection.
    pub factory: fn() -> ThemeVariant,
}

impl PaletteRegistration {
    /// Resolves a supported accent, or the palette default without accent metadata.
    ///
    /// # Errors
    /// Returns `UnknownAccent` for unsupported IDs, including an empty string.

    pub fn resolve(&self, accent_id: Option<&str>) -> Result<ThemeVariant, ResolveError> {
        match accent_id {
            None => Ok((self.factory)()),
            Some(id) => self
                .accents
                .iter()
                .find(|a| a.id == id)
                .map(|a| (a.factory)())
                .ok_or_else(|| ResolveError::UnknownAccent {
                    theme_id: self.metadata.id.to_string(),
                    accent_id: id.into(),
                }),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ThemeId {
    pub id: Cow<'static, str>,
    pub name: Cow<'static, str>,
}

impl From<PaletteRegistration> for ThemeId {
    fn from(palette_registration: PaletteRegistration) -> Self {
        Self {
            id: palette_registration.metadata.id.clone(),
            name: palette_registration.metadata.name.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ThemeLists {
    light: Vec<&'static PaletteRegistration>,
    dark: Vec<&'static PaletteRegistration>,
}

impl ThemeLists {
    pub fn new(palettes: &[&'static PaletteRegistration]) -> Self {
        let mut dark = vec![];
        let mut light = vec![];

        for &palette in palettes {
            if palette.metadata.support.supports(Appearance::Light) {
                light.push(palette);
            }
            if palette.metadata.support.supports(Appearance::Dark) {
                dark.push(palette);
            }
        }

        Self { light, dark }
    }

    pub fn light(&self) -> &[&'static PaletteRegistration] {
        &self.light
    }

    pub fn dark(&self) -> &[&'static PaletteRegistration] {
        &self.dark
    }

    pub fn get(&self, mode: Appearance) -> &[&'static PaletteRegistration] {
        match mode {
            Appearance::Dark => self.dark(),
            Appearance::Light => self.light(),
        }
    }
}

impl From<ThemeSelection> for ThemeLists {
    fn from(theme_selection: ThemeSelection) -> Self {
        ThemeLists::new(theme_selection.palettes())
    }
}

/// A selection absent from the built-in catalogue.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]

pub enum ResolveError {
    /// Unknown family/variant or malformed theme ID.
    #[error("unknown theme ID: {0}")]
    UnknownTheme(String),
    /// Known family and variant, but an unsupported or missing contrast.
    #[error("unsupported contrast selection: {0}")]
    UnsupportedContrast(String),
    /// The theme does not support this accent.
    #[error("unknown accent {accent_id} for {theme_id}")]
    UnknownAccent { theme_id: String, accent_id: String },
}

mod registrations;

pub use registrations::PALETTES;

/// Built-in registrations eligible for a mode, including palettes supporting both.
/// Resolution remains unrestricted by mode.

pub fn palettes_for(mode: Appearance) -> impl Iterator<Item = &'static PaletteRegistration> {
    PALETTES
        .iter()
        .copied()
        .filter(move |entry| entry.metadata.support.supports(mode))
}

/// Resolves the default variant of each palette eligible for a mode.

pub fn variants_for(mode: Appearance) -> impl Iterator<Item = ThemeVariant> {
    palettes_for(mode).map(|entry| (entry.factory)())
}

/// Looks up one declared theme ID. Labels and aliases are not accepted.
///
/// # Errors
/// Returns `UnknownTheme` or `UnsupportedContrast` for unregistered combinations.

pub fn get(id: &str) -> Result<&'static PaletteRegistration, ResolveError> {
    if let Some(entry) = PALETTES.iter().find(|p| p.metadata.id == id) {
        return Ok(entry);
    }

    let parts: Vec<_> = id.split('/').collect();

    if (parts.len() == 2 || parts.len() == 3)
        && PALETTES
            .iter()
            .any(|p| p.metadata.family_id == parts[0] && p.metadata.variant_id == parts[1])
    {
        return Err(ResolveError::UnsupportedContrast(id.into()));
    }

    Err(ResolveError::UnknownTheme(id.into()))
}

/// Resolves a persisted theme and optional accent selection.
///
/// # Errors
/// Rejects unknown themes, unsupported contrast combinations and unknown accents.

pub fn resolve(id: &str, accent_id: Option<&str>) -> Result<ThemeVariant, ResolveError> {
    get(id)?.resolve(accent_id)
}
