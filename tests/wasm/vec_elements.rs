//! Owned vectors of elements whose memory is not their ABI cross as their
//! slices do, as a buffer of each element's own ABI, in both directions:
//! JS decodes the buffer into a fresh `Array` and frees it, or encodes an
//! `Array` into a buffer that Rust takes ownership of. These tests round-trip
//! each such element through imports and exports, with `Option`, `Box<[T]>`,
//! empty and large vectors.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VecMode {
    Read = "read",
    Write = "write",
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VecLevel {
    Low = -1,
    High = 7,
}

#[wasm_bindgen(module = "tests/wasm/vec_elements.js")]
extern "C" {
    #[wasm_bindgen(js_name = echo)]
    fn echo_bools(v: Vec<bool>) -> Vec<bool>;
    #[wasm_bindgen(js_name = echo)]
    fn echo_chars(v: Vec<char>) -> Vec<char>;
    #[wasm_bindgen(js_name = echo)]
    fn echo_modes(v: Vec<VecMode>) -> Vec<VecMode>;
    #[wasm_bindgen(js_name = echo)]
    fn echo_opt_bools(v: Option<Vec<bool>>) -> Option<Vec<bool>>;
    #[wasm_bindgen(js_name = echo)]
    fn echo_boxed_chars(v: Box<[char]>) -> Box<[char]>;
    #[wasm_bindgen(js_name = echo)]
    fn echo_levels(v: Vec<VecLevel>) -> Vec<VecLevel>;
    #[wasm_bindgen(js_name = echo)]
    fn echo_opt_levels(v: Option<Vec<VecLevel>>) -> Option<Vec<VecLevel>>;
    #[wasm_bindgen(js_name = echo)]
    fn echo_boxed_levels(v: Box<[VecLevel]>) -> Box<[VecLevel]>;

    #[wasm_bindgen(js_name = describe)]
    fn describe_bools(v: Vec<bool>) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn describe_strs(v: Vec<&str>) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn describe_modes(v: Option<Vec<VecMode>>) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn describe_levels(v: Vec<VecLevel>) -> String;

    #[wasm_bindgen(js_name = modesWithUnknown)]
    fn modes_with_unknown() -> Vec<VecMode>;

    #[wasm_bindgen(js_name = describe)]
    fn describe_value(v: &JsValue) -> String;

    #[wasm_bindgen(js_name = throwingElementFreesBuffer)]
    fn throwing_element_frees_buffer(memory: JsValue);

    #[wasm_bindgen(js_name = callExports)]
    fn call_exports();
}

#[wasm_bindgen]
pub fn vec_elements_not(v: Vec<bool>) -> Vec<bool> {
    v.into_iter().map(|b| !b).collect()
}

#[wasm_bindgen]
pub fn vec_elements_next_chars(v: Vec<char>) -> Vec<char> {
    v.into_iter()
        .map(|c| char::from_u32(c as u32 + 1).unwrap())
        .collect()
}

#[wasm_bindgen]
pub fn vec_elements_flip_modes(v: Option<Vec<VecMode>>) -> Option<Vec<VecMode>> {
    v.map(|v| {
        v.into_iter()
            .map(|m| match m {
                VecMode::Read => VecMode::Write,
                _ => VecMode::Read,
            })
            .collect()
    })
}

#[wasm_bindgen]
pub fn vec_elements_flip_levels(v: Option<Vec<VecLevel>>) -> Option<Vec<VecLevel>> {
    v.map(|v| {
        v.into_iter()
            .map(|l| match l {
                VecLevel::Low => VecLevel::High,
                VecLevel::High => VecLevel::Low,
            })
            .collect()
    })
}

#[wasm_bindgen]
pub fn vec_elements_boxed_levels(v: Box<[VecLevel]>) -> Box<[VecLevel]> {
    v
}

#[wasm_bindgen]
pub fn vec_elements_strs() -> Vec<&'static str> {
    vec!["a", "", "ü🌍"]
}

#[wasm_bindgen_test]
fn rust_to_js() {
    assert_eq!(describe_bools(vec![true, false]), "Array:[true,false]");
    assert_eq!(describe_bools(vec![]), "Array:[]");
    assert_eq!(describe_strs(vec!["x", "", "ü"]), r#"Array:["x","","ü"]"#);
    assert_eq!(
        describe_modes(Some(vec![VecMode::Write, VecMode::Read])),
        r#"Array:["write","read"]"#
    );
    assert_eq!(describe_modes(None), "undefined");
    assert_eq!(
        describe_levels(vec![VecLevel::Low, VecLevel::High]),
        "Array:[-1,7]"
    );
}

#[wasm_bindgen_test]
fn round_trips_through_js() {
    assert_eq!(echo_bools(vec![true, false, true]), [true, false, true]);
    assert_eq!(echo_bools(vec![]), [] as [bool; 0]);
    assert_eq!(echo_chars(vec!['a', 'ü', '🌍']), ['a', 'ü', '🌍']);
    assert_eq!(
        echo_modes(vec![VecMode::Read, VecMode::Write]),
        [VecMode::Read, VecMode::Write]
    );
    assert_eq!(echo_opt_bools(Some(vec![false])), Some(vec![false]));
    assert_eq!(echo_opt_bools(Some(vec![])), Some(vec![]));
    assert_eq!(echo_opt_bools(None), None);
    assert_eq!(&*echo_boxed_chars(Box::new(['z'])), ['z']);
    assert_eq!(
        echo_levels(vec![VecLevel::High, VecLevel::Low]),
        [VecLevel::High, VecLevel::Low]
    );
    assert_eq!(
        echo_opt_levels(Some(vec![VecLevel::Low])),
        Some(vec![VecLevel::Low])
    );
    assert_eq!(echo_opt_levels(None), None);
    assert_eq!(
        &*echo_boxed_levels(Box::new([VecLevel::Low])),
        [VecLevel::Low]
    );

    let many: Vec<char> = (0..100_000)
        .map(|i| char::from_u32(0x41 + i % 26).unwrap())
        .collect();
    assert_eq!(echo_chars(many.clone()), many);
}

// As for a string enum argument, an unknown string is the hidden invalid
// variant.
#[wasm_bindgen_test]
fn unknown_string_enum_values() {
    let modes = modes_with_unknown();
    assert_eq!(modes[0], VecMode::Read);
    assert!(!matches!(modes[1], VecMode::Read | VecMode::Write));
}

#[wasm_bindgen_test]
fn js_to_rust() {
    call_exports();
}

#[wasm_bindgen_test]
fn throwing_element() {
    throwing_element_frees_buffer(wasm_bindgen::memory());
}

#[wasm_bindgen_test]
fn into_js_value() {
    let describe = |v: JsValue| describe_value(&v);
    assert_eq!(describe(vec![true, false].into()), "Array:[true,false]");
    assert_eq!(describe(vec!['🌍'].into()), r#"Array:["🌍"]"#);
    assert_eq!(describe(vec![VecMode::Write].into()), r#"Array:["write"]"#);
    assert_eq!(describe(vec!["x"].into()), r#"Array:["x"]"#);
    assert_eq!(describe(vec![VecLevel::Low].into()), "Array:[-1]");
}
