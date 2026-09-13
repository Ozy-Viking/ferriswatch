#![cfg(feature = "dioxus")]

use dioxus::prelude::*;
use ferriswatch::{
    catalogue::{AccentRegistration, PaletteRegistration},
    dioxus::{
        ThemeConfig, ThemeError, ThemePicker, ThemeProvider, ThemeSelection, ThemeState, theme_css,
        use_theme,
    },
    palette::{
        Accent, NoAccent, Palette,
        catppuccin::{
            Latte, Mocha,
            mocha::{Blue, Mauve},
        },
        families::{Catppuccin, RosePine},
    },
    theme::{Appearance, Theme},
    theme_variant::{ResolvedAccent, ThemePalette, ThemeSupport, ThemeVariant},
};
use std::{cell::RefCell, sync::LazyLock};

// These tests use the same variant in both slots to exercise advisory support.
fn single_default(variant: ThemeVariant) -> ferriswatch::dioxus::ThemeConfigBuilder {
    ThemeConfig::with_default(Theme::new(variant.clone(), variant), Appearance::Dark)
}

#[test]
fn selection_is_additive_ordered_and_deduplicated() {
    let selected = ThemeSelection::new()
        .with_palette::<Mocha>()
        .with_family::<Catppuccin>()
        .with_palette::<Mocha>()
        .without_palette::<Latte>()
        .with_palette::<Latte>();
    let ids: Vec<_> = selected
        .palettes()
        .iter()
        .map(|p| p.metadata.id.as_ref())
        .collect();
    assert_eq!(
        ids,
        [
            "catppuccin/mocha",
            "catppuccin/frappe",
            "catppuccin/macchiato",
            "catppuccin/latte"
        ]
    );
    let selected = ThemeSelection::all()
        .without_family::<Catppuccin>()
        .with_palette::<Mocha>();
    assert_eq!(selected.palettes().len(), 61);
    assert_eq!(
        selected.palettes().last().unwrap().metadata.id,
        "catppuccin/mocha"
    );
}

#[test]
fn builder_requires_an_available_exact_default() {
    let default = Mocha::variant::<Mauve>();
    assert_eq!(
        single_default(default.clone())
            .available(ThemeSelection::new())
            .build()
            .unwrap_err(),
        ThemeError::EmptySelection
    );
    assert!(matches!(
        single_default(default.clone())
            .available(ThemeSelection::all().without_family::<Catppuccin>())
            .build(),
        Err(ThemeError::Unavailable(_))
    ));
    let mut colors = *default.colors();
    colors.focus = ferriswatch::color::Color::TRANSPARENT;
    // A custom default cannot be silently replaced by an unrelated registration.
    let custom = ThemeVariant::new(
        "custom/missing",
        "Missing",
        default.metadata().appearance,
        colors,
        None,
    )
    .unwrap();
    assert!(matches!(
        single_default(custom).build(),
        Err(ThemeError::Unavailable(_))
    ));
    let config = single_default(default.clone())
        .available(ThemeSelection::all().without_family::<RosePine>())
        .build()
        .unwrap();
    assert_eq!(config.default_theme().dark, default);
    assert!(config.resolve("rose_pine/main", None).is_err());
    assert!(config.resolve(default.id(), Some("not_an_accent")).is_err());
}

struct Custom;
struct Violet;
impl Accent<Custom> for Violet {
    const ACCENT: Option<ferriswatch::color::Color> = Some(Mocha::MAUVE);
    const ID: Option<&'static str> = Some("violet");
    const NAME: Option<&'static str> = Some("Violet");
}
impl ThemePalette for Custom {
    fn variant<A: Accent<Self>>() -> ThemeVariant {
        let base = Mocha::variant::<NoAccent>();
        ThemeVariant::new(
            "custom/example",
            "Example",
            base.metadata().appearance,
            *base.colors(),
            A::ACCENT
                .map(|color| ResolvedAccent::new(A::ID.unwrap(), A::NAME.unwrap(), color).unwrap()),
        )
        .unwrap()
        .with_support(ThemeSupport::Both)
    }
}
impl Palette for Custom {
    fn registration() -> &'static PaletteRegistration {
        static REGISTRATION: LazyLock<PaletteRegistration> =
            LazyLock::new(|| PaletteRegistration {
                metadata: Custom::variant::<NoAccent>().metadata().clone(),
                accents: &[AccentRegistration {
                    id: "violet",
                    name: "Violet",
                    color: Mocha::MAUVE,
                    factory: Custom::variant::<Violet>,
                }],
                default_accent: "violet",
                sources: &[],
                raw_colors: &[],
                factory: Custom::variant::<NoAccent>,
            });
        &REGISTRATION
    }
}

