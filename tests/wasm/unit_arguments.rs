//! `()` is `undefined` as an argument, which takes its place among the
//! others and no word of the call.

use wasm_bindgen::convert::{IntoWasmAbi, RefIntoWasmAbi};
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen_test::*;

#[allow(clippy::unused_unit)]
#[wasm_bindgen(module = "tests/wasm/unit_arguments.js")]
extern "C" {
    #[wasm_bindgen(js_name = showArguments)]
    fn show_flat(a: u32, unit: (), b: &str) -> String;
    #[wasm_bindgen(js_name = showArguments)]
    fn show_flat_ref(unit: &(), b: u32) -> String;
    #[wasm_bindgen(js_name = showArguments)]
    fn show_only(unit: ()) -> String;
    #[wasm_bindgen(js_name = showArguments)]
    fn show_beside(names: Vec<String>, unit: (), units: Vec<()>, last: ()) -> String;
    #[wasm_bindgen(js_name = showArguments)]
    fn show_values(unit: (), lent: &JsValue, given: JsValue, last: ()) -> String;
    #[wasm_bindgen(variadic, js_name = showArguments)]
    fn show_rest(unit: (), rest: Vec<u32>) -> String;
    #[wasm_bindgen(catch, js_name = showArguments)]
    fn show_caught(unit: (), b: u32) -> Result<String, JsValue>;
    #[wasm_bindgen(js_name = showArguments)]
    fn show_clamped(a: u32, unit: Clamped<()>) -> String;

    type Shown;
    #[wasm_bindgen(constructor)]
    fn new(unit: (), b: u32) -> Shown;
    #[wasm_bindgen(method)]
    fn shown(this: &Shown, unit: (), b: u32) -> String;
    #[wasm_bindgen(static_method_of = Shown, js_name = shownStatic)]
    fn shown_static(a: u32, unit: ()) -> String;
    #[wasm_bindgen(method, setter)]
    fn set_unit(this: &Shown, unit: ());

    #[wasm_bindgen(experimental_generic_mono, js_name = showArguments)]
    fn generic_flat<T: IntoWasmAbi>(a: T, b: u32) -> String;
    #[wasm_bindgen(experimental_generic_mono, js_name = showArguments)]
    fn generic_flat_ref<T: RefIntoWasmAbi>(a: &T, b: u32) -> String;

    #[wasm_bindgen(js_name = callWithUnit)]
    fn call_with_unit(f: &mut dyn FnMut(u32, (), u32) -> u32) -> u32;
    #[wasm_bindgen(js_name = callWithUnit)]
    fn call_owned_with_unit(f: &Closure<dyn FnMut(u32, (), u32) -> u32>) -> u32;
    #[wasm_bindgen(js_name = callWithWrongUnit)]
    fn call_with_wrong_unit(f: &mut dyn FnMut(u32, (), u32) -> u32);

    #[wasm_bindgen(js_name = callReturning)]
    fn call_returning_unit_ref(f: &dyn Fn() -> &'static ()) -> String;
    #[wasm_bindgen(catch, js_name = callReturning)]
    fn call_returning_unit_ref_result(
        f: &Closure<dyn Fn() -> Result<&'static (), JsValue>>,
    ) -> Result<String, JsValue>;

    #[wasm_bindgen(js_name = resolvesToANumber)]
    async fn resolves_to_a_number() -> ();
    #[wasm_bindgen(js_name = resolvesToANumber)]
    async fn resolves_to_a_number_undeclared();
    #[wasm_bindgen(catch, js_name = resolvesToANumber)]
    async fn resolves_to_a_number_caught() -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = unitArgumentExports)]
    fn js_exports();
}

