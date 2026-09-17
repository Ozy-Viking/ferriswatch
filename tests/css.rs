use ferriswatch::color::Color;
use ferriswatch::css::{DEFAULT_CSS, theme_css};
use ferriswatch::theme::Appearance;
use ferriswatch::theme_variant::{
    ActionColors, StatusColors, SurfaceColors, TextColors, ThemeVariant, ThemeVariantColors,
};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn css_lines() -> Vec<&'static str> {
    DEFAULT_CSS
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect()
}

#[test]

fn default_css_has_exact_public_rules_and_declaration_allowlist() {
    let expected = [
        ".fs-bg-background, :where(.fs-page) { background-color: var(--fs-background); }",
        ".fs-bg-surface, :where(.fs-panel, .fs-card) { background-color: var(--fs-surface); }",
        ".fs-bg-raised, :where(.fs-popover) { background-color: var(--fs-raised); }",
        ".fs-bg-overlay { background-color: var(--fs-overlay); }",
        ".fs-bg-hover { background-color: var(--fs-hover); }",
        ".fs-bg-alt-background { background-color: var(--fs-alt-background); }",
        ".fs-bg-alt-surface { background-color: var(--fs-alt-surface); }",
        ".fs-bg-alt-raised { background-color: var(--fs-alt-raised); }",
        ".fs-bg-alt-overlay { background-color: var(--fs-alt-overlay); }",
        ".fs-bg-alt-hover { background-color: var(--fs-alt-hover); }",
        ".fs-text, :where(.fs-page, .fs-panel, .fs-card, .fs-popover) { color: var(--fs-text); }",
        ".fs-text-muted { color: var(--fs-muted); }",
        ".fs-text-subtle { color: var(--fs-subtle); }",
        ".fs-text-alt { color: var(--fs-alt-text); }",
        ".fs-text-alt-muted { color: var(--fs-alt-muted); }",
        ".fs-text-alt-subtle { color: var(--fs-alt-subtle); }",
        ".fs-primary { background-color: var(--fs-primary); }",
        ".fs-primary { color: var(--fs-on-primary); }",
        ".fs-primary:hover:not(:disabled, [aria-disabled=\"true\" i]) { background-color: var(--fs-primary-hover); }",
        ".fs-primary:hover:not(:disabled, [aria-disabled=\"true\" i]) { color: var(--fs-on-primary-hover); }",
        ".fs-primary:active:not(:disabled, [aria-disabled=\"true\" i]) { background-color: var(--fs-primary-pressed); }",
        ".fs-primary:active:not(:disabled, [aria-disabled=\"true\" i]) { color: var(--fs-on-primary-pressed); }",
        ".fs-primary:disabled, .fs-primary[aria-disabled=\"true\" i] { background-color: var(--fs-primary-disabled); }",
        ".fs-primary:disabled, .fs-primary[aria-disabled=\"true\" i] { color: var(--fs-on-primary-disabled); }",
        ".fs-secondary { background-color: var(--fs-secondary); }",
        ".fs-secondary { color: var(--fs-on-secondary); }",
        ".fs-secondary:hover:not(:disabled, [aria-disabled=\"true\" i]) { background-color: var(--fs-secondary-hover); }",
        ".fs-secondary:hover:not(:disabled, [aria-disabled=\"true\" i]) { color: var(--fs-on-secondary-hover); }",
        ".fs-secondary:active:not(:disabled, [aria-disabled=\"true\" i]) { background-color: var(--fs-secondary-pressed); }",
        ".fs-secondary:active:not(:disabled, [aria-disabled=\"true\" i]) { color: var(--fs-on-secondary-pressed); }",
        ".fs-secondary:disabled, .fs-secondary[aria-disabled=\"true\" i] { background-color: var(--fs-secondary-disabled); }",
        ".fs-secondary:disabled, .fs-secondary[aria-disabled=\"true\" i] { color: var(--fs-on-secondary-disabled); }",
        ".fs-border, :where(.fs-popover) { border-color: var(--fs-border); }",
        ".fs-border-muted, :where(.fs-card) { border-color: var(--fs-border-muted); }",
        ".fs-success { color: var(--fs-success); }",
        ".fs-warning { color: var(--fs-warning); }",
        ".fs-error { color: var(--fs-error); }",
        ".fs-critical { color: var(--fs-critical); }",
        ".fs-info { color: var(--fs-info); }",
        ".fs-debug { color: var(--fs-debug); }",
        ".fs-trace { color: var(--fs-trace); }",
        ".fs-focus:focus-visible { outline: 2px solid var(--fs-focus); }",
        ".fs-focus:focus-visible { outline-offset: 2px; }",
        ".fs-chromatic-red { color: var(--fs-chromatic-red); }",
        ".fs-chromatic-orange { color: var(--fs-chromatic-orange); }",
        ".fs-chromatic-yellow { color: var(--fs-chromatic-yellow); }",
        ".fs-chromatic-green { color: var(--fs-chromatic-green); }",
        ".fs-chromatic-cyan { color: var(--fs-chromatic-cyan); }",
        ".fs-chromatic-blue { color: var(--fs-chromatic-blue); }",
        ".fs-chromatic-purple { color: var(--fs-chromatic-purple); }",
        ".fs-chromatic-pink { color: var(--fs-chromatic-pink); }",
        ".fs-syntax-attribute { color: var(--fs-syntax-attribute); }",
        ".fs-syntax-boolean { color: var(--fs-syntax-boolean); }",
        ".fs-syntax-builtin { color: var(--fs-syntax-builtin); }",
        ".fs-syntax-builtin-function { color: var(--fs-syntax-builtin-function); }",
        ".fs-syntax-builtin-type { color: var(--fs-syntax-builtin-type); }",
        ".fs-syntax-comment { color: var(--fs-syntax-comment); }",
        ".fs-syntax-constant { color: var(--fs-syntax-constant); }",
        ".fs-syntax-control-keyword { color: var(--fs-syntax-control-keyword); }",
        ".fs-syntax-deleted { color: var(--fs-syntax-deleted); }",
        ".fs-syntax-deprecated { color: var(--fs-syntax-deprecated); }",
        ".fs-syntax-documentation { color: var(--fs-syntax-documentation); }",
        ".fs-syntax-escape { color: var(--fs-syntax-escape); }",
        ".fs-syntax-foreground { color: var(--fs-syntax-foreground); }",
        ".fs-syntax-function { color: var(--fs-syntax-function); }",
        ".fs-syntax-heading { color: var(--fs-syntax-heading); }",
        ".fs-syntax-inserted { color: var(--fs-syntax-inserted); }",
        ".fs-syntax-invalid { color: var(--fs-syntax-invalid); }",
        ".fs-syntax-keyword { color: var(--fs-syntax-keyword); }",
        ".fs-syntax-link { color: var(--fs-syntax-link); }",
        ".fs-syntax-macro-name { color: var(--fs-syntax-macro-name); }",
        ".fs-syntax-markup-bold { color: var(--fs-syntax-markup-bold); }",
        ".fs-syntax-markup-italic { color: var(--fs-syntax-markup-italic); }",
        ".fs-syntax-modifier { color: var(--fs-syntax-modifier); }",
        ".fs-syntax-namespace { color: var(--fs-syntax-namespace); }",
        ".fs-syntax-number { color: var(--fs-syntax-number); }",
        ".fs-syntax-operator { color: var(--fs-syntax-operator); }",
        ".fs-syntax-parameter { color: var(--fs-syntax-parameter); }",
        ".fs-syntax-property { color: var(--fs-syntax-property); }",
        ".fs-syntax-punctuation { color: var(--fs-syntax-punctuation); }",
        ".fs-syntax-string { color: var(--fs-syntax-string); }",
        ".fs-syntax-tag { color: var(--fs-syntax-tag); }",
        ".fs-syntax-type-keyword { color: var(--fs-syntax-type-keyword); }",
        ".fs-syntax-type-name { color: var(--fs-syntax-type-name); }",
        ".fs-syntax-variable { color: var(--fs-syntax-variable); }",
    ];

    let mut expected_file = vec![
        "/* Generated by ferriswatch-generate-css. Edit src/css/definitions.rs instead. */",
        "@layer ferriswatch {",
    ];

    expected_file.extend(expected);

    expected_file.push("}");

    assert_eq!(css_lines(), expected_file);

    let tracked = fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("crates/ferriswatch-core/src/css/default.css"),
    )
    .unwrap();

    assert_eq!(tracked, DEFAULT_CSS.as_bytes());

    assert!(DEFAULT_CSS.starts_with("/* Generated by ferriswatch-generate-css"));

    assert!(DEFAULT_CSS.contains("@layer ferriswatch {") && DEFAULT_CSS.ends_with("}\n"));

    assert!(!DEFAULT_CSS.contains("!important"));

    for line in &expected {
        let declaration = line.split_once('{').unwrap().1;

        let property = declaration.split(':').next().unwrap().trim();

        assert!(matches!(
            property,
            "background-color" | "color" | "border-color" | "outline" | "outline-offset"
        ));
    }
}