#[test]
fn custom_registration_supplies_accents_and_default() {
    let config = single_default(Custom::variant::<Violet>())
        .available(ThemeSelection::new().with_custom::<Custom>())
        .build()
        .unwrap();
    let implicit = config.resolve("custom/example", None).unwrap();
    let explicit = config.resolve("custom/example", Some("violet")).unwrap();
    assert!(implicit.selected_accent().is_none());
    assert_eq!(explicit.accent_id(), Some("violet"));
    assert_eq!(implicit.colors(), explicit.colors());
}

thread_local! {
    static STATES: RefCell<Vec<ThemeState>> = const { RefCell::new(Vec::new()) };
    static READS: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}
#[component]
fn Observer() -> Element {
    let state = use_theme();
    use_hook(|| STATES.with(|states| states.borrow_mut().push(state)));
    let theme = state.current();
    READS.with(|reads| {
        reads
            .borrow_mut()
            .push(format!("{}:{:?}", theme.id(), theme.accent_id()))
    });
    rsx! { ThemePicker {} p { "{theme.name()}" } }
}
fn app() -> Element {
    let config = ThemeConfig::with_default(
        Theme::new(Latte::variant::<NoAccent>(), Mocha::variant::<Mauve>()),
        Appearance::Dark,
    )
    .build()
    .unwrap();
    rsx! { ThemeProvider { config, Observer {} } }
}
#[test]
fn providers_react_reset_reject_bad_input_and_isolate_apps() {
    STATES.with(|s| s.borrow_mut().clear());
    READS.with(|s| s.borrow_mut().clear());
    let mut first = VirtualDom::new(app);
    first.rebuild_in_place();
    let mut second = VirtualDom::new(app);
    second.rebuild_in_place();
    let (mut a, b) = STATES.with(|s| (s.borrow()[0], s.borrow()[1]));
    first.in_runtime(|| {
        a.select("catppuccin/mocha", Some("blue")).unwrap();
        assert_eq!(a.current(), Mocha::variant::<Blue>());
        assert!(a.select("missing", None).is_err());
        assert_eq!(a.current(), Mocha::variant::<Blue>());
    });
    let mutations = first.render_immediate_to_vec();
    assert!(!mutations.edits.is_empty());
    READS.with(|r| {
        assert_eq!(
            r.borrow().last().unwrap(),
            "catppuccin/mocha:Some(\"blue\")"
        )
    });
    second.in_runtime(|| assert_eq!(b.current(), Mocha::variant::<Mauve>()));
    first.in_runtime(|| a.reset());
    first.render_immediate_to_vec();
    first.in_runtime(|| assert_eq!(a.current(), Mocha::variant::<Mauve>()));
}

#[test]
fn css_preserves_transparency_and_every_semantic_role() {
    let base = Mocha::variant::<Mauve>();
    let mut colors = *base.colors();
    colors.surface.overlay = ferriswatch::color::Color::TRANSPARENT;
    let custom = ThemeVariant::new(
        "custom/transparent",
        "Transparent",
        base.metadata().appearance,
        colors,
        None,
    )
    .unwrap();
    let css = theme_css(&custom);
    assert!(css.contains(&format!("--fw-overlay:{};", colors.surface.overlay)));
    assert_eq!(css.matches("--fw-").count(), 34); // 32 declarations and two style references.
    assert!(css.contains("--fw-alt-raised:"));
    assert!(css.contains("--fw-primary-pressed:"));
    assert!(css.contains("color-scheme:dark;"));
}

