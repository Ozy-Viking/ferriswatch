//! Night Owl palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod main;

pub use main::Main;

/// Pinned upstream palette resources.

pub(super) const SOURCE: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/sdras/night-owl-vscode-theme",
    revision: "cc291eba7976b20d7c66bde6883c27b902196b07",
    path: "themes/Night Owl-color-theme.json",
    licence: Some("MIT"),
}];
