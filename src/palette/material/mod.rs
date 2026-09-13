//! Material palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod lighter;
pub mod oceanic;
pub mod palenight;

pub use lighter::Lighter;
pub use oceanic::Oceanic;
pub use palenight::Palenight;

/// Pinned upstream palette resources.
pub(super) const SOURCE_OCEANIC: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/marko-cerovac/material.nvim",
    revision: "92f7366a9315cd386bc4fa4d716fde375dda210f",
    path: "lua/material/colors/init.lua",
    licence: Some("GNU GPL version 2"),
}];

/// Pinned upstream palette resources.
pub(super) const SOURCE_PALENIGHT: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/marko-cerovac/material.nvim",
    revision: "92f7366a9315cd386bc4fa4d716fde375dda210f",
    path: "lua/material/colors/init.lua",
    licence: Some("GNU GPL version 2"),
}];

/// Pinned upstream palette resources.
pub(super) const SOURCE_LIGHTER: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/marko-cerovac/material.nvim",
    revision: "92f7366a9315cd386bc4fa4d716fde375dda210f",
    path: "lua/material/colors/init.lua",
    licence: Some("GNU GPL version 2"),
}];
