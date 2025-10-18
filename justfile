default:
    @just --list

clippy *ARGS="":
    cargo clippy --all-features --all-targets --workspace {{ARGS}} -- -D warnings

test:
    just test-cli
    just test-macro
    just test-macro-support
    just test-ui
    just test-wasm-bindgen
    just test-wasm-bindgen-futures

test-cli *ARGS="":
    cargo test -p wasm-bindgen-cli {{ARGS}}

test-cli-overwrite:
    BLESS=1 cargo test -p wasm-bindgen-cli

test-macro *ARGS="":
    cargo test -p wasm-bindgen-test-macro {{ARGS}}

test-macro-support *ARGS="":
    cargo test -p wasm-bindgen-macro-support {{ARGS}}

test-ui *ARGS="":
    cargo test -p wasm-bindgen-macro {{ARGS}}

test-ui-overwrite:
    TRYBUILD=overwrite cargo test -p wasm-bindgen-macro --test ui

test-wasm-bindgen *ARGS="":
    NODE_ARGS="--stack-trace-limit=100" RUST_BACKTRACE=1 WASM_BINDGEN_TEST_ONLY_NODE=1 WASM_BINDGEN_SPLIT_LINKED_MODULES=1 cargo test --target wasm32-unknown-unknown {{ARGS}}

test-wasm-bindgen-futures *ARGS="":
    NODE_ARGS="--stack-trace-limit=100" RUST_BACKTRACE=1 cargo test --target wasm32-unknown-unknown -p wasm-bindgen-futures {{ARGS}}
