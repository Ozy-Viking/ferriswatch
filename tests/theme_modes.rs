use ferriswatch::{
    catalogue,
    palette::{
        NoAccent,
        catppuccin::{Latte, Mocha, mocha::Mauve},
    },
    theme::{Appearance, Theme, ThemeSupport},
    theme_variant::ThemePalette,
};

#[test]

fn mode_filtering_matches_declared_support_for_all_palette_accents() {
    for entry in catalogue::PALETTES {
        for variant in
            std::iter::once((entry.factory)()).chain(entry.accents.iter().map(|a| (a.factory)()))
        {
            assert_eq!(variant.support(), entry.metadata.support);

            for mode in [Appearance::Light, Appearance::Dark] {
                assert_eq!(
                    variant.supports(mode),
                    entry.metadata.support.supports(mode)
                );
            }
        }
    }

    for mode in [Appearance::Light, Appearance::Dark] {
        let variants: Vec<_> = catalogue::variants_for(mode).collect();

        assert!(!variants.is_empty());

        assert!(variants.iter().all(|v| v.supports(mode)));

        assert_eq!(
            variants.len(),
            catalogue::PALETTES
                .iter()
                .filter(|p| p.metadata.support.supports(mode))
                .count()
        );
    }
}

#[test]

fn both_is_eligible_in_either_mode_without_changing_appearance() {
    let variant = Mocha::variant::<Mauve>().with_support(ThemeSupport::Both);

    assert!(variant.supports(Appearance::Light));

    assert!(variant.supports(Appearance::Dark));

    assert_eq!(variant.metadata().appearance, Appearance::Dark);
}

#[test]

fn paired_theme_updates_one_slot_without_enforcing_support() {
    let dark = Mocha::variant::<Mauve>();

    let light = Latte::variant::<NoAccent>();

    let mut theme = Theme::new(light.clone(), dark.clone());

    assert_eq!(theme.variant(Appearance::Light), &light);

    theme.set(Appearance::Light, dark.clone());

    assert_eq!(theme.light, dark);

    assert_eq!(theme.dark, dark);

    theme.set(Appearance::Dark, light.clone());

    assert_eq!(theme.dark, light);

    assert_eq!(theme.light, dark);
}
