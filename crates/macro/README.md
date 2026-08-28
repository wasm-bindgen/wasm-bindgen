# `wasm-bindgen-macro`

Implementation of the `#[wasm_bindgen]` attribute. See the `wasm-bindgen`
documentation for more information about what this macro does.

## Testing

Testing of this macro is done through "ui tests" in the `ui-tests` subdirectory
of this crate. Each Rust file in this folder is compiled with the `wasm_bindgen`
crate, and the `*.stderr` file sitting next to it is the asserted output of the
compiler. If the output matches, the test passes, and if the output doesn't
match the test fails. Note that for these tests it is also considered a failure
if a test actually compiles successfully.

There is also a `pass-tests` subdirectory for the opposite assertion: each Rust
file there must compile *successfully*, with no `*.stderr` file. Use it to pin
that a supported shape keeps working, especially one whose generated code is
easy to break (for example the generic import paths, where a mistake in codegen
surfaces as a rustc error against code the user never wrote). Note that a
`#[wasm_bindgen(experimental_generic_mono)]` import only generates code when it is
actually instantiated, so a pass test needs a call site, not just the
declaration.

Both directories are registered in `tests/ui.rs`.

To add a test:

* Create `ui-tests/my-awesome-test.rs`
* Write an invalid `#[wasm_bindgen]` invocation, testing the error you're
  generating
* Execute `cargo test -p wasm-bindgen-macro --test ui`, the test will fail
* From within the `ui-tests` folder, execute `./update-all-references.sh`. This
  should create a `my-awesome-test.stderr` file.

  OR if you are on Windows, set the `TRYBUILD=overwrite` environment variable (this is done as `$env:TRYBUILD="overwrite"` [in powershell](https://stackoverflow.com/a/1333717/7595472)) and run the command again.
* Inspect `my-awesome-test.stderr` to make sure it looks ok
* Rerun `cargo test -p wasm-bindgen-macro --test ui` and your tests should pass!
