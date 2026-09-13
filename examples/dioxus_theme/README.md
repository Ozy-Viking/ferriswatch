# Ferriswatch theme workbench

A Dioxus web application demonstrating shared theme state with a project board
and a live reference for all 32 semantic colors. It uses the optional
`ferriswatch::dioxus` module and keeps renderer dependencies in this standalone
example.

Run from the repository root:

```sh
(cd examples/dioxus_theme && dx serve --web)
```

Open the address printed by `dx`, normally http://localhost:8080.

## Try it

- Choose Light or Dark, then a palette and accent from that mode's list.
  Light starts with Catppuccin Latte / Blue, dark with Catppuccin Mocha / Mauve.
  Each mode remembers its own selection. All built-in palettes are available
  in the appropriate list; palettes declaring Both appear in both.
- Edit the project name and save it, add a task, or complete an existing task.
  Progress reflects the number of completed tasks.
- Switch to Color reference to inspect the semantic variables, then return to
  Interface. Click a swatch to copy its CSS variable name, such as `--fs-primary`.
  Clipboard access requires localhost or HTTPS and browser permission.
  The project remains mounted, so its state survives view changes.
- Reset the theme. This restores both defaults and activates Dark without
  resetting the project.
- Expand the sample error and use the keyboard to inspect focus styles.

Project changes are held in memory for this session. Reloading resets them.
Status messages are illustrative; there is no backend or publishing operation.

## Source map

`src/main.rs` keeps configuration in `App`, theme controls in `Workbench`,
independent application state in `ProjectPreview`, and color samples in
`ColorReference`. `App` opts into Ferriswatch's generated default classes and
uses a root-scoped provider so the semantic variables and color scheme are
available throughout the document. `src/style.css` owns the page reset and
responsive layout while the default classes provide semantic surfaces, text,
actions, focus, borders, and feedback colors.

The example uses the library's `ThemePicker`, which combines the mode slider
with `ThemeCombobox`. Its left half searches themes and its right half searches
accents; both lists support scrolling and keyboard selection. Styling its
`.fs-theme-picker` class shows how an application can customize its presentation
without replacing selection behavior.

## Verify

With the server running, from this directory:

```sh
uv run --with playwright playwright install chromium
uv run --with playwright python test_browser.py
```

The browser checks cover palette and accent selection, computed backgrounds,
reset, saving edits, task progress, state retention, all 32 swatches, and
horizontal overflow at desktop, tablet, and phone widths. Use
`FERRISWATCH_EXAMPLE_URL` for another address or
`PLAYWRIGHT_CHROMIUM_EXECUTABLE` for an existing Chromium installation.

### CSS integration checks

The separate browser fixture checks root/scoped updates, unmount cleanup,
child-state preservation, optional stylesheet loading, class precedence, and
hover/pressed/disabled/focus behavior without application CSS overrides:

```sh
dx serve --example css_scope --web --port 8081
# In another terminal:
uv run --with playwright python test_css_browser.py
```

Run one `dx` server/build at a time in this example directory. The fixture is a
separate Cargo example and does not add controls to the workbench.

The workbench also opts into `.override_dx_components_theme(true)` for components
using upstream color variables. To test that adapter against the pinned upstream
stylesheet, including load order, mode switches, nesting, and opt-out behavior:

```sh
dx serve --example dx_theme --web --port 8081
# In another terminal:
uv run --with playwright python test_dx_theme.py
```
