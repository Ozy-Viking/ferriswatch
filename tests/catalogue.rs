use ferriswatch::{
    catalogue::{self, PALETTES, ResolveError},
    color::Color,
    theme_variant::{Appearance, Contrast, IdentityError, ResolvedAccent, ThemeVariant},
};
use rstest::rstest;
use std::collections::{HashMap, HashSet};

#[test]

fn full_catalogue_has_unique_ids_and_consistent_factories() {
    assert_eq!(PALETTES.len(), 64);

    assert_eq!(
        PALETTES
            .iter()
            .map(|p| &p.metadata.family_id)
            .collect::<HashSet<_>>()
            .len(),
        28
    );

    let mut ids = HashSet::new();

    for entry in PALETTES {
        let m = &entry.metadata;

        assert!(ids.insert(m.id.as_ref()), "duplicate {}", m.id);

        let expected = match m.contrast {
            Some(c) => format!("{}/{}/{}", m.family_id, m.variant_id, c.id()),
            None => format!("{}/{}", m.family_id, m.variant_id),
        };

        assert_eq!(m.id, expected);

        let default = catalogue::resolve(&m.id, None).unwrap();

        assert_eq!(default.metadata(), m);

        assert!(default.selected_accent().is_none());

        assert!(!entry.sources.is_empty());

        for source in entry.sources {
            assert_eq!(source.revision.len(), 40);

            assert!(source.revision.bytes().all(|b| b.is_ascii_hexdigit()));

            assert!(!source.permalink().contains(' '));

            assert!(source.permalink().starts_with(source.repository));
        }

        let mut accents = HashSet::new();

        for accent in entry.accents {
            assert!(accents.insert(accent.id));

            let selected = catalogue::resolve(&m.id, Some(accent.id)).unwrap();

            assert_eq!(selected.metadata(), m);

            assert_eq!(selected.accent_id(), Some(accent.id));

            assert_eq!(selected.accent_name(), Some(accent.name));

            assert_eq!(selected.accent(), Some(accent.color));

            assert_eq!(selected.focus(), accent.color);

            assert_eq!(selected.colors().surfaces, default.colors().surfaces);

            assert_eq!(
                selected.colors().surfaces_alt,
                default.colors().surfaces_alt
            );

            assert_eq!(selected.colors().status, default.colors().status);

            assert_eq!(selected.colors().syntax, default.colors().syntax);

            assert_eq!(selected.colors().chromatic, default.colors().chromatic);
        }

        let explicit_default = entry.resolve(Some(entry.default_accent)).unwrap();

        assert_eq!(default.colors(), explicit_default.colors());
    }
}

#[rstest]
#[case("catppuccin/mocha/hard", true)]
#[case("gruvbox/dark", true)]
#[case("everforest/light/extreme", true)]
#[case("tokyo_night/moon/medium", true)]
#[case("nord/light", false)]
#[case("Nord", false)]
#[case("custom/my_theme", false)]
#[case("CATPPUCCIN/mocha", false)]
#[case("catppuccin/mocha/soft/extra", false)]

fn rejects_unregistered_theme_combinations(#[case] id: &str, #[case] contrast: bool) {
    let err = catalogue::resolve(id, None).unwrap_err();

    assert_eq!(
        matches!(err, ResolveError::UnsupportedContrast(_)),
        contrast
    );

    if !contrast {
        assert!(matches!(err, ResolveError::UnknownTheme(_)));
    }
}

#[test]

fn accent_errors_do_not_silently_select_defaults() {
    for id in ["", "NoAccent", "Blue", "not_a_colour"] {
        assert!(matches!(
            catalogue::resolve("catppuccin/mocha", Some(id)),
            Err(ResolveError::UnknownAccent { .. })
        ));
    }

    assert!(catalogue::resolve("kanagawa/wave", Some("blue")).is_err());

    assert_eq!(
        catalogue::resolve("kanagawa/wave", Some("crystal_blue"))
            .unwrap()
            .accent_name(),
        Some("Crystal Blue")
    );
}

