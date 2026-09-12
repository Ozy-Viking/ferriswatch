# Palette mapping validation

The common inspector renders all 64 palettes and exercises 560 selections, counting
`NoAccent` and every registered accent. Browser checks cover selection changes,
search, transparent CSS colours and a 390px layout. Eight screenshot sheets cover
all default palettes. The inspector is generated from the runtime registrations.

## Mapping decisions

| Family | Reviewed assignment |
| --- | --- |
| Catppuccin | Alternate surfaces use Crust, Mantle and Surface 0, with Base as their popup. Normal raised content uses Surface 2. The original normal overlay, hover, text and four status mappings stay intact. Critical retains Red; trace uses Subtext 0. |
| Gruvbox | Alternate panels use BG 1/2/3 and hover uses BG 2. Normal raised content retains BG 2. Critical retains the palette error red; trace uses FG 4. All three background contrasts stay independent. |
| Everforest | Alternate chrome starts at BG Dim and steps through BG 0/1. Normal raised content retains BG 2. Critical retains Red. Dark trace uses Grey 2; light trace uses FG because the grey choices are too faint over the light canvases. |
| Kanagawa | Alternate groups use the quieter Sumi Ink, Dragon Black and Lotus White shades. Existing normal hover, floating and diagnostic colours are preserved. Critical retains the source error hue. Trace uses Old White, Dragon Grey 3 or Lotus Grey 2. |
| Tokyo Night | Alternate chrome uses BG Dark 1, BG Dark and BG. Critical uses Red, distinct from the existing Red 1 error. Trace uses FG Dark. Existing normal hover and overlay assignments remain. |
| Rosé Pine | Raised content uses Overlay, distinct from Highlight Med hover. Alternate chrome uses NC/Base/Surface and Highlight Low hover. Critical retains Love; trace uses Subtle. |
| Solarized, One, Ayu, Panda | These selected sources expose few opaque neutral steps. Placement roles intentionally repeat rather than inventing upstream shades. Borders and action states separate controls. One imports the original Atom HSL palette; Ayu pins v7.0.1 literals. |
| Nord, Base16 | Neutral source slots supply placement. Nord retains the single Aurora error hue for critical and uses Frost Nord 10 for trace. Base16 keeps the hexadecimal slot names and uses Base 0F for critical. |
| Nightfox | BG 0 remains floating chrome, BG 2/3 provide placement and Sel 0 provides hover. Carbonfox's generated neutral shades are reproduced from its HSV brightening and sRGB blending formulas. Error and critical share each palette's red. |
| Material | The default Oceanic/Palenight/Lighter branches are imported with high visibility disabled. Lighter hover uses the neutral Active colour rather than the coloured selection fill. Its trace uses the source normal foreground. |
| GitHub | Default dark/light themes use Primer 7.10.0 plus the theme's explicit foreground/accent overrides. Canvas inset/subtle/overlay remain separate. Filled actions use the curated accent scale rather than claiming to reproduce GitHub's green default button. |
| VS Code | Dark+/Light+ merge their explicit colours over the included VS theme. Menu/tab backgrounds and named Plus token colours supply the mappings. Quiet Light remains a separate family and uses readable token colours for statuses rather than pale validation borders. |
| JetBrains | Default → Darcula → Dark editor inheritance is resolved. Each UI theme retains its own surfaces. Secondary hover and pressed colours come from ActionButton states, including Dark's alpha. Darcula and Dark remain distinct registrations. |
| Other editor palettes | Source editor, sidebar, widget and list colours supply placement and interaction. Source alpha is retained. Horizon and Synthwave use their yellow terminal colour for warning text; Cobalt2 uses terminal red instead of its dark underline colour. Poimandres and Horizon trace use the normal foreground because their gutter colours nearly disappear. |

Derived primary hover retains 15% linear-RGB darkening. Pressed actions use 30%.
Action foregrounds choose black or white by maximizing the minimum contrast over
those states. Secondary muted colours use neutral surfaces. JetBrains' explicit
secondary interaction colours remain mapped directly.

## Suitability and limits

