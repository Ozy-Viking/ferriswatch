//! Regression anchors from the upstream palettes listed in docs/Palettes.md.

use ferriswatch::color::Color;
use ferriswatch::palette::{self, Accent, NoAccent, Palette};
use ferriswatch::theme_variant::ThemePalette;

struct Transparent;

impl<P: Palette> Accent<P> for Transparent {
    const ACCENT: Option<Color> = Some(Color::TRANSPARENT);

    const ID: &'static str = "transparent";

    const NAME: &'static str = "Transparent";
}

macro_rules! check_variant {
    ($test:ident, $family:ident, $module:ident, $ty:ident, $name:literal,
     $default:literal, $bg:literal, $fg:literal, $surface:literal,
     $success:literal, $warning:literal, $error:literal,
     [$( $accent:ident = $hex:literal ),+]) => {
        #[test]
        fn $test() {
            use palette::$family::$module::*;
            let default = $ty::variant::<NoAccent>();
            assert_eq!(default.name(), $name);
            assert_eq!(default.accent(), None);
            assert_eq!(default.accent_name(), None);
            assert_eq!(default.primary().normal.background, Color::hex($default));
            assert_eq!(default.focus(), default.primary().normal.background);
            $({
                let theme = $ty::variant::<$accent>();
                let colour = Color::hex($hex);
                let name = <$accent as Accent<$ty>>::NAME;
                assert_eq!(theme.name(), $name);
                assert_eq!(theme.accent_name(), Some(name));
                assert_eq!(theme.accent(), Some(colour));
                assert_eq!(theme.primary().normal.background, colour);
                assert_eq!(theme.focus(), colour);
                assert_ne!(theme.primary().hover.background, colour);
                assert_eq!(theme.primary().hover.background.a(), colour.a());
                assert!(theme.primary().hover.background.r() <= colour.r());
                assert!(theme.primary().hover.background.g() <= colour.g());
                assert!(theme.primary().hover.background.b() <= colour.b());
                assert_eq!(theme.surfaces().background, Color::hex($bg));
                assert_eq!(theme.text().normal, Color::hex($fg));
                assert_eq!(theme.surfaces().base, Color::hex($surface));
                assert_eq!(theme.status().success, Color::hex($success));
                assert_eq!(theme.status().warning, Color::hex($warning));
                assert_eq!(theme.status().error, Color::hex($error));
                assert_eq!(theme.surfaces().background, default.surfaces().background);
                assert_eq!(theme.text().normal, default.text().normal);
                assert_eq!(theme.border(), default.border());
            })+
            let transparent = $ty::variant::<Transparent>();
            assert_eq!(transparent.accent_name(), Some("Transparent"));
            assert_eq!(transparent.accent(), Some(Color::TRANSPARENT));
            assert_eq!(transparent.primary().normal.background, Color::TRANSPARENT);
            assert_eq!(transparent.primary().hover.background, Color::TRANSPARENT);
        }
    };
}

check_variant!(
    tokyo_night_night,
    tokyo_night,
    night,
    Night,
    "Tokyo Night",
    0x7aa2f7,
    0x1a1b26,
    0xc0caf5,
    0x16161e,
    0x9ece6a,
    0xe0af68,
    0xdb4b4b,
    [
        Blue = 0x7aa2f7,
        Cyan = 0x7dcfff,
        Green = 0x9ece6a,
        Magenta = 0xbb9af7,
        Orange = 0xff9e64,
        Purple = 0x9d7cd8,
        Red = 0xf7768e,
        Teal = 0x1abc9c,
        Yellow = 0xe0af68
    ]
);

check_variant!(
    tokyo_night_storm,
    tokyo_night,
    storm,
    Storm,
    "Tokyo Night Storm",
    0x7aa2f7,
    0x24283b,
    0xc0caf5,
    0x1f2335,
    0x9ece6a,
    0xe0af68,
    0xdb4b4b,
    [
        Blue = 0x7aa2f7,
        Cyan = 0x7dcfff,
        Green = 0x9ece6a,
        Magenta = 0xbb9af7,
        Orange = 0xff9e64,
        Purple = 0x9d7cd8,
        Red = 0xf7768e,
        Teal = 0x1abc9c,
        Yellow = 0xe0af68
    ]
);

