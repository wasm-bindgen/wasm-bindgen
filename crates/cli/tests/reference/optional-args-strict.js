/* @ts-self-types="./reference_test.d.ts" */
import * as wasm from "./reference_test_bg.wasm";
import { __wbg_set_wasm } from "./reference_test_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    Opt, all_optional, optional_bytes, optional_string, optional_string_enum, optional_struct, some_optional
} from "./reference_test_bg.js";
