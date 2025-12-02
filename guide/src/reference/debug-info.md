# Debug Information

Currently, debug information in the form of DWARF, is stripped away from the output module.
To keep it, use [`--keep-debug`](cli.html#--keep-debug) with the CLI.

Most environments don't support DWARF information natively but you can use a JavaScript library
such as [wasm-stack-trace](https://github.com/membrane-io/wasm-stack-trace) to get file,
line/column, and demangled symbols in your stack traces.

You can also follow the [Debug C/C++ WebAssembly](https://developer.chrome.com/docs/devtools/wasm) 
guide to get DWARF support in Chrome. This doesn't just demangle symbols in your stacktraces, but 
also allows for live debugging in the dev-tools or in external editors have a debugger bridge to
Chrome.

The `wasm-bindgen-test-runner` currently generates DWARF debug information for tests by default.