The report uses 4.5 as a diagnostic threshold for small text. It is not a claim
that every palette is suitable for every application. It composites translucent
colours in encoded sRGB over the palette canvas before calculating luminance.
Applications with different underlying backgrounds need their own compositing check.

- Catppuccin's preserved normal overlay colours have normal-text ratios from 2.809
  to 3.377. Use the new alternate popup mapping for normal body text, or supply a
  custom mapping if the original overlay is a large text-bearing surface.
- Tokyo Night Day's preserved hover/overlay shades give normal text about 3.5.
  Their visual hierarchy is retained; a small-text interface should override
  those particular pairings or use the canvas for text-bearing content.
- Some saturated accents cross the ideal black/white threshold as they darken.
  One shared foreground cannot reach 4.5 in all three fixed states for every
  accent. The worst registered action pairing is about 4.018 after choosing the
  better foreground across the full range, improved from about 3.515 when only
  the normal fill controlled foreground choice. Use a larger label or a custom
  action mapping where a strict small-text threshold is required.
- Subtle text and several light-theme status hues are intentionally low contrast.
  Use status colours for an icon beside normal text when a colour alone is too
  faint. Keep labels for error and critical even where their hues repeat.
- Repeated neutral surfaces in simple palettes need borders or spacing for panel
  hierarchy. The inspector includes both. Synthwave's glow is outside this model.

The table reports minima across the actual checked pairings. Normal text includes
all ten surfaces. Action text includes normal/hover/pressed for both actions and
all eligible accents. Trace is checked over the main canvas.

| Theme ID | Lowest normal-text ratio | Lowest action-text ratio | Trace/canvas ratio |
| --- | ---: | ---: | ---: |
| `ayu/dark` | 9.657 | 5.437 | 2.275 |
| `ayu/light` | 4.431 | 4.649 | 2.630 |
| `ayu/mirage` | 9.262 | 6.262 | 3.459 |
| `base16/ocean` | 4.012 | 4.091 | 2.711 |
| `catppuccin/frappe` | 2.809 | 5.847 | 5.552 |
| `catppuccin/latte` | 3.068 | 4.039 | 4.369 |
| `catppuccin/macchiato` | 3.144 | 6.255 | 6.618 |
| `catppuccin/mocha` | 3.377 | 6.649 | 7.367 |
| `cobalt2/main` | 12.752 | 4.476 | 5.489 |
| `dracula/main` | 8.585 | 4.978 | 3.026 |
| `everforest/dark/hard` | 6.200 | 5.662 | 5.651 |
| `everforest/dark/medium` | 5.567 | 5.662 | 5.117 |
| `everforest/dark/soft` | 4.995 | 5.662 | 4.612 |
| `everforest/light/hard` | 4.838 | 4.655 | 5.399 |
| `everforest/light/medium` | 4.656 | 4.655 | 5.178 |
| `everforest/light/soft` | 4.174 | 4.655 | 4.660 |
| `github/dark` | 10.328 | 4.634 | 4.120 |
| `github/light` | 10.875 | 4.868 | 4.547 |
| `gruvbox/dark/hard` | 4.748 | 4.573 | 5.898 |
| `gruvbox/dark/medium` | 4.748 | 4.573 | 5.304 |
| `gruvbox/dark/soft` | 4.748 | 4.573 | 4.723 |
| `gruvbox/light/hard` | 5.323 | 4.196 | 4.420 |
| `gruvbox/light/medium` | 5.323 | 4.196 | 4.290 |
| `gruvbox/light/soft` | 5.323 | 4.196 | 3.873 |
| `horizon/main` | 10.359 | 4.547 | 11.610 |
| `jetbrains/darcula` | 3.988 | 4.330 | 3.207 |
| `jetbrains/dark` | 6.032 | 4.279 | 4.754 |
| `kanagawa/dragon` | 6.991 | 4.549 | 4.631 |
| `kanagawa/lotus` | 4.661 | 4.055 | 4.263 |
| `kanagawa/wave` | 8.163 | 4.504 | 8.886 |
| `material/lighter` | 4.370 | 4.075 | 5.174 |
| `material/oceanic` | 3.855 | 5.437 | 2.329 |
| `material/palenight` | 4.033 | 4.818 | 2.755 |
| `monokai/main` | 9.441 | 4.184 | 4.630 |
| `night_owl/main` | 13.539 | 4.516 | 2.967 |
| `nightfox/carbonfox` | 11.140 | 4.719 | 4.331 |
| `nightfox/dawnfox` | 5.019 | 4.018 | 2.248 |
| `nightfox/dayfox` | 7.482 | 5.076 | 5.963 |
| `nightfox/duskfox` | 7.847 | 5.245 | 3.031 |
| `nightfox/nightfox` | 7.214 | 4.354 | 4.094 |
| `nightfox/nordfox` | 5.459 | 4.091 | 3.201 |
| `nightfox/terafox` | 8.995 | 4.557 | 3.416 |
| `nord/main` | 5.461 | 4.029 | 3.100 |
| `oceanic_next/main` | 6.803 | 4.793 | 2.987 |
| `one/dark` | 6.568 | 4.900 | 2.316 |
| `one/light` | 10.863 | 4.049 | 5.015 |
| `panda/main` | 12.959 | 4.383 | 2.709 |
| `poimandres/main` | 6.525 | 4.576 | 7.446 |
| `quiet_light/main` | 9.571 | 4.178 | 4.107 |
| `rose_pine/dawn` | 6.856 | 4.180 | 4.024 |
| `rose_pine/main` | 7.933 | 5.217 | 5.480 |
| `rose_pine/moon` | 7.408 | 4.333 | 4.856 |
| `shades_of_purple/main` | 13.222 | 4.228 | 5.246 |
| `solarized/dark` | 4.111 | 4.294 | 2.790 |
| `solarized/light` | 3.636 | 4.294 | 4.130 |
| `synthwave_84/main` | 14.060 | 4.610 | 4.271 |
| `tokyo_night/day` | 3.518 | 4.018 | 3.574 |
| `tokyo_night/moon` | 8.282 | 5.972 | 4.611 |
| `tokyo_night/night` | 8.318 | 4.716 | 8.096 |
| `tokyo_night/storm` | 8.318 | 4.716 | 6.899 |
| `tomorrow/day` | 7.360 | 4.117 | 3.222 |
| `tomorrow/night` | 8.523 | 4.262 | 5.691 |
| `vscode/dark_plus` | 7.612 | 4.511 | 6.848 |
| `vscode/light_plus` | 17.139 | 4.511 | 4.542 |

