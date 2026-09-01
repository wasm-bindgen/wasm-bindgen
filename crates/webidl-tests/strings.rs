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
