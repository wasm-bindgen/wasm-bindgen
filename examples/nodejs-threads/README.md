# Node.js worker_threads Example

This example demonstrates using wasm-bindgen with Node.js `worker_threads` for multithreading, supporting both CommonJS and ESM targets.

## Features Tested

1. **Main thread auto-initialization** - Backwards compatibility with existing code
2. **Worker thread initialization** - Using `initSync({ module, memory })`
3. **Shared atomic counter** - Verifies memory is actually shared between threads
4. **Memory growth detection** - Tests the byteLength fix for SharedArrayBuffer
5. **Memory growth visibility** - Tests that memory growth is visible across threads

## Building

```bash
npm run build
```

## Running

```bash
npm test
```

## How It Works

When targeting Node.js (CJS or ESM) with threads enabled, wasm-bindgen generates:

- `initSync(opts)` - Initialize the WASM module synchronously
- `__wbg_get_imports(memory)` - Get the imports object with optional custom memory
- `__wbg_wasm_module` - The compiled WebAssembly.Module for sharing with workers
- `__wbg_memory` - The shared WebAssembly.Memory

Main thread auto-initializes on `require()` or `import`. Worker threads should call:

```javascript
// CJS
const wasm = require('./pkg/nodejs_threads.js');
wasm.initSync({
    module: workerData.wasmModule,  // from main thread
    memory: workerData.memory       // from main thread
});

// ESM
import { initSync } from './pkg/nodejs_threads.js';
initSync({
    module: workerData.wasmModule,  // from main thread
    memory: workerData.memory       // from main thread
});
```

## TypeScript Support

Full TypeScript declarations are generated for all exports:

```typescript
import {
    initSync,
    __wbg_get_imports,
    __wbg_wasm_module,
    __wbg_memory,
    type InitSyncOptions
} from './pkg/nodejs_threads.js';
```
