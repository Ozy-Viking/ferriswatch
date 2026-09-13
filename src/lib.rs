//! Shared raw palettes and fully resolved application themes.
//!
//! See [`catalogue`] for runtime discovery and [`palette`] for typed factories.

pub mod catalogue;
pub mod color;
#[cfg(feature = "dioxus")]
pub mod dioxus;
pub mod error;
pub mod palette;
pub mod theme;
pub mod theme_variant;
