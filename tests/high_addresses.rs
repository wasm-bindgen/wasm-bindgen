//! Every allocation here is above 2 GiB, where a wasm32 pointer reaches JS
//! as a negative number. Each crossing gives back what it was given, and
//! leaves no allocation and no handle behind.

#![cfg(all(target_arch = "wasm32", target_os = "unknown"))]

use js_sys::JsString;
use std::alloc::{GlobalAlloc, Layout, System};
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

const HIGH: usize = 1 << 31;

static REACHED: AtomicBool = AtomicBool::new(false);
static UNREACHABLE: AtomicBool = AtomicBool::new(false);
static LIVE: AtomicIsize = AtomicIsize::new(0);

/// Once [`high`] has reached them, hands out high addresses only: a low
/// one is kept, never to be handed out.
struct HighOnly;

unsafe impl GlobalAlloc for HighOnly {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        loop {
            let ptr = System.alloc(layout);
            if ptr.is_null() {
                return ptr;
            }
            if !REACHED.load(Ordering::Relaxed) || ptr as usize >= HIGH {
                LIVE.fetch_add(1, Ordering::Relaxed);
                return ptr;
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        LIVE.fetch_sub(1, Ordering::Relaxed);
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static ALLOCATOR: HighOnly = HighOnly;

/// Whether memory could be grown past 2 GiB, which a build with a lower
/// maximum can't: it gets back what the attempt took.
fn high() -> bool {
    if REACHED.load(Ordering::Relaxed) || UNREACHABLE.load(Ordering::Relaxed) {
        return REACHED.load(Ordering::Relaxed);
    }
    let chunk = Layout::from_size_align(1 << 20, 8).unwrap();
    let mut held = ptr::null_mut::<u8>();
    loop {
        let ptr = unsafe { System.alloc(chunk) };
        if ptr.is_null() {
            while !held.is_null() {
                unsafe {
                    let next = held.cast::<*mut u8>().read();
                    System.dealloc(held, chunk);
                    held = next;
                }
            }
            UNREACHABLE.store(true, Ordering::Relaxed);
            return false;
        }
        unsafe { ptr.cast::<*mut u8>().write(held) };
        held = ptr;
        if ptr as usize >= HIGH {
            REACHED.store(true, Ordering::Relaxed);
            return true;
        }
    }
}

fn balanced(f: impl FnOnce()) {
    let before = (
        LIVE.load(Ordering::Relaxed),
        wasm_bindgen::externref_heap_live_count(),
    );
    f();
    let after = (
        LIVE.load(Ordering::Relaxed),
        wasm_bindgen::externref_heap_live_count(),
    );
    assert_eq!(before, after, "(allocations, handles)");
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct HighItem {
    id: u32,
}

#[wasm_bindgen]
impl HighItem {
    #[wasm_bindgen(constructor)]
    pub fn new(id: u32) -> HighItem {
        HighItem { id }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u32 {
        self.id
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct HighRecord {
    pub names: Vec<String>,
    pub values: Vec<JsValue>,
    pub bytes: Vec<u8>,
}

#[wasm_bindgen]
impl HighRecord {
    #[wasm_bindgen(constructor)]
    pub fn new() -> HighRecord {
        HighRecord {
            names: Vec::new(),
            values: Vec::new(),
            bytes: Vec::new(),
        }
    }
}

#[wasm_bindgen(module = "tests/high_addresses.js")]
extern "C" {
    #[wasm_bindgen(js_name = echo)]
    fn values(v: Vec<JsValue>) -> Vec<JsValue>;
    #[wasm_bindgen(js_name = echo)]
    fn opt_values(v: Option<Vec<JsValue>>) -> Option<Vec<JsValue>>;
    #[wasm_bindgen(js_name = echo)]
    fn js_strings(v: Vec<JsString>) -> Vec<JsString>;
    #[wasm_bindgen(js_name = echo)]
    fn boxed(v: Box<[JsValue]>) -> Box<[JsValue]>;
    #[wasm_bindgen(variadic, js_name = count)]
    fn variadic(first: u32, rest: Vec<JsValue>) -> u32;
    #[wasm_bindgen(catch, js_name = fail)]
    fn values_throw(v: Vec<JsValue>) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, js_name = fail)]
    fn strings_throw(v: Vec<String>, w: Vec<Vec<JsValue>>) -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = echo)]
    fn strings(v: Vec<String>) -> Vec<String>;
    #[wasm_bindgen(js_name = echo)]
    fn opt_strings(v: Option<Vec<String>>) -> Option<Vec<String>>;
    #[wasm_bindgen(js_name = echo)]
    fn nested(v: Vec<Vec<u8>>) -> Vec<Vec<u8>>;
    #[wasm_bindgen(js_name = echo)]
    fn nested_values(v: Vec<Vec<JsValue>>) -> Vec<Vec<JsValue>>;
    #[wasm_bindgen(js_name = echo)]
    fn nested_strings(v: Vec<Option<Vec<String>>>) -> Vec<Option<Vec<String>>>;
    #[wasm_bindgen(js_name = echo)]
    fn items(v: Vec<HighItem>) -> Vec<HighItem>;
    #[wasm_bindgen(js_name = echo)]
    fn opt_items(v: Option<Vec<HighItem>>) -> Option<Vec<HighItem>>;
    #[wasm_bindgen(js_name = second)]
    fn item_beside(item: HighItem, v: Vec<String>) -> Vec<String>;
    #[wasm_bindgen(js_name = echo)]
    fn bytes(v: Vec<u8>) -> Vec<u8>;
    #[wasm_bindgen(js_name = echo)]
    fn floats(v: Option<Box<[f64]>>) -> Option<Box<[f64]>>;
    #[wasm_bindgen(js_name = sum)]
    fn slice(v: &[u8]) -> u32;
    #[wasm_bindgen(js_name = sum)]
    fn vec_ref(v: &Vec<u8>) -> u32;
    #[wasm_bindgen(js_name = sum)]
    fn opt_slice(v: Option<&[u32]>) -> u32;
    #[wasm_bindgen(js_name = fill)]
    fn mut_slice(v: &mut [u8]);
    #[wasm_bindgen(js_name = join)]
    fn string_slice(v: &[String]) -> String;
    #[wasm_bindgen(js_name = join)]
    fn value_slice(v: &[JsValue]) -> String;
    #[wasm_bindgen(js_name = join)]
    fn byte_slices(v: Vec<&[u8]>) -> String;
    #[wasm_bindgen(js_name = call)]
    fn call(
        f: &mut dyn FnMut(Vec<JsValue>, Vec<String>, Vec<u8>) -> Vec<String>,
        v: Vec<JsValue>,
    ) -> Vec<String>;
    #[wasm_bindgen(js_name = call)]
    fn call_owned(
        f: &Closure<dyn FnMut(Vec<JsValue>, Vec<String>, Vec<u8>) -> Vec<String>>,
        v: Vec<JsValue>,
    ) -> Vec<String>;
    #[wasm_bindgen(js_name = exports)]
    fn js_exports();
}

fn s(v: &str) -> String {
    v.into()
}

fn js_values() -> Vec<JsValue> {
    vec![JsValue::from(1), JsValue::from("two"), JsValue::NULL]
}

#[wasm_bindgen_test]
fn allocations_are_high() {
    if high() {
        assert!(Vec::<u8>::with_capacity(8).as_ptr() as usize >= HIGH);
        assert!(Vec::<u64>::with_capacity(1 << 16).as_ptr() as usize >= HIGH);
    }
}

#[wasm_bindgen_test]
fn an_import_is_given_handles() {
    if !high() {
        return;
    }
    for _ in 0..3 {
        balanced(|| assert_eq!(values(js_values()), js_values()));
        balanced(|| assert_eq!(opt_values(Some(js_values())), Some(js_values())));
        balanced(|| assert_eq!(opt_values(None), None));
        balanced(|| {
            let given: Vec<JsString> = vec!["a".into(), "bc".into()];
            assert_eq!(js_strings(given.clone()), given);
        });
        balanced(|| assert_eq!(*boxed(js_values().into()), *js_values()));
        balanced(|| assert_eq!(variadic(7, js_values()), 4));
        balanced(|| assert!(values_throw(js_values()).is_err()));
        balanced(|| assert_eq!(value_slice(&js_values()), "1,two,null"));
    }
}

#[wasm_bindgen_test]
fn an_import_is_given_cells() {
    if !high() {
        return;
    }
    let names = || vec![s("a"), s(""), s("cde")];
    for _ in 0..3 {
        balanced(|| assert_eq!(strings(names()), names()));
        balanced(|| assert_eq!(opt_strings(Some(names())), Some(names())));
        balanced(|| assert_eq!(opt_strings(None), None));
        balanced(|| {
            let given = vec![vec![1, 2, 3], vec![], vec![4]];
            assert_eq!(nested(given.clone()), given);
        });
        balanced(|| {
            let given = vec![js_values(), vec![], js_values()];
            assert_eq!(nested_values(given.clone()), given);
        });
        balanced(|| {
            let given = vec![Some(names()), None, Some(vec![])];
            assert_eq!(nested_strings(given.clone()), given);
        });
        balanced(|| assert!(strings_throw(names(), vec![js_values(), js_values()]).is_err()));
        balanced(|| assert_eq!(string_slice(&names()), "a,,cde"));
        balanced(|| assert_eq!(byte_slices(vec![&[1, 2][..], &[], &[3]]), "1,2,,3"));
    }
}

#[wasm_bindgen_test]
fn an_import_is_given_instances() {
    if !high() {
        return;
    }
    let given = || vec![HighItem::new(1), HighItem::new(2)];
    for _ in 0..3 {
        balanced(|| assert_eq!(items(given()), given()));
        balanced(|| assert_eq!(opt_items(Some(given())), Some(given())));
        balanced(|| assert_eq!(opt_items(None), None));
        balanced(|| assert_eq!(item_beside(HighItem::new(3), vec![s("x")]), [s("x")]));
    }
}

#[wasm_bindgen_test]
fn an_import_is_given_numbers() {
    if !high() {
        return;
    }
    for _ in 0..3 {
        balanced(|| assert_eq!(bytes(vec![1, 2, 3]), [1, 2, 3]));
        balanced(|| {
            assert_eq!(
                floats(Some(vec![1.5, 2.5].into())).as_deref(),
                Some(&[1.5, 2.5][..])
            )
        });
        balanced(|| assert_eq!(floats(None), None));
        balanced(|| assert_eq!(slice(&[1, 2, 3]), 6));
        balanced(|| assert_eq!(vec_ref(&vec![1, 2, 3]), 6));
        balanced(|| assert_eq!(opt_slice(Some(&[1, 2, 3])), 6));
        balanced(|| assert_eq!(opt_slice(None), 0));
        balanced(|| {
            let mut given = vec![0; 4];
            mut_slice(&mut given);
            assert_eq!(given, [7; 4]);
        });
    }
}

#[wasm_bindgen_test]
fn a_closure_is_given_sequences() {
    if !high() {
        return;
    }
    fn reply(values: Vec<JsValue>, mut names: Vec<String>, bytes: Vec<u8>) -> Vec<String> {
        assert_eq!(values, js_values());
        assert_eq!(bytes, [1, 2, 3]);
        names.reverse();
        names
    }
    for _ in 0..3 {
        balanced(|| assert_eq!(call(&mut reply, js_values()), [s("b"), s("a")]));
    }
    let owned = Closure::new(reply);
    for _ in 0..3 {
        balanced(|| assert_eq!(call_owned(&owned, js_values()), [s("b"), s("a")]));
    }
}

#[wasm_bindgen]
pub fn high_values(v: Vec<JsValue>) -> Vec<JsValue> {
    v
}

#[wasm_bindgen]
pub fn high_opt_values(v: Option<Vec<JsValue>>) -> Option<Vec<JsValue>> {
    v
}

#[wasm_bindgen]
pub fn high_strings(v: Vec<String>) -> Vec<String> {
    v
}

#[wasm_bindgen]
pub fn high_opt_strings(v: Option<Vec<String>>) -> Option<Vec<String>> {
    v
}

#[wasm_bindgen]
pub fn high_nested(v: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    v
}

#[wasm_bindgen]
pub fn high_nested_strings(v: Vec<Option<Vec<String>>>) -> Vec<Option<Vec<String>>> {
    v
}

#[wasm_bindgen]
pub fn high_items(v: Vec<HighItem>) -> Vec<HighItem> {
    v
}

#[wasm_bindgen]
pub fn high_bytes(v: &[u8]) -> Vec<u8> {
    v.to_vec()
}

#[wasm_bindgen]
pub fn high_fill(v: &mut [u8]) {
    v.fill(7);
}

#[wasm_bindgen]
pub fn high_text(v: &str, w: String) -> String {
    format!("{v}{w}")
}

#[wasm_bindgen]
pub fn high_rejects(_v: Vec<String>, _w: Vec<Vec<JsValue>>, _item: &HighItem) {}

#[wasm_bindgen_test]
fn exports_are_given_and_give_sequences() {
    if !high() {
        return;
    }
    for _ in 0..3 {
        balanced(js_exports);
    }
}
