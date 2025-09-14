# Debugging in the browser

The test generates DWARF by default.

Install the [wasm-debugging-extension](https://goo.gle/wasm-debugging-extension) plugin and
follow the [Debug C/C++ WebAssembly](https://developer.chrome.com/docs/devtools/wasm) tutorial.
When the test is configured to run in the browser via `wasm_bindgen_test::wasm_bindgen_test_configure!`,
you can debug the source code in the browser.
