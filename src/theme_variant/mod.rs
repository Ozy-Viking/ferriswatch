mod identity;

pub use identity::{
    Appearance, Contrast, IdentityError, ResolvedAccent, ThemeMetadata, ThemeSupport,
};

use crate::{
    color::Color,
    palette::{Accent, Palette},
};

/// A pair of colors intended to be used together.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub struct ColorPair {
    /// Foreground colour intended to be rendered against [`Self::background`].
    pub foreground: Color,
    /// Fill or surface behind [`Self::foreground`].
    pub background: Color,
}

impl ColorPair {
    pub const fn new(foreground: Color, background: Color) -> Self {

        Self {
            foreground,
            background,
        }
    }
}

/// Fully resolved semantic colours. Palette factories supply every field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub struct ThemeVariantColors {
    pub surfaces: SurfaceColors,
    pub surfaces_alt: SurfaceColors,
    pub text: TextColors,
    /// For text on [`ThemeVariantColors::surfaces_alt`]
    pub text_alt: Option<TextColors>,
    pub primary: ActionColors,
    pub secondary: ActionColors,
    pub status: StatusColors,
    pub border: Color,
    pub border_muted: Color,
    pub focus: Color,
    pub syntax: SyntaxColors,
    pub chromatic: ChromaticColors,
}

impl ThemeVariantColors {
    /// For text on [`ThemeVariantColors::surfaces_alt`]
    ///
    /// Falls back to [`Self::text`] when [`Self::text_alt`] is not set.

    pub const fn resolved_text_alt(&self) -> &TextColors {

        if let Some(text_alt) = self.text_alt.as_ref() {

            text_alt
        } else {

            &self.text
        }
    }
}

/// Backgrounds and interaction colour for one set of surfaces.
/// Raised surfaces describe placement, not a guaranteed brightness ordering.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub struct SurfaceColors {
    pub background: Color,
    pub base: Color,
    pub raised: Color,
    pub overlay: Color,
    pub hover: Color,
}

/// Foreground colours for normal application text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub struct TextColors {
    pub normal: Color,
    pub muted: Color,
    pub subtle: Color,
}

/// Colours used to render an action and its interaction states.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub struct ActionColors {
    pub normal: ColorPair,
    pub hover: ColorPair,
    pub pressed: ColorPair,
    /// Reduced-emphasis colours for an action that remains interactive.
    pub muted: ColorPair,
    /// Colours for an action that cannot currently be interacted with.
    pub disabled: ColorPair,
}

/// Foreground or icon colours for statuses and diagnostic levels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub struct StatusColors {
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub critical: Color,
    pub info: Color,
    pub debug: Color,
    pub trace: Color,
}

/// Resolved colours used for syntax highlighting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub struct SyntaxColors {
    pub attribute: Color,
    pub boolean: Color,
    pub builtin: Color,
    pub builtin_function: Color,
    pub builtin_type: Color,
    pub comment: Color,
    pub constant: Color,
    pub control_keyword: Color,
    pub deleted: Color,
    pub deprecated: Color,
    pub documentation: Color,
    pub escape: Color,
    pub foreground: Color,
    pub function: Color,
    pub heading: Color,
    pub inserted: Color,
    pub invalid: Color,
    pub keyword: Color,
    pub link: Color,
    pub macro_name: Color,
    pub markup_bold: Color,
    pub markup_italic: Color,
    pub modifier: Color,
    pub namespace: Color,
    pub number: Color,
    pub operator: Color,
    pub parameter: Color,
    pub property: Color,
    pub punctuation: Color,
    pub string: Color,
    pub tag: Color,
    pub type_keyword: Color,
    pub type_name: Color,
    pub variable: Color,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]

pub enum ChromaticHue {
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Blue,
    Purple,
    Pink,
}

impl ChromaticHue {
    pub const ALL: [Self; 8] = [
        Self::Red,
        Self::Orange,
        Self::Yellow,
        Self::Green,
        Self::Cyan,
        Self::Blue,
        Self::Purple,
        Self::Pink,
    ];
}

/// Generic chromatic colors supplied by the palette.
///
/// These represent the palette's closest fitting version of each broad hue.
/// They are not semantic colors, and do not imply meaning such as error,
/// success, warning, or information.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub struct ChromaticColors {
    pub red: Color,
    pub orange: Color,
    pub yellow: Color,
    pub green: Color,
    pub cyan: Color,
    pub blue: Color,
    pub purple: Color,
    pub pink: Color,
}

impl ChromaticColors {
    pub const fn get(&self, hue: ChromaticHue) -> Color {

        match hue {
            ChromaticHue::Red => self.red,
            ChromaticHue::Orange => self.orange,
            ChromaticHue::Yellow => self.yellow,
            ChromaticHue::Green => self.green,
            ChromaticHue::Cyan => self.cyan,
            ChromaticHue::Blue => self.blue,
            ChromaticHue::Purple => self.purple,
            ChromaticHue::Pink => self.pink,
        }
    }
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
                support: appearance.into(),
                contrast: None,
            },
            colors,
            accent,
        })
    }

    pub fn from_palette(
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

    /// Declares which mode lists should include this variant. This is advisory.

    pub fn with_support(mut self, support: ThemeSupport) -> Self {

        self.metadata.support = support;

        self
    }

    /// Mode eligibility declared by the palette, independent of its appearance.

    pub const fn support(&self) -> ThemeSupport {

        self.metadata.support
    }

    /// Whether this variant belongs in the requested mode's filtered list.

    pub const fn supports(&self, mode: Appearance) -> bool {

        self.support().supports(mode)
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

    pub const fn primary(&self) -> &ActionColors {

        &self.colors.primary
    }

    pub const fn secondary(&self) -> &ActionColors {

        &self.colors.secondary
    }

    pub const fn status(&self) -> &StatusColors {

        &self.colors.status
    }

    pub const fn surfaces(&self) -> &SurfaceColors {

        &self.colors.surfaces
    }

    pub const fn surfaces_alt(&self) -> &SurfaceColors {

        &self.colors.surfaces_alt
    }

    pub const fn text(&self) -> &TextColors {

        &self.colors.text
    }

    pub const fn text_alt(&self) -> &TextColors {

        self.colors.resolved_text_alt()
    }

    pub const fn border(&self) -> Color {

        self.colors.border
    }

    pub const fn border_muted(&self) -> Color {

        self.colors.border_muted
    }

    pub const fn focus(&self) -> Color {

        self.colors.focus
    }

    pub const fn syntax(&self) -> &SyntaxColors {

        &self.colors.syntax
    }

    pub const fn chromatic(&self) -> &ChromaticColors {

        &self.colors.chromatic
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