fn rgba(n: u32) -> Color {
    Color::hex_alpha(n)
}

fn custom_theme() -> ThemeVariant {
    let colors = ThemeVariantColors {
        chromatic: ferriswatch::catalogue::resolve("catppuccin/mocha", None)
            .unwrap()
            .colors()
            .chromatic,
        syntax: ferriswatch::catalogue::resolve("catppuccin/mocha", None)
            .unwrap()
            .colors()
            .syntax,
        surfaces: SurfaceColors {
            background: rgba(0x01020304),
            base: rgba(0x11121314),
            raised: rgba(0x21222324),
            overlay: rgba(0x31323334),
            hover: rgba(0x41424344),
        },
        surfaces_alt: SurfaceColors {
            background: rgba(0x51525354),
            base: rgba(0x61626364),
            raised: rgba(0x71727374),
            overlay: rgba(0x81828384),
            hover: rgba(0x91929394),
        },
        text: TextColors {
            normal: rgba(0xa1a2a3a4),
            muted: rgba(0xb1b2b3b4),
            subtle: rgba(0xc1c2c3c4),
        },
        text_alt: None,
        primary: ActionColors {
            normal: ferriswatch::theme_variant::ColorPair::new(rgba(0xd1d2d3d4), rgba(0x02030405)),
            hover: ferriswatch::theme_variant::ColorPair::new(rgba(0xd1d2d3d4), rgba(0x12131415)),
            pressed: ferriswatch::theme_variant::ColorPair::new(rgba(0xd1d2d3d4), rgba(0x22232425)),
            muted: ferriswatch::theme_variant::ColorPair::new(rgba(0xd1d2d3d4), rgba(0x32333435)),
            disabled: ferriswatch::theme_variant::ColorPair::new(
                rgba(0xd1d2d3d4),
                rgba(0x03040506),
            ),
        },
        secondary: ActionColors {
            normal: ferriswatch::theme_variant::ColorPair::new(rgba(0xe1e2e3e4), rgba(0x42434445)),
            hover: ferriswatch::theme_variant::ColorPair::new(rgba(0xe1e2e3e4), rgba(0x52535455)),
            pressed: ferriswatch::theme_variant::ColorPair::new(rgba(0xe1e2e3e4), rgba(0x62636465)),
            muted: ferriswatch::theme_variant::ColorPair::new(rgba(0xe1e2e3e4), rgba(0x72737475)),
            disabled: ferriswatch::theme_variant::ColorPair::new(
                rgba(0xe1e2e3e4),
                rgba(0x03040506),
            ),
        },
        status: StatusColors {
            success: rgba(0x82838485),
            warning: rgba(0x92939495),
            error: rgba(0xa2a3a4a5),
            critical: rgba(0xb2b3b4b5),
            info: rgba(0xc2c3c4c5),
            debug: rgba(0x04050607),
            trace: rgba(0xd2d3d4d5),
        },
        border: rgba(0xe2e3e4e5),
        border_muted: rgba(0xf2f3f4f5),
        focus: rgba(0x03040506),
    };

    ThemeVariant::new(
        "custom/css_test",
        "CSS test",
        Appearance::Dark,
        colors,
        None,
    )
    .unwrap()
}

