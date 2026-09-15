use ferriswatch::color::Color;
use ferriswatch::palette::catppuccin::mocha::{
    Blue, Flamingo, Green, Lavender, Maroon, Mauve, Mocha, Peach, Pink, Red, Rosewater, Sapphire,
    Sky, Teal, Yellow,
};
use ferriswatch::palette::{Accent, NoAccent};
use ferriswatch::theme_variant::{ThemePalette, ThemeVariant};

fn assert_mocha_chrome(theme: &ThemeVariant) {

    assert_eq!(theme.name(), "Catppuccin Mocha");

    assert_eq!(theme.primary().muted.background, Mocha::SURFACE_2);

    assert_eq!(theme.secondary().normal.background, Mocha::BLUE);

    assert_eq!(theme.status().success, Mocha::GREEN);

    assert_eq!(theme.status().warning, Mocha::YELLOW);

    assert_eq!(theme.status().error, Mocha::RED);

    assert_eq!(theme.status().info, Mocha::TEAL);

    assert_eq!(theme.surfaces().background, Mocha::BASE);

    assert_eq!(theme.surfaces().base, Mocha::SURFACE_0);

    assert_eq!(theme.surfaces().hover, Mocha::SURFACE_1);

    assert_eq!(theme.surfaces().overlay, Mocha::OVERLAY_0);

    assert_eq!(theme.text().normal, Mocha::TEXT);

    assert_eq!(theme.text().muted, Mocha::SUBTEXT_1);

    assert_eq!(theme.text().subtle, Mocha::OVERLAY_1);

    assert_eq!(theme.border(), Mocha::OVERLAY_0);

    assert_eq!(theme.border_muted(), Mocha::SURFACE_1);
}

fn assert_accent<A: Accent<Mocha>>(expected: Color, name: &str) {

    let theme = Mocha::variant::<A>();

    assert_eq!(theme.primary().normal.background, expected);

    assert_ne!(theme.primary().hover.background, expected);

    assert!(theme.primary().hover.background.r() < expected.r());

    assert!(theme.primary().hover.background.g() < expected.g());

    assert!(theme.primary().hover.background.b() < expected.b());

    assert_eq!(theme.primary().hover.background.a(), expected.a());

    assert_eq!(theme.focus(), expected);

    assert_eq!(theme.accent(), Some(expected));

    assert_eq!(theme.accent_name(), Some(name));

    assert_mocha_chrome(&theme);
}

#[test]

fn mocha_accents_drive_primary_hover_and_focus() {

    assert_accent::<Rosewater>(Mocha::ROSEWATER, "Rosewater");

    assert_accent::<Flamingo>(Mocha::FLAMINGO, "Flamingo");

    assert_accent::<Pink>(Mocha::PINK, "Pink");

    assert_accent::<Mauve>(Mocha::MAUVE, "Mauve");

    assert_accent::<Red>(Mocha::RED, "Red");

    assert_accent::<Maroon>(Mocha::MAROON, "Maroon");

    assert_accent::<Peach>(Mocha::PEACH, "Peach");

    assert_accent::<Yellow>(Mocha::YELLOW, "Yellow");

    assert_accent::<Green>(Mocha::GREEN, "Green");

    assert_accent::<Teal>(Mocha::TEAL, "Teal");

    assert_accent::<Sky>(Mocha::SKY, "Sky");

    assert_accent::<Sapphire>(Mocha::SAPPHIRE, "Sapphire");

    assert_accent::<Blue>(Mocha::BLUE, "Blue");

    assert_accent::<Lavender>(Mocha::LAVENDER, "Lavender");
}

#[test]

fn no_accent_falls_back_to_mauve_for_primary_hover_and_focus() {

    let theme = Mocha::variant::<NoAccent>();

    assert_eq!(theme.primary().normal.background, Mocha::MAUVE);

    assert_eq!(
        theme.primary().hover.background,
        Mocha::variant::<Mauve>().primary().hover.background
    );

    assert_eq!(theme.focus(), Mocha::MAUVE);

    assert_eq!(theme.accent(), None);

    assert_eq!(theme.accent_name(), None);

    assert_mocha_chrome(&theme);
}

#[test]

fn transparent_accent_is_preserved_without_defaulting() {

    struct Transparent;

    impl Accent<Mocha> for Transparent {
        const ACCENT: Option<Color> = Some(Color::TRANSPARENT);

        const ID: &'static str = "transparent";

        const NAME: &'static str = "Transparent";
    }

    let theme = Mocha::variant::<Transparent>();

    assert_eq!(theme.accent(), Some(Color::TRANSPARENT));

    assert_eq!(theme.accent_name(), Some("Transparent"));

    assert_eq!(theme.primary().normal.background, Color::TRANSPARENT);

    assert_eq!(theme.primary().hover.background, Color::TRANSPARENT);

    assert_eq!(theme.focus(), Color::TRANSPARENT);
}

