# Vendored crates

These packages contain the Dioxus component crates required by Ferriswatch but
not available from crates.io at the versions used here. Their source comes from
`DioxusLabs/dioxus-components` revision
`9a758255ea26e2b20c8cecf4c1c946feb9e71da7`.

The package names use the `ferriswatch-vendor-*` prefix to avoid claiming the
upstream package names. The Rust library names remain unchanged so downstream
source imports stay compatible. Upstream MIT and Apache 2.0 license texts are
included with each package.
