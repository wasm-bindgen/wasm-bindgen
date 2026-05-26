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

// Generic methods on imported types using non-handle types as the type
// argument, routed through `IntoJsGeneric` / `FromJsGeneric`.

#[wasm_bindgen(module = "tests/wasm/generics.js")]
extern "C" {
    pub type Cell;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Cell;

    #[wasm_bindgen(method, js_name = "getValue")]
    pub fn get_value<T>(this: &Cell) -> T;

    #[wasm_bindgen(method, js_name = "setValue")]
    pub fn set_value<T>(this: &Cell, v: T);

    // Root-`&T` codegen path via `IntoJsGeneric::ref_to_js`; same JS-side fn.
    #[wasm_bindgen(method, js_name = "setValue")]
    pub fn set_value_ref<T>(this: &Cell, v: &T);
}

#[wasm_bindgen_test]
fn generic_get_set_string() {
    let cell = Cell::new();
    cell.set_value::<String>("hello".to_string());
    let out: String = cell.get_value::<String>();
    assert_eq!(out, "hello");

    // Round-trip with non-ASCII to exercise the UTF-16 -> UTF-8 boundary.
    cell.set_value::<String>("héllo 🌍".to_string());
    let out: String = cell.get_value::<String>();
    assert_eq!(out, "héllo 🌍");
}

#[wasm_bindgen_test]
fn generic_get_set_u32() {
    let cell = Cell::new();
    cell.set_value::<u32>(42);
    let out: u32 = cell.get_value::<u32>();
    assert_eq!(out, 42);

    // Boundary value within the lossless f64 safe-integer range.
    cell.set_value::<u32>(u32::MAX);
    let out: u32 = cell.get_value::<u32>();
    assert_eq!(out, u32::MAX);
}

#[wasm_bindgen_test]
fn generic_get_set_other_primitives() {
    let cell = Cell::new();

    cell.set_value::<i32>(-7);
    assert_eq!(cell.get_value::<i32>(), -7);

    cell.set_value::<f64>(2.5);
    assert_eq!(cell.get_value::<f64>(), 2.5);

    cell.set_value::<bool>(true);
    assert!(cell.get_value::<bool>());
    cell.set_value::<bool>(false);
    assert!(!cell.get_value::<bool>());
}

// `set_value_ref<T>(&T)` — the root-`&T` codegen path. Verifies that
// borrow-on-the-Rust-side / owned-on-the-JS-side works for both identity
// types (JsValue) and non-identity (String, u32).
#[wasm_bindgen_test]
fn generic_set_value_ref() {
    let cell = Cell::new();

    let s = String::from("hello");
    cell.set_value_ref::<String>(&s);
    assert_eq!(cell.get_value::<String>(), "hello");
    // Original String is still usable — proves we didn't move it.
    assert_eq!(s, "hello");

    let n: u32 = 42;
    cell.set_value_ref::<u32>(&n);
    assert_eq!(cell.get_value::<u32>(), 42);
    assert_eq!(n, 42);
}

// Additional `&T` cases through the root-`&T` path. The blanket
// `impl<T: IntoJsGeneric + Clone> IntoJsGeneric for &T` covers Clone-able
// types uniformly; this exercises a representative spread.
#[wasm_bindgen_test]
fn generic_set_value_ref_primitives() {
    let cell = Cell::new();

    let f: f64 = 3.25;
    cell.set_value_ref::<f64>(&f);
    assert_eq!(cell.get_value::<f64>(), 3.25);

    let b: bool = true;
    cell.set_value_ref::<bool>(&b);
    assert!(cell.get_value::<bool>());

    let i: i32 = -123;
    cell.set_value_ref::<i32>(&i);
    assert_eq!(cell.get_value::<i32>(), -123);
}

// `Option<T>` flows recursively: `Option<T>::JsCanon = Option<T::JsCanon>`,
// erases to `Option<JsValue>`. Verifies both directions.
#[wasm_bindgen_test]
fn generic_option() {
    let cell = Cell::new();

    cell.set_value::<Option<String>>(Some(String::from("present")));
    let out: Option<String> = cell.get_value::<Option<String>>();
    assert_eq!(out, Some(String::from("present")));

    cell.set_value::<Option<String>>(None);
    let out: Option<String> = cell.get_value::<Option<String>>();
    assert_eq!(out, None);

    cell.set_value::<Option<u32>>(Some(7));
    assert_eq!(cell.get_value::<Option<u32>>(), Some(7));

    cell.set_value::<Option<u32>>(None);
    assert_eq!(cell.get_value::<Option<u32>>(), None);
}

