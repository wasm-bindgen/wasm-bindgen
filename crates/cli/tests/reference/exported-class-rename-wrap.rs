use wasm_bindgen::prelude::*;

// A struct renamed via `js_name` (Rust ident `RustRenamed` != JS class name
// `Renamed`), with no `js_namespace`, that is converted to `JsValue` via
// `.into()`.
//
// The `From<RustRenamed> for JsValue` conversion needs a "wrap a pointer in
// the exported class" import. That import must target the struct's JS-side
// class identity (`Renamed`), which is how `exported_classes` is keyed. A
// regression keyed the wrap/unwrap imports by the Rust ident (`RustRenamed`)
// instead, which minted a fresh empty `exported_classes` entry and emitted a
// duplicate phantom `class RustRenamed` (whose `free()` referenced a
// nonexistent `__wbg_rustrenamed_free` wasm export).
//
// Correct output has a single `class Renamed`; the `__wbg_renamed_new` wrap
// import calls `Renamed.__wrap(...)` and the `__wbg_renamed_unwrap` import
// calls `Renamed.__unwrap(...)` (a plain `instanceof Renamed` check, since
// this class has no subclass). Both must reference the qualified JS identity
// `Renamed`, not the Rust ident `RustRenamed`.

#[wasm_bindgen(js_name = "Renamed")]
pub struct RustRenamed {
    value: i32,
}

#[wasm_bindgen(js_class = "Renamed")]
impl RustRenamed {
    #[wasm_bindgen(constructor)]
    pub fn new(value: i32) -> RustRenamed {
        RustRenamed { value }
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> i32 {
        self.value
    }
}

// Returning the renamed struct as a `JsValue` exercises the
// `WrapInExportedClass` import path that the regression mis-keyed.
#[wasm_bindgen(js_name = "makeRenamed")]
pub fn make_renamed(value: i32) -> JsValue {
    RustRenamed::new(value).into()
}

// A `Vec<RustRenamed>` argument exercises the sibling `UnwrapExportedClass`
// path: each element is unwrapped inside wasm via `Renamed.__unwrap`, which
// the regression keyed by the Rust ident the same way as the wrap import.
// (A single by-value argument would instead lower through `_assertClass`.)
#[wasm_bindgen(js_name = "readRenameds")]
pub fn read_renameds(renameds: Vec<RustRenamed>) -> usize {
    renameds.len()
}
