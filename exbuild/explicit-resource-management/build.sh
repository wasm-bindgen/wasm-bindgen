#!/bin/sh

set -eux

cargo build --target wasm32-unknown-unknown --release
WASM_BINDGEN_EXPERIMENTAL_SYMBOL_DISPOSE=1 cargo run --package wasm-bindgen-cli --bin wasm-bindgen -- \
	--out-dir pkg --target deno ${CARGO_TARGET_DIR:-../../target}/wasm32-unknown-unknown/release/explicit_resource_management.wasm
