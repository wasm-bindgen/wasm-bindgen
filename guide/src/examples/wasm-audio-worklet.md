# Wasm audio worklet

[View full source code][code] or [view the compiled example online][online]

[online]: https://wasm-bindgen.netlify.app/exbuild/wasm-audio-worklet/
[code]: https://github.com/wasm-bindgen/wasm-bindgen/tree/master/examples/wasm-audio-worklet

This is an example of using threads inside specific worklets with WebAssembly,
Rust, and `wasm-bindgen`, culminating in an oscillator demo. This demo should
complement the [parallel-raytrace][parallel-raytrace] example by
demonstrating an alternative approach using ES modules with on-the-fly module
creation.

[parallel-raytrace]: https://wasm-bindgen.github.io/wasm-bindgen/examples/raytrace.html

### Building the demo

One of the major gotchas with threaded WebAssembly is that Rust does not ship a
precompiled target (e.g. standard library) which has threading support enabled.
This means that you'll need to recompile the standard library with the
appropriate rustc flags, namely
`-C target-feature=+atomics`.
Note that this requires a nightly Rust toolchain. See the [more detailed
instructions][build] of the parallel-raytrace example.

[build]: https://wasm-bindgen.github.io/wasm-bindgen/examples/raytrace.html#building-the-demo

### Caveats

This example shares most of its [caveats][caveats] with the parallel-raytrace
example. However, it tries to encapsulate worklet creation in a Rust module, so
the application developer does not need to maintain custom JS code.

[caveats]: https://wasm-bindgen.github.io/wasm-bindgen/examples/raytrace.html#caveats

### Browser Requirements

This demo should work in the latest Chrome, Firefox and Safari versions at this time.
Note that this example requires HTTP headers to be set like in
[parallel-raytrace][headers].

[firefox-worklet-import]: https://bugzilla.mozilla.org/show_bug.cgi?id=1572644
[headers]: https://wasm-bindgen.github.io/wasm-bindgen/examples/raytrace.html#browser-requirements
