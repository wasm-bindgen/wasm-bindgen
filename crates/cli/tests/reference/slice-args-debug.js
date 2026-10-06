/* @ts-self-types="./reference_test.d.ts" */
import * as wasm from "./reference_test_bg.wasm";
import { __wbg_set_wasm } from "./reference_test_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    call_get_bytes, take_boxed_slice, take_clamped, take_jsvalue_vec, take_mut_slice, take_optional_vec, take_slice, take_vec
} from "./reference_test_bg.js";
export { wasm as __wasm }
