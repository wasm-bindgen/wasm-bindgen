// FLAGS: --target=bundler

// A resolved `js_namespace` is part of a binding's identity: two otherwise
// identical imports that differ only in their namespace resolve to different
// JS values and so must not share a shim name. This has to hold whether the
// namespace is written on the item or inherited from the enclosing
// `extern "C"` block.
//
// Each pair below declares the *same* Rust signature under two different
// namespaces. Every one must produce two distinct `__wbg_*` shims calling two
// distinct JS values; a single shim would mean one of the two call sites
// silently invokes the wrong JS value.

use wasm_bindgen::prelude::*;

// Block-level `js_namespace` on both sides.
mod block_level {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_namespace = ["alpha"])]
    extern "C" {
        pub fn log(s: &str);
    }
}

mod block_level_other {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_namespace = ["beta"])]
    extern "C" {
        pub fn log(s: &str);
    }
}

// Block-level on one side, absent on the other.
mod bare {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub fn warn(s: &str);
    }
}

mod namespaced {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_namespace = ["gamma"])]
    extern "C" {
        pub fn warn(s: &str);
    }
}

// Item-level `js_namespace` on both sides.
mod item_level {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["delta"])]
        pub fn info(s: &str);
    }
}

mod item_level_other {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["epsilon"])]
        pub fn info(s: &str);
    }
}

// An item-level `js_namespace` overrides the block-level one, so the resolved
// namespaces (and shims) differ even though the blocks agree.
mod override_block {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_namespace = ["zeta"])]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["eta"])]
        pub fn debug(s: &str);
    }
}

mod override_none {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_namespace = ["zeta"])]
    extern "C" {
        pub fn debug(s: &str);
    }
}

// Nested namespaces: `["a", "b"]` and `["a"]` must stay distinct.
mod nested_deep {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_namespace = ["a", "b"])]
    extern "C" {
        pub fn c(s: &str);
    }
}

mod nested_shallow {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_namespace = ["a"])]
    extern "C" {
        pub fn c(s: &str);
    }
}

// Imported statics have the same collision: the namespace must be part of the
// static accessor shim name too.
mod static_one {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_namespace = ["theta"])]
    extern "C" {
        #[wasm_bindgen(thread_local_v2)]
        pub static STATE: JsValue;
    }
}

mod static_other {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_namespace = ["iota"])]
    extern "C" {
        #[wasm_bindgen(thread_local_v2)]
        pub static STATE: JsValue;
    }
}

#[wasm_bindgen]
pub fn exported() {
    block_level::log("alpha");
    block_level_other::log("beta");

    bare::warn("bare");
    namespaced::warn("gamma");

    item_level::info("delta");
    item_level_other::info("epsilon");

    override_block::debug("eta");
    override_none::debug("zeta");

    nested_deep::c("deep");
    nested_shallow::c("shallow");

    let _ = static_one::STATE.with(JsValue::clone);
    let _ = static_other::STATE.with(JsValue::clone);
}
