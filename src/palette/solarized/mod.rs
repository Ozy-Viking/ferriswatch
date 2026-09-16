//! Solarized palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod dark;
pub mod light;

pub use dark::Dark;
pub use light::Light;

/// Pinned upstream palette resources.

pub(super) const SOURCE_DARK: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/altercation/solarized",
        revision: "62f656a02f93c5190a8753159e34b385588d5ff3",
        path: "gimp-palette-solarized/solarized.gpl",
        licence: Some("MIT"),
    },
    PaletteSource {
        repository: "https://github.com/altercation/solarized",
        revision: "62f656a02f93c5190a8753159e34b385588d5ff3",
        path: "README.md",
        licence: Some("MIT"),
    },
    PaletteSource {
        repository: "https://github.com/altercation/solarized",
        revision: "62f656a02f93c5190a8753159e34b385588d5ff3",
        path: "vim-colors-solarized/colors/solarized.vim",
        licence: Some("MIT"),
    },
];

/// Pinned upstream palette resources.

pub(super) const SOURCE_LIGHT: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/altercation/solarized",
        revision: "62f656a02f93c5190a8753159e34b385588d5ff3",
        path: "gimp-palette-solarized/solarized.gpl",
        licence: Some("MIT"),
    },
    PaletteSource {
        repository: "https://github.com/altercation/solarized",
        revision: "62f656a02f93c5190a8753159e34b385588d5ff3",
        path: "README.md",
        licence: Some("MIT"),
    },
    PaletteSource {
        repository: "https://github.com/altercation/solarized",
        revision: "62f656a02f93c5190a8753159e34b385588d5ff3",
        path: "vim-colors-solarized/colors/solarized.vim",
        licence: Some("MIT"),
    },
];