#[wasm_bindgen_test]
fn an_import_is_passed_undefined() {
    assert_eq!(show_flat(7, (), "x"), r#"3:7,undefined,"x""#);
    assert_eq!(show_flat_ref(&(), 7), "2:undefined,7");
    assert_eq!(show_only(()), "1:undefined");
    assert_eq!(
        show_beside(vec!["a".into()], (), vec![(), ()], ()),
        r#"4:["a"],undefined,[undefined,undefined],undefined"#
    );
    // A JS value keeps its place beside one.
    assert_eq!(
        show_values((), &JsValue::from(1), JsValue::from("x"), ()),
        r#"4:undefined,1,"x",undefined"#
    );
    assert_eq!(show_rest((), vec![1, 2]), "3:undefined,1,2");
    assert_eq!(show_caught((), 7).unwrap(), "2:undefined,7");
    assert_eq!(show_clamped(7, Clamped(())), "2:7,undefined");
}

#[wasm_bindgen_test]
fn an_imported_class_is_passed_undefined() {
    let shown = Shown::new((), 7);
    assert_eq!(shown.shown((), 8), "2:undefined,7|2:undefined,8");
    assert_eq!(Shown::shown_static(9, ()), "2:9,undefined");
    shown.set_unit(());
    assert_eq!(shown.shown((), 8), "1:undefined|2:undefined,8");
}

#[wasm_bindgen_test]
fn a_generic_import_is_passed_undefined() {
    assert_eq!(generic_flat((), 7), "2:undefined,7");
    assert_eq!(generic_flat(vec![(), ()], 7), "2:[undefined,undefined],7");
    assert_eq!(generic_flat_ref(&(), 7), "2:undefined,7");
}

#[wasm_bindgen_test]
fn a_closure_is_passed_undefined() {
    let mut join = |a: u32, (): (), b: u32| a * 10 + b;
    assert_eq!(call_with_unit(&mut join), 12);
    assert_eq!(call_owned_with_unit(&Closure::new(join)), 12);
    call_with_wrong_unit(&mut |a, (), b| a * 10 + b);
}

// A `&()` a function returns is nothing, as the `()` it is by copy.
#[wasm_bindgen_test]
fn a_returned_unit_reference_is_nothing() {
    assert_eq!(call_returning_unit_ref(&|| &()), "undefined");
    let ok = Closure::new(|| Ok(&()));
    assert_eq!(call_returning_unit_ref_result(&ok).unwrap(), "undefined");
    let err = Closure::new(|| Err(JsValue::from(7)));
    assert_eq!(call_returning_unit_ref_result(&err).unwrap_err(), 7);
}

// An `async` import that declares no return, or is caught with nothing in
// its `Ok`, drops what its promise resolves to.
#[wasm_bindgen_test]
async fn an_undeclared_unit_future_drops_what_its_promise_resolves_to() {
    resolves_to_a_number_undeclared().await;
    resolves_to_a_number_caught().await.unwrap();
}

// Known failure, in a debug build only: declared as `-> ()` the future's
// reaction is a closure taking `()`, whose check throws inside the promise,
// where nothing settles the future. A future of a `bool` whose promise
// resolves to a number fails the same way.
#[wasm_bindgen_test]
#[ignore = "a debug build never settles a future of `()` whose promise resolves to a value"]
async fn a_unit_future_settles_whatever_its_promise_resolves_to() {
    resolves_to_a_number().await;
}

#[wasm_bindgen]
pub fn unit_flat(a: u32, unit: (), b: u32) -> u32 {
    let () = unit;
    a * 10 + b
}

#[wasm_bindgen]
pub fn unit_only(unit: ()) -> u32 {
    let () = unit;
    1
}

#[wasm_bindgen]
pub fn unit_between(names: Vec<String>, unit: (), units: Vec<()>, last: ()) -> String {
    let ((), ()) = (unit, last);
    format!("{}:{}", names.join(","), units.len())
}

#[wasm_bindgen]
pub fn unit_beside_values(unit: (), lent: &JsValue, given: JsValue, last: ()) -> JsValue {
    let ((), ()) = (unit, last);
    let values = js_sys::Array::new();
    values.push(lent);
    values.push(&given);
    values.into()
}

#[wasm_bindgen]
pub struct UnitTaker(u32);

#[wasm_bindgen]
impl UnitTaker {
    #[wasm_bindgen(constructor)]
    pub fn new(unit: (), value: u32) -> UnitTaker {
        let () = unit;
        UnitTaker(value)
    }

    pub fn add(&self, unit: (), more: u32) -> u32 {
        let () = unit;
        self.0 + more
    }

    #[wasm_bindgen(setter)]
    pub fn set_unit(&mut self, unit: ()) {
        let () = unit;
        self.0 += 1;
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct UnitField {
    pub unit: (),
    pub value: u32,
}

#[wasm_bindgen]
impl UnitField {
    #[wasm_bindgen(constructor)]
    pub fn new() -> UnitField {
        UnitField::default()
    }
}

#[wasm_bindgen_test]
fn exports_are_passed_undefined() {
    js_exports();
}
