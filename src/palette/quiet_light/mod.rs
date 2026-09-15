//! Quiet Light palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod main;

pub use main::Main;

/// Pinned upstream palette resources.

pub(super) const SOURCE: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/microsoft/vscode",
    revision: "a8f49160195d9e967d2d51e8544dc895207518e7",
    path: "extensions/theme-quietlight/themes/quietlight-color-theme.json",
    licence: Some("MIT"),
}];
