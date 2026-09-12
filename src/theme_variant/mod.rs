mod identity;
pub use identity::{Appearance, Contrast, IdentityError, ResolvedAccent, ThemeMetadata};

use crate::{
    color::Color,
    palette::{Accent, Palette},
};

/// Fully resolved semantic colours. Palette factories supply every field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThemeVariantColors {
    pub surface: SurfaceColors,
    pub surface_alt: SurfaceColors,
    pub text: TextColors,
    pub primary: ActionColors,
    pub secondary: ActionColors,
    pub status: StatusColors,
    pub border: Color,
    pub border_muted: Color,
    pub focus: Color,
}

/// Backgrounds and interaction colour for one set of surfaces.
/// Raised surfaces describe placement, not a guaranteed brightness ordering.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceColors {
    pub background: Color,
    pub surface: Color,
    pub raised: Color,
    pub overlay: Color,
    pub hover: Color,
}

/// Text colours, including foregrounds for filled primary and secondary actions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextColors {
    pub normal: Color,
    pub muted: Color,
    pub subtle: Color,
    pub on_primary: Color,
    pub on_secondary: Color,
}

/// Colours for an action in each state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionColors {
    pub normal: Color,
    pub hover: Color,
    pub pressed: Color,
    pub muted: Color,
}

/// Foreground or icon colours for statuses and diagnostic levels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatusColors {
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub critical: Color,
    pub info: Color,
    pub trace: Color,
}

/// Resolved semantic colours, independent of the factory's palette and accent types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeVariant {
    metadata: ThemeMetadata,
    colors: ThemeVariantColors,
    accent: Option<ResolvedAccent>,
}

impl ThemeVariant {
    /// Creates a custom theme with a caller-supplied `custom/identifier`.
    ///
    /// # Errors
    /// Rejects malformed custom IDs and empty display labels.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        appearance: Appearance,
        colors: ThemeVariantColors,
        accent: Option<ResolvedAccent>,
    ) -> Result<Self, IdentityError> {
        let id = id.into();
        let variant = id
            .strip_prefix("custom/")
            .filter(|v| identity::valid_segment(v))
            .ok_or(IdentityError::InvalidCustomId)?
            .to_owned();
        let name = name.into();
        if name.trim().is_empty() {
            return Err(IdentityError::EmptyName);
        }
        Ok(Self {
            metadata: ThemeMetadata {
                id: id.into(),
                family_id: "custom".into(),
                family_name: "Custom".into(),
                variant_id: variant.into(),
                variant_name: name.clone().into(),
                name: name.into(),
                appearance,
                contrast: None,
            },
            colors,
            accent,
        })
    }

    pub(crate) fn from_palette(
        metadata: &ThemeMetadata,
        colors: ThemeVariantColors,
        accent: Option<ResolvedAccent>,
    ) -> Self {
        Self {
            metadata: metadata.clone(),
            colors,
            accent,
        }
    }

    /// Stable theme identity and display metadata.
    pub const fn metadata(&self) -> &ThemeMetadata {
        &self.metadata
    }
    /// Canonical ID for persistence.
    pub fn id(&self) -> &str {
        &self.metadata.id
    }
    /// Complete display label supplied by the palette or application.
    pub fn name(&self) -> &str {
        &self.metadata.name
    }
    /// Explicit accent metadata, absent for `NoAccent`.
    pub const fn selected_accent(&self) -> Option<&ResolvedAccent> {
        self.accent.as_ref()
    }
    /// Explicit accent colour, including transparency.
    pub fn accent(&self) -> Option<Color> {
        self.accent.as_ref().map(ResolvedAccent::color)
    }
    /// Explicit accent's display label.
    pub fn accent_name(&self) -> Option<&str> {
        self.accent.as_ref().map(ResolvedAccent::name)
    }
    /// Explicit accent's persisted ID.
    pub fn accent_id(&self) -> Option<&str> {
        self.accent.as_ref().map(ResolvedAccent::id)
    }

    /// All resolved colour groups, including the alternate surfaces.
    pub const fn colors(&self) -> &ThemeVariantColors {
        &self.colors
    }

    pub fn critical(&self) -> Color {
        self.colors.status.critical
    }

    pub fn trace(&self) -> Color {
        self.colors.status.trace
    }

    pub fn primary(&self) -> Color {
        self.colors.primary.normal
    }

    pub fn primary_hover(&self) -> Color {
        self.colors.primary.hover
    }

    pub fn primary_muted(&self) -> Color {
        self.colors.primary.muted
    }

    pub fn secondary(&self) -> Color {
        self.colors.secondary.normal
    }

    pub fn success(&self) -> Color {
        self.colors.status.success
    }

    pub fn warning(&self) -> Color {
        self.colors.status.warning
    }

    pub fn error(&self) -> Color {
        self.colors.status.error
    }

    pub fn info(&self) -> Color {
        self.colors.status.info
    }

    pub fn background(&self) -> Color {
        self.colors.surface.background
    }

    pub fn surface(&self) -> Color {
        self.colors.surface.surface
    }

    pub fn surface_hover(&self) -> Color {
        self.colors.surface.hover
    }

    pub fn overlay(&self) -> Color {
        self.colors.surface.overlay
    }

    pub fn text(&self) -> Color {
        self.colors.text.normal
    }

    pub fn text_muted(&self) -> Color {
        self.colors.text.muted
    }

    pub fn text_subtle(&self) -> Color {
        self.colors.text.subtle
    }

    pub fn border(&self) -> Color {
        self.colors.border
    }

    pub fn border_muted(&self) -> Color {
        self.colors.border_muted
    }

    pub fn focus(&self) -> Color {
        self.colors.focus
    }
}

/// A palette that can produce a [`ThemeVariant`].
pub trait ThemePalette: Palette {
    /// Semantic colors for this palette, with `A` as the accent.
    ///
    /// # Panics
    /// Built-in implementations panic if a custom accent violates the
    /// [`Accent`] metadata contract.
    fn variant<A>() -> ThemeVariant
    where
        A: Accent<Self>,
        Self: Sized;
}
