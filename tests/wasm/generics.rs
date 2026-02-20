use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

mod __t {
    pub trait DurableObject {
        type DurableObjectStub;
    }
}

#[wasm_bindgen]
extern "C" {
    pub type MyDurableObject;
    pub type MyDurableObjectStub;
    pub type DurableObjectNamespace<T: __t::DurableObject = MyDurableObject>;

    #[wasm_bindgen(method, js_name = "getByName")]
    pub fn get_by_name<T: __t::DurableObject = MyDurableObject>(
        this: &DurableObjectNamespace<T>,
        name: &str,
    ) -> <T as __t::DurableObject>::DurableObjectStub;
}

impl __t::DurableObject for MyDurableObject {
    type DurableObjectStub = MyDurableObjectStub;
}

#[wasm_bindgen_test]
fn generic_with_default_import_type() {
    // This test verifies that when an imported type has a generic parameter
    // with a default (e.g., DurableObjectNamespace<T = MyDurableObject>),
    // the default type is used as the concrete base instead of JsValue.

    // Just test that the types are properly generated and compile
    // The actual runtime behavior would be tested by JS interop
}
