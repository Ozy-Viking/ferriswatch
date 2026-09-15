//! Optional Dioxus theme configuration, application state and CSS integration.
//! The consuming application selects its Dioxus renderer.
#![doc = include_str!("../../docs/Dioxus.md")]

mod dx_theme;
pub mod persistence;
mod theme_config;
mod theme_provider;

pub use crate::css::{DEFAULT_CSS, DEFAULT_STYLESHEET, DefaultStyles, ThemeScope, theme_css};
pub use dioxus_sdk_storage::{LocalStorage, SessionStorage};
pub use persistence::{Memory, ThemeSnapshot, ThemeStorage};
pub use theme_config::{ThemeConfig, ThemeConfigBuilder, ThemeError, ThemeSelection};
pub use theme_provider::{ThemeProvider, ThemeState, use_theme};
