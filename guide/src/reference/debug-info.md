# Debug Information

Currently, debug information in the form of DWARF, is stripped away from the output module.
To keep it, use [`--keep-debug`](cli.html#--keep-debug) with the CLI.

However, currently there are no known environments that support DWARF information with Wasm.
You can follow the [Debug C/C++ WebAssembly](https://developer.chrome.com/docs/devtools/wasm) guide
to get DWARF support in Chrome. This doesn't just demangle symbols in your stacktraces, but also
allows for live debugging in the dev-tools or in external editors have a debugger bridge to Chrome.

The `wasm-bindgen-test-runner` currently generates DWARF debug information for tests by default.

Use [`--split-debug-info`](cli.html#--split-debug-info) to write the DWARF to a
separate `<name>_bg.debug.wasm` file. For debuggers like VS Code DWARF extension, provide `--debug-info-url` with the absolute URL where the debug Wasm module will be served in order to satisfy the debuggers absolute URL requirement.