#[test]

fn labels_and_appearance_preserve_upstream_identity() {
    for (id, label, appearance) in [
        ("catppuccin/frappe", "Catppuccin Frappé", Appearance::Dark),
        ("rose_pine/main", "Rosé Pine", Appearance::Dark),
        ("tokyo_night/night", "Tokyo Night", Appearance::Dark),
        ("synthwave_84/main", "Synthwave '84", Appearance::Dark),
        ("vscode/light_plus", "VS Code Light+", Appearance::Light),
        ("nightfox/dawnfox", "Dawnfox", Appearance::Light),
        ("ayu/mirage", "Ayu Mirage", Appearance::Dark),
    ] {
        let t = catalogue::resolve(id, None).unwrap();

        assert_eq!(t.name(), label);

        assert_eq!(t.metadata().appearance, appearance);
    }

    for family in ["gruvbox", "everforest"] {
        for variant in ["dark", "light"] {
            for contrast in [Contrast::Soft, Contrast::Medium, Contrast::Hard] {
                let t = catalogue::resolve(&format!("{family}/{variant}/{}", contrast.id()), None)
                    .unwrap();

                assert_eq!(t.metadata().contrast, Some(contrast));
            }
        }
    }
}

#[test]

fn custom_ids_are_explicit_and_names_can_change_without_changing_identity() {
    let original = catalogue::resolve("nord/main", None).unwrap();

    let colors = *original.colors();

    let accent = ResolvedAccent::new("clear", "Clear", Color::TRANSPARENT).unwrap();

    for id in [
        "",
        "nord/main",
        "custom/",
        "custom/My Theme",
        "custom/nested/path",
        "custom/a__b",
    ] {
        assert_eq!(
            ThemeVariant::new(id, "Custom", Appearance::Dark, colors, None).unwrap_err(),
            IdentityError::InvalidCustomId
        );
    }

    let first = ThemeVariant::new(
        "custom/my_theme",
        "My theme",
        Appearance::Dark,
        colors,
        Some(accent.clone()),
    )
    .unwrap();

    let renamed = ThemeVariant::new(
        "custom/my_theme",
        "Another label",
        Appearance::Dark,
        colors,
        Some(accent),
    )
    .unwrap();

    assert_eq!(first.id(), renamed.id());

    assert_ne!(first.name(), renamed.name());

    assert_eq!(first.accent(), Some(Color::TRANSPARENT));

    assert_eq!(first.accent_id(), Some("clear"));
}

#[test]

fn imports_match_independent_upstream_literals_for_every_palette_and_accent() {
    let mut expected: HashMap<&str, HashMap<&str, Color>> = HashMap::new();
    let mut missing = Vec::new();

    for line in include_str!("fixtures/palettes.tsv")
        .lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
    {
        let mut cols = line.split('\t');

        let id = cols.next().unwrap();

        let name = cols.next().unwrap();

        let rgba = cols.next().unwrap();

        assert!(
            expected
                .entry(id)
                .or_default()
                .insert(
                    name,
                    Color::hex_alpha(u32::from_str_radix(rgba, 16).unwrap())
                )
                .is_none()
        );
    }

    for entry in PALETTES {
        let values = expected
            .get(entry.metadata.id.as_ref())
            .unwrap_or_else(|| panic!("missing fixture {}", entry.metadata.id));

        for (name, color) in entry.raw_colors {
            match values.get(name) {
                Some(expected) => assert_eq!(expected, color, "{}/{}", entry.metadata.id, name),
                None => missing.push(format!("{}/{}", entry.metadata.id, name)),
            }
        }

        for accent in entry.accents {
            assert!(
                values.values().any(|v| *v == accent.color),
                "missing accent {} {}",
                entry.metadata.id,
                accent.id
            );
        }
    }

    assert!(missing.is_empty(), "missing fixture literals: {missing:#?}");
}

#[test]

