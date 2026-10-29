/* @ts-self-types="./reference_test.d.ts" */
import * as wasm from "./reference_test_bg.wasm";
import { __wbg_set_wasm } from "./reference_test_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    Point, driver, maybe_bytes, maybe_foos, maybe_names, maybe_nullable_points, maybe_points, names, nullable_foos, nullable_points
} from "./reference_test_bg.js";
