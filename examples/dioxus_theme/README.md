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
  Interface. Click a swatch to copy its CSS variable name, such as `--fw-primary`.
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
`ColorReference`. `src/style.css` uses the provider's semantic CSS variables
for surfaces, text, controls, focus, and feedback. It also owns the page reset
and responsive layout; the library provider does not change global page styles.

The example deliberately uses the library's `ThemePicker`. Styling its
`.fw-theme-picker` class shows how an application can customize its presentation
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