#[test]

fn theme_css_exports_all_roles_once_and_preserves_alpha() {
    let theme = custom_theme();

    let exported = theme_css(&theme);

    let declarations: BTreeMap<_, _> = exported
        .split(';')
        .filter_map(|entry| entry.split_once(':'))
        .collect();

    let c = theme.colors();

    let expected = [
        ("background", c.surfaces.background),
        ("surface", c.surfaces.base),
        ("raised", c.surfaces.raised),
        ("overlay", c.surfaces.overlay),
        ("hover", c.surfaces.hover),
        ("alt-background", c.surfaces_alt.background),
        ("alt-surface", c.surfaces_alt.base),
        ("alt-raised", c.surfaces_alt.raised),
        ("alt-overlay", c.surfaces_alt.overlay),
        ("alt-hover", c.surfaces_alt.hover),
        ("text", c.text.normal),
        ("muted", c.text.muted),
        ("subtle", c.text.subtle),
        ("alt-text", c.text.normal),
        ("alt-muted", c.text.muted),
        ("alt-subtle", c.text.subtle),
        ("on-primary", c.primary.normal.foreground),
        ("primary", c.primary.normal.background),
        ("on-primary-hover", c.primary.hover.foreground),
        ("primary-hover", c.primary.hover.background),
        ("on-primary-pressed", c.primary.pressed.foreground),
        ("primary-pressed", c.primary.pressed.background),
        ("on-primary-muted", c.primary.muted.foreground),
        ("primary-muted", c.primary.muted.background),
        ("on-primary-disabled", c.primary.disabled.foreground),
        ("primary-disabled", c.primary.disabled.background),
        ("on-secondary", c.secondary.normal.foreground),
        ("secondary", c.secondary.normal.background),
        ("on-secondary-hover", c.secondary.hover.foreground),
        ("secondary-hover", c.secondary.hover.background),
        ("on-secondary-pressed", c.secondary.pressed.foreground),
        ("secondary-pressed", c.secondary.pressed.background),
        ("on-secondary-muted", c.secondary.muted.foreground),
        ("secondary-muted", c.secondary.muted.background),
        ("on-secondary-disabled", c.secondary.disabled.foreground),
        ("secondary-disabled", c.secondary.disabled.background),
        ("border", c.border),
        ("border-muted", c.border_muted),
        ("focus", c.focus),
        ("success", c.status.success),
        ("warning", c.status.warning),
        ("error", c.status.error),
        ("critical", c.status.critical),
        ("info", c.status.info),
        ("debug", c.status.debug),
        ("trace", c.status.trace),
        ("chromatic-red", c.chromatic.red),
        ("chromatic-orange", c.chromatic.orange),
        ("chromatic-yellow", c.chromatic.yellow),
        ("chromatic-green", c.chromatic.green),
        ("chromatic-cyan", c.chromatic.cyan),
        ("chromatic-blue", c.chromatic.blue),
        ("chromatic-purple", c.chromatic.purple),
        ("chromatic-pink", c.chromatic.pink),
        ("syntax-attribute", c.syntax.attribute),
        ("syntax-boolean", c.syntax.boolean),
        ("syntax-builtin", c.syntax.builtin),
        ("syntax-builtin-function", c.syntax.builtin_function),
        ("syntax-builtin-type", c.syntax.builtin_type),
        ("syntax-comment", c.syntax.comment),
        ("syntax-constant", c.syntax.constant),
        ("syntax-control-keyword", c.syntax.control_keyword),
        ("syntax-deleted", c.syntax.deleted),
        ("syntax-deprecated", c.syntax.deprecated),
        ("syntax-documentation", c.syntax.documentation),
        ("syntax-escape", c.syntax.escape),
        ("syntax-foreground", c.syntax.foreground),
        ("syntax-function", c.syntax.function),
        ("syntax-heading", c.syntax.heading),
        ("syntax-inserted", c.syntax.inserted),
        ("syntax-invalid", c.syntax.invalid),
        ("syntax-keyword", c.syntax.keyword),
        ("syntax-link", c.syntax.link),
        ("syntax-macro-name", c.syntax.macro_name),
        ("syntax-markup-bold", c.syntax.markup_bold),
        ("syntax-markup-italic", c.syntax.markup_italic),
        ("syntax-modifier", c.syntax.modifier),
        ("syntax-namespace", c.syntax.namespace),
        ("syntax-number", c.syntax.number),
        ("syntax-operator", c.syntax.operator),
        ("syntax-parameter", c.syntax.parameter),
        ("syntax-property", c.syntax.property),
        ("syntax-punctuation", c.syntax.punctuation),
        ("syntax-string", c.syntax.string),
        ("syntax-tag", c.syntax.tag),
        ("syntax-type-keyword", c.syntax.type_keyword),
        ("syntax-type-name", c.syntax.type_name),
        ("syntax-variable", c.syntax.variable),
    ];

    assert_eq!(declarations.len(), expected.len());

    for (name, color) in expected {
        assert_eq!(
            declarations.get(format!("--fs-{name}").as_str()),
            Some(&color.to_string().as_str())
        );
    }

    assert!(theme_css(&theme).contains(" / 0.016"));
}

