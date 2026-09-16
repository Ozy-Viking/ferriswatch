//! Stable identity for built-in and application-defined themes.

use crate::color::Color;
use std::borrow::Cow;

/// Overall appearance, independent of a variant's upstream name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]

pub enum Appearance {
    Dark,
    Light,
}

/// Advisory eligibility for the light and dark selections in a theme.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]

pub enum ThemeSupport {
    /// Include in light-mode lists.
    Light,
    /// Include in dark-mode lists.
    Dark,
    /// Include in both lists.
    Both,
}

impl ThemeSupport {
    /// Filters a choice list; it does not restrict theme construction or selection.

    pub const fn supports(self, mode: Appearance) -> bool {
        matches!(
            (self, mode),
            (Self::Both, _) | (Self::Light, Appearance::Light) | (Self::Dark, Appearance::Dark)
        )
    }
}

impl From<Appearance> for ThemeSupport {
    fn from(appearance: Appearance) -> Self {
        match appearance {
            Appearance::Light => Self::Light,
            Appearance::Dark => Self::Dark,
        }
    }
}

/// A palette's explicitly supported background contrast.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]

pub enum Contrast {
    /// Soft background contrast.
    Soft,
    /// Medium background contrast.
    Medium,
    /// Hard background contrast.
    Hard,
}

impl Contrast {
    /// Persisted contrast segment.

    pub const fn id(self) -> &'static str {
        match self {
            Self::Soft => "soft",
            Self::Medium => "medium",
            Self::Hard => "hard",
        }
    }
}

/// Declared identifiers and labels. Display labels never act as lookup keys.
#[derive(Clone, Debug, PartialEq, Eq)]

pub struct ThemeMetadata {
    /// Canonical `family/variant[/contrast]` or `custom/identifier`.
    pub id: Cow<'static, str>,
    /// Machine-readable family identifier.
    pub family_id: Cow<'static, str>,
    /// Human-readable family name.
    pub family_name: Cow<'static, str>,
    /// Upstream variant identifier, or `main` for a single-variant family.
    pub variant_id: Cow<'static, str>,
    /// Human-readable variant name.
    pub variant_name: Cow<'static, str>,
    /// Complete display label.
    pub name: Cow<'static, str>,
    /// Light or dark appearance.
    pub appearance: Appearance,
    /// Advisory mode eligibility, used to filter palette choices.
    pub support: ThemeSupport,
    /// Only present for a supported contrast setting.
    pub contrast: Option<Contrast>,
}

/// An explicit accent selection. Transparency does not mean absence.
#[derive(Clone, Debug, PartialEq, Eq)]

pub struct ResolvedAccent {
    id: Cow<'static, str>,
    name: Cow<'static, str>,
    color: Color,
}

impl ResolvedAccent {
    /// Creates an accent with a stable identifier and a separate display name.
    ///
    /// # Errors
    /// Rejects identifiers outside ASCII snake_case or empty display names.

    pub fn new(
        id: impl Into<Cow<'static, str>>,
        name: impl Into<Cow<'static, str>>,
        color: Color,
    ) -> Result<Self, IdentityError> {
        let id = id.into();

        let name = name.into();

        if !valid_segment(&id) || name.trim().is_empty() {
            return Err(IdentityError::InvalidAccent);
        }

        Ok(Self { id, name, color })
    }

    /// Stable accent identifier.

    pub fn id(&self) -> &str {
        &self.id
    }

    /// Display label.

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Explicit colour, including its alpha channel.

    pub const fn color(&self) -> Color {
        self.color
    }
}

/// Invalid custom identity or accent metadata.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]

pub enum IdentityError {
    /// Custom themes require a caller-supplied identifier.
    #[error("custom theme ID must be custom/<snake_case_identifier>")]
    InvalidCustomId,
    /// Labels cannot be empty.
    #[error("theme display name must not be empty")]
    EmptyName,
    /// Accent metadata must contain a valid ID and label.
    #[error("accent requires a snake_case ID and a nonempty display name")]
    InvalidAccent,
}

pub(crate) fn valid_segment(value: &str) -> bool {
    !value.is_empty()
        && value.as_bytes()[0].is_ascii_lowercase()
        && value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
        && !value.ends_with('_')
        && !value.contains("__")
}
