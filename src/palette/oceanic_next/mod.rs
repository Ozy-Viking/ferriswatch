//! Oceanic Next palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod main;

pub use main::Main;

/// Pinned upstream palette resources.

pub(super) const SOURCE: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/voronianski/oceanic-next-color-scheme",
    revision: "44a1eca851f86d4cc82f5c41734c917c65fa5710",
    path: "Oceanic Next.tmTheme",
    licence: None,
}];
