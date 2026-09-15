//! GitHub palette variants and typed factories.

use crate::catalogue::PaletteSource;

pub mod dark;
pub mod light;

pub use dark::Dark;
pub use light::Light;

/// Pinned upstream palette resources.

pub(super) const SOURCE_DARK: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/primer/github-vscode-theme",
        revision: "cd78e5e4e7bcf132a6f428ae0f32264bb1b729cf",
        path: "src/theme.js",
        licence: Some("MIT"),
    },
    PaletteSource {
        repository: "https://github.com/primer/github-vscode-theme",
        revision: "cd78e5e4e7bcf132a6f428ae0f32264bb1b729cf",
        path: "src/colors.js",
        licence: Some("MIT"),
    },
    PaletteSource {
        repository: "https://github.com/primer/github-vscode-theme",
        revision: "cd78e5e4e7bcf132a6f428ae0f32264bb1b729cf",
        path: "package.json",
        licence: Some("MIT"),
    },
    PaletteSource {
        repository: "https://github.com/primer/primitives",
        revision: "f82864eb33c37f8624704bd996bc21b97d3c311b",
        path: "data/colors/themes/dark.ts",
        licence: Some("MIT"),
    },
    PaletteSource {
        repository: "https://github.com/primer/primitives",
        revision: "f82864eb33c37f8624704bd996bc21b97d3c311b",
        path: "data/colors/vars/global_dark.ts",
        licence: Some("MIT"),
    },
    PaletteSource {
        repository: "https://github.com/primer/primitives",
        revision: "f82864eb33c37f8624704bd996bc21b97d3c311b",
        path: "data/colors/vars/component_dark.ts",
        licence: Some("MIT"),
    },
];

/// Pinned upstream palette resources.

pub(super) const SOURCE_LIGHT: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/primer/github-vscode-theme",
        revision: "cd78e5e4e7bcf132a6f428ae0f32264bb1b729cf",
        path: "src/theme.js",
        licence: Some("MIT"),
    },
    PaletteSource {
        repository: "https://github.com/primer/github-vscode-theme",
        revision: "cd78e5e4e7bcf132a6f428ae0f32264bb1b729cf",
        path: "src/colors.js",
        licence: Some("MIT"),
    },
    PaletteSource {
        repository: "https://github.com/primer/github-vscode-theme",
        revision: "cd78e5e4e7bcf132a6f428ae0f32264bb1b729cf",
        path: "package.json",
        licence: Some("MIT"),
    },
    PaletteSource {
        repository: "https://github.com/primer/primitives",
        revision: "f82864eb33c37f8624704bd996bc21b97d3c311b",
        path: "data/colors/themes/light.ts",
        licence: Some("MIT"),
    },
    PaletteSource {
        repository: "https://github.com/primer/primitives",
        revision: "f82864eb33c37f8624704bd996bc21b97d3c311b",
        path: "data/colors/vars/global_light.ts",
        licence: Some("MIT"),
    },
    PaletteSource {
        repository: "https://github.com/primer/primitives",
        revision: "f82864eb33c37f8624704bd996bc21b97d3c311b",
        path: "data/colors/vars/component_light.ts",
        licence: Some("MIT"),
    },
];
