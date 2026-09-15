//! Tomorrow palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod day;
pub mod night;

pub use day::Day;
pub use night::Night;

/// Pinned upstream palette resources.

pub(super) const SOURCE_DAY: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/chriskempson/tomorrow-theme",
    revision: "ccf6666d888198d341b26b3a99d0bc96500ad503",
    path: "textmate/Tomorrow.tmTheme",
    licence: Some("MIT"),
}];

/// Pinned upstream palette resources.

pub(super) const SOURCE_NIGHT: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/chriskempson/tomorrow-theme",
    revision: "ccf6666d888198d341b26b3a99d0bc96500ad503",
    path: "textmate/Tomorrow-Night.tmTheme",
    licence: Some("MIT"),
}];
