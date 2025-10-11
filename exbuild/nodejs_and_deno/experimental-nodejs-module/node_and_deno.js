
let imports = {};
import * as import0 from './node_and_deno_bg.js';
import * as import1 from './snippets/node_and_deno-c2aa0fc652329151/inline1.js';
imports['./node_and_deno_bg.js'] = import0;
imports['./snippets/node_and_deno-c2aa0fc652329151/inline1.js'] = import1;

import { readFileSync } from 'node:fs';

const wasmUrl = new URL('node_and_deno_bg.wasm', import.meta.url);
const wasmBytes = readFileSync(wasmUrl);
const wasmModule = new WebAssembly.Module(wasmBytes);
const wasm = new WebAssembly.Instance(wasmModule, imports).exports;
export { wasm as __wasm };

imports["./node_and_deno_bg.js"].__wbg_set_wasm(wasm, wasmModule);
wasm.__wbindgen_start();

export * from "./node_and_deno_bg.js";