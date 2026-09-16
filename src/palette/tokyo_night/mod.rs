//! Palettes sourced from <https://github.com/folke/tokyonight.nvim>.
//!
//! Raw colours preserve the pinned upstream palettes and resolved Day export.
//! Actions follow Tokyo Night's authored lualine on-accent pairings, while
//! syntax roles follow its TextMate/Sublime mappings where those correspond
//! directly to Ferriswatch's Syntect scopes.

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
        path: "lua/tokyonight/colors/day.lua",
        licence: Some("Apache-2.0"),
    },
    PaletteSource {
        repository: "https://github.com/folke/tokyonight.nvim",
        revision: "cdc07ac78467a233fd62c493de29a17e0cf2b2b6",
        path: "extras/lua/tokyonight_day.lua",
        licence: Some("Apache-2.0"),
    },
    PaletteSource {
        repository: "https://github.com/folke/tokyonight.nvim",
        revision: "cdc07ac78467a233fd62c493de29a17e0cf2b2b6",
        path: "lua/tokyonight/colors/init.lua",
        licence: Some("Apache-2.0"),
    },
    PaletteSource {
        repository: "https://github.com/folke/tokyonight.nvim",
        revision: "cdc07ac78467a233fd62c493de29a17e0cf2b2b6",
        path: "lua/tokyonight/util.lua",
        licence: Some("Apache-2.0"),
    },
    PaletteSource {
        repository: "https://github.com/folke/tokyonight.nvim",
        revision: "cdc07ac78467a233fd62c493de29a17e0cf2b2b6",
        path: "lua/tokyonight/groups/base.lua",
        licence: Some("Apache-2.0"),
    },
    PaletteSource {
        repository: "https://github.com/folke/tokyonight.nvim",
        revision: "cdc07ac78467a233fd62c493de29a17e0cf2b2b6",
        path: "lua/tokyonight/groups/treesitter.lua",
        licence: Some("Apache-2.0"),
    },
    PaletteSource {
        repository: "https://github.com/folke/tokyonight.nvim",
        revision: "cdc07ac78467a233fd62c493de29a17e0cf2b2b6",
        path: "lua/tokyonight/groups/notify.lua",
        licence: Some("Apache-2.0"),
    },
    PaletteSource {
        repository: "https://github.com/folke/tokyonight.nvim",
        revision: "cdc07ac78467a233fd62c493de29a17e0cf2b2b6",
        path: "lua/lualine/themes/_tokyonight.lua",
        licence: Some("Apache-2.0"),
    },
    PaletteSource {
        repository: "https://github.com/folke/tokyonight.nvim",
        revision: "cdc07ac78467a233fd62c493de29a17e0cf2b2b6",
        path: "lua/tokyonight/extra/sublime.lua",
        licence: Some("Apache-2.0"),
    },
];
