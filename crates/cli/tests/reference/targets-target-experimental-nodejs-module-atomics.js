
let imports = {};
import * as import0 from './reference_test_bg.js';
imports['./reference_test_bg.js'] = import0;
imports.wbg = { memory: new WebAssembly.Memory({initial:18,maximum:16384,shared:true}) };
import { readFileSync } from 'node:fs';

const wasmUrl = new URL('reference_test_bg.wasm', import.meta.url);
const wasmBytes = readFileSync(wasmUrl);
const wasmModule = new WebAssembly.Module(wasmBytes);
const wasm = new WebAssembly.Instance(wasmModule, imports).exports;
export { wasm as __wasm };

imports["./reference_test_bg.js"].__wbg_set_wasm(wasm, wasmModule);
wasm.__wbindgen_start();

export * from "./reference_test_bg.js";