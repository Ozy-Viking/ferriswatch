//! Synthwave '84 palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod main;

pub use main::Main;

/// Pinned upstream palette resources.
pub(super) const SOURCE: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/robb0wen/synthwave-vscode",
    revision: "ecfa2fe1279f7233663fa3f98a96e6756000567b",
    path: "themes/synthwave-color-theme.json",
    licence: Some("MIT"),
}];
