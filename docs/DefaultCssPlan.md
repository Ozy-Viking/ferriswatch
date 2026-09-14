# Optional default CSS implementation plan

Status: implemented. This document records the agreed scope. See `Css.md` for
usage and regeneration commands. The embedded stylesheet is generated into
Cargo OUT_DIR from the same definitions as the tracked asset, so the generator
can bootstrap a missing asset without an include_str! compilation failure.
Contract tests assert that both copies match.

## Ownership and public interface

Ferriswatch owns semantic colors and color interaction states. The default CSS
does not set layout, spacing, dimensions, radius, typography, shadows, or
animation. The separate focus helper is the one exception: a 2px solid focus
outline with a 2px offset.

Provide a framework-independent `ferriswatch::css` module containing
`theme_css(&ThemeVariant) -> String`, `DEFAULT_CSS: &str`, and:

```rust
pub enum ThemeScope {
    Scoped,
    Root,
}
```

The existing `dioxus` Cargo feature exposes Dioxus adapters from the CSS module,
including `DefaultStyles`. Re-export applicable adapters through
`ferriswatch::dioxus` for convenient existing imports. Keep the existing
`ferriswatch::dioxus::theme_css` import working. The CSS exporter, generated file,
constant, and generator must build without Dioxus. Enabling the Dioxus feature
must not automatically inject the stylesheet.

## Dynamic variables and scope

`theme_css` exports only the existing 32 `--fs-*` declarations, preserving alpha.
It accepts a resolved variant and does not alter the semantic model.

Add a `scope` prop to `ThemeProvider`, defaulting to `ThemeScope::Scoped`.
The recommended scope contract is:

- `Scoped` attaches variables and the variant's `color-scheme` to the existing
  `.fs-theme` wrapper. This remains the default for compatibility.
- `Root` publishes variables and `color-scheme` in a document `:root` rule.
  It makes the theme available to DOM content outside the provider wrapper,
  including portals. It does not make Dioxus context available outside the
  provider's component subtree.
- Root scope applies to the current document. It does not cross iframe or
  separate-window boundaries. Use one root provider per document. Nested scoped
  providers override inherited root variables normally.
- Preserve the provider's current wrapper background and text styling in the
  provider implementation. Root scope does not add body/html background, text,
  or layout rules. These remain application choices.
- Root declarations update when the active theme changes, disappear on unmount
  or a switch to scoped mode, and reveal any pre-existing root declarations.
  Avoid imperative replacement of the document element's entire style attribute.

Use a reactive, provider-owned style element for the dynamic root rule. Dioxus
0.7.10's `document::Style` explicitly ignores prop changes after mounting, so it
must not receive the changing root theme CSS. Scope changes must preserve child
component state.

Direct callers of `theme_css` need migration documentation for the removed
background/text and `color-scheme` declarations. Scheme follows actual variant
`Appearance`, not advisory `ThemeSupport` or the selected storage slot.

## Generated static CSS and loading

Keep a small internal declaration model as the source of truth for selector
mappings, grouping, and rule order. Add a Cargo binary named
`ferriswatch-generate-css` that deterministically writes `src/css/default.css`. The file is
tracked and marked as generated. `DEFAULT_CSS` embeds an identical generated copy. Generation
does not read a palette or depend on the previous generated contents.

Commands:

```sh
cargo run --no-default-features --bin ferriswatch-generate-css
cargo run --no-default-features --bin ferriswatch-generate-css -- --check
```

Normal mode writes only when bytes differ. Check mode generates in memory,
compares against the tracked artifact, and exits nonzero for a missing or stale
file without modifying it. Emit no timestamps or machine-specific paths. Ensure
the binary can generate a missing artifact without an `include_str!` compile
failure; isolate its compilation from the embedded artifact if necessary.

Expose Dioxus loading as:

```rust
rsx! {
    DefaultStyles {}
    ThemeProvider { config, scope: ThemeScope::Root,
        // Application content
    }
}
```

