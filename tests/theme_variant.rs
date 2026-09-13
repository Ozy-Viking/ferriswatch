use ferriswatch::color::Color;
use ferriswatch::palette::catppuccin::mocha::{
    Blue, Flamingo, Green, Lavender, Maroon, Mauve, Mocha, Peach, Pink, Red, Rosewater, Sapphire,
    Sky, Teal, Yellow,
};
use ferriswatch::palette::{Accent, NoAccent};
use ferriswatch::theme_variant::{ThemePalette, ThemeVariant};

fn assert_mocha_chrome(theme: &ThemeVariant) {
    assert_eq!(theme.name(), "Catppuccin Mocha");
    assert_eq!(theme.primary_muted(), Mocha::SURFACE_2);
    assert_eq!(theme.secondary(), Mocha::BLUE);
    assert_eq!(theme.success(), Mocha::GREEN);
    assert_eq!(theme.warning(), Mocha::YELLOW);
    assert_eq!(theme.error(), Mocha::RED);
    assert_eq!(theme.info(), Mocha::TEAL);
    assert_eq!(theme.background(), Mocha::BASE);
    assert_eq!(theme.surface(), Mocha::SURFACE_0);
    assert_eq!(theme.surface_hover(), Mocha::SURFACE_1);
    assert_eq!(theme.overlay(), Mocha::OVERLAY_0);
    assert_eq!(theme.text(), Mocha::TEXT);
    assert_eq!(theme.text_muted(), Mocha::SUBTEXT_1);
    assert_eq!(theme.text_subtle(), Mocha::OVERLAY_1);
    assert_eq!(theme.border(), Mocha::OVERLAY_0);
    assert_eq!(theme.border_muted(), Mocha::SURFACE_1);
}

fn assert_accent<A: Accent<Mocha>>(expected: Color, name: &str) {
    let theme = Mocha::variant::<A>();
    assert_eq!(theme.primary(), expected);
    assert_ne!(theme.primary_hover(), expected);
    assert!(theme.primary_hover().r() < expected.r());
    assert!(theme.primary_hover().g() < expected.g());
    assert!(theme.primary_hover().b() < expected.b());
    assert_eq!(theme.primary_hover().a(), expected.a());
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
    assert_eq!(theme.primary(), Mocha::MAUVE);
    assert_eq!(
        theme.primary_hover(),
        Mocha::variant::<Mauve>().primary_hover()
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
        const ID: Option<&'static str> = Some("transparent");
        const NAME: Option<&'static str> = Some("Transparent");
    }
    let theme = Mocha::variant::<Transparent>();
    assert_eq!(theme.accent(), Some(Color::TRANSPARENT));
    assert_eq!(theme.accent_name(), Some("Transparent"));
    assert_eq!(theme.primary(), Color::TRANSPARENT);
    assert_eq!(theme.primary_hover(), Color::TRANSPARENT);
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
    colors.surface_alt = ferriswatch::theme_variant::SurfaceColors {
        background: Color::TRANSPARENT,
        surface: Mocha::MANTLE,
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
    assert_eq!(theme.colors().surface, original.colors().surface);
    assert_eq!(theme.colors().surface_alt.background, Color::TRANSPARENT);
    assert_eq!(theme.colors().surface_alt.surface, Mocha::MANTLE);
    assert_eq!(theme.colors().surface_alt.raised, Mocha::SURFACE_2);
    assert_eq!(theme.colors().surface_alt.overlay, Mocha::CRUST);
    assert_eq!(theme.colors().surface_alt.hover, Mocha::SURFACE_0);
    assert_eq!(theme.error(), original.error());
    assert_eq!(theme.critical(), Mocha::MAROON);
    assert_eq!(theme.trace(), Mocha::OVERLAY_2);
}

#[test]
fn action_foregrounds_follow_explicit_accent_colours() {
    struct Black;
    impl Accent<Mocha> for Black {
        const ACCENT: Option<Color> = Some(Color::hex(0x000000));
        const ID: Option<&'static str> = Some("black");
        const NAME: Option<&'static str> = Some("Black");
    }
    struct White;
    impl Accent<Mocha> for White {
        const ACCENT: Option<Color> = Some(Color::hex(0xffffff));
        const ID: Option<&'static str> = Some("white");
        const NAME: Option<&'static str> = Some("White");
    }
    let dark = Mocha::variant::<Black>();
    let light = Mocha::variant::<White>();
    assert_eq!(dark.colors().text.on_primary, Color::hex(0xffffff));
    assert_eq!(light.colors().text.on_primary, Color::hex(0x000000));
    assert_eq!(dark.colors().surface, light.colors().surface);
    assert_eq!(dark.colors().status, light.colors().status);
    assert_eq!(
        dark.colors().text.on_secondary,
        light.colors().text.on_secondary
    );
}

#[test]
fn action_text_uses_linear_channels_without_decoding_srgb_twice() {
    struct MidGray;
    impl Accent<Mocha> for MidGray {
        const ACCENT: Option<Color> = Some(Color::hex(0x808080));
        const ID: Option<&'static str> = Some("mid_gray");
        const NAME: Option<&'static str> = Some("Mid Gray");
    }
    // sRGB 128 has linear luminance about 0.21586. With a 30% darker pressed
    // fill, black's worst contrast is about 4.022, versus white's 3.949.
    let theme = Mocha::variant::<MidGray>();
    assert_eq!(theme.colors().text.on_primary, Color::hex(0));
}
