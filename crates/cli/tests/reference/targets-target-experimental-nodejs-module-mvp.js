
let imports = {};
import * as import0 from './reference_test_bg.js';
imports['./reference_test_bg.js'] = import0;

import { readFileSync } from 'node:fs';

const wasmUrl = new URL('reference_test_bg.wasm', import.meta.url);
const wasmBytes = readFileSync(wasmUrl);
const wasmModule = new WebAssembly.Module(wasmBytes);
const wasm = new WebAssembly.Instance(wasmModule, imports).exports;
export { wasm as __wasm };
imports["./reference_test_bg.js"].__wbg_set_wasm(wasm, wasmModule);
export * from "./reference_test_bg.js";