`DefaultStyles` loads the generated asset using Dioxus's document stylesheet
integration. Gate the required document/asset dependencies through the existing
Dioxus feature. Mount once at application level and use the framework's stable
asset URL for deduplication. Verify the actual example's web build before
settling the dependency changes. Other frameworks can use the CSS file or
`DEFAULT_CSS` directly.

## Commit guard

No active repository hook framework or custom hooks path was found during the
planning inspection. Add a tracked pre-commit hook and document installation;
install it in this checkout during implementation without replacing unrelated
hooks. A fresh clone needs the documented installation step.

The hook runs generation validation and rejects the commit if generation would
change the CSS. Tell the contributor to run the generator, review, and stage
the result. Never automatically stage generated changes.

Validate the staged snapshot, including the generated artifact, rather than
only the working tree. This catches stale staged CSS even when the working copy
is current and supports partially staged source changes. A temporary index
export may be used; leave the real index and working tree untouched. Run the
same `--check` validation in CI against the checked-out commit.

## Class contract and cascade

Ship precisely these classes:

- Convenience: `fs-page`, `fs-panel`, `fs-card`, `fs-popover`.
- Backgrounds: `fs-bg-background`, `fs-bg-surface`, `fs-bg-raised`,
  `fs-bg-overlay`, `fs-bg-hover`, and the five corresponding `fs-bg-alt-*`
  classes.
- Text: `fs-text`, `fs-text-muted`, `fs-text-subtle`.
- Actions: `fs-primary`, `fs-secondary`.
- Borders/focus: `fs-border`, `fs-border-muted`, `fs-focus`.
- Status foregrounds: `fs-success`, `fs-warning`, `fs-error`, `fs-critical`,
  `fs-info`, `fs-trace`.

Convenience mappings are page=background/text, panel=surface/text,
card=surface/text/border-muted, and popover=raised/text/border. No new semantic
fields are required. Border classes set only border color.

Emit grouped declarations inside `@layer ferriswatch`. Wrap convenience
selectors in `:where()` so explicit utilities override convenience defaults.
Normal unlayered application styles override the library. Document layer order
for applications using their own layers. No `!important`. Conflicting utilities
on one element have no supported class-order override contract.

Actions set their normal background and corresponding on-action text color.
Hover uses the modeled hover background; active uses pressed. Emit active after
hover so holding a pointer down while hovering displays pressed. Exclude
`:disabled` and `[aria-disabled="true"]` elements from both state rules. Leave
disabled colors, interaction behavior, cursor, and opacity to applications.
Action classes do not add focusability or imply selection state.

Keep focus separate and leave native focus behavior intact elsewhere. Status
classes set foreground only. Defer status backgrounds, interaction colors,
additional aliases, and changes to the meaning of palette accents.

## Verification and implementation order

1. Extract the CSS module, preserve imports, separate provider presentation, and
   add scope behavior with its lifecycle checks.
2. Add declaration definitions, generator, artifact, feature-gated
   `DefaultStyles`, and the commit/CI checks.
3. Test every selector/property/variable mapping and interaction rule. Verify all
   stylesheet variable references are exported and the exporter contains only
   semantic variable declarations. Preserve alpha coverage.
4. Enforce a declaration allowlist: background-color, color, border-color, and
   the exact focus outline/offset exception. Check the exact public class set
   and absence of palette values and global resets in the static stylesheet.
5. Test deterministic regeneration, missing/stale artifacts, and clean/stale/
   partially staged commit-hook cases without modifying the real index.
6. Browser-test utility precedence, application overrides, action states,
   disabled controls, keyboard focus, nested scoped themes, root updates and
   cleanup, and child-state preservation. Verify static stylesheet loading is
   independent of theme changes and absent when not requested.
7. Migrate example color rules to demonstrate the classes while retaining its
   application-owned geometry. Existing `.secondary` surface controls must not
   automatically become semantic secondary actions. Run the existing browser
   regression suite and web build.
8. Document generation, hook installation, opt-in loading, scope, class mappings,
   overrides, and custom-wrapper migration. Verify core builds without Dioxus
   and integration builds with it.
