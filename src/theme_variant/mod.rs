use std::marker::PhantomData;

use crate::{color::Color, pallet::Accent};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThemeVariant<A, P> {
    primary: Color,
    primary_hover: Color,
    primary_muted: Color,

    secondary: Color,

    success: Color,
    warning: Color,
    error: Color,
    info: Color,

    background: Color,
    surface: Color,
    surface_hover: Color,
    overlay: Color,

    text: Color,
    text_muted: Color,
    text_subtle: Color,

    border: Color,
    border_muted: Color,

    focus: Color,
    _accent: PhantomData<A>,
    _pallette: PhantomData<P>,
}

impl<A, P> ThemeVariant<A, P>
where
    A: Accent<P>,
{
    pub fn accent(&self) -> Color {
        A::ACCENT
    }
}

impl<A, P> ThemeVariant<A, P> {
    pub fn primary(&self) -> Color {
        self.primary
    }

    pub fn primary_hover(&self) -> Color {
        self.primary_hover
    }

    pub fn primary_muted(&self) -> Color {
        self.primary_muted
    }

    pub fn secondary(&self) -> Color {
        self.secondary
    }

    pub fn success(&self) -> Color {
        self.success
    }

    pub fn warning(&self) -> Color {
        self.warning
    }

    pub fn error(&self) -> Color {
        self.error
    }

    pub fn info(&self) -> Color {
        self.info
    }

    pub fn background(&self) -> Color {
        self.background
    }

    pub fn surface(&self) -> Color {
        self.surface
    }

    pub fn surface_hover(&self) -> Color {
        self.surface_hover
    }

    pub fn overlay(&self) -> Color {
        self.overlay
    }

    pub fn text(&self) -> Color {
        self.text
    }

    pub fn text_muted(&self) -> Color {
        self.text_muted
    }

    pub fn text_subtle(&self) -> Color {
        self.text_subtle
    }

    pub fn border(&self) -> Color {
        self.border
    }

    pub fn border_muted(&self) -> Color {
        self.border_muted
    }

    pub fn focus(&self) -> Color {
        self.focus
    }
}

pub trait ThemePalette {
    fn variant<A, P>() -> ThemeVariant<A, P>
    where
        A: Accent<Self>,
        Self: Sized;
}
