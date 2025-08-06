# wasm-bindgen justfile
# Run tests and build checks for wasm-bindgen project

# Show available commands
default:
    @just --list

# Setup Rust targets and tools
setup:
    rustup target add wasm32-unknown-unknown
    rustup component add rustfmt clippy

# Setup browser drivers (installs geckodriver for more stable browser testing)
setup-drivers:
    #!/usr/bin/env bash
    if [[ "$OSTYPE" == "darwin"* ]]; then
        if command -v brew &> /dev/null; then
            echo "Installing geckodriver via homebrew..."
            brew install geckodriver
        else
            echo "Homebrew not found. Please install geckodriver manually:"
            echo "https://github.com/mozilla/geckodriver/releases"
        fi
    else
        echo "Please install geckodriver for your platform:"
        echo "https://github.com/mozilla/geckodriver/releases"
    fi

# Format all code
fmt:
    cargo fmt --all

# Check formatting
fmt-check:
    cargo fmt --all -- --check

# Check TOML formatting
taplo-check:
    taplo fmt --check

# Run cargo check on everything
check:
    cargo check --all
    cargo check --no-default-features

# Run clippy on core crates
clippy:
    cargo clippy --no-deps --all-features -p wasm-bindgen-backend -- -D warnings
    cargo clippy --no-deps --all-features -p wasm-bindgen -- -D warnings
    cargo clippy --no-deps --all-features -p wasm-bindgen-cli -- -D warnings
    cargo clippy --no-deps --all-features -p wasm-bindgen-cli-support -- -D warnings
    cargo clippy --no-deps --all-features -p wasm-bindgen-macro -- -D warnings
    cargo clippy --no-deps --all-features -p wasm-bindgen-shared -- -D warnings
    cargo clippy --no-deps --all-features --target wasm32-unknown-unknown --tests -- -D warnings
    cargo clippy --no-deps --no-default-features --target wasm32-unknown-unknown -p wasm-bindgen -- -D warnings

# Run clippy on web-sys and js-sys
clippy-web:
    cargo clippy --no-deps --all-features --target wasm32-unknown-unknown -p js-sys --all-targets -- -D warnings
    cargo clippy --no-deps --all-features --target wasm32-unknown-unknown -p web-sys --all-targets -- -D warnings

# Run clippy on no-std crates
clippy-no-std:
    cargo clippy --no-deps --no-default-features --target wasm32-unknown-unknown -p wasm-bindgen -- -D warnings
    cargo clippy --no-deps --no-default-features --target wasm32-unknown-unknown -p js-sys -- -D warnings
    cargo clippy --no-deps --no-default-features --target wasm32-unknown-unknown -p web-sys -- -D warnings
    cargo clippy --no-deps --no-default-features --target wasm32-unknown-unknown -p wasm-bindgen-futures -- -D warnings
    cargo clippy --no-deps --no-default-features --target wasm32-unknown-unknown -p wasm-bindgen-test -- -D warnings

# Run clippy on the entire project
clippy-all:
    cargo clippy --no-deps --all-features --target wasm32-unknown-unknown -- -D warnings
    cargo clippy --no-deps --all-features --target wasm32-unknown-unknown --tests -- -D warnings

# Run native tests (tests that don't require WASM)
test-native:
    cargo test
    cargo test -p wasm-bindgen-cli-support
    cargo test -p wasm-bindgen-cli
    cargo test -p wasm-bindgen-externref-xform
    cargo test -p wasm-bindgen-macro-support
    cargo test -p wasm-bindgen-multi-value-xform
    cargo test -p wasm-bindgen-wasm-interpreter
    cargo test -p wasm-bindgen-futures
    cargo test -p wasm-bindgen-shared

# Run wasm-bindgen core tests (Node.js only - fast, no browser needed)
test-wasm-bindgen:
    WASM_BINDGEN_TEST_ONLY_NODE=1 WASM_BINDGEN_SPLIT_LINKED_MODULES=1 cargo test --target wasm32-unknown-unknown
    WASM_BINDGEN_TEST_ONLY_NODE=1 cargo test --target wasm32-unknown-unknown -p wasm-bindgen-futures

# Run wasm-bindgen core tests in browser (requires browser drivers)
test-wasm-bindgen-browser:
    WASM_BINDGEN_SPLIT_LINKED_MODULES=1 cargo test --target wasm32-unknown-unknown
    cargo test --target wasm32-unknown-unknown -p wasm-bindgen-futures

# Run wasm-bindgen tests with features
test-wasm-bindgen-features:
    cargo test --target wasm32-unknown-unknown --features serde-serialize
    cargo test --target wasm32-unknown-unknown --features enable-interning

