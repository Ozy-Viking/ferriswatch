//! Poimandres palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod main;

pub use main::Main;

/// Pinned upstream palette resources.

pub(super) const SOURCE: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/drcmda/poimandres-theme",
    revision: "fafa97959af95bd2cdbcff151c3e9c537d355f37",
    path: "themes/poimandres-color-theme.json",
    licence: Some("MIT"),
}];