fn generator() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ferriswatch-generate-css"))
}

fn temp_root() -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "ferriswatch-css-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));

    fs::create_dir_all(root.join("crates/ferriswatch-core/src/css")).unwrap();

    root
}

#[test]

fn generator_check_regeneration_and_determinism() {
    let root = temp_root();

    let path = root.join("crates/ferriswatch-core/src/css/default.css");

    let missing = generator()
        .current_dir(&root)
        .arg("--check")
        .output()
        .unwrap();

    assert!(!missing.status.success());

    assert!(!path.exists());

    assert!(
        generator()
            .current_dir(&root)
            .output()
            .unwrap()
            .status
            .success()
    );

    let first = fs::read(&path).unwrap();

    assert!(
        generator()
            .current_dir(&root)
            .output()
            .unwrap()
            .status
            .success()
    );

    assert_eq!(first, fs::read(&path).unwrap());

    fs::write(&path, b"stale").unwrap();

    let stale = generator()
        .current_dir(&root)
        .arg("--check")
        .output()
        .unwrap();

    assert!(!stale.status.success());

    assert_eq!(fs::read(&path).unwrap(), b"stale");

    assert!(
        generator()
            .current_dir(&root)
            .output()
            .unwrap()
            .status
            .success()
    );

    assert_eq!(first, fs::read(&path).unwrap());

    let _ = fs::remove_dir_all(root);
}

#[test]

fn generator_rejects_invalid_arguments() {
    let root = temp_root();

    let output = generator()
        .current_dir(&root)
        .args(["--check", "extra"])
        .output()
        .unwrap();

    assert!(!output.status.success());

    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("Usage: ferriswatch-generate-css [--check]")
    );

    let _ = fs::remove_dir_all(root);
}
