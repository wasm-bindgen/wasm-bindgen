// FLAGS: --target=bundler

// A `js_namespace` is part of a binding's identity: two otherwise identical
// imports that differ only in their namespace resolve to different JS values
// and so must not share a shim name. This has to hold whether the namespace is
// written on the item or inherited from the enclosing `extern "C"` block --
// the block-level form is the more common spelling.
//
// Each pair below declares the *same* Rust signature under two different
// namespaces. Every one must produce two distinct `__wbg_*` shims calling two
// distinct JS values; a single shim would mean one of the two call sites
// silently invokes the wrong JS function.

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

// Nested block-level namespaces. The separator used when folding the namespace
// into the shim key must keep `["a", "b"]` + `c` distinct from `["a"]` + `b.c`.
mod nested_split {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_namespace = ["a", "b"])]
    extern "C" {
        pub fn c(s: &str);
    }
}

mod nested_joined {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_namespace = ["a"])]
    extern "C" {
        #[wasm_bindgen(js_name = "c")]
        pub fn b_c(s: &str);
    }
}

#[wasm_bindgen]
pub fn exported() {
    block_level::log("alpha");
    block_level_other::log("beta");

    bare::warn("bare");
    namespaced::warn("gamma");

    nested_split::c("split");
    nested_joined::b_c("joined");
}
