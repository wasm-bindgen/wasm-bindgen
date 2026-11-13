//! A borrowed slice crosses as each element's own ABI, N times. Numbers and
//! JS handles are borrowed as a view of the slice's memory (their memory is
//! their ABI); every other element is encoded into a buffer of its ABIs,
//! which JS decodes with the element's own glue into a fresh `Array` and
//! frees. These tests cover every encoded element type, bare and under each
//! `slice` mode, with `Option`, empty and large slices, several slices in one
//! call, and the same shapes through `experimental_generic_mono`.

use std::borrow::Cow;

use js_sys::JsString;
use wasm_bindgen::convert::RefIntoWasmAbi;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsStringLike;
use wasm_bindgen::SafeInteger;
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

// Deliberately named like the `describe` module's seq-mode marker, which the
// generated describe code glob-imports: user types must not resolve to it.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum SeqMode {
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
    fn modes(v: &[SeqMode]) -> String;
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
    fn mixed(a: &[String], b: &[bool], c: &[u8], d: Option<&[char]>, e: &[SeqMode]) -> String;

    #[wasm_bindgen(js_name = retain)]
    fn retain_strings(v: &[String]);
    fn retained() -> String;

    #[wasm_bindgen(js_name = describe)]
    fn string_refs(v: &[&String]) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn boxed_str_refs(v: &[&Box<str>]) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn cow_refs(v: &[&Cow<str>]) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn handle_refs(v: &[&JsString]) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn opt_handle_refs(v: Option<&[&JsString]>) -> String;
}

// The modes act on typed sequences only: inherited, they leave an encoded
// slice a fresh `Array`, as without a mode.
#[wasm_bindgen(module = "tests/wasm/slice_elements.js", slice = copy)]
extern "C" {
    #[wasm_bindgen(js_name = describeAll)]
    fn strings_under_copy(strings: &[String], bools: &[bool]) -> String;
    #[wasm_bindgen(slice = array, js_name = describeAll)]
    fn strings_under_array(strings: &[String], bools: &[bool]) -> String;

    #[wasm_bindgen(slice_to_array, js_name = describe)]
    fn strings_slice_to_array(v: &[String]) -> String;
}

// Any imported type is a handle, so its slices are borrowed in place.
#[wasm_bindgen(module = "tests/wasm/slice_elements.js")]
extern "C" {
    pub type Thing;
    #[wasm_bindgen(js_name = makeThing)]
    fn make_thing(id: u32) -> Thing;
    #[wasm_bindgen(js_name = thingIds)]
    fn thing_ids(v: &[Thing]) -> String;
    #[wasm_bindgen(js_name = thingIds)]
    fn opt_thing_ids(v: Option<&[Thing]>) -> String;
    #[wasm_bindgen(js_name = thingIds, slice = array)]
    fn thing_ids_array(v: &[Thing]) -> String;
    #[wasm_bindgen(experimental_generic_mono, js_name = thingIds)]
    fn generic_thing_ids<T: RefIntoWasmAbi>(v: &[T]) -> String;
}

// Per instantiation, through the same element impls.
#[wasm_bindgen(module = "tests/wasm/slice_elements.js")]
extern "C" {
    #[wasm_bindgen(experimental_generic_mono, js_name = describe)]
    fn generic<T: RefIntoWasmAbi>(v: &[T]) -> String;

    #[wasm_bindgen(experimental_generic_mono, js_name = describe)]
    fn generic_opt<T: RefIntoWasmAbi>(v: Option<&[T]>) -> String;

    #[wasm_bindgen(experimental_generic_mono, js_name = describe)]
    fn js_strings(v: &[impl JsStringLike]) -> String;
}

