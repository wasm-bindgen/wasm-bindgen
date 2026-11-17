//! `()` is `undefined` as the element of a sequence, which is then only a
//! length.

use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi, RefIntoWasmAbi};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[allow(clippy::ptr_arg, clippy::unused_unit)]
#[wasm_bindgen(module = "tests/wasm/unit_values.js")]
extern "C" {
    #[wasm_bindgen(js_name = echo)]
    fn units(v: Vec<()>) -> Vec<()>;
    #[wasm_bindgen(js_name = echo)]
    fn boxed_units(v: Box<[()]>) -> Box<[()]>;
    #[wasm_bindgen(js_name = echo)]
    fn opt_units(v: Option<Vec<()>>) -> Option<Vec<()>>;
    #[wasm_bindgen(js_name = echo)]
    fn nullable_units(v: Nullable<Option<Vec<()>>>) -> Nullable<Option<Vec<()>>>;
    #[wasm_bindgen(js_name = echo)]
    fn nested_units(v: Vec<Vec<()>>) -> Vec<Vec<()>>;
    #[wasm_bindgen(js_name = echo)]
    fn opt_nested_units(v: Vec<Option<Vec<()>>>) -> Vec<Option<Vec<()>>>;

    #[wasm_bindgen(js_name = show)]
    fn show_vec(v: Vec<()>) -> String;
    #[wasm_bindgen(js_name = show)]
    fn show_slice(v: &[()]) -> String;
    #[wasm_bindgen(js_name = show)]
    fn show_vec_ref(v: &Vec<()>) -> String;
    #[wasm_bindgen(js_name = show)]
    fn show_opt_slice(v: Option<&[()]>) -> String;
    #[wasm_bindgen(js_name = show)]
    fn show_opt_vec_ref(v: Option<&Vec<()>>) -> String;
    #[wasm_bindgen(js_name = show)]
    fn show_slices(v: &[&[()]]) -> String;
    #[wasm_bindgen(js_name = show)]
    fn show_refs(v: &[&()]) -> String;
    #[wasm_bindgen(js_name = show)]
    fn show_value(v: &JsValue) -> String;

    #[wasm_bindgen(experimental_generic_mono, js_name = show)]
    fn generic_show<T: RefIntoWasmAbi>(v: &[T]) -> String;
    #[wasm_bindgen(experimental_generic_mono, js_name = show)]
    fn generic_show_vec<T>(v: Vec<T>) -> String
    where
        Vec<T>: IntoWasmAbi;
    #[wasm_bindgen(experimental_generic_mono, js_name = echo)]
    fn generic_echo<T>(v: Vec<T>) -> Vec<T>
    where
        Vec<T>: IntoWasmAbi + FromWasmAbi;

    #[wasm_bindgen(catch, js_name = unitsOf)]
    fn caught_units(kind: &str) -> Result<Vec<()>, JsValue>;
    #[wasm_bindgen(catch, js_name = unitsOf)]
    fn caught_nested_units(kind: &str) -> Result<Vec<Vec<()>>, JsValue>;

    #[wasm_bindgen(js_name = callWithUnits)]
    fn call_with_units(f: &mut dyn FnMut(Vec<()>, u32) -> Vec<()>) -> String;
    #[wasm_bindgen(js_name = callWithUnits)]
    fn call_owned_with_units(f: &Closure<dyn FnMut(Vec<()>, u32) -> Vec<()>>) -> String;
    #[wasm_bindgen(js_name = callWithWrongUnits)]
    fn call_with_wrong_units(f: &mut dyn FnMut(Vec<()>, u32) -> Vec<()>);

    #[wasm_bindgen(js_name = unitExports)]
    fn js_exports();
    #[wasm_bindgen(js_name = unitAsyncExports)]
    async fn js_async_exports();
}

fn n(len: usize) -> Vec<()> {
    vec![(); len]
}

#[wasm_bindgen_test]
fn an_import_is_given_and_gives_units() {
    assert_eq!(units(n(3)), n(3));
    assert_eq!(units(n(0)), n(0));
    assert_eq!(*boxed_units(n(2).into()), [(), ()]);
    assert_eq!(units(n(100_000)).len(), 100_000);

    assert_eq!(show_vec(n(3)), "[undefined,undefined,undefined]");
    assert_eq!(show_vec(n(0)), "[]");
    assert_eq!(
        show_value(&JsValue::from(n(3))),
        "[undefined,undefined,undefined]"
    );
    assert_eq!(show_value(&JsValue::from(n(0))), "[]");
    assert_eq!(
        show_value(&JsValue::from(n(2).into_boxed_slice())),
        "[undefined,undefined]"
    );
    assert_eq!(
        show_value(&JsValue::from(vec![n(1), n(0)])),
        "[[undefined],[]]"
    );
}

// An empty list is still a list: only the absent one is `undefined`.
#[wasm_bindgen_test]
fn an_optional_list_of_units_is_not_confused_with_none() {
    assert_eq!(opt_units(Some(n(2))), Some(n(2)));
    assert_eq!(opt_units(Some(n(0))), Some(n(0)));
    assert_eq!(opt_units(None), None);

    use Nullable::{Null, Value};
    assert_eq!(nullable_units(Value(Some(n(1)))), Value(Some(n(1))));
    assert_eq!(nullable_units(Value(Some(n(0)))), Value(Some(n(0))));
    assert_eq!(nullable_units(Value(None)), Value(None));
    assert_eq!(nullable_units(Null), Null);

    assert_eq!(show_opt_slice(Some(&[(), ()])), "[undefined,undefined]");
    assert_eq!(show_opt_slice(Some(&[])), "[]");
    assert_eq!(show_opt_slice(None), "undefined");
    assert_eq!(show_opt_vec_ref(Some(&n(1))), "[undefined]");
    assert_eq!(show_opt_vec_ref(Some(&n(0))), "[]");
    assert_eq!(show_opt_vec_ref(None), "undefined");
}

