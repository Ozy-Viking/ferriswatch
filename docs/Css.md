## Loading color classes

`theme_css(&ThemeVariant)` exports 32 semantic `--fs-*` variable declarations.
`DEFAULT_CSS` contains optional classes that consume them. Both are available
without the `dioxus` feature. No stylesheet is injected by importing the crate.

With the `dioxus` feature, `DEFAULT_STYLESHEET` exposes the generated file as an
`Asset`. Mount `document::Stylesheet { href: DEFAULT_STYLESHEET }` once beside
`ThemeProvider { config, ... }` and build assets with `dx`. `DefaultStyles {}`
remains a convenience wrapper using the same asset. The provider does not load
these classes automatically.

`ThemeProvider` defaults to `scope: ThemeScope::Scoped`. Its wrapper publishes
the variables and `color-scheme`, and preserves its background/text styling.
`ThemeScope::Root` publishes variables and scheme on `:root`, so portals in the
same document can inherit them. It does not move component context, style body
layout, or cross documents. Use one root provider per document. Nested scoped
providers can override it. Root rules update with the theme and are removed
when the provider unmounts or changes scope.

Custom wrappers using `theme_css` must now provide their own `color-scheme`,
background, and text declarations. The exporter previously included these.
Choose scheme from the variant's actual `Appearance`.

## Class reference

| Classes | Properties and semantic roles |
| --- | --- |
| `fs-page` | background; text |
| `fs-panel` | surface background; text |
| `fs-card` | surface background; text; muted border |
| `fs-popover` | raised background; text; border |
| `fs-bg-background`, `fs-bg-surface`, `fs-bg-raised`, `fs-bg-overlay`, `fs-bg-hover` | corresponding background role |
| `fs-bg-alt-background`, `fs-bg-alt-surface`, `fs-bg-alt-raised`, `fs-bg-alt-overlay`, `fs-bg-alt-hover` | corresponding alternate background role |
| `fs-text`, `fs-text-muted`, `fs-text-subtle` | normal, muted, subtle foreground |
| `fs-primary`, `fs-secondary` | action background and on-action foreground; hover and pressed backgrounds |
| `fs-border`, `fs-border-muted` | border color only |
| `fs-focus` | focus-visible outline: 2px solid focus color, offset 2px |
| `fs-success`, `fs-warning`, `fs-error`, `fs-critical`, `fs-info`, `fs-trace` | status foreground only |

Classes do not set spacing, layout, dimensions, radius, type scale, shadows, or
animation. Border classes do not create a border. The focus helper is the sole
exception to color-only declarations; native focus is not reset elsewhere.

Action classes do not make elements interactive or focusable. Pressed wins over
hover while both states match. Native disabled controls and elements with
`aria-disabled="true"` retain their normal action colors; the application owns
disabled behavior and presentation. ARIA alone does not disable a control.

The stylesheet uses `@layer ferriswatch`. Explicit utilities override the
zero-specificity convenience selectors, e.g. `fs-card fs-bg-raised`. Normal
unlayered application CSS overrides the library. If your app uses layers,
declare their order, for example `@layer ferriswatch, application;` before
loading either stylesheet. Conflicting utilities have no supported ordering
contract; rearranging HTML class names does not control the cascade.

## Regeneration and commit checks

Edit `src/css/definitions.rs`, then run from the repository root:

```sh
cargo run --no-default-features --bin generate-css
cargo run --no-default-features --bin generate-css -- --check
```

The tracked asset is `src/css/default.css`. The build script embeds an identical
copy generated from the same definitions, allowing the binary to regenerate a
missing asset. Check mode fails without writing if the asset is missing or stale.

Install the commit check in each clone with `sh tools/install-hooks.sh`.
It checks a temporary export of staged files, rejects stale generated CSS, and
leaves the index and working tree untouched. Regenerate, review, and stage the
CSS yourself before retrying. The hook needs Git and `rustc`; the CI check runs
the Cargo command above.
