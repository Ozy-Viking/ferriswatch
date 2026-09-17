//! Panda palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod main;

pub use main::Main;

/// Pinned upstream palette resources.

pub(super) const SOURCE: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/PandaTheme/panda-syntax-brackets",
    revision: "3a117beab16c1326ee543335fdd4f4081ea2a75b",
    path: "theme.less",
    licence: Some("MIT"),
}];