struct InvalidPalette;
impl Palette for InvalidPalette {
    fn registration() -> &'static PaletteRegistration {
        static REGISTRATION: LazyLock<PaletteRegistration> =
            LazyLock::new(|| PaletteRegistration {
                metadata: Custom::registration().metadata.clone(),
                accents: Custom::registration().accents,
                default_accent: "missing",
                sources: &[],
                raw_colors: &[],
                factory: Custom::variant::<NoAccent>,
            });
        &REGISTRATION
    }
}
impl ThemePalette for InvalidPalette {
    fn variant<A: Accent<Self>>() -> ThemeVariant {
        Custom::variant::<NoAccent>()
    }
}
#[test]
fn rejects_invalid_defaults_and_conflicting_custom_registrations() {
    let default = Custom::variant::<NoAccent>();
    let bad = ThemeSelection::new().with_custom::<InvalidPalette>();
    assert!(matches!(
        single_default(default.clone()).available(bad).build(),
        Err(ThemeError::InvalidRegistration(_))
    ));
    let conflict = ThemeSelection::new()
        .with_custom::<Custom>()
        .with_custom::<InvalidPalette>();
    assert!(matches!(
        single_default(default).available(conflict).build(),
        Err(ThemeError::ConflictingRegistration(_))
    ));
    let original = Custom::variant::<Violet>();
    let mut colors = *original.colors();
    colors.focus = ferriswatch::color::Color::TRANSPARENT;
    let altered = ThemeVariant::new(
        original.id(),
        original.name(),
        original.metadata().appearance,
        colors,
        original.selected_accent().cloned(),
    )
    .unwrap()
    .with_support(ThemeSupport::Both);
    assert_eq!(
        single_default(altered)
            .available(ThemeSelection::new().with_custom::<Custom>())
            .build()
            .unwrap_err(),
        ThemeError::InvalidDefault
    );
}

#[test]
fn removing_a_conflicting_palette_allows_a_later_replacement() {
    let selection = ThemeSelection::new()
        .with_custom::<Custom>()
        .with_custom::<InvalidPalette>()
        .without_palette::<Custom>()
        .with_custom::<Custom>();
    assert!(
        single_default(Custom::variant::<NoAccent>())
            .available(selection)
            .build()
            .is_ok()
    );
}

#[test]
fn mode_lists_include_both_and_selection_remains_advisory() {
    let config = ThemeConfig::with_default(
        Theme::new(Mocha::variant::<Mauve>(), Latte::variant::<NoAccent>()),
        Appearance::Light,
    )
    .available(
        ThemeSelection::new()
            .with_family::<Catppuccin>()
            .with_custom::<Custom>(),
    )
    .build()
    .unwrap();
    let light: Vec<_> = config
        .variants_for(Appearance::Light)
        .map(|v| v.id().to_owned())
        .collect();
    let dark: Vec<_> = config
        .variants_for(Appearance::Dark)
        .map(|v| v.id().to_owned())
        .collect();
    assert_eq!(light, ["catppuccin/latte", "custom/example"]);
    assert!(dark.contains(&"custom/example".to_owned()));
    assert!(dark.contains(&"catppuccin/mocha".to_owned()));
    assert!(!dark.contains(&"catppuccin/latte".to_owned()));
    assert_eq!(config.default_theme().light.id(), "catppuccin/mocha");
}

#[test]
fn both_defaults_are_validated_even_when_one_is_inactive() {
    let missing = Custom::variant::<NoAccent>();
    for defaults in [
        Theme::new(missing.clone(), Mocha::variant::<Mauve>()),
        Theme::new(Mocha::variant::<Mauve>(), missing),
    ] {
        assert!(matches!(
            ThemeConfig::with_default(defaults, Appearance::Dark).build(),
            Err(ThemeError::Unavailable(_))
        ));
    }
}

#[test]
fn mode_switches_preserve_both_accents_and_inactive_updates_do_not_rerender_current() {
    STATES.with(|s| s.borrow_mut().clear());
    READS.with(|s| s.borrow_mut().clear());
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let mut state = STATES.with(|s| s.borrow()[0]);
    let reads = READS.with(|r| r.borrow().len());
    dom.in_runtime(|| {
        state
            .select_for(Appearance::Light, "catppuccin/latte", Some("green"))
            .unwrap();
    });
    dom.render_immediate_to_vec();
    assert_eq!(READS.with(|r| r.borrow().len()), reads);
    dom.in_runtime(|| {
        state.select("catppuccin/mocha", Some("blue")).unwrap();
        state.set_mode(Appearance::Light);
        assert_eq!(state.current().id(), "catppuccin/latte");
        assert_eq!(state.current().accent_id(), Some("green"));
        state.set_mode(Appearance::Dark);
        assert_eq!(state.current(), Mocha::variant::<Blue>());
        let before = state.theme();
        assert!(
            state
                .select_for(Appearance::Light, "missing", None)
                .is_err()
        );
        assert_eq!(state.theme(), before);
        // No support enforcement: a dark palette can be stored in the light slot.
        state
            .select_for(Appearance::Light, "catppuccin/mocha", Some("mauve"))
            .unwrap();
        state.reset();
        assert_eq!(state.mode(), Appearance::Dark);
        assert_eq!(
            state.variant(Appearance::Light),
            Latte::variant::<NoAccent>()
        );
        assert_eq!(state.current(), Mocha::variant::<Mauve>());
    });
    dom.render_immediate_to_vec();
}