// `JsStringLike` alone proves the slice ABI, so plain Rust code can forward.
fn forward_js_strings(v: &[impl JsStringLike]) -> String {
    js_strings(v)
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
        modes(&[SeqMode::Write, SeqMode::Read]),
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
            &[SeqMode::Read]
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
    for under_mode in [strings_under_copy, strings_under_array] {
        assert_eq!(under_mode(&v, &[true]), r#"Array:["a"] Array:[true]"#);
    }
    assert_eq!(strings_slice_to_array(&v), r#"Array:["a"]"#);
}

#[wasm_bindgen_test]
fn per_mono_uses_the_same_element_abi() {
    assert_eq!(generic(&["a".to_string()]), r#"Array:["a"]"#);
    assert_eq!(generic(&["b"]), r#"Array:["b"]"#);
    assert_eq!(generic(&[true, false]), "Array:[true,false]");
    assert_eq!(generic(&['z']), r#"Array:["z"]"#);
    assert_eq!(generic(&[SliceColor::Blue]), "Array:[40]");
    assert_eq!(generic(&[SeqMode::Write]), r#"Array:["write"]"#);
    assert_eq!(generic(&[1u16, 2]), "Uint16Array:[1,2]");
    assert_eq!(generic_opt(Some(&[false][..])), "Array:[false]");
    assert_eq!(generic_opt::<String>(None), "undefined");
}

#[wasm_bindgen_test]
fn string_shape_refs() {
    let owned = ["a".to_string(), "é".to_string()];
    assert_eq!(string_refs(&[&owned[0], &owned[1]]), r#"Array:["a","é"]"#);
    let boxed: [Box<str>; 2] = ["b".into(), "".into()];
    assert_eq!(boxed_str_refs(&[&boxed[0], &boxed[1]]), r#"Array:["b",""]"#);
    let cows = [Cow::Borrowed("c"), Cow::Owned("d".to_string())];
    assert_eq!(cow_refs(&[&cows[0], &cows[1]]), r#"Array:["c","d"]"#);
}

#[wasm_bindgen_test]
fn handle_refs_are_borrowed() {
    let h = [JsString::from("x"), JsString::from("y")];
    assert_eq!(
        handle_refs(&[&h[0], &h[1], &h[0]]),
        r#"Array:["x","y","x"]"#
    );
    assert_eq!(handle_refs(&[]), "Array:[]");
    assert_eq!(opt_handle_refs(Some(&[&h[1]])), r#"Array:["y"]"#);
    assert_eq!(opt_handle_refs(None), "undefined");
    // Borrowed, not moved: the handles are still usable.
    assert_eq!(handles(&h), r#"Array:["x","y"]"#);
}

#[wasm_bindgen_test]
fn every_js_string_like_slice() {
    let owned = ["a".to_string()];
    let boxed: Box<str> = "b".into();
    let cow = Cow::Borrowed("c");
    let js = JsString::from("d");
    assert_eq!(forward_js_strings(&owned), r#"Array:["a"]"#);
    assert_eq!(forward_js_strings(&["a"]), r#"Array:["a"]"#);
    assert_eq!(forward_js_strings(&[&owned[0]]), r#"Array:["a"]"#);
    assert_eq!(forward_js_strings(&[&boxed]), r#"Array:["b"]"#);
    assert_eq!(forward_js_strings(&[&cow]), r#"Array:["c"]"#);
    assert_eq!(forward_js_strings(&[js.clone()]), r#"Array:["d"]"#);
    assert_eq!(forward_js_strings(&[&js]), r#"Array:["d"]"#);
}

#[wasm_bindgen_test]
fn imported_types_are_borrowed_handles() {
    let things = [make_thing(1), make_thing(2)];
    assert_eq!(thing_ids(&things), "Array:1,2");
    assert_eq!(thing_ids(&[]), "Array:");
    assert_eq!(opt_thing_ids(Some(&things)), "Array:1,2");
    assert_eq!(opt_thing_ids(None), "undefined");
    assert_eq!(thing_ids_array(&things), "Array:1,2");
    assert_eq!(generic_thing_ids(&things), "Array:1,2");
    // Borrowed, not moved: the handles are still usable.
    assert_eq!(thing_ids(&things), "Array:1,2");
}

macro_rules! shapes {
    ($($name:ident($ty:ty);)*) => {
        #[allow(clippy::ptr_arg, clippy::borrowed_box)]
        #[wasm_bindgen(module = "tests/wasm/slice_elements.js")]
        extern "C" {
            $(
                #[wasm_bindgen(js_name = shape)]
                fn $name(v: $ty) -> String;
            )*
        }
    };
}

// A sequence lends each of its elements, and a reference lends as itself, so
// an element may be a reference, or hold one.
shapes! {
    u32_refs(&[&u32]);
    opt_u32_refs(&[Option<&u32>]);
    null_u32_refs(&[Nullable<&u32>]);
    vec_u32_refs(&Vec<&u32>);
    f64_refs(&[&f64]);
    opt_f64_refs(&[Option<&f64>]);
    null_i64_refs(&[Nullable<&i64>]);
    safe_refs(&[&SafeInteger<i64>]);
    opt_safe_refs(&[Option<&SafeInteger<i64>>]);
    null_safe_refs(&[Nullable<&SafeInteger<i64>>]);
    opt_bool_refs(&[Option<&bool>]);
    null_char_refs(&[Nullable<&char>]);
    opt_str_refs(&[Option<&str>]);
    null_str_refs(&[Nullable<&str>]);
    vec_str_refs(&Vec<&str>);
    opt_string_refs(&[Option<&String>]);
    null_string_refs(&[Nullable<&String>]);
    vec_string_refs(&Vec<&String>);
    value_refs(&[&JsValue]);
    opt_value_refs(&[Option<&JsValue>]);
    null_value_refs(&[Nullable<&JsValue>]);
    vec_value_refs(&Vec<&JsValue>);
    thing_refs(&[&Thing]);
    opt_thing_refs(&[Option<&Thing>]);
    null_thing_refs(&[Nullable<&Thing>]);
    vec_thing_refs(&Vec<&Thing>);
    color_refs(&[&SliceColor]);
    opt_color_refs(&[Option<&SliceColor>]);
    null_color_refs(&[Nullable<&SliceColor>]);
    vec_color_refs(&Vec<&SliceColor>);
    null_opt_string_refs(&[Nullable<Option<&String>>]);

    ref_u32(&&u32);
    opt_ref_f64(Option<&&f64>);
    null_ref_str(Nullable<&&str>);
    ref_bytes(&&[u8]);
    opt_ref_bytes(Option<&&[u8]>);
    ref_strings(&&[String]);
}

#[wasm_bindgen_test]
fn elements_lent_by_reference() {
    use Nullable::{Null, Value};

    assert_eq!(u32_refs(&[&1, &u32::MAX]), "[1,4294967295]");
    assert_eq!(opt_u32_refs(&[Some(&1), None]), "[1,undefined]");
    assert_eq!(null_u32_refs(&[Value(&1), Null]), "[1,null]");
    assert_eq!(vec_u32_refs(&vec![&1, &2]), "[1,2]");
    assert_eq!(f64_refs(&[&0.5, &-1.0]), "[0.5,-1]");
    assert_eq!(opt_f64_refs(&[Some(&0.5), None]), "[0.5,undefined]");
    let safe = SafeInteger::<i64>::from(-7);
    assert_eq!(safe_refs(&[&safe]), "[-7]");
    assert_eq!(opt_safe_refs(&[Some(&safe), None]), "[-7,undefined]");
    assert_eq!(null_safe_refs(&[Value(&safe), Null]), "[-7,null]");
    assert_eq!(
        null_i64_refs(&[Value(&i64::MIN), Null]),
        "[-9223372036854775808n,null]"
    );
    assert_eq!(
        opt_bool_refs(&[Some(&false), None, Some(&true)]),
        "[false,undefined,true]"
    );
    assert_eq!(null_char_refs(&[Value(&'🌍'), Null]), r#"["🌍",null]"#);

    assert_eq!(
        opt_str_refs(&[Some("a"), None, Some("")]),
        r#"["a",undefined,""]"#
    );
    assert_eq!(null_str_refs(&[Value("a"), Null]), r#"["a",null]"#);
    assert_eq!(vec_str_refs(&vec!["a", "é"]), r#"["a","é"]"#);
    let owned = ["a".to_string(), String::new()];
    assert_eq!(
        opt_string_refs(&[Some(&owned[0]), None, Some(&owned[1])]),
        r#"["a",undefined,""]"#
    );
    assert_eq!(null_string_refs(&[Value(&owned[0]), Null]), r#"["a",null]"#);
    assert_eq!(vec_string_refs(&owned.iter().collect()), r#"["a",""]"#);
    assert_eq!(
        null_opt_string_refs(&[Value(Some(&owned[0])), Null, Value(None)]),
        r#"["a",null,undefined]"#
    );

    let values = [JsValue::from(1), JsValue::from("a"), JsValue::NULL];
    assert_eq!(
        value_refs(&[&values[0], &values[1], &values[2]]),
        r#"[1,"a",null]"#
    );
    assert_eq!(
        opt_value_refs(&[Some(&values[1]), None]),
        r#"["a",undefined]"#
    );
    assert_eq!(null_value_refs(&[Value(&values[0]), Null]), "[1,null]");
    assert_eq!(vec_value_refs(&values.iter().collect()), r#"[1,"a",null]"#);
    let things = [make_thing(1), make_thing(2)];
    assert_eq!(
        thing_refs(&[&things[1], &things[0]]),
        r#"[{"id":2},{"id":1}]"#
    );
    assert_eq!(
        opt_thing_refs(&[None, Some(&things[0])]),
        r#"[undefined,{"id":1}]"#
    );
    assert_eq!(
        null_thing_refs(&[Null, Value(&things[0])]),
        r#"[null,{"id":1}]"#
    );
    assert_eq!(
        vec_thing_refs(&things.iter().collect()),
        r#"[{"id":1},{"id":2}]"#
    );
    // Lent, not given: each handle is still Rust's.
    assert_eq!(thing_ids(&things), "Array:1,2");
    assert_eq!(value_refs(&[&values[1]]), r#"["a"]"#);

    use SliceColor::{Blue, Red};
    assert_eq!(color_refs(&[&Red, &Blue]), "[1,40]");
    assert_eq!(opt_color_refs(&[Some(&Blue), None]), "[40,undefined]");
    assert_eq!(null_color_refs(&[Value(&Red), Null]), "[1,null]");
    assert_eq!(vec_color_refs(&vec![&Blue]), "[40]");

    assert_eq!(u32_refs(&[]), "[]");
    assert_eq!(opt_value_refs(&[]), "[]");
    assert_eq!(vec_string_refs(&vec![]), "[]");

    // A reference to a reference is that reference wherever it crosses.
    assert_eq!(ref_u32(&&7), "7");
    assert_eq!(opt_ref_f64(Some(&&0.5)), "0.5");
    assert_eq!(opt_ref_f64(None), "undefined");
    assert_eq!(null_ref_str(Value(&"a")), r#""a""#);
    assert_eq!(null_ref_str(Null), "null");
    assert_eq!(ref_bytes(&&[1, 2][..]), "Uint8Array[1,2]");
    assert_eq!(opt_ref_bytes(Some(&&[1][..])), "Uint8Array[1]");
    assert_eq!(opt_ref_bytes(None), "undefined");
    assert_eq!(ref_strings(&&owned[..]), r#"["a",""]"#);

    assert_eq!(generic(&[&1u32, &2]), "Array:[1,2]");
    assert_eq!(generic(&[Some(&values[1]), None]), r#"Array:["a",null]"#);
    assert_eq!(generic(&[Value(&owned[0]), Null]), r#"Array:["a",null]"#);
    assert_eq!(generic_opt(Some(&[&Red][..])), "Array:[1]");
}

// A sequence that another lends is lent in turn, to any depth, and arrives as
// the sequence it is.
shapes! {
    byte_slices(&[&[u8]]);
    string_slices(&[&[String]]);
    opt_js_string_slices(&[&[Option<JsString>]]);
    opt_byte_slices(&[Option<&[u8]>]);
    null_float_slices(&[Nullable<&[f64]>]);
    vec_refs(&[&Vec<u32>]);
    boxed_refs(&[&Box<[String]>]);
    vec_of_byte_slices(&Vec<&[u8]>);
    byte_slices_deep(&[&[&[u8]]]);
    string_refs_deep(&[&[&String]]);
    opt_string_slices_deep(&[Vec<Option<&[String]>>]);
}

#[wasm_bindgen_test]
fn sequences_lent_by_reference() {
    use Nullable::{Null, Value};

    assert_eq!(
        byte_slices(&[&[1, 2], &[], &[3]]),
        "[Uint8Array[1,2],Uint8Array[],Uint8Array[3]]"
    );
    assert_eq!(byte_slices(&[]), "[]");
    let a = ["a".to_string(), "b".to_string()];
    let c = ["c".to_string()];
    assert_eq!(string_slices(&[&a, &[], &c]), r#"[["a","b"],[],["c"]]"#);
    let js = [Some(JsString::from("x")), None];
    assert_eq!(opt_js_string_slices(&[&js, &[]]), r#"[["x",undefined],[]]"#);
    // Lent, not given: the handle is still Rust's.
    assert_eq!(js[0].as_ref().unwrap(), "x");
    assert_eq!(
        opt_byte_slices(&[Some(&[1]), None, Some(&[])]),
        "[Uint8Array[1],undefined,Uint8Array[]]"
    );
    assert_eq!(
        null_float_slices(&[Value(&[0.5]), Null]),
        "[Float64Array[0.5],null]"
    );
    let numbers = vec![1, 2];
    assert_eq!(
        vec_refs(&[&numbers, &vec![]]),
        "[Uint32Array[1,2],Uint32Array[]]"
    );
    let boxed: Box<[String]> = a.to_vec().into();
    assert_eq!(boxed_refs(&[&boxed]), r#"[["a","b"]]"#);
    assert_eq!(vec_of_byte_slices(&vec![&[7][..]]), "[Uint8Array[7]]");

    assert_eq!(
        byte_slices_deep(&[&[&[1], &[2, 3]], &[], &[&[]]]),
        "[[Uint8Array[1],Uint8Array[2,3]],[],[Uint8Array[]]]"
    );
    assert_eq!(
        string_refs_deep(&[&[&a[0]], &[&c[0], &a[1]]]),
        r#"[["a"],["c","b"]]"#
    );
    assert_eq!(
        opt_string_slices_deep(&[vec![Some(&a), None], vec![]]),
        r#"[[["a","b"],undefined],[]]"#
    );

    assert_eq!(
        generic(&[&[1u8, 2][..], &[]]),
        r#"Array:[{"0":1,"1":2},{}]"#
    );
    assert_eq!(generic(&[&a[..]]), r#"Array:[["a","b"]]"#);
}

// A borrowed vector is absent as the slice of it is.
shapes! {
    opt_vec_ref(Option<&Vec<u32>>);
    opt_string_vec_ref(Option<&Vec<String>>);
    opt_value_vec_ref(Option<&Vec<JsValue>>);
    null_vec_ref(Nullable<&Vec<u32>>);
    null_string_vec_ref(Nullable<&Vec<String>>);
    opt_boxed_ref(Option<&Box<[f64]>>);
    opt_string_boxed_ref(Option<&Box<[String]>>);
    null_opt_vec_ref(Nullable<Option<&Vec<u32>>>);
    ref_opt_vec(&Option<Vec<u32>>);
    opt_vecs(&[Option<Vec<u32>>]);
    null_string_vecs(&[Nullable<Vec<String>>]);
    opt_boxed_slices(&[Option<Box<[bool]>>]);
    opt_vec_refs(&[Option<&Vec<String>>]);
}

#[wasm_bindgen_test]
fn optional_borrowed_vectors() {
    use Nullable::{Null, Value};

    let numbers = vec![1, 2];
    let strings = vec!["a".to_string(), String::new()];
    assert_eq!(opt_vec_ref(Some(&numbers)), "Uint32Array[1,2]");
    assert_eq!(opt_vec_ref(Some(&vec![])), "Uint32Array[]");
    assert_eq!(opt_vec_ref(None), "undefined");
    assert_eq!(opt_string_vec_ref(Some(&strings)), r#"["a",""]"#);
    assert_eq!(opt_string_vec_ref(Some(&vec![])), "[]");
    assert_eq!(opt_string_vec_ref(None), "undefined");
    let values = vec![JsValue::from(1), JsValue::NULL];
    assert_eq!(opt_value_vec_ref(Some(&values)), "[1,null]");
    assert_eq!(opt_value_vec_ref(None), "undefined");
    // Lent, not given: the handles are still Rust's.
    assert_eq!(values[0], 1);

    assert_eq!(null_vec_ref(Value(&numbers)), "Uint32Array[1,2]");
    assert_eq!(null_vec_ref(Null), "null");
    assert_eq!(null_string_vec_ref(Value(&strings)), r#"["a",""]"#);
    assert_eq!(null_string_vec_ref(Null), "null");
    assert_eq!(opt_boxed_ref(Some(&vec![0.5].into())), "Float64Array[0.5]");
    assert_eq!(opt_boxed_ref(None), "undefined");
    assert_eq!(
        opt_string_boxed_ref(Some(&strings.clone().into())),
        r#"["a",""]"#
    );
    assert_eq!(opt_string_boxed_ref(None), "undefined");
    assert_eq!(null_opt_vec_ref(Value(Some(&numbers))), "Uint32Array[1,2]");
    assert_eq!(null_opt_vec_ref(Value(None)), "undefined");
    assert_eq!(null_opt_vec_ref(Null), "null");
    assert_eq!(ref_opt_vec(&Some(numbers.clone())), "Uint32Array[1,2]");
    assert_eq!(ref_opt_vec(&None), "undefined");

    assert_eq!(
        opt_vecs(&[Some(numbers.clone()), None, Some(vec![])]),
        "[Uint32Array[1,2],undefined,Uint32Array[]]"
    );
    assert_eq!(
        null_string_vecs(&[Value(strings.clone()), Null, Value(vec![])]),
        r#"[["a",""],null,[]]"#
    );
    assert_eq!(
        opt_boxed_slices(&[Some(vec![true].into()), None]),
        "[[true],undefined]"
    );
    assert_eq!(
        opt_vec_refs(&[Some(&strings), None]),
        r#"[["a",""],undefined]"#
    );
    assert_eq!(numbers, [1, 2]);
    assert_eq!(strings, ["a", ""]);
}