## Reproduction

```sh
cargo run --locked --example catalogue -- html > target/palette-inspector.html
cargo run --locked --example catalogue -- contrast > target/palette-contrast.tsv
python -m http.server 8765 --bind 127.0.0.1 --directory target
# In another terminal, with Playwright and /usr/bin/chromium available:
python tools/check_palette_inspector.py
```

Screenshots are written under `target/palette-screenshots`. Source validation is
separate: `python tools/check_palette_sources.py` verifies 6,219 expected raw
literals and 57 pinned source snapshots without fetching or executing upstream
code. Integration tests also check semantic source anchors for all newly added
palettes and the pre-existing anchors for the original families.

CodeRabbit's suggestions to use Grey 2 for the three Everforest Light trace roles
were not applied. The stronger FG assignment is deliberate after checking the
light canvas pairings, rather than an accidental difference from the dark variants.

The review also identified three corrected assignments: GitHub Dark now uses its
explicit muted border, Dark+ uses a red source token for error and critical, and
JetBrains Dark muted text uses Gray 9 below its normal foreground. Suggestions to
dim Horizon trace and distinguish Catppuccin critical with Maroon were not applied:
the reviewed trace needs more contrast, and critical deliberately retains the
stronger error red. The paired labels preserve severity without inventing a new
status colour. A reported luminance conversion issue was rejected because `Color`
already stores linear sRGB; the mid-grey action regression test checks this contract.

Final local checks passed with Rust 1.97.1: 285 tests and doctests, formatting,
source reconstruction, named-accent and semantic anchors, and documentation
builds. Clippy reports only the two existing `collapsible_if` warnings in
`src/color/channel.rs`. The inspector passed 560 selection changes and mobile
layout checks. CodeRabbit returned the findings discussed above, then its
WebSocket disconnected before a final review result; it is not recorded as a
completed clean review.
