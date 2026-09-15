//! Shades of Purple palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod main;

pub use main::Main;

/// Pinned upstream palette resources.

pub(super) const SOURCE: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/ahmadawais/shades-of-purple-vscode",
    revision: "e8eb49f33e5db05ceba6677367b33ddb27ad821c",
    path: "themes/shades-of-purple-color-theme.json",
    licence: Some("MIT"),
}];
