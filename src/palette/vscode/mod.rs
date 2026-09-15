//! VS Code palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod dark_plus;
pub mod light_plus;

pub use dark_plus::DarkPlus;
pub use light_plus::LightPlus;

/// Pinned upstream palette resources.

pub(super) const SOURCE_DARK_PLUS: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/microsoft/vscode",
        revision: "a8f49160195d9e967d2d51e8544dc895207518e7",
        path: "extensions/theme-defaults/themes/dark_vs.json",
        licence: Some("MIT"),
    },
    PaletteSource {
        repository: "https://github.com/microsoft/vscode",
        revision: "a8f49160195d9e967d2d51e8544dc895207518e7",
        path: "extensions/theme-defaults/themes/dark_plus.json",
        licence: Some("MIT"),
    },
];

/// Pinned upstream palette resources.

pub(super) const SOURCE_LIGHT_PLUS: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/microsoft/vscode",
        revision: "a8f49160195d9e967d2d51e8544dc895207518e7",
        path: "extensions/theme-defaults/themes/light_vs.json",
        licence: Some("MIT"),
    },
    PaletteSource {
        repository: "https://github.com/microsoft/vscode",
        revision: "a8f49160195d9e967d2d51e8544dc895207518e7",
        path: "extensions/theme-defaults/themes/light_plus.json",
        licence: Some("MIT"),
    },
];
