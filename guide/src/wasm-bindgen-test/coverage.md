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

This adapts code taken from the [Rustc book], see that for more examples and general information on test coverage as well.

```sh
# Run the tests:
# `--tests` to not run documentation tests, which is currently not supported.
RUSTFLAGS="-Cinstrument-coverage -Zno-profiler-runtime --emit=llvm-ir --cfg=wasm_bindgen_unstable_test_coverage" \
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
cargo +nightly test --tests
# Compile to object files:
# - Extract a list of compiled artifacts from Cargo and filter them with `jq`.
# - Figure out the path to the LLVM IR file corresponding to an artifact.
# - Compile to object file with Clang and store for later usage with `llvm-cov`.
crate_name=name_of_the_tested_crate_in_snake_case
objects=()
IFS=$'\n'
for file in $(
    RUSTFLAGS="-Cinstrument-coverage -Zno-profiler-runtime --emit=llvm-ir --cfg=wasm_bindgen_unstable_test_coverage" \
    cargo +nightly test --tests --no-run --message-format=json | \
    jq -r "select(.reason == \"compiler-artifact\") | (select(.target.kind == [\"test\"]) // select(.target.name == \"$crate_name\")) | .filenames[0]"
)
do
    if [[ ${file##*.} == "rlib" ]]; then
        base=$(basename $file .rlib)
        file=$(dirname $file)/${base#"lib"}.ll
    else
        file=$(dirname $file)/$(basename $file .wasm).ll
    fi

    output = $(basename $file .ll).o
    clang-19 $file -Wno-override-module -c -o $output
    objects+=(-object $output)
done
# Merge all generated raw profiling data.
llvm-profdata-19 merge -sparse *.profraw -o coverage.profdata
# Produce test coverage data in the HTML format and pass the object files we generated earlier.
llvm-cov-19 show -show-instantiations=false -Xdemangler=rustfilt -output-dir coverage -format=html -instr-profile=coverage.profdata ${objects[@]} -sources src
```

[rustc book]: https://doc.rust-lang.org/nightly/rustc/instrument-coverage.html

## Attribution

These methods have originally been pioneered by [Hacken OÜ], see [their guide][3] as well.

[3]: https://hknio.github.io/wasmcov
[Hacken OÜ]: https://hacken.io
