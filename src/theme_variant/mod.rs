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
    name: String,
    colors: ThemeVariantColors,
    accent: Option<Color>,
    accent_name: Option<String>,
}

impl ThemeVariant {
    /// Builds a named variant with its explicitly selected accent, if any.
    /// Semantic colours already include any palette fallback.
    pub fn new(
        name: impl Into<String>,
        colors: ThemeVariantColors,
        accent: Option<Color>,
        accent_name: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            colors,
            accent,
            accent_name,
        }
    }

    /// The display name supplied by the palette or custom theme.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The explicit accent selection; `None` means palette defaults were used.
    pub const fn accent(&self) -> Option<Color> {
        self.accent
    }

    /// Display name of the explicit accent, if one was supplied.
    pub fn accent_name(&self) -> Option<&str> {
        self.accent_name.as_deref()
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
    fn variant<A>() -> ThemeVariant
    where
        A: Accent<Self>,
        Self: Sized;
}
