import * as wasm from "./pkg/synchronous_instantiation.js";

self.onmessage = ({ data: bytes }) => {
  /**
   * When we receive the bytes as an `ArrayBuffer` we can use that to
   * synchronously initialize the module as opposed to asynchronously
   * via the default export. The synchronous method internally uses
   * `new WebAssembly.Module()` and `new WebAssembly.Instance()`.
   */
  wasm.initSync({ module: bytes });

  /**
   * Once initialized we can call our exported `greet()` functions.
   */
  wasm.greet("Dominic");
};

/**
 * Once the Web Worker was spawned we ask the main thread to fetch the bytes
 * for the WebAssembly module. Once fetched it will send the bytes back via
 * a `postMessage` (see above).
 */
self.postMessage({ type: "FETCH_WASM" });
