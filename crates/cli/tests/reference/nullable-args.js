/* @ts-self-types="./reference_test.d.ts" */
import * as wasm from "./reference_test_bg.wasm";
import { __wbg_set_wasm } from "./reference_test_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    Opt, all_nullable, nullable_bytes, nullable_i64, nullable_string, nullable_string_enum, nullable_struct, optional_nullable_bytes, optional_nullable_string, optional_nullable_string_enum, optional_nullable_struct, some_nullable
} from "./reference_test_bg.js";
