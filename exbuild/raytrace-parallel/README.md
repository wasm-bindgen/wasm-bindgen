# Parallel Raytracing

[View documentation for this example online][dox] or [View compiled example
online][compiled]

[dox]: https://wasm-bindgen.github.io/wasm-bindgen/examples/raytrace.html
[compiled]: https://wasm-bindgen.netlify.app/exbuild/raytrace-parallel/

You can build the example locally with:

```
$ CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS="-Ctarget-feature=+atomics -Clink-args=--shared-memory -Clink-args=--max-memory=1073741824 -Clink-args=--import-memory -Clink-args=--export=__wasm_init_tls -Clink-args=--export=__tls_size -Clink-args=--export=__tls_align -Clink-args=--export=__tls_base" cargo +nightly build -Zbuild-std=std,panic_abort --target wasm32-unknown-unknown
$ wasm-bindgen ../../target/wasm32-unknown-unknown/debug/raytrace_parallel.wasm --target no-modules --out-dir .
$ python3 server.py
```

and then visiting http://localhost:8000 in a browser should run the example!