check_variant!(
    tokyo_night_moon,
    tokyo_night,
    moon,
    Moon,
    "Tokyo Night Moon",
    0x82aaff,
    0x222436,
    0xc8d3f5,
    0x1e2030,
    0xc3e88d,
    0xffc777,
    0xc53b53,
    [
        Blue = 0x82aaff,
        Cyan = 0x86e1fc,
        Green = 0xc3e88d,
        Magenta = 0xc099ff,
        Orange = 0xff966c,
        Purple = 0xfca7ea,
        Red = 0xff757f,
        Teal = 0x4fd6be,
        Yellow = 0xffc777
    ]
);

check_variant!(
    tokyo_night_day,
    tokyo_night,
    day,
    Day,
    "Tokyo Night Day",
    0x2e7de9,
    0xe1e2e7,
    0x3760bf,
    0xd0d5e3,
    0x587539,
    0x8c6c3e,
    0xc64343,
    [
        Blue = 0x2e7de9,
        Cyan = 0x007197,
        Green = 0x587539,
        Magenta = 0x9854f1,
        Orange = 0xb15c00,
        Purple = 0x7847bd,
        Red = 0xf52a65,
        Teal = 0x118c74,
        Yellow = 0x8c6c3e
    ]
);

check_variant!(
    rose_pine_main,
    rose_pine,
    main,
    Main,
    "Rosé Pine",
    0xc4a7e7,
    0x191724,
    0xe0def4,
    0x1f1d2e,
    0x95b1ac,
    0xf6c177,
    0xeb6f92,
    [
        Love = 0xeb6f92,
        Gold = 0xf6c177,
        Rose = 0xebbcba,
        Pine = 0x31748f,
        Foam = 0x9ccfd8,
        Iris = 0xc4a7e7,
        Leaf = 0x95b1ac
    ]
);

check_variant!(
    rose_pine_moon,
    rose_pine,
    moon,
    Moon,
    "Rosé Pine Moon",
    0xc4a7e7,
    0x232136,
    0xe0def4,
    0x2a273f,
    0x95b1ac,
    0xf6c177,
    0xeb6f92,
    [
        Love = 0xeb6f92,
        Gold = 0xf6c177,
        Rose = 0xea9a97,
        Pine = 0x3e8fb0,
        Foam = 0x9ccfd8,
        Iris = 0xc4a7e7,
        Leaf = 0x95b1ac
    ]
);

check_variant!(
    rose_pine_dawn,
    rose_pine,
    dawn,
    Dawn,
    "Rosé Pine Dawn",
    0x907aa9,
    0xfaf4ed,
    0x464261,
    0xfffaf3,
    0x6d8f89,
    0xea9d34,
    0xb4637a,
    [
        Love = 0xb4637a,
        Gold = 0xea9d34,
        Rose = 0xd7827e,
        Pine = 0x286983,
        Foam = 0x56949f,
        Iris = 0x907aa9,
        Leaf = 0x6d8f89
    ]
);

check_variant!(
    gruvbox_dark_hard,
    gruvbox,
    dark_hard,
    DarkHard,
    "Gruvbox Dark Hard",
    0xfe8019,
    0x1d2021,
    0xebdbb2,
    0x3c3836,
    0xb8bb26,
    0xfabd2f,
    0xfb4934,
    [
        Red = 0xfb4934,
        Green = 0xb8bb26,
        Yellow = 0xfabd2f,
        Blue = 0x83a598,
        Purple = 0xd3869b,
        Aqua = 0x8ec07c,
        Orange = 0xfe8019
    ]
);

check_variant!(
    gruvbox_dark_medium,
    gruvbox,
    dark_medium,
    DarkMedium,
    "Gruvbox Dark Medium",
    0xfe8019,
    0x282828,
    0xebdbb2,
    0x3c3836,
    0xb8bb26,
    0xfabd2f,
    0xfb4934,
    [
        Red = 0xfb4934,
        Green = 0xb8bb26,
        Yellow = 0xfabd2f,
        Blue = 0x83a598,
        Purple = 0xd3869b,
        Aqua = 0x8ec07c,
        Orange = 0xfe8019
    ]
);

