import * as import0 from "./reference_test_bg.js";
import * as import1 from "./snippets/import_reftest-a82831e16a4c30f1/inline0.js";
import * as import2 from "foo-raw";
import * as import3 from "pure-extern";
import * as import4 from "tests/wasm/imports.js";
import { __wbg_set_wasm } from "./reference_test_bg.js";
export * from "./reference_test_bg.js";
let wasm;

let imports = { __proto__: null };
imports["./reference_test_bg.js"] = import0;
imports["./snippets/import_reftest-a82831e16a4c30f1/inline0.js"] = import1;
imports["foo-raw"] = import2;
imports["pure-extern"] = import3;
imports["tests/wasm/imports.js"] = import4;

const EXPECTED_RESPONSE_TYPES = new Set(["basic", "cors", "default"]);

async function __wbg_load(module, imports) {
    if (typeof Response === "function" && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === "function") {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && EXPECTED_RESPONSE_TYPES.has(module.type);

                if (validResponse && module.headers.get("Content-Type") !== "application/wasm") {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }
}

function __wbg_finalize_init(instance, module) {
    __wbg_set_wasm(wasm = instance.exports, module);
    instance.exports.__wbindgen_start();
    return wasm;
}

export function initSync(module) {
    if (wasm !== void 0) return wasm;

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

export default async function __wbg_init(module_or_path) {
    if (wasm !== void 0) return wasm;

    if (typeof module_or_path !== "undefined") {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn("using deprecated parameters for the initialization function; pass a single object instead")
        }
    }

    if (typeof module_or_path === "undefined") {
        module_or_path = new URL("reference_test_bg.wasm", import.meta.url);
    }
    if (typeof module_or_path === "string" || (typeof Request === "function" && module_or_path instanceof Request) || (typeof URL === "function" && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}
