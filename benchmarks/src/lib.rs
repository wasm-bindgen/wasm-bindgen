extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use web_sys::Node;

#[wasm_bindgen(raw_module = "../globals.js")]
extern "C" {
    #[wasm_bindgen(js_name = jsthunk)]
    fn js_thunk();
    #[wasm_bindgen(js_name = add)]
    fn js_add(a: i32, b: i32) -> i32;

    #[wasm_bindgen(js_name = use_baz)]
    fn js_use_baz(val: Baz);

    pub type Foo;
    #[wasm_bindgen(method, final, js_name = bar)]
    fn bar_final(this: &Foo);
    #[wasm_bindgen(method, structural, js_name = bar)]
    fn bar_structural(this: &Foo);

    #[wasm_bindgen(js_name = jsthunk)]
    fn doesnt_throw();
    #[wasm_bindgen(catch, js_name = jsthunk)]
    fn doesnt_throw_catch() -> Result<(), JsValue>;
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum Baz {
    Variant1 = "variant-1",
    Variant2 = "variant-2",
    Variant3 = "variant-3",
}

#[wasm_bindgen]
pub fn call_js_thunk_n_times(n: usize) {
    for _ in 0..n {
        js_thunk();
    }
}

#[wasm_bindgen]
pub fn call_js_add_n_times(n: usize, a: i32, b: i32) {
    for _ in 0..n {
        js_add(a, b);
    }
}

#[wasm_bindgen]
pub fn thunk() {}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

static mut FIB_HIGH: i32 = 0;

#[wasm_bindgen]
pub fn fibonacci(n: i32) -> i32 {
    let mut a = 1u64;
    let mut b = 1;
    for _ in 0..n {
        let tmp = b;
        b += a;
        a = tmp;
    }
    unsafe {
        FIB_HIGH = (a >> 32) as i32;
    }
    a as i32
}

#[wasm_bindgen]
pub fn fibonacci_high() -> i32 {
    unsafe { FIB_HIGH }
}

#[wasm_bindgen]
pub fn call_foo_bar_final_n_times(n: usize, js_foo: &Foo) {
    for _ in 0..n {
        js_foo.bar_final();
    }
}

#[wasm_bindgen]
pub fn call_foo_bar_structural_n_times(n: usize, js_foo: &Foo) {
    for _ in 0..n {
        js_foo.bar_structural();
    }
}

#[wasm_bindgen]
pub fn call_use_baz_n_times(n: usize) {
    for _ in 0..n {
        js_use_baz(Baz::Variant2);
    }
}

#[wasm_bindgen]
pub fn call_doesnt_throw_n_times(n: usize) {
    for _ in 0..n {
        doesnt_throw();
    }
}

#[wasm_bindgen]
pub fn call_doesnt_throw_with_catch_n_times(n: usize) {
    for _ in 0..n {
        if let Err(e) = doesnt_throw_catch() {
            wasm_bindgen::throw_val(e);
        }
    }
}

#[wasm_bindgen]
extern "C" {
    pub type Element;

    #[wasm_bindgen(method, js_name = firstChild, final, getter)]
    fn first_child_final(this: &Element) -> Element;
    #[wasm_bindgen(method, js_name = firstChild, structural, getter)]
    fn first_child_structural(this: &Element) -> Element;
}

#[wasm_bindgen]
pub fn call_first_child_final_n_times(n: usize, element: &Element) {
    for _ in 0..n {
        drop(element.first_child_final());
    }
}

#[wasm_bindgen]
pub fn call_first_child_structural_n_times(n: usize, element: &Element) {
    for _ in 0..n {
        drop(element.first_child_structural());
    }
}

#[wasm_bindgen]
pub fn call_node_first_child_n_times(n: usize, elements: Vec<JsValue>) {
    for _ in 0..n {
        for element in elements.iter() {
            let element = element.unchecked_ref::<Node>();
            assert!(element.first_child().is_some());
        }
    }
}

#[wasm_bindgen]
pub fn call_node_node_type_n_times(n: usize, elements: Vec<JsValue>) {
    for _ in 0..n {
        for element in elements.iter() {
            let element = element.unchecked_ref::<Node>();
            assert!(element.node_type() != 100);
        }
    }
}

#[wasm_bindgen]
pub fn call_node_has_child_nodes_n_times(n: usize, elements: Vec<JsValue>) {
    for _ in 0..n {
        for element in elements.iter() {
            let element = element.unchecked_ref::<Node>();
            assert!(element.has_child_nodes());
        }
    }
}

#[wasm_bindgen]
pub fn count_node_types(element: Node) {
    let mut count = Vec::new();
    count_node_types(element, &mut count);

    fn count_node_types(mut element: Node, count: &mut Vec<u32>) {
        loop {
            let t = element.node_type();
            if t as usize >= count.len() {
                count.resize(t as usize + 1, 0);
            }
            count[t as usize] += 1;
            if let Some(s) = element.first_child() {
                count_node_types(s, count);
            }
            match element.next_sibling() {
                Some(s) => element = s,
                None => break,
            }
        }
    }
}

#[wasm_bindgen]
pub fn str_roundtrip(s: String) -> String {
    s
}