// `&Option<T>` through the root-`&T` path — exercises the blanket
// `IntoJsGeneric for &T` impl on a container type.
#[wasm_bindgen_test]
fn generic_set_value_ref_container() {
    let cell = Cell::new();

    let opt: Option<String> = Some("x".to_string());
    cell.set_value_ref::<Option<String>>(&opt);
    assert_eq!(cell.get_value::<Option<String>>(), opt);
    assert_eq!(opt, Some("x".to_string()));
}

// `Vec<T>` / `Box<[T]>` as direct fn-generic arguments will be supported
// once `js-sys` is merged into `wasm_bindgen::sys` — the canon refinement
// `Vec<T>::JsCanon = Array<T::JsCanon>` (and `Box<[T]>` similarly) is
// blocked today by the orphan rule because `Array` lives in `js-sys` while
// the `IntoJsGeneric` impl for `Vec` must live in `wasm-bindgen` core.
//
// Indirect positions (`-> Vec<T>` literally appearing in an extern
// signature) continue to work via the `ErasableGeneric` element-wise
// transmute path.

// User-facing `T: JsGeneric` bounds on hand-written helpers. Before the
// supertrait simplification, `JsGeneric` required
// `ErasableGeneric<Repr = JsValue>`, which excluded `String` / `u32` /
// `bool` — their canon is `JsValue`, but their own layout is not. After
// dropping that supertrait, `JsGeneric` carries only round-trip semantics
// (`IntoJsGeneric + FromJsGeneric + 'static`), so the leaf types qualify
// on both the owned and the root-`&T` borrow paths.
fn roundtrip_through_cell<T: wasm_bindgen::JsGeneric>(cell: &Cell, value: T) -> T {
    cell.set_value::<T>(value);
    cell.get_value::<T>()
}

fn set_cell_ref<T: wasm_bindgen::JsGeneric>(cell: &Cell, value: &T) {
    cell.set_value_ref::<T>(value);
}

#[wasm_bindgen_test]
fn js_generic_bound_accepts_leaf_types() {
    let cell = Cell::new();

    // Identity-canon type — the pre-existing `JsGeneric` surface.
    let v: JsValue = JsValue::from_f64(1.5);
    let out = roundtrip_through_cell::<JsValue>(&cell, v.clone());
    assert_eq!(out, v);

    // Non-identity canon (`JsCanon = JsValue`). Reachable through
    // `T: JsGeneric` only after the supertrait drop.
    let out = roundtrip_through_cell::<String>(&cell, "hello".to_string());
    assert_eq!(out, "hello");

    let out = roundtrip_through_cell::<u32>(&cell, 42);
    assert_eq!(out, 42);

    let out = roundtrip_through_cell::<bool>(&cell, true);
    assert!(out);

    // Borrow path: `&String` / `&u32` through `T: JsGeneric`, value not
    // consumed.
    let s = String::from("borrowed");
    set_cell_ref::<String>(&cell, &s);
    assert_eq!(cell.get_value::<String>(), "borrowed");
    assert_eq!(s, "borrowed");

    let n: u32 = 99;
    set_cell_ref::<u32>(&cell, &n);
    assert_eq!(cell.get_value::<u32>(), 99);
    assert_eq!(n, 99);

    // Container × leaf composition under `T: JsGeneric`. `Option<String>`'s
    // canon is `JsOption<JsValue>` which itself satisfies the canon bound,
    // so `Option<String>: JsGeneric` holds through the blanket impl.
    //
    // Note: this exercises the *trait-driven* path. The *syntactic* path
    // (e.g. `fn maybe_get<T>() -> Option<T>` literally in extern) still
    // requires `T: ErasableGeneric<Repr = JsValue>` and excludes leaves
    // by design — see the codegen-unification discussion in the PR.
    let out = roundtrip_through_cell::<Option<String>>(&cell, Some("nested".to_string()));
    assert_eq!(out, Some("nested".to_string()));
    let out = roundtrip_through_cell::<Option<String>>(&cell, None);
    assert_eq!(out, None);
}
