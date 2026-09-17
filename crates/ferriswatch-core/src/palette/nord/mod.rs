//! Nord palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod main;

pub use main::Main;

/// Pinned upstream palette resources.

pub(super) const SOURCE: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/nordtheme/nord",
    revision: "1cef71605416a222e57225b544540ce0fcec18d4",
    path: "src/nord.scss",
    licence: Some("MIT"),
}];