fn transparency_survives_all_surface_roles_and_both_actions() {
    let original = catalogue::resolve("catppuccin/mocha", None).unwrap();

    let mut c = *original.colors();

    let transparent = ferriswatch::theme_variant::SurfaceColors {
        background: Color::TRANSPARENT,
        base: Color::TRANSPARENT,
        raised: Color::TRANSPARENT,
        overlay: Color::TRANSPARENT,
        hover: Color::TRANSPARENT,
    };

    c.surfaces_alt = transparent;

    c.primary.normal.background = Color::TRANSPARENT;

    c.secondary.normal.background = Color::TRANSPARENT;

    c.primary.normal.foreground = Color::TRANSPARENT;

    c.secondary.normal.foreground = Color::TRANSPARENT;

    let custom = ThemeVariant::new("custom/clear", "Clear", Appearance::Dark, c, None).unwrap();

    assert_eq!(custom.colors().surfaces, original.colors().surfaces);

    assert_eq!(custom.colors().surfaces_alt, transparent);

    assert_eq!(custom.primary().normal.background, Color::TRANSPARENT);

    assert_eq!(custom.secondary().normal.background, Color::TRANSPARENT);

    assert_eq!(
        custom.colors().primary.normal.foreground,
        Color::TRANSPARENT
    );

    assert_eq!(
        custom.colors().secondary.normal.foreground,
        Color::TRANSPARENT
    );
}

#[test]

fn new_palette_semantic_anchors_match_imported_source_fixtures() {
    let mut themes = HashSet::new();

    for line in include_str!("fixtures/semantic.tsv")
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        let fields: Vec<_> = line.split('\t').collect();

        assert_eq!(fields.len(), 7);

        let theme = catalogue::resolve(fields[0], None).unwrap();

        assert!(themes.insert(theme.id().to_owned()));

        for (actual, expected) in [
            theme.surfaces().background,
            theme.text().normal,
            theme.focus(),
            theme.status().success,
            theme.status().warning,
            theme.status().error,
        ]
        .iter()
        .zip(&fields[1..])
        {
            assert_eq!(
                *actual,
                Color::hex_alpha(u32::from_str_radix(expected, 16).unwrap()),
                "{}",
                theme.id()
            );
        }
    }

    assert_eq!(themes.len(), 38);
}

#[test]

fn source_permalinks_encode_path_characters_without_losing_directories() {
    let source = catalogue::PaletteSource {
        repository: "https://github.com/example/palettes",
        revision: "0123456789abcdef0123456789abcdef01234567",
        path: "themes/Rosé Pine #1.json",
        licence: None,
    };

    assert_eq!(
        source.permalink(),
        "https://github.com/example/palettes/blob/0123456789abcdef0123456789abcdef01234567/themes/Ros%C3%A9%20Pine%20%231.json"
    );
}

#[test]

fn every_new_named_accent_matches_its_source_assignment() {
    let mut fixtures = HashMap::new();

    for line in include_str!("fixtures/accents.tsv")
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        let fields: Vec<_> = line.split('\t').collect();

        assert_eq!(fields.len(), 3);

        let expected = Color::hex_alpha(u32::from_str_radix(fields[2], 16).unwrap());

        assert!(fixtures.insert((fields[0], fields[1]), expected).is_none());

        let theme = catalogue::resolve(fields[0], Some(fields[1])).unwrap();

        assert_eq!(
            theme.accent(),
            Some(expected),
            "{} {}",
            fields[0],
            fields[1]
        );
    }

    for entry in PALETTES.iter().filter(|p| {
        ![
            "catppuccin",
            "everforest",
            "gruvbox",
            "kanagawa",
            "rose_pine",
            "tokyo_night",
        ]
        .contains(&p.metadata.family_id.as_ref())
    }) {
        for accent in entry.accents {
            assert!(
                fixtures.contains_key(&(entry.metadata.id.as_ref(), accent.id)),
                "{} {}",
                entry.metadata.id,
                accent.id
            );
        }
    }
}
