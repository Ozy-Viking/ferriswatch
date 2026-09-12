//! Pinned upstream palette resources.
use crate::catalogue::PaletteSource;

pub(super) const CATPPUCCIN: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/catppuccin/palette",
    revision: "07d02aa110ef9eb7e7427afca5c73ba9cf7f8ebd",
    path: "palette.json",
}];

pub(super) const TOKYO_NIGHT: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/folke/tokyonight.nvim",
        revision: "cdc07ac78467a233fd62c493de29a17e0cf2b2b6",
        path: "lua/tokyonight/colors/storm.lua",
    },
    PaletteSource {
        repository: "https://github.com/folke/tokyonight.nvim",
        revision: "cdc07ac78467a233fd62c493de29a17e0cf2b2b6",
        path: "lua/tokyonight/colors/night.lua",
    },
    PaletteSource {
        repository: "https://github.com/folke/tokyonight.nvim",
        revision: "cdc07ac78467a233fd62c493de29a17e0cf2b2b6",
        path: "lua/tokyonight/colors/moon.lua",
    },
    PaletteSource {
        repository: "https://github.com/folke/tokyonight.nvim",
        revision: "cdc07ac78467a233fd62c493de29a17e0cf2b2b6",
        path: "extras/lua/tokyonight_day.lua",
    },
];

pub(super) const ROSE_PINE: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/rose-pine/neovim",
    revision: "ff483051a47e27d84bdef47703538df1ed9f4a47",
    path: "lua/rose-pine/palette.lua",
}];

pub(super) const GRUVBOX: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/morhetz/gruvbox",
    revision: "5d15b2765f59754d7ac263c88a0f6e3e58124951",
    path: "colors/gruvbox.vim",
}];

pub(super) const EVERFOREST: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/sainnhe/everforest",
    revision: "85a86eb62409e3ec88713bff3d1b9d7374e112e4",
    path: "autoload/everforest.vim",
}];

pub(super) const KANAGAWA: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/rebelot/kanagawa.nvim",
        revision: "bb85e4bfc8d89b0e62c8fa53ccdd13d12e2f77b3",
        path: "lua/kanagawa/colors.lua",
    },
    PaletteSource {
        repository: "https://github.com/rebelot/kanagawa.nvim",
        revision: "bb85e4bfc8d89b0e62c8fa53ccdd13d12e2f77b3",
        path: "lua/kanagawa/themes.lua",
    },
];

pub(super) const SOLARIZED_DARK: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/altercation/solarized",
    revision: "62f656a02f93c5190a8753159e34b385588d5ff3",
    path: "gimp-palette-solarized/solarized.gpl",
}];

pub(super) const SOLARIZED_LIGHT: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/altercation/solarized",
    revision: "62f656a02f93c5190a8753159e34b385588d5ff3",
    path: "gimp-palette-solarized/solarized.gpl",
}];

pub(super) const NORD_MAIN: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/nordtheme/nord",
    revision: "1cef71605416a222e57225b544540ce0fcec18d4",
    path: "src/nord.scss",
}];

pub(super) const BASE_16_OCEAN: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/tinted-theming/base16-schemes",
    revision: "2b6f2d0677216ddda50c9cabd6ee70fae4665f81",
    path: "ocean.yaml",
}];

pub(super) const ONE_DARK: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/atom/one-dark-syntax",
    revision: "9c96f4454362267ac45322063e193ccf9d2debb1",
    path: "styles/colors.less",
}];

pub(super) const ONE_LIGHT: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/atom/one-light-syntax",
    revision: "d84579027410c576086dfca14d934c4bd74b0438",
    path: "styles/colors.less",
}];

pub(super) const AYU_DARK: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/ayu-theme/ayu-colors",
    revision: "0f8a14da078dcafd62a47e60162a9f7722c55f49",
    path: "src/dark.ts",
}];

pub(super) const AYU_MIRAGE: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/ayu-theme/ayu-colors",
    revision: "0f8a14da078dcafd62a47e60162a9f7722c55f49",
    path: "src/mirage.ts",
}];

pub(super) const AYU_LIGHT: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/ayu-theme/ayu-colors",
    revision: "0f8a14da078dcafd62a47e60162a9f7722c55f49",
    path: "src/light.ts",
}];

pub(super) const NIGHTFOX_NIGHTFOX: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/EdenEast/nightfox.nvim",
    revision: "4dacd3f0185a2227bdf3b6c0975a8f0bf87cac9a",
    path: "lua/nightfox/palette/nightfox.lua",
}];

pub(super) const NIGHTFOX_DAYFOX: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/EdenEast/nightfox.nvim",
    revision: "4dacd3f0185a2227bdf3b6c0975a8f0bf87cac9a",
    path: "lua/nightfox/palette/dayfox.lua",
}];

pub(super) const NIGHTFOX_DAWNFOX: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/EdenEast/nightfox.nvim",
    revision: "4dacd3f0185a2227bdf3b6c0975a8f0bf87cac9a",
    path: "lua/nightfox/palette/dawnfox.lua",
}];

