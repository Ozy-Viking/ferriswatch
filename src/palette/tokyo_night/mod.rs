//! Palettes sourced from <https://github.com/folke/tokyonight.nvim>.
//! Semantic roles and default accents are Ferriswatch mappings.

use crate::catalogue::PaletteSource;

pub mod night;

pub use night::Night;

pub mod storm;

pub use storm::Storm;

pub mod moon;

pub use moon::Moon;

pub mod day;

pub use day::Day;

/// Pinned upstream palette resources.

pub(super) const SOURCE: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/folke/tokyonight.nvim",
        revision: "cdc07ac78467a233fd62c493de29a17e0cf2b2b6",
        path: "lua/tokyonight/colors/storm.lua",
        licence: Some("Apache-2.0"),
    },
    PaletteSource {
        repository: "https://github.com/folke/tokyonight.nvim",
        revision: "cdc07ac78467a233fd62c493de29a17e0cf2b2b6",
        path: "lua/tokyonight/colors/night.lua",
        licence: Some("Apache-2.0"),
    },
    PaletteSource {
        repository: "https://github.com/folke/tokyonight.nvim",
        revision: "cdc07ac78467a233fd62c493de29a17e0cf2b2b6",
        path: "lua/tokyonight/colors/moon.lua",
        licence: Some("Apache-2.0"),
    },
    PaletteSource {
        repository: "https://github.com/folke/tokyonight.nvim",
        revision: "cdc07ac78467a233fd62c493de29a17e0cf2b2b6",
        path: "extras/lua/tokyonight_day.lua",
        licence: Some("Apache-2.0"),
    },
];
