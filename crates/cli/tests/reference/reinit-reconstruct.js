/* @ts-self-types="./reference_test.d.ts" */
import * as wasm from "./reference_test_bg.wasm";
import { __wbg_set_wasm } from "./reference_test_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    ReinitCounter, ReinitParent, ReinitPlain, reinit_read_counter, reinit_schedule, ReinitChild
} from "./reference_test_bg.js";
