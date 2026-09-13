//! JetBrains palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod darcula;
pub mod dark;

pub use darcula::Darcula;
pub use dark::Dark;

/// Pinned upstream palette resources.
pub(super) const SOURCE_DARCULA: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/JetBrains/intellij-community",
        revision: "9fc75e389cab8af1041095694549316c26e6df1f",
        path: "platform/platform-resources/src/DefaultColorSchemesManager.xml",
        licence: Some("JetBrains Open-Source Build Terms 1.3"),
    },
    PaletteSource {
        repository: "https://github.com/JetBrains/intellij-community",
        revision: "9fc75e389cab8af1041095694549316c26e6df1f",
        path: "platform/platform-resources/src/themes/darcula.theme.json",
        licence: Some("JetBrains Open-Source Build Terms 1.3"),
    },
];

/// Pinned upstream palette resources.
pub(super) const SOURCE_DARK: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/JetBrains/intellij-community",
        revision: "9fc75e389cab8af1041095694549316c26e6df1f",
        path: "platform/platform-resources/src/DefaultColorSchemesManager.xml",
        licence: Some("JetBrains Open-Source Build Terms 1.3"),
    },
    PaletteSource {
        repository: "https://github.com/JetBrains/intellij-community",
        revision: "9fc75e389cab8af1041095694549316c26e6df1f",
        path: "platform/platform-resources/src/themes/darcula.theme.json",
        licence: Some("JetBrains Open-Source Build Terms 1.3"),
    },
    PaletteSource {
        repository: "https://github.com/JetBrains/intellij-community",
        revision: "9fc75e389cab8af1041095694549316c26e6df1f",
        path: "platform/platform-resources/src/themes/expUI/expUI_darkScheme.xml",
        licence: Some("JetBrains Open-Source Build Terms 1.3"),
    },
    PaletteSource {
        repository: "https://github.com/JetBrains/intellij-community",
        revision: "9fc75e389cab8af1041095694549316c26e6df1f",
        path: "platform/platform-resources/src/themes/expUI/expUI_dark.theme.json",
        licence: Some("JetBrains Open-Source Build Terms 1.3"),
    },
];
