/* @ts-self-types="./reference_test.d.ts" */

import * as wasm from "./reference_test_bg.wasm";
import { __wbg_set_wasm } from "./reference_test_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    Color, Rectangle, regular_function
} from "./reference_test_bg.js";
