/* @ts-self-types="./reference_test.d.ts" */
import * as wasm from "./reference_test_bg.wasm";
import { __wbg_set_wasm } from "./reference_test_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    NamespaceConsumer, Point, RefToBar, RefToFoo, Status, greet
} from "./reference_test_bg.js";
