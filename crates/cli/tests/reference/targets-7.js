
let imports = {};
import * as import0 from './reference_test_bg.js';
imports['./reference_test_bg.js'] = import0;

import { readFile } from 'node:fs/promises';

const wasmUrl = new URL('reference_test_bg.wasm', import.meta.url);
const wasmBytes = await readFile(wasmUrl);
const { module: wasmModule, instance: wasmInstance } = await WebAssembly.instantiate(wasmBytes, imports);
const wasm = wasmInstance.exports;
export { wasm as __wasm };

imports["./reference_test_bg.js"].__wbg_set_wasm(wasm, wasmModule);
wasm.__wbindgen_start();

export * from "./reference_test_bg.js";