import * as wasm from "./node_and_deno_bg.wasm";
export * from "./node_and_deno_bg.js";
import { __wbg_set_wasm } from "./node_and_deno_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
