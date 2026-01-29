use js_sys::{global, Array, Object, Promise, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::wasm_bindgen_test;

// Array
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Array, extends = Object, is_type_of = Array::is_array, typescript_type = "Array<any>")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type ArrayUnwind;

    #[wasm_bindgen(constructor, js_class = Array)]
    pub fn new() -> ArrayUnwind;

    #[wasm_bindgen(method, js_class = Array)]
    pub fn push(this: &ArrayUnwind, value: &JsValue) -> u32;

    // Predicate throw or panic will panic, but be caught as Result::Err
    // Unless a WebAssembly.RuntimeError, in which case, abort propagates
    #[wasm_bindgen(method, catch, js_class = Array, js_name = every)]
    pub fn try_every_result(
        this: &ArrayUnwind,
        predicate: &Closure<dyn FnMut(JsValue, u32, Array) -> Result<bool, JsValue>>,
    ) -> Result<bool, JsValue>;

    // This currently aborts correctly
    // TODO: support &mut dyn FnMut as unwind safe to not abort (defaults as abort for now)
    // this would involve macro rewriting it into:
    // #[wasm_bindgen(method, catch, js_class = Array, js_name = every)]
    // pub fn try_every_result<T: __rt::marker::MaybeUnwindSafe + nMut(JsValue, u32, Array) -> Result<bool, JsValue>>(
    //     this: &ArrayUnwind,
    //     predicate: &mut T,
    // ) -> Result<bool, JsValue>;
    // #[wasm_bindgen(method, catch, js_class = Array, js_name = every)]
    // pub fn try_every_result_aborting(
    //     this: &ArrayUnwind,
    //     predicate: &mut dyn FnMut(JsValue, u32, Array) -> Result<bool, JsValue>,
    // ) -> Result<bool, JsValue>;
}

// global
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setTimeout)]
    pub fn set_timeout(handler: &Closure<dyn FnMut()>);
}

macro_rules! js_array {
    ($($e:expr),*) => ({
        let __x = ArrayUnwind::new();
        $(__x.push(&JsValue::from($e));)*
        __x
    })
}

#[wasm_bindgen_test]
async fn try_promise_all() {
    let mut resolve_ = Default::default();
    let promise = Promise::new(&mut |resolve, _reject| {
        resolve_ = resolve;
    });
    let closure1 = Closure::new(|_v| {
        panic!("CLOSURE PANIC");
    });
    let promise2 = promise.then(&closure1);
    let future = JsFuture::from(promise2);
    let closure2 = Closure::new(move || {
        resolve_
            .call1(&JsValue::undefined(), &JsValue::undefined())
            .unwrap();
    });
    set_timeout(&closure2);
    let result = future.await;
    assert!(result.is_err());
    let err = result.err().unwrap();
    let msg = Reflect::get(&err, &"message".into()).unwrap();
    assert_eq!(msg, "CLOSURE PANIC");
}

#[wasm_bindgen_test]
fn try_every() {
    let even = js_array![2, 4, 6, 8];
    Reflect::set(&global(), &"dropped".into(), &JsValue::FALSE).unwrap();
    Reflect::set(&global(), &"food".into(), &JsValue::FALSE).unwrap();
    assert!(even
        .try_every_result(&Closure::new(|_, _, _| {
            struct Foo {}
            impl Drop for Foo {
                fn drop(&mut self) {
                    Reflect::set(&global(), &"dropped".into(), &JsValue::TRUE).unwrap();
                }
            }
            impl Foo {
                fn foo(&self) {
                    let _ = Reflect::set(&global(), &"food".into(), &JsValue::TRUE);
                }
            }
            let foo = Foo {};
            if std::hint::black_box(true) {
                panic!("PANIC");
            }
            foo.foo();
            Ok(true)
        }))
        .is_err());
    assert!(!Reflect::get(&global(), &"food".into())
        .unwrap()
        .as_bool()
        .unwrap());
    assert!(Reflect::get(&global(), &"dropped".into())
        .unwrap()
        .as_bool()
        .unwrap());
}

// #[wasm_bindgen_test]
// fn try_every_aborting() {
//     let even = js_array![2, 4, 6, 8];
//     Reflect::set(&global(), &"dropped".into(), &JsValue::FALSE).unwrap();
//     Reflect::set(&global(), &"food".into(), &JsValue::FALSE).unwrap();
//     assert!(even
//         .try_every_result_aborting(&mut |_, _, _| {
//             struct Foo {}
//             impl Drop for Foo {
//                 fn drop(&mut self) {
//                     Reflect::set(&global(), &"dropped".into(), &JsValue::TRUE).unwrap();
//                 }
//             }
//             impl Foo {
//                 fn foo(&self) {
//                     let _ = Reflect::set(&global(), &"food".into(), &JsValue::TRUE);
//                 }
//             }
//             let foo = Foo {};
//             if std::hint::black_box(true) {
//                 panic!("PANIC");
//             }
//             foo.foo();
//             Ok(true)
//         })
//         .is_err());
//     assert!(!Reflect::get(&global(), &"food".into())
//         .unwrap()
//         .as_bool()
//         .unwrap());
//     assert!(Reflect::get(&global(), &"dropped".into())
//         .unwrap()
//         .as_bool()
//         .unwrap());
// }