pub(super) const NIGHTFOX_DUSKFOX: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/EdenEast/nightfox.nvim",
    revision: "4dacd3f0185a2227bdf3b6c0975a8f0bf87cac9a",
    path: "lua/nightfox/palette/duskfox.lua",
}];

pub(super) const NIGHTFOX_NORDFOX: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/EdenEast/nightfox.nvim",
    revision: "4dacd3f0185a2227bdf3b6c0975a8f0bf87cac9a",
    path: "lua/nightfox/palette/nordfox.lua",
}];

pub(super) const NIGHTFOX_TERAFOX: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/EdenEast/nightfox.nvim",
    revision: "4dacd3f0185a2227bdf3b6c0975a8f0bf87cac9a",
    path: "lua/nightfox/palette/terafox.lua",
}];

pub(super) const NIGHTFOX_CARBONFOX: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/EdenEast/nightfox.nvim",
        revision: "4dacd3f0185a2227bdf3b6c0975a8f0bf87cac9a",
        path: "lua/nightfox/palette/carbonfox.lua",
    },
    PaletteSource {
        repository: "https://github.com/EdenEast/nightfox.nvim",
        revision: "4dacd3f0185a2227bdf3b6c0975a8f0bf87cac9a",
        path: "lua/nightfox/lib/color.lua",
    },
];

pub(super) const MATERIAL_OCEANIC: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/marko-cerovac/material.nvim",
    revision: "92f7366a9315cd386bc4fa4d716fde375dda210f",
    path: "lua/material/colors/init.lua",
}];

pub(super) const MATERIAL_PALENIGHT: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/marko-cerovac/material.nvim",
    revision: "92f7366a9315cd386bc4fa4d716fde375dda210f",
    path: "lua/material/colors/init.lua",
}];

pub(super) const MATERIAL_LIGHTER: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/marko-cerovac/material.nvim",
    revision: "92f7366a9315cd386bc4fa4d716fde375dda210f",
    path: "lua/material/colors/init.lua",
}];

pub(super) const PANDA_MAIN: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/PandaTheme/Panda-Theme",
    revision: "3a117beab16c1326ee543335fdd4f4081ea2a75b",
    path: "theme.less",
}];

pub(super) const DRACULA_MAIN: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/dracula/visual-studio-code",
    revision: "1b9ecf4d7e0c8cc2e2e890a7a41ad1db5fff1e6c",
    path: "src/dracula.yml",
}];

pub(super) const TOMORROW_DAY: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/chriskempson/tomorrow-theme",
    revision: "ccf6666d888198d341b26b3a99d0bc96500ad503",
    path: "textmate/Tomorrow.tmTheme",
}];

pub(super) const TOMORROW_NIGHT: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/chriskempson/tomorrow-theme",
    revision: "ccf6666d888198d341b26b3a99d0bc96500ad503",
    path: "textmate/Tomorrow-Night.tmTheme",
}];

pub(super) const OCEANIC_NEXT_MAIN: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/voronianski/oceanic-next-color-scheme",
    revision: "44a1eca851f86d4cc82f5c41734c917c65fa5710",
    path: "Oceanic Next.tmTheme",
}];

pub(super) const NIGHT_OWL_MAIN: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/sdras/night-owl-vscode-theme",
    revision: "cc291eba7976b20d7c66bde6883c27b902196b07",
    path: "themes/Night Owl-color-theme.json",
}];

pub(super) const POIMANDRES_MAIN: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/drcmda/poimandres-theme",
    revision: "fafa97959af95bd2cdbcff151c3e9c537d355f37",
    path: "themes/poimandres-color-theme.json",
}];

pub(super) const SYNTHWAVE_84_MAIN: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/robb0wen/synthwave-vscode",
    revision: "ecfa2fe1279f7233663fa3f98a96e6756000567b",
    path: "themes/synthwave-color-theme.json",
}];

pub(super) const COBALT_2_MAIN: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/wesbos/cobalt2-vscode",
    revision: "c4e9574372b85afad1682ed0fdd1ac0411c62512",
    path: "theme/cobalt2.json",
}];

pub(super) const SHADES_OF_PURPLE_MAIN: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/ahmadawais/shades-of-purple-vscode",
    revision: "e8eb49f33e5db05ceba6677367b33ddb27ad821c",
    path: "themes/shades-of-purple-color-theme.json",
}];

pub(super) const HORIZON_MAIN: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/jolaleye/horizon-theme-vscode",
    revision: "5ae91b6d49bf291e0a34c0a1cb277d9738aadd90",
    path: "themes/horizon.json",
}];

pub(super) const MONOKAI_MAIN: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/microsoft/vscode",
    revision: "a8f49160195d9e967d2d51e8544dc895207518e7",
    path: "extensions/theme-monokai/themes/monokai-color-theme.json",
}];

pub(super) const QUIET_LIGHT_MAIN: &[PaletteSource] = &[PaletteSource {
    repository: "https://github.com/microsoft/vscode",
    revision: "a8f49160195d9e967d2d51e8544dc895207518e7",
    path: "extensions/theme-quietlight/themes/quietlight-color-theme.json",
}];

