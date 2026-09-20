//! Dioxus widgets for Ferriswatch that wrap dioxus-primitives.
//!
//! Theme state lives in [`ferriswatch::dioxus`]. This crate owns the combobox,
//! theme picker, and other widgets that need `dioxus-primitives` or
//! `dioxus-attributes`. [`components::ThemeCombobox`] ships no CSS; pass classes from the
//! host. [`components::ThemePicker`] supplies Ferriswatch's default classes.

mod accent_select;
mod combobox;
pub(crate) mod progressively_render;
mod theme_combobox;
mod theme_picker;
mod theme_select;
use ferriswatch_core as ferriswatch;

#[doc = include_str!("../docs/Components.md")]
pub mod components {
    pub use crate::accent_select::*;
    pub use crate::combobox::*;
    pub use crate::theme_combobox::*;
    pub use crate::theme_picker::*;
    pub use crate::theme_select::*;
}
