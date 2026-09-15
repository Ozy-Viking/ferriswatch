//! One palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod dark;
pub mod light;

pub use dark::Dark;
pub use light::Light;

/// Pinned upstream palette resources.

pub(super) const SOURCE_DARK: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/atom/one-dark-syntax",
    revision: "9c96f4454362267ac45322063e193ccf9d2debb1",
    path: "styles/colors.less",
    licence: Some("MIT"),
}];

/// Pinned upstream palette resources.

pub(super) const SOURCE_LIGHT: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/atom/one-light-syntax",
    revision: "d84579027410c576086dfca14d934c4bd74b0438",
    path: "styles/colors.less",
    licence: Some("MIT"),
}];
