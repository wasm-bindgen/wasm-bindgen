# Generating Coverage Data

You can ask the runner to generate coverage data from functions marked as `#[wasm_bindgen_test]` in the `.profraw` format.

<div class="warning">
  Coverage is still in an experimental state, requires Rust Nightly, may be
  unreliable and could experience breaking changes at any time.
</div>

## Enabling the feature

To enable this feature, you need to enable `cfg(wasm_bindgen_unstable_test_coverage)`.

## Generating the data

### `RUSTFLAGS` that need to be present

Make sure you are using `RUSTFLAGS=-Cinstrument-coverage -Zno-profiler-runtime`.

Due to the current limitation of `llvm-cov`, we can't collect profiling symbols from the generated `.wasm` files. Instead, we can grab them from the LLVM IR with `--emit=llvm-ir` by using Clang. Usage of Clang or any LLVM tools must match the LLVM version used by Rust.

### Arguments to the test runner

Like with Rust test coverage, you can use the [`LLVM_PROFILE_FILE`][1] environment variable to specify a path for the generated `.profraw` files.

[1]: https://releases.llvm.org/19.1.0/tools/clang/docs/SourceBasedCodeCoverage.html#running-the-instrumented-program

### Target features

This feature relies on the [minicov] crate, which provides a profiling runtime for WebAssembly. It in turn uses [cc] to compile the runtime to Wasm, which [currently doesn't support accounting for target feature][2]. Use e.g. `CFLAGS_wasm32_unknown_unknown="-matomics -mbulk-memory"` to account for that.

[2]: https://github.com/rust-lang/cc-rs/issues/268
[cc]: https://crates.io/crates/cc
[minicov]: https://crates.io/crates/minicov

### Example

_Requires rust >= 1.87.0 and wasm-bindgen-test >= 0.3.57._

Install [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) and run with:

```sh
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS="-Cinstrument-coverage -Zno-profiler-runtime -Clink-args=--no-gc-sections --cfg=wasm_bindgen_unstable_test_coverage" \
cargo +nightly llvm-cov test --target wasm32-unknown-unknown
```

## Attribution

These methods have originally been pioneered by [Hacken OÜ], see [their guide][3] as well.

[3]: https://hknio.github.io/wasmcov
[Hacken OÜ]: https://hacken.io
