/* @ts-self-types="./node_and_deno.d.ts" */

import * as wasm from "./node_and_deno_bg.wasm";
import { __wbg_set_wasm } from "./node_and_deno_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    greet
} from "./node_and_deno_bg.js";
