# Node.js worker_threads Example

This example demonstrates using wasm-bindgen with Node.js CommonJS and `worker_threads` for multithreading.

## Features Tested

1. **Main thread auto-initialization** - Backwards compatibility with existing code
2. **Worker thread initialization** - Using `initSync({ module, memory })`
3. **Shared atomic counter** - Verifies memory is actually shared between threads
4. **Memory growth detection** - Tests the byteLength fix for SharedArrayBuffer

## Building

```bash
./build.sh
```

## Running

```bash
node test.js
```

## How It Works

When targeting Node.js CJS with threads enabled, wasm-bindgen generates:

- `initSync(opts)` - Initialize the WASM module synchronously
- `__wbg_get_imports(memory)` - Get the imports object with optional custom memory
- `__wbindgen_wasm_module` - The compiled WebAssembly.Module for sharing with workers

Main thread auto-initializes on `require()`. Worker threads should call:

```javascript
const wasm = require('./pkg/nodejs_threads.js');
wasm.initSync({
    module: workerData.wasmModule,  // from main thread
    memory: workerData.memory       // from main thread
});
```
