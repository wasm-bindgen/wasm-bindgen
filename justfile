default:
    @just --list

clippy:
    cargo clippy --all-features -- -D warnings

test-macro:
    cargo test -p wasm-bindgen-test-macro
    cargo test -p wasm-bindgen-macro-support

test-wasm-bindgen:
    WASM_BINDGEN_TEST_ONLY_NODE=1 WASM_BINDGEN_SPLIT_LINKED_MODULES=1 cargo test --target wasm32-unknown-unknown
    WASM_BINDGEN_TEST_ONLY_NODE=1 cargo test --target wasm32-unknown-unknown -p wasm-bindgen-futures
