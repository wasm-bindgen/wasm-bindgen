default:
    @just --list

clippy *ARGS="":
    cargo clippy --all-features --workspace --lib --bins --tests --examples {{ARGS}} -- -D warnings

test:
    just test-cli
    just test-macro
    just test-macro-support
    just test-ui
    just test-wasm-bindgen
    just test-wasm-bindgen-futures

test-cli *ARGS="":
    cargo test -p wasm-bindgen-cli {{ARGS}}

test-cli-overwrite *ARGS="":
    BLESS=1 cargo test -p wasm-bindgen-cli {{ARGS}}

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

bench:
    cargo bench --target wasm32-unknown-unknown
    cargo bench --target wasm32-unknown-unknown -p js-sys
    cargo bench --target wasm32-unknown-unknown -p wasm-bindgen-futures

cov *ARGS="":
  CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS="-Cinstrument-coverage -Zno-profiler-runtime -Clink-args=--no-gc-sections --cfg=wasm_bindgen_unstable_test_coverage" \
  WASM_BINDGEN_SPLIT_LINKED_MODULES=1 cargo +nightly llvm-cov test \
  --coverage-target-only \
  -p js-sys \
  -p wasm-bindgen \
  -p wasm-bindgen-futures \
  -p wasm-bindgen-test \
  --all-features \
  --target wasm32-unknown-unknown {{ARGS}}
