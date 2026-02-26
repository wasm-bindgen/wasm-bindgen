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
    just test-wasm-bindgen-unwind
    just test-wasm-bindgen-futures
    just test-js-sys
    just test-web-sys
    just test-webidl
    just test-webidl-tests
    just test-webidl-tests-compat

test-cli *ARGS="":
    cargo test -p wasm-bindgen-cli {{ARGS}} > /tmp/test-cli.log 2>&1 || (cat /tmp/test-cli.log && exit 1)

test-cli-overwrite:
    BLESS=1 cargo test -p wasm-bindgen-cli -- --skip headless_streaming_tests

test-macro *ARGS="":
    cargo test -p wasm-bindgen-test-macro {{ARGS}}

test-macro-support *ARGS="":
    cargo test -p wasm-bindgen-macro-support {{ARGS}}

test-ui *ARGS="":
    cargo test -p wasm-bindgen-macro {{ARGS}}

test-ui-overwrite:
    TRYBUILD=overwrite cargo test -p wasm-bindgen-macro --test ui

test-wasm-bindgen *ARGS="":
    NODE_ARGS="--stack-trace-limit=100" \
    RUST_BACKTRACE=1 \
    WASM_BINDGEN_TEST_ONLY_NODE=1 \
    WASM_BINDGEN_SPLIT_LINKED_MODULES=1 \
    cargo test --target wasm32-unknown-unknown {{ARGS}}

test-wasm-bindgen-unwind *ARGS="":
    RUSTFLAGS="-Cpanic=unwind" \
    RUSTDOCFLAGS="-Cpanic=unwind" \
    NODE_ARGS="--stack-trace-limit=100" \
    RUST_BACKTRACE=1 \
    WASM_BINDGEN_TEST_ONLY_NODE=1 \
    WASM_BINDGEN_SPLIT_LINKED_MODULES=1 \
    cargo +nightly test \
        -Zbuild-std=std,panic_unwind \
        --target wasm32-unknown-unknown \
        {{ARGS}}

test-wasm-bindgen-unwind-eh *ARGS="":
    RUSTFLAGS="-Cpanic=unwind -Cllvm-args=-wasm-use-legacy-eh=false" \
    RUSTDOCFLAGS="-Cpanic=unwind" \
    NODE_ARGS="--stack-trace-limit=100" \
    RUST_BACKTRACE=1 \
    WASM_BINDGEN_TEST_ONLY_NODE=1 \
    WASM_BINDGEN_SPLIT_LINKED_MODULES=1 \
    cargo +nightly test \
        -Zbuild-std=std,panic_unwind \
        --target wasm32-unknown-unknown \
        {{ARGS}}

test-wasm-bindgen-futures *ARGS="":
    NODE_ARGS="--stack-trace-limit=100" \
    RUST_BACKTRACE=1 \
    cargo test --target wasm32-unknown-unknown -p wasm-bindgen-futures {{ARGS}}

test-js-sys *ARGS="":
    cargo test -p js-sys --target wasm32-unknown-unknown {{ARGS}}

test-js-sys-next *ARGS="":
    RUSTFLAGS="--cfg=js_sys_unstable_apis" cargo test -p js-sys --target wasm32-unknown-unknown {{ARGS}}

test-web-sys *ARGS="":
    cargo test -p web-sys --target wasm32-unknown-unknown --all-features {{ARGS}}

test-web-sys-next *ARGS="":
    RUSTFLAGS="--cfg=wbg_next_unstable" cargo test -p web-sys --target wasm32-unknown-unknown --all-features {{ARGS}}

test-webidl *ARGS="":
    cargo test -p wasm-bindgen-webidl {{ARGS}}

test-webidl-tests *ARGS="":
    cargo test -p webidl-tests --target wasm32-unknown-unknown {{ARGS}}

test-webidl-tests-unstable *ARGS="":
    RUSTFLAGS="--cfg=web_sys_unstable_apis" cargo test -p webidl-tests --target wasm32-unknown-unknown {{ARGS}}

test-webidl-tests-next *ARGS="":
    WBG_NEXT_UNSTABLE=1 RUSTFLAGS="--cfg=wbg_next_unstable --cfg=web_sys_unstable_apis" cargo test -p webidl-tests --target wasm32-unknown-unknown {{ARGS}}

generate-web-sys:
    RUST_LOG=warn cargo run --release --package wasm-bindgen-webidl -- crates/web-sys/webidls crates/web-sys/src/features crates/web-sys/Cargo.toml

generate-web-sys-next:
    RUST_LOG=warn cargo run --release --package wasm-bindgen-webidl -- crates/web-sys/webidls crates/web-sys/src/features --next-unstable crates/web-sys/Cargo.toml

bench:
    cargo bench --target wasm32-unknown-unknown
    cargo bench --target wasm32-unknown-unknown -p js-sys
    cargo bench --target wasm32-unknown-unknown -p wasm-bindgen-futures
    cargo bench --target wasm32-unknown-unknown -p wasm-bindgen-test

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
