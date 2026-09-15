//! Horizon palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod main;

pub use main::Main;

/// Pinned upstream palette resources.

pub(super) const SOURCE: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/jolaleye/horizon-theme-vscode",
    revision: "5ae91b6d49bf291e0a34c0a1cb277d9738aadd90",
    path: "themes/horizon.json",
    licence: Some("MIT"),
}];
