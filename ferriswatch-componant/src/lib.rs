//! Dioxus widgets for Ferriswatch that wrap dioxus-primitives.
//!
//! Theme state lives in [`ferriswatch::dioxus`]. This crate owns the combobox,
//! theme picker, and other widgets that need `dioxus-primitives` or
//! `dioxus-attributes`. [`ThemeCombobox`] ships no CSS; pass classes from the
//! host. [`ThemePicker`] supplies Ferriswatch's default classes.

pub mod combobox;
mod accent_select;
mod theme_select;
mod theme_combobox;
mod theme_picker;

pub use combobox::{Combobox, ComboboxEmpty, ComboboxOption, ComboboxProps};
pub use accent_select::AccentSelect;
pub use theme_select::ThemeSelect;
pub use theme_combobox::ThemeCombobox;
pub use theme_picker::ThemePicker;
