//! Theme identity and resolution.
//!
//! Use the catalogue instead of the former incomplete `ThemeName` enum.
pub use crate::catalogue::{ResolveError, get, resolve};
pub use crate::theme_variant::{Appearance, Contrast, ThemeMetadata, ThemeVariant};
