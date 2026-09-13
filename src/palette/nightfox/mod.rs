//! Nightfox palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod carbonfox;
pub mod dawnfox;
pub mod dayfox;
pub mod duskfox;
// Upstream names both the family and its original variant Nightfox.
#[allow(clippy::module_inception)]
pub mod nightfox;
pub mod nordfox;
pub mod terafox;

pub use carbonfox::Carbonfox;
pub use dawnfox::Dawnfox;
pub use dayfox::Dayfox;
pub use duskfox::Duskfox;
pub use nightfox::Nightfox;
pub use nordfox::Nordfox;
pub use terafox::Terafox;

/// Pinned upstream palette resources.
pub(super) const SOURCE_NIGHTFOX: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/EdenEast/nightfox.nvim",
    revision: "4dacd3f0185a2227bdf3b6c0975a8f0bf87cac9a",
    path: "lua/nightfox/palette/nightfox.lua",
    licence: Some("MIT"),
}];

/// Pinned upstream palette resources.
pub(super) const SOURCE_DAYFOX: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/EdenEast/nightfox.nvim",
    revision: "4dacd3f0185a2227bdf3b6c0975a8f0bf87cac9a",
    path: "lua/nightfox/palette/dayfox.lua",
    licence: Some("MIT"),
}];

/// Pinned upstream palette resources.
pub(super) const SOURCE_DAWNFOX: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/EdenEast/nightfox.nvim",
    revision: "4dacd3f0185a2227bdf3b6c0975a8f0bf87cac9a",
    path: "lua/nightfox/palette/dawnfox.lua",
    licence: Some("MIT"),
}];

/// Pinned upstream palette resources.
pub(super) const SOURCE_DUSKFOX: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/EdenEast/nightfox.nvim",
    revision: "4dacd3f0185a2227bdf3b6c0975a8f0bf87cac9a",
    path: "lua/nightfox/palette/duskfox.lua",
    licence: Some("MIT"),
}];

/// Pinned upstream palette resources.
pub(super) const SOURCE_NORDFOX: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/EdenEast/nightfox.nvim",
    revision: "4dacd3f0185a2227bdf3b6c0975a8f0bf87cac9a",
    path: "lua/nightfox/palette/nordfox.lua",
    licence: Some("MIT"),
}];

/// Pinned upstream palette resources.
pub(super) const SOURCE_TERAFOX: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/EdenEast/nightfox.nvim",
    revision: "4dacd3f0185a2227bdf3b6c0975a8f0bf87cac9a",
    path: "lua/nightfox/palette/terafox.lua",
    licence: Some("MIT"),
}];

/// Pinned upstream palette resources.
pub(super) const SOURCE_CARBONFOX: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/EdenEast/nightfox.nvim",
        revision: "4dacd3f0185a2227bdf3b6c0975a8f0bf87cac9a",
        path: "lua/nightfox/palette/carbonfox.lua",
        licence: Some("MIT"),
    },
    PaletteSource {
        repository: "https://github.com/EdenEast/nightfox.nvim",
        revision: "4dacd3f0185a2227bdf3b6c0975a8f0bf87cac9a",
        path: "lua/nightfox/lib/color.lua",
        licence: Some("MIT"),
    },
];
