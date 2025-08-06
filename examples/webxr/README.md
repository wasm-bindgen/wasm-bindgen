# WebXR Example

[View documentation for this example online][dox] or [View compiled example
online][compiled]

[compiled]: https://wasm-bindgen.github.io/wasm-bindgen/exbuild/webxr/
[dox]: https://wasm-bindgen.github.io/wasm-bindgen/examples/webxr.html

You can build the example locally with:

```
# Note: Requires unstable flag whilst WebXR in development
$ RUSTFLAGS=--cfg=web_sys_unstable_apis npm run serve
```

and then visiting http://localhost:8080 in a browser should run the example!