check_variant!(
    gruvbox_dark_soft,
    gruvbox,
    dark_soft,
    DarkSoft,
    "Gruvbox Dark Soft",
    0xfe8019,
    0x32302f,
    0xebdbb2,
    0x3c3836,
    0xb8bb26,
    0xfabd2f,
    0xfb4934,
    [
        Red = 0xfb4934,
        Green = 0xb8bb26,
        Yellow = 0xfabd2f,
        Blue = 0x83a598,
        Purple = 0xd3869b,
        Aqua = 0x8ec07c,
        Orange = 0xfe8019
    ]
);

check_variant!(
    gruvbox_light_hard,
    gruvbox,
    light_hard,
    LightHard,
    "Gruvbox Light Hard",
    0xaf3a03,
    0xf9f5d7,
    0x3c3836,
    0xebdbb2,
    0x79740e,
    0xb57614,
    0x9d0006,
    [
        Red = 0x9d0006,
        Green = 0x79740e,
        Yellow = 0xb57614,
        Blue = 0x076678,
        Purple = 0x8f3f71,
        Aqua = 0x427b58,
        Orange = 0xaf3a03
    ]
);

check_variant!(
    gruvbox_light_medium,
    gruvbox,
    light_medium,
    LightMedium,
    "Gruvbox Light Medium",
    0xaf3a03,
    0xfbf1c7,
    0x3c3836,
    0xebdbb2,
    0x79740e,
    0xb57614,
    0x9d0006,
    [
        Red = 0x9d0006,
        Green = 0x79740e,
        Yellow = 0xb57614,
        Blue = 0x076678,
        Purple = 0x8f3f71,
        Aqua = 0x427b58,
        Orange = 0xaf3a03
    ]
);

check_variant!(
    gruvbox_light_soft,
    gruvbox,
    light_soft,
    LightSoft,
    "Gruvbox Light Soft",
    0xaf3a03,
    0xf2e5bc,
    0x3c3836,
    0xebdbb2,
    0x79740e,
    0xb57614,
    0x9d0006,
    [
        Red = 0x9d0006,
        Green = 0x79740e,
        Yellow = 0xb57614,
        Blue = 0x076678,
        Purple = 0x8f3f71,
        Aqua = 0x427b58,
        Orange = 0xaf3a03
    ]
);

check_variant!(
    kanagawa_wave,
    kanagawa,
    wave,
    Wave,
    "Kanagawa Wave",
    0x7e9cd8,
    0x1f1f28,
    0xdcd7ba,
    0x2a2a37,
    0x98bb6c,
    0xff9e3b,
    0xe82424,
    [
        CrystalBlue = 0x7e9cd8,
        OniViolet = 0x957fb8,
        SpringGreen = 0x98bb6c,
        CarpYellow = 0xe6c384,
        SakuraPink = 0xd27e99,
        WaveRed = 0xe46876,
        SurimiOrange = 0xffa066,
        WaveAqua2 = 0x7aa89f
    ]
);

check_variant!(
    kanagawa_dragon,
    kanagawa,
    dragon,
    Dragon,
    "Kanagawa Dragon",
    0x8ba4b0,
    0x181616,
    0xc5c9c5,
    0x282727,
    0x98bb6c,
    0xff9e3b,
    0xe82424,
    [
        DragonBlue2 = 0x8ba4b0,
        DragonViolet = 0x8992a7,
        DragonGreen = 0x87a987,
        DragonYellow = 0xc4b28a,
        DragonPink = 0xa292a3,
        DragonRed = 0xc4746e,
        DragonOrange = 0xb6927b,
        DragonAqua = 0x8ea4a2
    ]
);

check_variant!(
    kanagawa_lotus,
    kanagawa,
    lotus,
    Lotus,
    "Kanagawa Lotus",
    0x4d699b,
    0xf2ecbc,
    0x545464,
    0xe7dba0,
    0x6f894e,
    0xe98a00,
    0xe82424,
    [
        LotusBlue4 = 0x4d699b,
        LotusViolet4 = 0x624c83,
        LotusGreen = 0x6f894e,
        LotusYellow = 0x77713f,
        LotusPink = 0xb35b79,
        LotusRed = 0xc84053,
        LotusOrange = 0xcc6d00,
        LotusAqua = 0x597b75
    ]
);

