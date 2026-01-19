/* @ts-self-types="./reference_test.d.ts" */

import * as wasm from "./reference_test_bg.wasm";
import { __wbg_set_wasm } from "./reference_test_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    Color, ImplicitDiscriminant, Ordering, enum_echo, get_name, option_enum_echo, option_order, option_string_enum_echo
} from "./reference_test_bg.js";
