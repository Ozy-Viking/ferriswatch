//! Palettes sourced from <https://github.com/rebelot/kanagawa.nvim>.
//! Semantic roles and default accents are Ferriswatch mappings.

use crate::catalogue::PaletteSource;

pub mod wave;
pub use wave::Wave;
pub mod dragon;
pub use dragon::Dragon;
pub mod lotus;
pub use lotus::Lotus;

/// Pinned upstream palette resources.
pub(super) const SOURCE: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/rebelot/kanagawa.nvim",
        revision: "bb85e4bfc8d89b0e62c8fa53ccdd13d12e2f77b3",
        path: "lua/kanagawa/colors.lua",
        licence: Some("MIT"),
    },
    PaletteSource {
        repository: "https://github.com/rebelot/kanagawa.nvim",
        revision: "bb85e4bfc8d89b0e62c8fa53ccdd13d12e2f77b3",
        path: "lua/kanagawa/themes.lua",
        licence: Some("MIT"),
    },
];
