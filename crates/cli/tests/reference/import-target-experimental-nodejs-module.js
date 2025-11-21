
let imports = {};
import * as import0 from './reference_test_bg.js';
import * as import1 from './snippets/import_reftest-a82831e16a4c30f1/inline0.js';
import * as import2 from 'foo-raw';
import * as import3 from 'pure-extern';
import * as import4 from 'tests/wasm/imports.js';
imports['./reference_test_bg.js'] = import0;
imports['./snippets/import_reftest-a82831e16a4c30f1/inline0.js'] = import1;
imports['foo-raw'] = import2;
imports['pure-extern'] = import3;
imports['tests/wasm/imports.js'] = import4;

import { readFileSync } from 'node:fs';

const wasmUrl = new URL('reference_test_bg.wasm', import.meta.url);
const wasmBytes = readFileSync(wasmUrl);
const wasmModule = new WebAssembly.Module(wasmBytes);
const wasm = new WebAssembly.Instance(wasmModule, imports).exports;
export { wasm as __wasm };
imports["./reference_test_bg.js"].__wbg_set_wasm(wasm, wasmModule);
wasm.__wbindgen_start();

export * from "./reference_test_bg.js";