check_variant!(
    everforest_dark_hard,
    everforest,
    dark_hard,
    DarkHard,
    "Everforest Dark Hard",
    0xa7c080,
    0x272e33,
    0xd3c6aa,
    0x2e383c,
    0xa7c080,
    0xdbbc7f,
    0xe67e80,
    [
        Red = 0xe67e80,
        Orange = 0xe69875,
        Yellow = 0xdbbc7f,
        Green = 0xa7c080,
        Aqua = 0x83c092,
        Blue = 0x7fbbb3,
        Purple = 0xd699b6
    ]
);

check_variant!(
    everforest_light_hard,
    everforest,
    light_hard,
    LightHard,
    "Everforest Light Hard",
    0x8da101,
    0xfffbef,
    0x5c6a72,
    0xf8f5e4,
    0x8da101,
    0xdfa000,
    0xf85552,
    [
        Red = 0xf85552,
        Orange = 0xf57d26,
        Yellow = 0xdfa000,
        Green = 0x8da101,
        Aqua = 0x35a77c,
        Blue = 0x3a94c5,
        Purple = 0xdf69ba
    ]
);

check_variant!(
    everforest_dark_medium,
    everforest,
    dark_medium,
    DarkMedium,
    "Everforest Dark Medium",
    0xa7c080,
    0x2d353b,
    0xd3c6aa,
    0x343f44,
    0xa7c080,
    0xdbbc7f,
    0xe67e80,
    [
        Red = 0xe67e80,
        Orange = 0xe69875,
        Yellow = 0xdbbc7f,
        Green = 0xa7c080,
        Aqua = 0x83c092,
        Blue = 0x7fbbb3,
        Purple = 0xd699b6
    ]
);

check_variant!(
    everforest_light_medium,
    everforest,
    light_medium,
    LightMedium,
    "Everforest Light Medium",
    0x8da101,
    0xfdf6e3,
    0x5c6a72,
    0xf4f0d9,
    0x8da101,
    0xdfa000,
    0xf85552,
    [
        Red = 0xf85552,
        Orange = 0xf57d26,
        Yellow = 0xdfa000,
        Green = 0x8da101,
        Aqua = 0x35a77c,
        Blue = 0x3a94c5,
        Purple = 0xdf69ba
    ]
);

check_variant!(
    everforest_dark_soft,
    everforest,
    dark_soft,
    DarkSoft,
    "Everforest Dark Soft",
    0xa7c080,
    0x333c43,
    0xd3c6aa,
    0x3a464c,
    0xa7c080,
    0xdbbc7f,
    0xe67e80,
    [
        Red = 0xe67e80,
        Orange = 0xe69875,
        Yellow = 0xdbbc7f,
        Green = 0xa7c080,
        Aqua = 0x83c092,
        Blue = 0x7fbbb3,
        Purple = 0xd699b6
    ]
);

check_variant!(
    everforest_light_soft,
    everforest,
    light_soft,
    LightSoft,
    "Everforest Light Soft",
    0x8da101,
    0xf3ead3,
    0x5c6a72,
    0xeae4ca,
    0x8da101,
    0xdfa000,
    0xf85552,
    [
        Red = 0xf85552,
        Orange = 0xf57d26,
        Yellow = 0xdfa000,
        Green = 0x8da101,
        Aqua = 0x35a77c,
        Blue = 0x3a94c5,
        Purple = 0xdf69ba
    ]
);

#[test]

fn families_can_share_runtime_theme_selection() {

    let themes = [
        palette::tokyo_night::Night::variant::<NoAccent>(),
        palette::rose_pine::Dawn::variant::<NoAccent>(),
        palette::gruvbox::DarkMedium::variant::<NoAccent>(),
        palette::kanagawa::Dragon::variant::<NoAccent>(),
        palette::everforest::LightSoft::variant::<NoAccent>(),
        palette::catppuccin::Mocha::variant::<NoAccent>(),
    ];

    let names: Vec<_> = themes.iter().map(|theme| theme.name()).collect();

    assert_eq!(
        names,
        [
            "Tokyo Night",
            "Rosé Pine Dawn",
            "Gruvbox Dark Medium",
            "Kanagawa Dragon",
            "Everforest Light Soft",
            "Catppuccin Mocha"
        ]
    );
}