#[wasm_bindgen_test]
fn units_nest() {
    let given = vec![n(2), n(0), n(1)];
    assert_eq!(nested_units(given.clone()), given);
    assert_eq!(nested_units(vec![]), Vec::<Vec<()>>::new());
    let given = vec![Some(n(2)), None, Some(n(0))];
    assert_eq!(opt_nested_units(given.clone()), given);
}

#[wasm_bindgen_test]
fn units_are_lent() {
    assert_eq!(show_slice(&[(), ()]), "[undefined,undefined]");
    assert_eq!(show_slice(&[]), "[]");
    assert_eq!(show_vec_ref(&n(2)), "[undefined,undefined]");
    assert_eq!(
        show_slices(&[&[(), ()], &[], &[()]]),
        "[[undefined,undefined],[],[undefined]]"
    );
    assert_eq!(show_refs(&[&(), &()]), "[undefined,undefined]");
}

#[wasm_bindgen_test]
fn a_generic_import_takes_units() {
    assert_eq!(generic_show(&[(), ()]), "[undefined,undefined]");
    assert_eq!(generic_show::<()>(&[]), "[]");
    assert_eq!(generic_show(&[n(1), n(0)]), "[[undefined],[]]");
    assert_eq!(generic_show_vec(n(2)), "[undefined,undefined]");
    assert_eq!(generic_echo(n(3)), n(3));
    assert_eq!(generic_echo(vec![n(1), n(0)]), [n(1), n(0)]);
}

// What JS returns is checked as what it passes is, and with `catch` a value
// that isn't `undefined` is the import's error.
#[wasm_bindgen_test]
fn a_wrong_unit_returned_is_caught() {
    assert_eq!(caught_units("units").unwrap(), n(2));
    assert_eq!(caught_units("empty").unwrap(), n(0));
    assert_eq!(caught_units("holes").unwrap(), n(3));
    for kind in ["null", "number", "nested"] {
        let err = caught_units(kind).unwrap_err();
        let message = String::from(err.unchecked_into::<js_sys::Error>().message());
        assert_eq!(
            message, "array contains a value of the wrong type",
            "{kind}"
        );
    }
    assert_eq!(caught_nested_units("nested").unwrap(), [n(1), n(0)]);
    for kind in ["units", "nested null"] {
        assert!(caught_nested_units(kind).is_err(), "{kind}");
    }
}

#[wasm_bindgen_test]
fn a_closure_is_given_and_gives_units() {
    let mut reply = |v: Vec<()>, last: u32| {
        assert_eq!(last, 7);
        let mut v = v;
        v.push(());
        v
    };
    assert_eq!(
        call_with_units(&mut reply),
        "[undefined,undefined,undefined]"
    );
    let owned = Closure::new(reply);
    assert_eq!(
        call_owned_with_units(&owned),
        "[undefined,undefined,undefined]"
    );
    call_with_wrong_units(&mut |v, _| v);
}

#[wasm_bindgen]
pub fn unit_vec(v: Vec<()>) -> Vec<()> {
    v
}

#[wasm_bindgen]
pub fn unit_boxed(v: Box<[()]>) -> Box<[()]> {
    v
}

#[wasm_bindgen]
pub fn unit_opt(v: Option<Vec<()>>) -> Option<Vec<()>> {
    v
}

/// The length of the list, or -1 for none.
#[wasm_bindgen]
pub fn unit_opt_len(v: Option<Vec<()>>) -> i32 {
    v.map_or(-1, |v| v.len() as i32)
}

#[wasm_bindgen]
pub fn unit_nullable(v: Nullable<Option<Vec<()>>>) -> Nullable<Option<Vec<()>>> {
    v
}

#[wasm_bindgen]
pub fn unit_nested(v: Vec<Vec<()>>) -> Vec<Vec<()>> {
    v
}

#[wasm_bindgen]
pub fn unit_len(v: Vec<()>) -> usize {
    v.len()
}

#[wasm_bindgen]
pub fn unit_give(len: usize) -> Vec<()> {
    n(len)
}

#[wasm_bindgen]
pub fn unit_value(len: usize) -> JsValue {
    JsValue::from(n(len))
}

#[wasm_bindgen]
pub fn unit_beside(names: Vec<String>, units: Vec<()>, more: Vec<()>) -> String {
    format!("{}:{}:{}", names.join(","), units.len(), more.len())
}

#[wasm_bindgen]
pub async fn unit_async_give(len: usize) -> Vec<()> {
    n(len)
}

#[wasm_bindgen]
pub async fn unit_async_take(v: Vec<()>, last: u32) -> u32 {
    v.len() as u32 * 10 + last
}

#[wasm_bindgen(getter_with_clone)]
pub struct UnitRecord {
    pub units: Vec<()>,
    pub nested: Vec<Vec<()>>,
    pub maybe: Option<Vec<()>>,
}

#[wasm_bindgen]
impl UnitRecord {
    #[wasm_bindgen(constructor)]
    pub fn new(units: Vec<()>) -> UnitRecord {
        UnitRecord {
            units,
            nested: Vec::new(),
            maybe: None,
        }
    }

    pub fn count(&self, more: Vec<()>) -> usize {
        self.units.len() + more.len()
    }
}

#[wasm_bindgen_test]
fn exports_are_given_and_give_units() {
    js_exports();
}

#[wasm_bindgen_test]
async fn async_exports_are_given_and_give_units() {
    js_async_exports().await;
}
