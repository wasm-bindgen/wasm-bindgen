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

// Extern types with lifetime parameters. Before the fix, codegen would produce
// invalid impls (missing generics on `From<JsValue>`, missing `ty_generics` on
// `UpcastFrom` identity impls, and no PhantomData for the lifetime).
#[wasm_bindgen]
extern "C" {
    /// Lifetime-only generic extern type.
    pub type LifetimeOnly<'a>;

    /// Mixed lifetime + type generic extern type.
    pub type LifetimeAndType<'a, T>;
}

#[wasm_bindgen_test]
fn generic_with_default_import_type() {
    // This test verifies that when an imported type has a generic parameter
    // with a default (e.g., DurableObjectNamespace<T = MyDurableObject>),
    // the default type is used as the concrete base instead of JsValue.

    // Just test that the types are properly generated and compile
    // The actual runtime behavior would be tested by JS interop
}

#[wasm_bindgen_test]
fn lifetime_generic_extern_types() {
    // Verify that extern types with lifetime parameters produce valid codegen.
    // This is a compile-time test: if the generated `From<JsValue>`,
    // `UpcastFrom`, and `PhantomData` code is wrong, this will not compile.
    let val = JsValue::NULL;

    // Lifetime-only type: From<JsValue> and upcast must all work.
    let lo: LifetimeOnly<'_> = val.clone().into();
    let _: &JsValue = lo.as_ref();
    let _: JsValue = lo.into();

    // Mixed lifetime + type param: same checks.
    let lt: LifetimeAndType<'_, JsValue> = val.into();
    let _: &JsValue = lt.as_ref();
    let _: JsValue = lt.into();
}
