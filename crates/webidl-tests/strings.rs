//! String argument positions under typed generics.
//!
//! `[WbgGeneric]` (and every declaration in next_unstable mode) turns each
//! top-level string argument position into a generic parameter bounded by
//! `JsStringLike`, so callers pass `&str`, `String`, `JsString` or
//! `&JsString`, each crossing at its native wire format. Returns stay
//! concrete `String`. The JS side asserts every shape arrives as a string
//! value.

use crate::generated::*;
use js_sys::JsString;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn generic_string_arguments_cross_at_native_shapes() {
    let f = GenericStrings::new().unwrap();

    assert_eq!(f.echo("borrowed"), "borrowed");
    assert_eq!(f.echo(String::from("owned")), "owned");

    let handle = JsString::from("handle");
    assert_eq!(f.echo(&handle), "handle");
    assert_eq!(f.echo(handle), "handle");

    assert_eq!(f.join("a", JsString::from("b"), 3), "ab3");

    assert_eq!(GenericStrings::echo_static(JsString::from("s")), "s");
}

#[wasm_bindgen_test]
fn generic_string_attributes() {
    let f = GenericStrings::new().unwrap();

    f.set_title("t1");
    assert_eq!(f.title(), "t1");

    f.set_title(JsString::from("t2"));
    assert_eq!(f.title(), "t2");

    assert_eq!(f.nickname(), None);
    f.set_nickname(Some("n"));
    assert_eq!(f.nickname().as_deref(), Some("n"));
    f.set_nickname(None::<&str>);
    assert_eq!(f.nickname(), None);
}

#[wasm_bindgen_test]
fn generic_string_nullable_signature() {
    let f = GenericStrings::new().unwrap();

    assert_eq!(f.maybe(Some(JsString::from("v"))).as_deref(), Some("v"));
    assert_eq!(f.maybe(None::<&str>), None);
}

#[wasm_bindgen_test]
fn generic_string_catch() {
    let f = GenericStrings::new().unwrap();

    assert_eq!(f.try_echo("fine").unwrap(), "fine");
    assert!(f.try_echo("boom").is_err());
}

#[wasm_bindgen_test]
fn generic_string_dictionary() {
    let f = GenericStrings::new().unwrap();

    let dict = GenericStringDict::new("l1");
    assert_eq!(f.describe_dict(&dict), "l1:none");

    let dict = GenericStringDict::new(JsString::from("l2"));
    dict.set_note(String::from("n2"));
    assert_eq!(f.describe_dict(&dict), "l2:n2");

    assert_eq!(dict.get_label(), "l2");
    assert_eq!(dict.get_note().as_deref(), Some("n2"));
}

// Every string return also gains a `_js_string` variant handing back the JS
// string by handle, in all generation modes.
#[wasm_bindgen_test]
fn js_string_return_variants() {
    let f = GenericStrings::new().unwrap();

    assert_eq!(f.echo_js_string("x"), "x");
    assert_eq!(f.join_js_string("a", "b", 3), "ab3");

    f.set_title("t");
    assert_eq!(f.title_js_string(), "t");

    assert_eq!(f.nickname_js_string(), None);
    f.set_nickname(Some("n"));
    assert_eq!(f.nickname_js_string().unwrap(), "n");

    assert_eq!(f.maybe_js_string(Some("v")).unwrap(), "v");
    assert_eq!(f.maybe_js_string(None::<&str>), None);

    assert_eq!(f.try_echo_js_string("fine").unwrap(), "fine");
    assert!(f.try_echo_js_string("boom").is_err());

    assert_eq!(GenericStrings::echo_static_js_string("s"), "s");

    let dict = GenericStringDict::new("l");
    dict.set_note("n");
    assert_eq!(dict.get_label_js_string(), "l");
    assert_eq!(dict.get_note_js_string().unwrap(), "n");

    let p = PlainStrings::new().unwrap();
    p.set_title("pt");
    assert_eq!(p.title_js_string(), "pt");
    assert_eq!(p.echo_js_string("e"), "e");
}

#[wasm_bindgen_test]
fn js_string_return_variants_in_namespaces() {
    assert_eq!(string_ns::JsNamespaceStringNs::tag(), "ns");
    assert_eq!(string_ns::JsNamespaceStringNs::tag_js_string(), "ns");
    assert_eq!(string_ns::concat("a", "b"), "ab");
    assert_eq!(string_ns::concat_js_string("a", "b"), "ab");
}

// In stable modes the plain interface keeps the legacy concrete signatures;
// in next_unstable mode its arguments are genericised like everything else.
#[cfg(not(wbg_next_unstable))]
#[wasm_bindgen_test]
fn plain_strings_stay_concrete_in_stable_modes() {
    let f = PlainStrings::new().unwrap();
    f.set_title("t");
    assert_eq!(f.title(), "t");
    assert_eq!(f.echo("e"), "e");
}

#[cfg(wbg_next_unstable)]
#[wasm_bindgen_test]
fn plain_strings_genericised_in_next_mode() {
    let f = PlainStrings::new().unwrap();
    f.set_title(JsString::from("t"));
    assert_eq!(f.title(), "t");
    assert_eq!(f.echo(JsString::from("e")), "e");
}
