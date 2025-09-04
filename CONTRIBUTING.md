# Contributing

See the ["Contributing" section of the `wasm-bindgen`
guide](https://wasm-bindgen.github.io/wasm-bindgen/contributing/index.html).

## Justfile

This project includes a [`justfile`](https://github.com/casey/just) as a convenient store of commonly used commands. The justfile is entirely optional - all tasks can still be run using their underlying commands directly. It simply provides shortcuts for script invocations you would otherwise need to remember or look up.

Available commands:
- `just clippy` - Run clippy linting
- `just test-wasm-bindgen` - Run main tests
- `just test-macro` - Run macro tests
