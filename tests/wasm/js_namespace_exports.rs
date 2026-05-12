use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(js_namespace = api)]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen(js_namespace = api)]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[wasm_bindgen(js_namespace = ["utils", "math"])]
pub fn divide(a: f64, b: f64) -> f64 {
    a / b
}

#[wasm_bindgen(js_namespace = ["utils", "math"], js_name = "subtract")]
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[wasm_bindgen(js_namespace = models)]
pub struct Counter {
    value: i32,
}

#[wasm_bindgen]
impl Counter {
    #[wasm_bindgen(constructor)]
    pub fn new(initial: i32) -> Counter {
        Counter { value: initial }
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> i32 {
        self.value
    }

    #[wasm_bindgen(setter)]
    pub fn set_value(&mut self, val: i32) {
        self.value = val;
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }

    pub fn add(&mut self, amount: i32) {
        self.value += amount;
    }
}

#[wasm_bindgen(js_namespace = types)]
pub enum Status {
    Pending = 0,
    Active = 1,
    Complete = 2,
}

#[wasm_bindgen(js_namespace = ["types", "http"])]
pub enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    ServerError = 500,
}

#[wasm_bindgen(js_namespace = shapes)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen(js_namespace = ["shapes", "3d"])]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// ---------------------------------------------------------------------------
// Regression coverage: a struct with `js_name` (rename) + `js_namespace`,
// combined with an `impl` that uses `js_class = "<renamed>"`. The macro emits
// methods on a class named after `js_name` ("RenamedCounter"), but the
// namespace export `wasm.<ns>.RenamedCounter` is bound to the qualified-name
// class (`<ns>__RenamedCounter`) which has no methods or working constructor
// — so `new wasm.<ns>.RenamedCounter(args)` throws `cannot invoke 'new'
// directly`.
//
// The same-name test below (no `js_name`) shows the bug does not surface when
// the Rust ident matches the JS export name; the rename is required to
// trigger the codegen split between qualified- and unqualified-name classes.
// ---------------------------------------------------------------------------

#[wasm_bindgen(js_name = "RenamedCounter", js_namespace = renamed_models)]
pub struct RenamedCounterImpl {
    value: i32,
}

#[wasm_bindgen(js_class = "RenamedCounter", js_namespace = renamed_models)]
impl RenamedCounterImpl {
    #[wasm_bindgen(constructor)]
    pub fn new(initial: i32) -> RenamedCounterImpl {
        RenamedCounterImpl { value: initial }
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }
}

// Variant: rename on struct + namespace, but the impl block uses ONLY
// `js_class` (no `js_namespace` on the impl). Checks whether the regression
// also surfaces when only the struct carries the namespace attribute.

#[wasm_bindgen(js_name = "RenamedOnlyStructNs", js_namespace = struct_only_ns)]
pub struct RenamedOnlyStructNsImpl {
    value: i32,
}

#[wasm_bindgen(js_class = "RenamedOnlyStructNs")]
impl RenamedOnlyStructNsImpl {
    #[wasm_bindgen(constructor)]
    pub fn new(initial: i32) -> RenamedOnlyStructNsImpl {
        RenamedOnlyStructNsImpl { value: initial }
    }

    pub fn double(&self) -> i32 {
        self.value * 2
    }
}

// Variant: no rename, but both struct and impl carry the same `js_namespace`.
// Confirms whether the rename is necessary to trigger the bug.

#[wasm_bindgen(js_namespace = same_name_ns)]
pub struct SameNameNs {
    value: i32,
}

#[wasm_bindgen(js_namespace = same_name_ns)]
impl SameNameNs {
    #[wasm_bindgen(constructor)]
    pub fn new(initial: i32) -> SameNameNs {
        SameNameNs { value: initial }
    }

    pub fn triple(&self) -> i32 {
        self.value * 3
    }
}

#[wasm_bindgen(module = "tests/wasm/js_namespace_exports.js")]
extern "C" {
    fn test_api_namespace();
    fn test_nested_namespace();
    fn test_class_namespace();
    fn test_enum_namespace();
    fn test_nested_enum_namespace();
    fn test_struct_namespace();
    fn test_nested_struct_namespace();
    fn test_renamed_namespaced_class_methods();
    fn test_renamed_class_namespace_on_struct_only();
    fn test_namespaced_class_methods_same_name();
}

#[wasm_bindgen_test]
fn test_namespaced_exports() {
    test_api_namespace();
    test_nested_namespace();
    test_class_namespace();
    test_enum_namespace();
    test_nested_enum_namespace();
    test_struct_namespace();
    test_nested_struct_namespace();
}

#[wasm_bindgen_test]
fn renamed_namespaced_class_methods() {
    test_renamed_namespaced_class_methods();
}

#[wasm_bindgen_test]
fn renamed_class_namespace_on_struct_only() {
    test_renamed_class_namespace_on_struct_only();
}

#[wasm_bindgen_test]
fn namespaced_class_methods_same_name() {
    test_namespaced_class_methods_same_name();
}
