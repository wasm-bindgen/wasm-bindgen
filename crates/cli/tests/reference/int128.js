/* @ts-self-types="./reference_test.d.ts" */

import * as wasm from "./reference_test_bg.wasm";
import { __wbg_set_wasm } from "./reference_test_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    echo_i128, echo_option_i128, echo_option_u128, echo_u128, throw_i128
} from "./reference_test_bg.js";
