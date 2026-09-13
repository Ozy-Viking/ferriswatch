//! Optional Dioxus theme configuration, application state and CSS integration.
//! The consuming application selects its Dioxus renderer.
#![doc = include_str!("../../docs/Dioxus.md")]
mod css;
mod theme_config;
mod theme_provider;

pub use css::theme_css;
pub use theme_config::{ThemeConfig, ThemeConfigBuilder, ThemeError, ThemeSelection};
pub use theme_provider::{ThemePicker, ThemeProvider, ThemeState, use_theme};