# Run js-sys tests (Node.js only - fast, no browser needed)
test-js-sys:
    WASM_BINDGEN_TEST_ONLY_NODE=1 cargo test -p js-sys --target wasm32-unknown-unknown

# Run js-sys tests in browser (requires geckodriver - install with 'just setup-drivers')
test-js-sys-browser:
    #!/usr/bin/env bash
    # Set timeouts and environment for browser stability
    export WASM_BINDGEN_TEST_DRIVER_TIMEOUT=60
    export WASM_BINDGEN_TEST_TIMEOUT=120
    # Remove chromedriver from PATH and force geckodriver
    if command -v geckodriver &> /dev/null; then
        export GECKODRIVER=$(which geckodriver)
        export PATH=$(echo $PATH | tr ':' '\n' | grep -v chromedriver | tr '\n' ':')
        cargo test -p js-sys --target wasm32-unknown-unknown
    else
        echo "geckodriver not found. Install with 'just setup-drivers'"
        exit 1
    fi

# Run web-sys tests in browser with geckodriver (Firefox - more stable)
test-web-sys-browser:
    #!/usr/bin/env bash
    export WASM_BINDGEN_TEST_DRIVER_TIMEOUT=60
    export WASM_BINDGEN_TEST_TIMEOUT=120
    cargo test --manifest-path crates/web-sys/Cargo.toml --target wasm32-unknown-unknown --all-features

# Run web-sys tests in browser forcing ChromeDriver (disabled on macOS due to stability issues)
test-web-sys-browser-chrome:
    #!/usr/bin/env bash
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo "ChromeDriver tests are disabled on macOS due to stability issues."
        echo "Use 'just test-web-sys-browser' (geckodriver/Firefox) instead."
        exit 1
    fi
    # Force Chrome and set longer timeouts for ChromeDriver issues
    export WASM_BINDGEN_TEST_DRIVER_TIMEOUT=120
    export WASM_BINDGEN_TEST_TIMEOUT=180
    # Remove geckodriver from PATH temporarily to force chromedriver usage
    export PATH=$(echo $PATH | tr ':' '\n' | grep -v geckodriver | tr '\n' ':')
    cargo test --manifest-path crates/web-sys/Cargo.toml --target wasm32-unknown-unknown --all-features

# Run webidl tests
test-webidl:
    cargo test -p wasm-bindgen-webidl
    WBINDGEN_I_PROMISE_JS_SYNTAX_WORKS_IN_NODE=1 cargo test -p webidl-tests --target wasm32-unknown-unknown

# Run TypeScript tests
test-typescript:
    cd crates/typescript-tests && ./run.sh

# Run UI compile-fail tests
test-ui:
    cargo test -p wasm-bindgen-macro
    cargo test -p wasm-bindgen-test-macro

# Run all native tests
test-all-native: test-native test-ui

# Run all WASM tests (Node.js only - fast, no browser drivers needed)
test-all-wasm: test-wasm-bindgen test-wasm-bindgen-features test-js-sys

# Run all WASM tests in browsers (requires Node.js and browser drivers)
test-all-wasm-browser: test-wasm-bindgen test-wasm-bindgen-features test-js-sys-browser

# Run all tests (native + WASM)
test-all: test-all-native test-all-wasm

# Run all linting checks
lint: fmt-check clippy clippy-web clippy-no-std clippy-all

# Run basic development checks (fast)
dev-check: fmt-check check clippy test-native

# Run full CI-like checks (slow)
ci: lint test-all

# Build examples
build-examples:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Building examples..."
    cd examples
    for dir in */; do
        if [[ -f "$dir/Cargo.toml" ]]; then
            echo "Building $dir"
            cd "$dir"
            if [[ -f "build.sh" ]]; then
                ./build.sh
            else
                cargo build --target wasm32-unknown-unknown
            fi
            cd ..
        fi
    done

# Clean build artifacts
clean:
    cargo clean
    rm -rf target/
    find examples -name target -type d -exec rm -rf {} + 2>/dev/null || true

# Install wasm-bindgen CLI locally
install-cli:
    cargo install --path crates/cli --force

# Verify web-sys is compiled correctly
verify-web-sys:
    cd crates/web-sys && cargo run --release --package wasm-bindgen-webidl -- webidls src/features ./Cargo.toml
    git diff --exit-code

# Quick smoke test for development
smoke: fmt-check check test-native

# Check MSRV compatibility
check-msrv:
    cd crates/msrv/lib && cargo build --no-default-features
    cd crates/msrv/lib && cargo build --no-default-features --features std
    cd crates/msrv/resolver && cargo build --no-default-features