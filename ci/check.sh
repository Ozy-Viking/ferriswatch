#!/usr/bin/env bash
set -euo pipefail

mkdir -p target/clippy
status=0
cargo run --locked --release --no-default-features --bin generate-css -- --check || status=1
cargo nextest run --locked --release --no-default-features --profile ci --no-fail-fast || status=1
cargo nextest run -p ferriswatch-componant --locked --release --profile ci --no-fail-fast || status=1
cargo test --locked --release --no-default-features --doc || status=1
cargo test -p ferriswatch-componant --locked --release --doc || status=1
cargo clippy --locked --release --no-default-features --all-targets --no-deps --message-format=json > target/clippy/native.json || status=1
cargo clippy -p ferriswatch-componant --locked --release --all-targets --no-deps --message-format=json > target/clippy/componant.json || status=1
cargo clippy --locked --release --target wasm32-unknown-unknown --features dioxus --lib --no-deps --message-format=json > target/clippy/wasm.json || status=1
cargo clippy -p ferriswatch-componant --locked --release --target wasm32-unknown-unknown --lib --no-deps --message-format=json > target/clippy/componant-wasm.json || status=1
exit "$status"
