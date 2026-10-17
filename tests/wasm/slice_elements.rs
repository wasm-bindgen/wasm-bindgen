//! A borrowed slice crosses as each element's own ABI, N times. Numbers and
//! JS handles are borrowed as a view of the slice's memory (their memory is
//! their ABI); every other element is encoded into a buffer of its ABIs,
//! which JS decodes with the element's own glue into a fresh `Array` and
//! frees. These tests cover every encoded element type, bare and under each
//! `slice` mode, with `Option`, empty and large slices, several slices in one
//! call, and the same shapes through `experimental_generic_mono`.

use js_sys::JsString;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum SliceColor {
    Red = 1,
    Green = 2,
    Blue = 40,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum SliceSigned {
    Low = -5,
    Zero = 0,
    High = 7,
}

// Deliberately named like the `describe` module's slice-mode marker, which
// the generated describe code glob-imports: user types must not resolve to it.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum SliceMode {
    Read = "read",
    Write = "write",
}

#[wasm_bindgen(module = "tests/wasm/slice_elements.js")]
extern "C" {
    #[wasm_bindgen(js_name = describe)]
    fn strings(v: &[String]) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn strs(v: &[&str]) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn bools(v: &[bool]) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn chars(v: &[char]) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn colors(v: &[SliceColor]) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn signed(v: &[SliceSigned]) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn modes(v: &[SliceMode]) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn opt_strings(v: Option<&[String]>) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn opt_bools(v: Option<&[bool]>) -> String;

    // The borrowed tiers, alongside for contrast.
    #[wasm_bindgen(js_name = describe)]
    fn handles(v: &[JsString]) -> String;
    #[wasm_bindgen(js_name = errorMessages)]
    fn errors(v: &[JsError]) -> String;

    #[wasm_bindgen(js_name = describeAll)]
    fn mixed(a: &[String], b: &[bool], c: &[u8], d: Option<&[char]>, e: &[SliceMode]) -> String;

    #[wasm_bindgen(js_name = retain)]
    fn retain_strings(v: &[String]);
    fn retained() -> String;
}

// The modes act on typed sequences only; an encoded slice is a fresh `Array`
// under every one of them.
#[wasm_bindgen(module = "tests/wasm/slice_elements.js")]
extern "C" {
    #[wasm_bindgen(js_name = describeAll)]
    fn strings_under_modes(
        #[wasm_bindgen(slice = view)] view: &[String],
        #[wasm_bindgen(slice = copy)] copy: &[String],
        #[wasm_bindgen(slice = array)] array: &[String],
        #[wasm_bindgen(slice = copy)] bools: &[bool],
    ) -> String;

    #[wasm_bindgen(slice_to_array, js_name = describe)]
    fn strings_slice_to_array(v: &[String]) -> String;
}

// Per instantiation, through the same element impls.
#[wasm_bindgen(module = "tests/wasm/slice_elements.js")]
extern "C" {
    #[wasm_bindgen(experimental_generic_mono, js_name = describe)]
    fn generic<T>(v: &[T]) -> String;

    #[wasm_bindgen(experimental_generic_mono, js_name = describe)]
    fn generic_opt<T>(v: Option<&[T]>) -> String;
}

