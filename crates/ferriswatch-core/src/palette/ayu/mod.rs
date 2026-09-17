//! Ayu palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod dark;
pub mod light;
pub mod mirage;

pub use dark::Dark;
pub use light::Light;
pub use mirage::Mirage;

/// Pinned upstream palette resources.

pub(super) const SOURCE_DARK: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/ayu-theme/ayu-colors",
    revision: "0f8a14da078dcafd62a47e60162a9f7722c55f49",
    path: "src/dark.ts",
    licence: Some("MIT"),
}];

/// Pinned upstream palette resources.

pub(super) const SOURCE_MIRAGE: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/ayu-theme/ayu-colors",
    revision: "0f8a14da078dcafd62a47e60162a9f7722c55f49",
    path: "src/mirage.ts",
    licence: Some("MIT"),
}];

/// Pinned upstream palette resources.

pub(super) const SOURCE_LIGHT: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/ayu-theme/ayu-colors",
    revision: "0f8a14da078dcafd62a47e60162a9f7722c55f49",
    path: "src/light.ts",
    licence: Some("MIT"),
}];
