# Emscripten side-module bindings

This test loads a Rust side module into an Emscripten main module and calls its
generated JavaScript bindings. It checks captured callback state, independent
objects, calls in both directions, JavaScript exceptions, a caught Rust panic,
explicit destruction, and rejection of a callback after its owner is freed.
It runs with and without another side module loaded first, which changes the
memory and table layout.

Use an Emscripten frontend with `-sWASM_BINDGEN=auto` support, a Rust toolchain
with `wasm32-unknown-emscripten` installed, Python 3, and Node 24 or newer.
From the repository root:

```sh
cargo build -p wasm-bindgen-cli --bin wasm-bindgen
python3 crates/cli/tests/emscripten-side-module/run.py \
  --cli target/debug/wasm-bindgen \
  --emcc /path/to/emcc \
  --node /path/to/node
```

Add `--release` to test optimized Rust and JavaScript output.
`--rust-toolchain` selects a rustup toolchain. `--work-dir` retains the generated
files for inspection; otherwise the test uses a temporary directory. The test
uses Emscripten's debug-save option to retain the generated binding library and
passes it to the main module with `--js-library`, without editing it.

The intentional Rust panic is caught by the generated binding and asserted by
the test. Success prints `EMSCRIPTEN-SIDE-MODULE-BINDINGS-OK`.
