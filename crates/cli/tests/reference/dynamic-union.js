/* @ts-self-types="./reference_test.d.ts" */
import * as wasm from "./reference_test_bg.wasm";
import { __wbg_set_wasm } from "./reference_test_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    ExportedStruct, echo_fallback, echo_optional_wrapper, echo_response, echo_status, echo_wrapper
} from "./reference_test_bg.js";