#[wasm_bindgen_test]
fn strings_cross_as_their_own_abi() {
    let v = ["a".to_string(), String::new(), "héllo 🌍".to_string()];
    assert_eq!(strings(&v), r#"Array:["a","","héllo 🌍"]"#);
    assert_eq!(strs(&["x", "", "ü"]), r#"Array:["x","","ü"]"#);
}

// Under `enable-interning` an interned string's ABI is its cached JS value
// rather than its bytes, decoded by the element's own glue all the same.
#[wasm_bindgen_test]
fn interned_strings() {
    let interned = wasm_bindgen::intern("interned");
    assert_eq!(
        strs(&["plain", interned, "plain"]),
        r#"Array:["plain","interned","plain"]"#
    );
    let v = ["interned".to_string(), "plain".to_string()];
    assert_eq!(strings(&v), r#"Array:["interned","plain"]"#);
}

#[wasm_bindgen_test]
fn scalars_cross_as_their_own_abi() {
    assert_eq!(bools(&[true, false, true]), "Array:[true,false,true]");
    assert_eq!(chars(&['a', 'é', '🌍']), r#"Array:["a","é","🌍"]"#);
}

#[wasm_bindgen_test]
fn enums_cross_as_their_own_abi() {
    use SliceColor::*;
    assert_eq!(colors(&[Red, Blue, Green]), "Array:[1,40,2]");
    assert_eq!(
        signed(&[SliceSigned::Low, SliceSigned::Zero, SliceSigned::High]),
        "Array:[-5,0,7]"
    );
    assert_eq!(
        modes(&[SliceMode::Write, SliceMode::Read]),
        r#"Array:["write","read"]"#
    );
}

#[wasm_bindgen_test]
fn empty_and_option() {
    assert_eq!(strings(&[]), "Array:[]");
    assert_eq!(bools(&[]), "Array:[]");
    assert_eq!(opt_strings(Some(&["a".to_string()])), r#"Array:["a"]"#);
    assert_eq!(opt_strings(Some(&[])), "Array:[]");
    assert_eq!(opt_strings(None), "undefined");
    assert_eq!(opt_bools(Some(&[false])), "Array:[false]");
    assert_eq!(opt_bools(None), "undefined");
}

#[wasm_bindgen_test]
fn large_slices() {
    let v: Vec<String> = (0..1000).map(|i| i.to_string()).collect();
    let expected = format!(
        "Array:[{}]",
        (0..1000)
            .map(|i| format!("\"{i}\""))
            .collect::<Vec<_>>()
            .join(",")
    );
    assert_eq!(strings(&v), expected);
    let b: Vec<bool> = (0..1001).map(|i| i % 3 == 0).collect();
    assert_eq!(bools(&b).matches("true").count(), 334);
}

#[wasm_bindgen_test]
fn several_slices_in_one_call() {
    assert_eq!(
        mixed(
            &["s".to_string()],
            &[true],
            &[7],
            Some(&['c']),
            &[SliceMode::Read]
        ),
        r#"Array:["s"] Array:[true] Uint8Array:[7] Array:["c"] Array:["read"]"#
    );
    assert_eq!(
        mixed(&[], &[], &[], None, &[]),
        "Array:[] Array:[] Uint8Array:[] undefined Array:[]"
    );
}

#[wasm_bindgen_test]
fn the_array_is_fresh() {
    let mut v = vec!["kept".to_string()];
    retain_strings(&v);
    v[0].clear();
    v.push("more".to_string());
    drop(v);
    assert_eq!(retained(), r#"Array:["kept"]"#);
}

#[wasm_bindgen_test]
fn borrowed_tiers_are_unchanged() {
    let h = [JsString::from("a"), JsString::from("b")];
    assert_eq!(handles(&h), r#"Array:["a","b"]"#);
    assert_eq!(errors(&[JsError::new("x"), JsError::new("y")]), "x,y");
}

#[wasm_bindgen_test]
fn modes_leave_encoded_slices_alone() {
    let v = ["a".to_string()];
    assert_eq!(
        strings_under_modes(&v, &v, &v, &[true]),
        r#"Array:["a"] Array:["a"] Array:["a"] Array:[true]"#
    );
    assert_eq!(strings_slice_to_array(&v), r#"Array:["a"]"#);
}

#[wasm_bindgen_test]
fn per_mono_uses_the_same_element_abi() {
    assert_eq!(generic(&["a".to_string()]), r#"Array:["a"]"#);
    assert_eq!(generic(&["b"]), r#"Array:["b"]"#);
    assert_eq!(generic(&[true, false]), "Array:[true,false]");
    assert_eq!(generic(&['z']), r#"Array:["z"]"#);
    assert_eq!(generic(&[SliceColor::Blue]), "Array:[40]");
    assert_eq!(generic(&[SliceMode::Write]), r#"Array:["write"]"#);
    assert_eq!(generic(&[1u16, 2]), "Uint16Array:[1,2]");
    assert_eq!(generic_opt(Some(&[false][..])), "Array:[false]");
    assert_eq!(generic_opt::<String>(None), "undefined");
}
