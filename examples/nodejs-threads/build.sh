#!/bin/bash
set -e

cd "$(dirname "$0")"

# Build with atomics support
RUSTFLAGS='-Ctarget-feature=+atomics,+bulk-memory -Clink-args=--shared-memory -Clink-args=--max-memory=1073741824 -Clink-args=--import-memory' \
    cargo +nightly build --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort

# Run wasm-bindgen with nodejs target
cargo run -p wasm-bindgen-cli -- \
    --target nodejs \
    --out-dir pkg \
    ../../target/wasm32-unknown-unknown/release/nodejs_threads.wasm

echo "Build complete! Output in pkg/"
