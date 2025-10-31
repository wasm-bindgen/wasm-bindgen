# Contributing

See the ["Contributing" section of the `wasm-bindgen`
guide](https://wasm-bindgen.github.io/wasm-bindgen/contributing/index.html).

## Justfile

This project includes a [`justfile`](https://github.com/casey/just) as a convenient store of commonly used commands. The justfile is entirely optional - all tasks can still be run using their underlying commands directly. It simply provides shortcuts for script invocations you would otherwise need to remember or look up.

Available commands:
- `just clippy` - Run clippy linting (accepts optional args)
- `just test-wasm-bindgen` - Run main wasm tests (accepts optional args)
- `just test-macro` - Run macro tests
- `just test-ui` - Run UI tests for macros
- `just test-ui-overwrite` - Overwrite UI test outputs
- `just test-macro-support` - Run macro support tests
