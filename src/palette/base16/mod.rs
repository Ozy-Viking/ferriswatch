//! Base16 palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod ocean;

pub use ocean::Ocean;

/// Pinned upstream palette resources.

pub(super) const SOURCE: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/tinted-theming/base16-schemes",
    revision: "2b6f2d0677216ddda50c9cabd6ee70fae4665f81",
    path: "ocean.yaml",
    licence: None,
}];
