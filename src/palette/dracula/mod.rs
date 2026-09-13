//! Dracula palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod main;

pub use main::Main;

/// Pinned upstream palette resources.
pub(super) const SOURCE: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/dracula/visual-studio-code",
    revision: "1b9ecf4d7e0c8cc2e2e890a7a41ad1db5fff1e6c",
    path: "src/dracula.yml",
    licence: Some("MIT"),
}];
