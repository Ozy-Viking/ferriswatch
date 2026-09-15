//! Palettes sourced from <https://github.com/rose-pine/neovim>.
//! Semantic roles and default accents are Ferriswatch mappings.

use crate::catalogue::PaletteSource;

pub mod main;

pub use main::Main;

pub mod moon;

pub use moon::Moon;

pub mod dawn;

pub use dawn::Dawn;

/// Pinned upstream palette resources.

pub(super) const SOURCE: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/rose-pine/neovim",
    revision: "ff483051a47e27d84bdef47703538df1ed9f4a47",
    path: "lua/rose-pine/palette.lua",
    licence: Some("MIT"),
}];
