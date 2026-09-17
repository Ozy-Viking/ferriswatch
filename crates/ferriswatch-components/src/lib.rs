//! Dioxus widgets for Ferriswatch that wrap dioxus-primitives.
//!
//! Theme state lives in [`ferriswatch::dioxus`]. This crate owns the combobox,
//! theme picker, and other widgets that need `dioxus-primitives` or
//! `dioxus-attributes`. [`ThemeCombobox`] ships no CSS; pass classes from the
//! host. [`ThemePicker`] supplies Ferriswatch's default classes.

mod accent_select;
pub mod combobox;
pub(crate) mod progressively_render;
mod theme_combobox;
mod theme_picker;
mod theme_select;

pub use accent_select::AccentSelect;
pub use combobox::{Combobox, ComboboxEmpty, ComboboxOption, ComboboxProps};
pub use theme_combobox::ThemeCombobox;
pub use theme_picker::ThemePicker;
pub use theme_select::ThemeSelect;

use ferriswatch_core as ferriswatch;
