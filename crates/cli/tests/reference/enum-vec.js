/* @ts-self-types="./reference_test.d.ts" */
import * as wasm from "./reference_test_bg.wasm";
import { __wbg_set_wasm } from "./reference_test_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    Color, HiddenErr, RenamedErr, enum_vec_echo, hidden_err_vec_echo, ns, ns_err_vec_echo, option_enum_vec_echo, renamed_err_vec_echo
} from "./reference_test_bg.js";