#[test]

fn accent_choices_can_be_selected_at_runtime() {

    let variants = [
        Mocha::variant::<Mauve>(),
        Mocha::variant::<Blue>(),
        Mocha::variant::<NoAccent>(),
    ];

    let mut active = variants[0].clone();

    assert_eq!(active.accent(), Some(Mocha::MAUVE));

    assert_eq!(active.accent_name(), Some("Mauve"));

    active = variants[1].clone();

    assert_eq!(active.accent(), Some(Mocha::BLUE));

    assert_eq!(active.accent_name(), Some("Blue"));

    active = variants[2].clone();

    assert_eq!(active.accent(), None);

    assert_eq!(active.accent_name(), None);
}

#[test]

fn custom_palette_returns_the_same_runtime_type() {

    struct CustomPalette;

    impl ferriswatch::palette::Palette for CustomPalette {
        fn registration() -> &'static ferriswatch::catalogue::PaletteRegistration {

            &Mocha::REGISTRATION
        }
    }

    impl ThemePalette for CustomPalette {
        fn variant<A: Accent<Self>>() -> ThemeVariant {

            Mocha::variant::<NoAccent>()
        }
    }

    let variants = [
        Mocha::variant::<Mauve>(),
        CustomPalette::variant::<NoAccent>(),
    ];

    assert_eq!(variants[0].accent(), Some(Mocha::MAUVE));

    assert_eq!(variants[1].accent(), None);
}

#[test]

fn alternate_surfaces_and_statuses_are_independent_concrete_values() {

    let original = Mocha::variant::<Mauve>();

    let mut colors = *original.colors();

    colors.surfaces_alt = ferriswatch::theme_variant::SurfaceColors {
        background: Color::TRANSPARENT,
        base: Mocha::MANTLE,
        raised: Mocha::SURFACE_2,
        overlay: Mocha::CRUST,
        hover: Mocha::SURFACE_0,
    };

    colors.status.critical = Mocha::MAROON;

    colors.status.trace = Mocha::OVERLAY_2;

    let theme = ThemeVariant::new(
        "custom/test",
        "Custom",
        original.metadata().appearance,
        colors,
        None,
    )
    .unwrap();

    assert_eq!(theme.colors().surfaces, original.colors().surfaces);

    assert_eq!(theme.colors().surfaces_alt.background, Color::TRANSPARENT);

    assert_eq!(theme.colors().surfaces_alt.base, Mocha::MANTLE);

    assert_eq!(theme.colors().surfaces_alt.raised, Mocha::SURFACE_2);

    assert_eq!(theme.colors().surfaces_alt.overlay, Mocha::CRUST);

    assert_eq!(theme.colors().surfaces_alt.hover, Mocha::SURFACE_0);

    assert_eq!(theme.status().error, original.status().error);

    assert_eq!(theme.status().critical, Mocha::MAROON);

    assert_eq!(theme.status().trace, Mocha::OVERLAY_2);
}

#[test]

fn action_foregrounds_follow_explicit_accent_colours() {

    struct Black;

    impl Accent<Mocha> for Black {
        const ACCENT: Option<Color> = Some(Color::hex(0x000000));

        const ID: &'static str = "black";

        const NAME: &'static str = "Black";
    }

    struct White;

    impl Accent<Mocha> for White {
        const ACCENT: Option<Color> = Some(Color::hex(0xffffff));

        const ID: &'static str = "white";

        const NAME: &'static str = "White";
    }

    let dark = Mocha::variant::<Black>();

    let light = Mocha::variant::<White>();

    assert_eq!(
        dark.colors().primary.normal.foreground,
        Color::hex(0xffffff)
    );

    assert_eq!(
        light.colors().primary.normal.foreground,
        Color::hex(0x000000)
    );

    assert_eq!(dark.colors().surfaces, light.colors().surfaces);

    assert_eq!(dark.colors().status, light.colors().status);

    assert_eq!(
        dark.colors().secondary.normal.foreground,
        light.colors().secondary.normal.foreground
    );
}

#[test]

fn action_text_uses_linear_channels_without_decoding_srgb_twice() {

    struct MidGray;

    impl Accent<Mocha> for MidGray {
        const ACCENT: Option<Color> = Some(Color::hex(0x808080));

        const ID: &'static str = "mid_gray";

        const NAME: &'static str = "Mid Gray";
    }

    // sRGB 128 has linear luminance about 0.21586. With a 30% darker pressed
    // fill, black's worst contrast is about 4.022, versus white's 3.949.
    let theme = Mocha::variant::<MidGray>();

    assert_eq!(theme.colors().primary.normal.foreground, Color::hex(0));
}
