//! Any type that crosses works as the element of a sequence. The table at the
//! end of this file is that claim: a row is an element type, a column is a
//! position a sequence of it can take, and `-` is a position that does not
//! work, with its reason on the `hole:` line above the row.
//!
//! JS is generic over the element. It renders what it was given, reverses
//! it, or hands back what an export gave it, so the values in the table are
//! the whole of a row.

use js_sys::JsString;
use paste::paste;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

use crate::third_party_abi::{point, Point};

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct MatrixItem {
    id: u32,
}

#[wasm_bindgen]
impl MatrixItem {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u32 {
        self.id
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MatrixLevel {
    Low = 1,
    High = 2,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MatrixName {
    A = "a",
    B = "b",
}

/// A dynamic union: any of its variants' JS values.
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub enum MatrixUnion {
    Loading = "loading",
    Count(u32),
    Name(MatrixName),
}

#[wasm_bindgen(module = "tests/wasm/element_matrix.js")]
extern "C" {
    #[derive(Clone, Debug, PartialEq)]
    pub type MatrixThing;

    fn thing(id: u32) -> MatrixThing;
    #[wasm_bindgen(js_name = show)]
    fn show_value(v: &JsValue) -> String;
    #[wasm_bindgen(js_name = exportRet)]
    fn js_export_ret(row: &str, view: &str);
    #[wasm_bindgen(js_name = exportArg)]
    fn js_export_arg(row: &str);
    #[wasm_bindgen(js_name = exportOption)]
    fn js_export_option(row: &str, view: &str);
    #[wasm_bindgen(js_name = exportBad)]
    fn js_export_bad(row: &str);
}

fn s(v: &str) -> String {
    v.into()
}

fn js(v: &str) -> JsString {
    v.into()
}

fn item(id: u32) -> MatrixItem {
    MatrixItem { id }
}

use MatrixLevel::{High, Low};
use MatrixName::{A, B};
use Nullable::{Null, Value};

/// The samples as given, reversed, or none of them.
fn arrange<T>(mut samples: Vec<T>, mode: u8) -> Vec<T> {
    match mode {
        0 => {}
        1 => samples.reverse(),
        _ => samples.clear(),
    }
    samples
}

fn empty(view: &str) -> String {
    format!("{}]", &view[..=view.find('[').unwrap()])
}

macro_rules! cell {
    ($column:ident - $($row:tt)*) => {};

    (export_arg + $name:ident, $ty:ty, $view:literal) => {
        paste! {
            #[wasm_bindgen]
            pub fn [<em_ $name _take>](v: Vec<$ty>, mode: u8) {
                assert_eq!(v, arrange(samples(), mode));
            }

            #[wasm_bindgen_test]
            fn export_arg() {
                js_export_arg(stringify!($name));
            }
        }
    };

    (export_ret + $name:ident, $ty:ty, $view:literal) => {
        paste! {
            #[wasm_bindgen]
            pub fn [<em_ $name _give>](mode: u8) -> Vec<$ty> {
                arrange(samples(), mode)
            }

            #[wasm_bindgen_test]
            fn export_ret() {
                js_export_ret(stringify!($name), $view);
            }
        }
    };

    (import_arg + $name:ident, $ty:ty, $view:literal) => {
        #[wasm_bindgen(module = "tests/wasm/element_matrix.js")]
        extern "C" {
            #[wasm_bindgen(js_name = show)]
            fn show_vec(v: Vec<$ty>) -> String;
        }

        #[wasm_bindgen_test]
        fn import_arg() {
            assert_eq!(show_vec(samples()), $view);
            assert_eq!(show_vec(vec![]), empty($view));
        }
    };

    (import_ret + $name:ident, $ty:ty, $view:literal) => {
        #[wasm_bindgen(module = "tests/wasm/element_matrix.js")]
        extern "C" {
            #[wasm_bindgen(js_name = reversed)]
            fn reversed(v: Vec<$ty>) -> Vec<$ty>;
        }

        #[wasm_bindgen_test]
        fn import_ret() {
            assert_eq!(reversed(samples()), arrange(samples(), 1));
            assert_eq!(reversed(vec![]), arrange(samples(), 2));
        }
    };

    (import_slice + $name:ident, $ty:ty, $view:literal) => {
        #[wasm_bindgen(module = "tests/wasm/element_matrix.js")]
        extern "C" {
            #[wasm_bindgen(js_name = show)]
            fn show_slice(v: &[$ty]) -> String;
        }

        #[wasm_bindgen_test]
        fn import_slice() {
            assert_eq!(show_slice(&samples()), $view);
            assert_eq!(show_slice(&[]), empty($view));
        }
    };

    (option + $name:ident, $ty:ty, $view:literal) => {
        paste! {
            #[wasm_bindgen(module = "tests/wasm/element_matrix.js")]
            extern "C" {
                #[wasm_bindgen(js_name = reversed)]
                fn reversed_opt(v: Option<Vec<$ty>>) -> Option<Vec<$ty>>;
            }

            /// Checks a list it is given and returns none, or gives one.
            #[wasm_bindgen]
            pub fn [<em_ $name _opt>](v: Option<Vec<$ty>>, mode: u8) -> Option<Vec<$ty>> {
                match v {
                    Some(v) => {
                        assert_eq!(v, arrange(samples(), mode));
                        None
                    }
                    None => (mode < 3).then(|| arrange(samples(), mode)),
                }
            }

            #[wasm_bindgen_test]
            fn option() {
                assert_eq!(reversed_opt(Some(samples())), Some(arrange(samples(), 1)));
                assert_eq!(reversed_opt(Some(vec![])), Some(arrange(samples(), 2)));
                assert_eq!(reversed_opt(None), None);
                js_export_option(stringify!($name), $view);
            }
        }
    };

    (bad + $name:ident, $ty:ty, $view:literal) => {
        #[wasm_bindgen_test]
        fn bad() {
            js_export_bad(stringify!($name));
        }
    };

    (js_value + $name:ident, $ty:ty, $view:literal) => {
        #[wasm_bindgen_test]
        fn js_value() {
            assert_eq!(show_value(&JsValue::from(samples())), $view);
            assert_eq!(show_value(&JsValue::from(Vec::<$ty>::new())), empty($view));
        }
    };
}

macro_rules! matrix {
    ($(
        $name:ident [$ea:tt $er:tt $ia:tt $ir:tt $sl:tt $op:tt $jv:tt $bd:tt]:
        $ty:ty = [$($sample:expr),*] => $view:literal;
    )*) => { $(
        pub mod $name {
            use super::*;

            fn samples() -> Vec<$ty> {
                vec![$($sample),*]
            }

            cell!(export_arg $ea $name, $ty, $view);
            cell!(export_ret $er $name, $ty, $view);
            cell!(import_arg $ia $name, $ty, $view);
            cell!(import_ret $ir $name, $ty, $view);
            cell!(import_slice $sl $name, $ty, $view);
            cell!(option $op $name, $ty, $view);
            cell!(js_value $jv $name, $ty, $view);
            cell!(bad $bd $name, $ty, $view);
        }
    )* };
}

// Columns, each over `Vec<E>` unless said otherwise:
//   ea  export argument        ia  import argument     sl  import argument `&[E]`
//   er  export return          ir  import return       op  `Option<Vec<E>>`, both ways
//   jv  `JsValue::from`     bd  a value JS passes that is rejected, for an
//                               element that is checked, leaves the rest with JS
//
// `ea` is fed by `er`, and `ir` by `ia`: JS has no other source of a row's
// values.
#[rustfmt::skip]
matrix! {
//  row               ea er ia ir sl op jv bd   element                    samples => as JS sees them
    of_u8             [+  +  +  +  +  +  +  -]: u8                       = [0, 128, 255] => "Uint8Array[0,128,255]";
    of_u32            [+  +  +  +  +  +  +  -]: u32                      = [0, 7, u32::MAX] => "Uint32Array[0,7,4294967295]";
    of_i64            [+  +  +  +  +  +  +  -]: i64                      = [-1, 0, i64::MAX] => "BigInt64Array[-1n,0n,9223372036854775807n]";
    of_f64            [+  +  +  +  +  +  +  -]: f64                      = [0.5, -1.0, 1e100] => "Float64Array[0.5,-1,1e+100]";
    of_bool           [+  +  +  +  +  +  +  -]: bool                     = [true, false, false] => "[true,false,false]";
    of_char           [+  +  +  +  +  +  +  -]: char                     = ['a', '🌍', 'z'] => r#"["a","🌍","z"]"#;
    of_string         [+  +  +  +  +  +  +  +]: String                   = [s("a"), s(""), s("🌍")] => r#"["a","","🌍"]"#;
    // A borrowed string is lent, so it only goes to JS.
    of_str            [-  +  +  -  +  -  +  -]: &'static str             = ["a", "", "🌍"] => r#"["a","","🌍"]"#;
    // As is a borrowed slice of numbers, which each reach JS as a copy.
    // hole: of_bytes sl, `&[u8]: RefIntoWasmAbi` is not satisfied
    of_bytes          [-  +  +  -  -  -  +  -]: &'static [u8]            = [&[1, 2][..], &[], &[3]] => "[Uint8Array[1,2],Uint8Array[],Uint8Array[3]]";
    // hole: of_floats sl, `&[f64]: RefIntoWasmAbi` is not satisfied
    of_floats         [-  +  +  -  -  -  +  -]: &'static [f64]           = [&[0.5][..], &[], &[-1.0, 2.0]] => "[Float64Array[0.5],Float64Array[],Float64Array[-1,2]]";
    of_js_value       [+  +  +  +  +  +  +  -]: JsValue                  = [1.into(), JsValue::NULL, "a".into(), JsValue::UNDEFINED] => r#"[1,null,"a",undefined]"#;
    of_imported       [+  +  +  +  +  +  +  -]: MatrixThing              = [thing(1), thing(2), thing(3)] => r#"[{"thing":1},{"thing":2},{"thing":3}]"#;
    of_js_string      [+  +  +  +  +  +  +  -]: JsString                 = [js("a"), js(""), js("c")] => r#"["a","","c"]"#;
    // hole: of_exported sl, `MatrixItem: RefIntoWasmAbi` is not satisfied
    of_exported       [+  +  +  +  -  +  +  +]: MatrixItem               = [item(1), item(2), item(3)] => "[Item(1),Item(2),Item(3)]";
    of_c_enum         [+  +  +  +  +  +  +  -]: MatrixLevel              = [Low, High, High] => "[1,2,2]";
    of_string_enum    [+  +  +  +  +  +  +  -]: MatrixName               = [A, B, B] => r#"["a","b","b"]"#;
    // hole: of_union sl, `MatrixUnion: RefIntoWasmAbi` is not satisfied
    of_union          [+  +  +  +  -  +  +  +]: MatrixUnion              = [MatrixUnion::Loading, MatrixUnion::Count(7), MatrixUnion::Name(A)] => r#"["loading",7,"a"]"#;
    of_hand_written   [+  +  +  +  +  +  +  -]: Point                    = [point(1., 2.), point(3., 4.)] => r#"[{"x":1,"y":2},{"x":3,"y":4}]"#;

    opt_u32           [+  +  +  +  +  +  +  -]: Option<u32>              = [Some(1), None, Some(u32::MAX)] => "[1,undefined,4294967295]";
    opt_i64           [+  +  +  +  +  +  +  -]: Option<i64>              = [Some(-1), None, Some(i64::MAX)] => "[-1n,undefined,9223372036854775807n]";
    opt_f64           [+  +  +  +  +  +  +  -]: Option<f64>              = [Some(0.5), None, Some(-1.0)] => "[0.5,undefined,-1]";
    opt_bool          [+  +  +  +  +  +  +  -]: Option<bool>             = [Some(true), None, Some(false)] => "[true,undefined,false]";
    opt_char          [+  +  +  +  +  +  +  -]: Option<char>             = [Some('a'), None, Some('🌍')] => r#"["a",undefined,"🌍"]"#;
    opt_string        [+  +  +  +  +  +  +  +]: Option<String>           = [Some(s("a")), None, Some(s(""))] => r#"["a",undefined,""]"#;
    opt_imported      [+  +  +  +  +  +  +  -]: Option<MatrixThing>      = [Some(thing(1)), None, Some(thing(3))] => r#"[{"thing":1},undefined,{"thing":3}]"#;
    opt_js_string     [+  +  +  +  +  +  +  -]: Option<JsString>         = [Some(js("a")), None, Some(js(""))] => r#"["a",undefined,""]"#;
    // hole: opt_exported sl, `Option<MatrixItem>: RefIntoWasmAbi` is not satisfied
    opt_exported      [+  +  +  +  -  +  +  +]: Option<MatrixItem>       = [Some(item(1)), None, Some(item(3))] => "[Item(1),undefined,Item(3)]";
    opt_c_enum        [+  +  +  +  +  +  +  -]: Option<MatrixLevel>      = [Some(Low), None, Some(High)] => "[1,undefined,2]";
    opt_string_enum   [+  +  +  +  +  +  +  -]: Option<MatrixName>       = [Some(A), None, Some(B)] => r#"["a",undefined,"b"]"#;
    // hole: opt_union sl, `Option<MatrixUnion>: RefIntoWasmAbi` is not satisfied
    opt_union         [+  +  +  +  -  +  +  +]: Option<MatrixUnion>      = [Some(MatrixUnion::Loading), None, Some(MatrixUnion::Count(7))] => r#"["loading",undefined,7]"#;
    // hole: opt_hand_written sl, `Option<Point>: RefIntoWasmAbi` is not satisfied
    opt_hand_written  [+  +  +  +  -  +  +  -]: Option<Point>            = [Some(point(1., 2.)), None, Some(point(3., 4.))] => r#"[{"x":1,"y":2},undefined,{"x":3,"y":4}]"#;

    null_u32          [+  +  +  +  +  +  +  -]: Nullable<u32>            = [Value(1), Null, Value(u32::MAX)] => "[1,null,4294967295]";
    null_i64          [+  +  +  +  +  +  +  -]: Nullable<i64>            = [Value(-1), Null, Value(i64::MAX)] => "[-1n,null,9223372036854775807n]";
    null_f64          [+  +  +  +  +  +  +  -]: Nullable<f64>            = [Value(0.5), Null, Value(-1.0)] => "[0.5,null,-1]";
    null_bool         [+  +  +  +  +  +  +  -]: Nullable<bool>           = [Value(true), Null, Value(false)] => "[true,null,false]";
    null_char         [+  +  +  +  +  +  +  -]: Nullable<char>           = [Value('a'), Null, Value('🌍')] => r#"["a",null,"🌍"]"#;
    null_string       [+  +  +  +  +  +  +  +]: Nullable<String>         = [Value(s("a")), Null, Value(s(""))] => r#"["a",null,""]"#;
    null_imported     [+  +  +  +  +  +  +  -]: Nullable<MatrixThing>    = [Value(thing(1)), Null, Value(thing(3))] => r#"[{"thing":1},null,{"thing":3}]"#;
    null_js_string    [+  +  +  +  +  +  +  -]: Nullable<JsString>       = [Value(js("a")), Null, Value(js(""))] => r#"["a",null,""]"#;
    // hole: null_exported sl, `Nullable<MatrixItem>: RefIntoWasmAbi` is not satisfied
    null_exported     [+  +  +  +  -  +  +  +]: Nullable<MatrixItem>     = [Value(item(1)), Null, Value(item(3))] => "[Item(1),null,Item(3)]";
    null_c_enum       [+  +  +  +  +  +  +  -]: Nullable<MatrixLevel>    = [Value(Low), Null, Value(High)] => "[1,null,2]";
    null_string_enum  [+  +  +  +  +  +  +  -]: Nullable<MatrixName>     = [Value(A), Null, Value(B)] => r#"["a",null,"b"]"#;
    // hole: null_union sl, `Nullable<MatrixUnion>: RefIntoWasmAbi` is not satisfied
    null_union        [+  +  +  +  -  +  +  +]: Nullable<MatrixUnion>    = [Value(MatrixUnion::Loading), Null, Value(MatrixUnion::Count(7))] => r#"["loading",null,7]"#;
    null_opt_string   [+  +  +  +  +  +  +  +]: Nullable<Option<String>> = [Value(Some(s("a"))), Null, Value(None)] => r#"["a",null,undefined]"#;

    vec_u32           [+  +  +  +  +  +  +  +]: Vec<u32>                 = [vec![1, 2], vec![], vec![3]] => "[Uint32Array[1,2],Uint32Array[],Uint32Array[3]]";
    vec_string        [+  +  +  +  +  +  +  +]: Vec<String>              = [vec![s("a"), s("b")], vec![], vec![s("c")]] => r#"[["a","b"],[],["c"]]"#;
    vec_js_string     [+  +  +  +  +  +  +  +]: Vec<JsString>            = [vec![js("a"), js("b")], vec![], vec![js("c")]] => r#"[["a","b"],[],["c"]]"#;
    vec_opt_string    [+  +  +  +  +  +  +  +]: Vec<Option<String>>      = [vec![Some(s("a")), None], vec![], vec![None]] => r#"[["a",undefined],[],[undefined]]"#;
}

#[allow(clippy::ptr_arg, clippy::borrowed_box)]
#[wasm_bindgen(module = "tests/wasm/element_matrix.js")]
extern "C" {
    #[wasm_bindgen(js_name = show)]
    fn show_vec_ref(v: &Vec<String>) -> String;
    #[wasm_bindgen(js_name = show)]
    fn show_boxed_ref(v: &Box<[u32]>) -> String;
}

/// A borrowed vector on its own is lent as the slice of it is.
#[wasm_bindgen_test]
fn borrowed_vectors() {
    assert_eq!(show_vec_ref(&vec![s("a"), s("b")]), r#"["a","b"]"#);
    assert_eq!(show_boxed_ref(&vec![1, 2].into()), "Uint32Array[1,2]");
}
