use wasm_bindgen_test::*;

pub mod same_function_different_locations_a {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "tests/wasm/duplicates_a.js")]
    extern "C" {
        pub fn foo();
        #[wasm_bindgen(thread_local_v2, js_name = bar)]
        pub static BAR: JsValue;
    }
}

pub mod same_function_different_locations_b {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "tests/wasm/duplicates_a.js")]
    extern "C" {
        pub fn foo();
        #[wasm_bindgen(thread_local_v2, js_name = bar)]
        pub static BAR: JsValue;
    }
}

#[wasm_bindgen_test]
fn same_function_different_locations() {
    same_function_different_locations_a::foo();
    same_function_different_locations_b::foo();
    same_function_different_locations_a::BAR.with(|bar| assert_eq!(*bar, 3));
    same_function_different_locations_a::BAR.with(|bar| assert_eq!(*bar, 3));
}

pub mod same_function_different_modules_a {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "tests/wasm/duplicates_b.js")]
    extern "C" {
        pub fn foo() -> bool;
        #[wasm_bindgen(thread_local_v2, js_name = bar)]
        pub static BAR: JsValue;
    }
}

pub mod same_function_different_modules_b {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "tests/wasm/duplicates_c.js")]
    extern "C" {
        pub fn foo() -> bool;
        #[wasm_bindgen(thread_local_v2, js_name = bar)]
        pub static BAR: JsValue;
    }
}

#[wasm_bindgen_test]
fn same_function_different_modules() {
    assert!(same_function_different_modules_a::foo());
    assert!(!same_function_different_modules_b::foo());
    same_function_different_modules_a::BAR.with(|bar| assert_eq!(*bar, 4));
    same_function_different_modules_b::BAR.with(|bar| assert_eq!(*bar, 5));
}

// Every `inline_js` snippet is snippet 0 of its own macro invocation, so the
// snippet contents, not just its index, must keep these two apart.
pub mod same_function_different_inline_js_a {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = "export const foo = () => 6; export const X = 6;")]
    extern "C" {
        pub fn foo() -> u32;
        #[wasm_bindgen(thread_local_v2)]
        pub static X: JsValue;
    }
}

pub mod same_function_different_inline_js_b {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = "export const foo = () => 7; export const X = 7;")]
    extern "C" {
        pub fn foo() -> u32;
        #[wasm_bindgen(thread_local_v2)]
        pub static X: JsValue;
    }
}

#[wasm_bindgen_test]
fn same_function_different_inline_js() {
    assert_eq!(same_function_different_inline_js_a::foo(), 6);
    assert_eq!(same_function_different_inline_js_b::foo(), 7);
    same_function_different_inline_js_a::X.with(|x| assert_eq!(*x, 6));
    same_function_different_inline_js_b::X.with(|x| assert_eq!(*x, 7));
}

// Imported types with the same Rust name but a different JS source must get
// distinct `instanceof` checks.
pub mod same_type_different_modules_a {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "tests/wasm/duplicates_b.js")]
    extern "C" {
        pub type Thing;
        #[wasm_bindgen(constructor)]
        pub fn new() -> Thing;
    }
}

pub mod same_type_different_modules_b {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "tests/wasm/duplicates_c.js")]
    extern "C" {
        pub type Thing;
        #[wasm_bindgen(constructor)]
        pub fn new() -> Thing;
    }
}

#[wasm_bindgen_test]
fn same_type_different_modules() {
    use same_type_different_modules_a as a;
    use same_type_different_modules_b as b;
    use wasm_bindgen::prelude::*;

    let x = JsValue::from(a::Thing::new());
    let y = JsValue::from(b::Thing::new());
    assert!(x.is_instance_of::<a::Thing>());
    assert!(!x.is_instance_of::<b::Thing>());
    assert!(y.is_instance_of::<b::Thing>());
    assert!(!y.is_instance_of::<a::Thing>());
}

pub mod same_type_different_inline_js_a {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = "export class Thing { a() {} }")]
    extern "C" {
        pub type Thing;
        #[wasm_bindgen(constructor)]
        pub fn new() -> Thing;
    }
}

pub mod same_type_different_inline_js_b {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = "export class Thing { b() {} }")]
    extern "C" {
        pub type Thing;
        #[wasm_bindgen(constructor)]
        pub fn new() -> Thing;
    }
}

#[wasm_bindgen_test]
fn same_type_different_inline_js() {
    use same_type_different_inline_js_a as a;
    use same_type_different_inline_js_b as b;
    use wasm_bindgen::prelude::*;

    let x = JsValue::from(a::Thing::new());
    let y = JsValue::from(b::Thing::new());
    assert!(x.is_instance_of::<a::Thing>());
    assert!(!x.is_instance_of::<b::Thing>());
    assert!(y.is_instance_of::<b::Thing>());
    assert!(!y.is_instance_of::<a::Thing>());
}

pub mod same_type_different_js_name_a {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = Map)]
        pub type Collection;
    }
}

pub mod same_type_different_js_name_b {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = Set)]
        pub type Collection;
    }
}

#[wasm_bindgen_test]
fn same_type_different_js_name() {
    use same_type_different_js_name_a as a;
    use same_type_different_js_name_b as b;
    use wasm_bindgen::prelude::*;

    let map = JsValue::from(js_sys::Map::new());
    let set = JsValue::from(js_sys::Set::new(&JsValue::UNDEFINED));
    assert!(map.is_instance_of::<a::Collection>());
    assert!(!map.is_instance_of::<b::Collection>());
    assert!(set.is_instance_of::<b::Collection>());
    assert!(!set.is_instance_of::<a::Collection>());
}

pub mod same_type_different_js_namespace_a {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "tests/wasm/duplicates_b.js", js_namespace = nsA)]
    extern "C" {
        pub type Thing;
        #[wasm_bindgen(constructor)]
        pub fn new() -> Thing;
    }
}

pub mod same_type_different_js_namespace_b {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "tests/wasm/duplicates_b.js", js_namespace = nsB)]
    extern "C" {
        pub type Thing;
        #[wasm_bindgen(constructor)]
        pub fn new() -> Thing;
    }
}

#[wasm_bindgen_test]
fn same_type_different_js_namespace() {
    use same_type_different_js_namespace_a as a;
    use same_type_different_js_namespace_b as b;
    use wasm_bindgen::prelude::*;

    let x = JsValue::from(a::Thing::new());
    let y = JsValue::from(b::Thing::new());
    assert!(x.is_instance_of::<a::Thing>());
    assert!(!x.is_instance_of::<b::Thing>());
    assert!(y.is_instance_of::<b::Thing>());
    assert!(!y.is_instance_of::<a::Thing>());
}

pub mod same_static_string_different_locations_a {
    use js_sys::JsString;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    // Rustfmt removes the value on this static, see rustfmt#6267.
    #[rustfmt::skip]
    extern "C" {
        #[wasm_bindgen(thread_local_v2, static_string)]
        pub static S: JsString = "a";
    }
}

pub mod same_static_string_different_locations_b {
    use js_sys::JsString;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    #[rustfmt::skip]
    extern "C" {
        #[wasm_bindgen(thread_local_v2, static_string)]
        pub static S: JsString = "b";
    }
}

#[wasm_bindgen_test]
fn same_static_string_different_locations() {
    use same_static_string_different_locations_a as a;
    use same_static_string_different_locations_b as b;

    assert_eq!(a::S.with(|s| String::from(s)), "a");
    assert_eq!(b::S.with(|s| String::from(s)), "b");
}
