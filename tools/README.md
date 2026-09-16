# Development tools

For collaboration sync setup, operation, and recovery, see
[OneDev documentation](../docs/Onedev.md#collaboration-sync).

## Palette validation

`check_palette_sources.py` checks the pinned source snapshot hashes and reconstructs
imported colour literals using Python's standard library. It also verifies the
new palettes' semantic source anchors. It runs offline and does not execute source
code from the upstream repositories.

`check_palette_inspector.py` exercises the generated HTML with Playwright and saves
screenshots under `target/`. It expects `target/` served on localhost port 8765,
the Python `playwright` package, and Chromium at `/usr/bin/chromium`. See
[palette validation](../docs/PaletteValidation.md) for commands and interpretation.
