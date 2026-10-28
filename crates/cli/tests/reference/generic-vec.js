/* @ts-self-types="./reference_test.d.ts" */
import * as wasm from "./reference_test_bg.wasm";
import { __wbg_set_wasm } from "./reference_test_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    Point, bytes, floats, foos, give_nested_strings, imports, nested, opt_bytes, opt_foos, points, strings, values, wide
} from "./reference_test_bg.js";
