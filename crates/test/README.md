# `wasm-bindgen-test`

[**Read the "Testing with `wasm-bindgen-test`" section of the
guide!**](https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html)

## Components

The test harness is made of three separate components, but you typically don't
have to worry about most of them. They're documented here for documentation
purposes!

### `wasm-bindgen-test-macro`

This crate, living at `crates/test-macro`, is a procedural macro that defines
the `#[wasm_bindgen_test]` macro. **The normal `#[test]` cannot be used and will
not work.** Eventually it's intended that the `#[wasm_bindgen_test]` attribute
could gain arguments like "run in a browser" or something like a minimum Node
version.

For now though the macro is pretty simple and reexported from the next crate,
`wasm-bindgen-test`.

### `wasm-bindgen-test`

This is the runtime support needed to execute tests. This is basically the same
thing as the `test` crate in the Rust repository, and one day it will likely use
the `test` crate itself! For now though it's a minimal reimplementation that
provides the support for:

* Printing what test cases are running
* Collecting `console.log` and `console.error` output of each test case for
  printing later
* Rendering the failure output of each test case
* Catching JS exceptions so tests can continue to run after a test fails
* Driving execution of all tests

This is the crate which you actually link to in your Wasm test and through which
you import the `#[wasm_bindgen_test]` macro. Otherwise this crate provides a
`console_log!` macro that's a utility like `println!` only using `console.log`.

This crate may grow more functionality in the future, but for now it's somewhat
bare bones!

### `wasm-bindgen-test-runner`

This is where the secret sauce comes into play. We configured Cargo to execute
this binary *instead* of directly executing the `*.wasm` file (which Cargo would
otherwise try to do). This means that whenever a test is executed it executes
this binary with the Wasm file as an argument, allowing it to take full control
over the test process!

The test runner is currently pretty simple, executing a few steps:

* First, it runs the equivalent of `wasm-bindgen`. This'll generate wasm-bindgen
  output in a temporary directory.
* Next, it generates a small shim JS file which imports these
  wasm-bindgen-generated files and executes the test harness.
* Finally, it executes `node` over the generated JS file, executing all of your
  tests.

In essence what happens is that this test runner automatically executes
`wasm-bindgen` and then uses Node to actually execute the Wasm file, meaning
that your Wasm code currently runs in a Node environment.

## Future Work

Things that'd be awesome to support in the future:

* Arguments to `wasm-bindgen-test-runner` which are the same as `wasm-bindgen`,
  for example `--debug` to affect the generated output.
* Running each test in its own Wasm instance to avoid poisoning the environment
  on panic
