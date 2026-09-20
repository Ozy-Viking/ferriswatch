#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(auto_cfg))]

//! Shared raw palettes and fully resolved application themes.
//!
//! See [`catalogue`] for runtime discovery and [`palette`] for typed factories.

pub mod catalogue;
pub mod color;
pub mod css;
#[cfg(feature = "dioxus")]
pub mod dioxus;
pub mod error;
pub mod integrations;
pub mod palette;
#[cfg(feature = "parser")]
pub mod parser;
pub mod theme;
pub mod theme_variant;

#[doc(inline)]
pub use catalogue::PALETTES;
#[doc(inline)]
pub use color::Color;
#[doc(inline)]
pub use theme::Theme;
#[doc(inline)]
pub use theme_variant::ThemeVariant;
