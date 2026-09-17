//! Cobalt2 palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod main;

pub use main::Main;

/// Pinned upstream palette resources.

pub(super) const SOURCE: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/wesbos/cobalt2-vscode",
    revision: "c4e9574372b85afad1682ed0fdd1ac0411c62512",
    path: "theme/cobalt2.json",
    licence: Some("MIT"),
}];
