default:
    @just --list

clippy *ARGS="":
    cargo clippy --all-features --all-targets --workspace {{ARGS}} -- -D warnings

test-macro:
    cargo test -p wasm-bindgen-test-macro
    cargo test -p wasm-bindgen-macro-support

test-ui:
    cargo test -p wasm-bindgen-macro
    cargo test -p wasm-bindgen-test-macro

test-ui-overwrite:
    TRYBUILD=overwrite cargo test -p wasm-bindgen-macro --test ui

test-macro-support:
    cargo test -p wasm-bindgen-macro-support

test-wasm-bindgen *ARGS="":
    NODE_ARGS="--stack-trace-limit=100" RUST_BACKTRACE=1 WASM_BINDGEN_TEST_ONLY_NODE=1 WASM_BINDGEN_SPLIT_LINKED_MODULES=1 cargo test --target wasm32-unknown-unknown {{ARGS}}
    NODE_ARGS="--stack-trace-limit=100" RUST_BACKTRACE=1 cargo test --target wasm32-unknown-unknown -p wasm-bindgen-futures {{ARGS}}