pub(super) const VSCODE_DARK_PLUS: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/microsoft/vscode",
        revision: "a8f49160195d9e967d2d51e8544dc895207518e7",
        path: "extensions/theme-defaults/themes/dark_vs.json",
    },
    PaletteSource {
        repository: "https://github.com/microsoft/vscode",
        revision: "a8f49160195d9e967d2d51e8544dc895207518e7",
        path: "extensions/theme-defaults/themes/dark_plus.json",
    },
];

pub(super) const VSCODE_LIGHT_PLUS: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/microsoft/vscode",
        revision: "a8f49160195d9e967d2d51e8544dc895207518e7",
        path: "extensions/theme-defaults/themes/light_vs.json",
    },
    PaletteSource {
        repository: "https://github.com/microsoft/vscode",
        revision: "a8f49160195d9e967d2d51e8544dc895207518e7",
        path: "extensions/theme-defaults/themes/light_plus.json",
    },
];

pub(super) const GITHUB_DARK: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/primer/github-vscode-theme",
        revision: "cd78e5e4e7bcf132a6f428ae0f32264bb1b729cf",
        path: "src/theme.js",
    },
    PaletteSource {
        repository: "https://github.com/primer/github-vscode-theme",
        revision: "cd78e5e4e7bcf132a6f428ae0f32264bb1b729cf",
        path: "src/colors.js",
    },
    PaletteSource {
        repository: "https://github.com/primer/github-vscode-theme",
        revision: "cd78e5e4e7bcf132a6f428ae0f32264bb1b729cf",
        path: "package.json",
    },
    PaletteSource {
        repository: "https://github.com/primer/primitives",
        revision: "f82864eb33c37f8624704bd996bc21b97d3c311b",
        path: "data/colors/themes/dark.ts",
    },
    PaletteSource {
        repository: "https://github.com/primer/primitives",
        revision: "f82864eb33c37f8624704bd996bc21b97d3c311b",
        path: "data/colors/vars/global_dark.ts",
    },
    PaletteSource {
        repository: "https://github.com/primer/primitives",
        revision: "f82864eb33c37f8624704bd996bc21b97d3c311b",
        path: "data/colors/vars/component_dark.ts",
    },
];

pub(super) const GITHUB_LIGHT: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/primer/github-vscode-theme",
        revision: "cd78e5e4e7bcf132a6f428ae0f32264bb1b729cf",
        path: "src/theme.js",
    },
    PaletteSource {
        repository: "https://github.com/primer/github-vscode-theme",
        revision: "cd78e5e4e7bcf132a6f428ae0f32264bb1b729cf",
        path: "src/colors.js",
    },
    PaletteSource {
        repository: "https://github.com/primer/github-vscode-theme",
        revision: "cd78e5e4e7bcf132a6f428ae0f32264bb1b729cf",
        path: "package.json",
    },
    PaletteSource {
        repository: "https://github.com/primer/primitives",
        revision: "f82864eb33c37f8624704bd996bc21b97d3c311b",
        path: "data/colors/themes/light.ts",
    },
    PaletteSource {
        repository: "https://github.com/primer/primitives",
        revision: "f82864eb33c37f8624704bd996bc21b97d3c311b",
        path: "data/colors/vars/global_light.ts",
    },
    PaletteSource {
        repository: "https://github.com/primer/primitives",
        revision: "f82864eb33c37f8624704bd996bc21b97d3c311b",
        path: "data/colors/vars/component_light.ts",
    },
];

pub(super) const JETBRAINS_DARCULA: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/JetBrains/intellij-community",
        revision: "9fc75e389cab8af1041095694549316c26e6df1f",
        path: "platform/platform-resources/src/DefaultColorSchemesManager.xml",
    },
    PaletteSource {
        repository: "https://github.com/JetBrains/intellij-community",
        revision: "9fc75e389cab8af1041095694549316c26e6df1f",
        path: "platform/platform-resources/src/themes/darcula.theme.json",
    },
];

pub(super) const JETBRAINS_DARK: &[PaletteSource] = &[
    PaletteSource {
        repository: "https://github.com/JetBrains/intellij-community",
        revision: "9fc75e389cab8af1041095694549316c26e6df1f",
        path: "platform/platform-resources/src/DefaultColorSchemesManager.xml",
    },
    PaletteSource {
        repository: "https://github.com/JetBrains/intellij-community",
        revision: "9fc75e389cab8af1041095694549316c26e6df1f",
        path: "platform/platform-resources/src/themes/darcula.theme.json",
    },
    PaletteSource {
        repository: "https://github.com/JetBrains/intellij-community",
        revision: "9fc75e389cab8af1041095694549316c26e6df1f",
        path: "platform/platform-resources/src/themes/expUI/expUI_darkScheme.xml",
    },
    PaletteSource {
        repository: "https://github.com/JetBrains/intellij-community",
        revision: "9fc75e389cab8af1041095694549316c26e6df1f",
        path: "platform/platform-resources/src/themes/expUI/expUI_dark.theme.json",
    },
];
