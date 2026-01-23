# Microbenchmarks for `wasm-bindgen`

This folder houses a number of microbenchmarks for `wasm-bindgen`. These, like
all microbenchmarks, should be taken with a grain of salt. They are intended to
help developers understand changes over time, but they are not intended to be a
performance suite for WebAssembly for Rust.

[View benchmarks for `master` branch online][online]

[online]: https://wasm-bindgen.github.io/wasm-bindgen/benchmarks/

## Building and Running

```
$ cd benchmarks
$ cargo build --release --target wasm32-unknown-unknown
$ cargo run --package wasm-bindgen-cli -- --out-dir pkg --target web ../target/wasm32-unknown-unknown/release/wasm_bindgen_benchmark.wasm
```

Next, use your favorite static file server to host the current directory. For
example using the [`https` crate](https://crates.io/crates/https):

```
$ http
```

Then open up a web browser and view http://localhost:8000, for example.

You should be presented a page with lots of `(run)` links, where when you click
them it will execute the benchmark and then display the result.

## Benchmark Architecture

Currently benchmarks are pretty bare bones. They just use benchmark.js to
generate statistics which are then rendered to the screen. Benchmarks are listed
one-by-one in `index.html` where a `td` exists for each benchmark. In `index.js`
each of the `td`'s `id` properties are hooked up to an actual function to
benchmark, depending on what's being benchmarked.

Relevant files are:

* `index.html` - the page showing all benchmarks
* `index.js` - the driver JS for all benchmarks
* `globals.js` - global JS functions imported by all other benchmarks
* `js-bencharks.js` - the JS functions that we're benchmarking
* `src/lib.rs` - the Rust/`wasm-bindgen` functions we're benchmarking
* `raw.wast`/`raw.wasm` - a raw handwritten WebAssembly file used in some
  benchmarks. A compiled version of this is checked into the repository.
