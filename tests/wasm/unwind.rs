use js_sys::{global, Array, Object, Promise, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{throw_str, JsValue};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::wasm_bindgen_test;

// JS functions for testing unwind on JS throw (without catch/Result)
#[wasm_bindgen(module = "tests/wasm/unwind.js")]
extern "C" {
    // This JS function throws an error - no `catch` attribute, so it will unwind
    fn js_throw_error();

    // Check if the drop flag was set
    fn js_check_dropped() -> bool;

    // Reset the drop flag
    fn js_reset_dropped();

    // Call a Rust function that will call js_throw_error and catch the result
    #[wasm_bindgen(catch)]
    fn js_trigger_unwind_test() -> Result<(), JsValue>;
}

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
        predicate: &mut dyn FnMut(JsValue, u32, Array) -> Result<bool, JsValue>,
    ) -> Result<bool, JsValue>;
    #[wasm_bindgen(method, catch, js_class = Array, js_name = every)]
    pub fn try_every_result_closure(
        this: &ArrayUnwind,
        predicate: &Closure<dyn FnMut(JsValue, u32, Array) -> Result<bool, JsValue>>,
    ) -> Result<bool, JsValue>;
    #[wasm_bindgen(method, catch, js_class = Array, js_name = every)]
    pub fn try_every_result_closure_borrow(
        this: &ArrayUnwind,
        predicate: &ScopedClosure<dyn FnMut(JsValue, u32, Array) -> Result<bool, JsValue>>,
    ) -> Result<bool, JsValue>;

    // This currently aborts correctly
    // TODO: support &mut dyn FnMut as unwind safe to not abort (defaults as abort for now)
    // this would involve macro rewriting it into:
    // #[wasm_bindgen(method, catch, js_class = Array, js_name = every)]
    // pub fn try_every_result<T: __rt::marker::MaybeUnwindSafe + FnMut(JsValue, u32, Array) -> Result<bool, JsValue>>(
    //     this: &ArrayUnwind,
    //     predicate: &mut T,
    // ) -> Result<bool, JsValue>;
    #[wasm_bindgen(method, catch, js_class = Array, js_name = every)]
    pub fn try_every_result_aborting(
        this: &ArrayUnwind,
        predicate: &mut dyn FnMut(JsValue, u32, Array) -> Result<bool, JsValue>,
    ) -> Result<bool, JsValue>;
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
    let closure1 = Closure::own(|_v| {
        panic!("CLOSURE PANIC");
    });
    let promise2 = promise.then(&closure1);
    let future = JsFuture::from(promise2);
    let closure2 = Closure::own(move || {
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
        .try_every_result_closure(&Closure::own(|_, _, _| {
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

#[cfg(panic = "unwind")]
#[wasm_bindgen_test]
fn drop_throw_str_aborting() {
    Reflect::set(&global(), &"dropped_throw_str".into(), &JsValue::FALSE).unwrap();
    Reflect::set(&global(), &"food_throw_str".into(), &JsValue::FALSE).unwrap();
    assert!(js_array![0]
        .try_every_result(&mut |_, _, _| {
            struct Foo {}
            impl Drop for Foo {
                fn drop(&mut self) {
                    Reflect::set(&global(), &"dropped_throw_str".into(), &JsValue::TRUE).unwrap();
                }
            }
            impl Foo {
                fn foo(&self) {
                    let _ = Reflect::set(&global(), &"food_throw_str".into(), &JsValue::TRUE);
                }
            }
            let foo = Foo {};
            if std::hint::black_box(true) {
                throw_str("THROW_STR");
            }
            foo.foo();
            Ok(true)
        })
        .is_err());
    assert!(!Reflect::get(&global(), &"food_throw_str".into())
        .unwrap()
        .as_bool()
        .unwrap());
    assert!(Reflect::get(&global(), &"dropped_throw_str".into())
        .unwrap()
        .as_bool()
        .unwrap());
}

#[cfg(panic = "unwind")]
#[wasm_bindgen_test]
fn drop_throw_str() {
    Reflect::set(&global(), &"dropped_throw_str".into(), &JsValue::FALSE).unwrap();
    Reflect::set(&global(), &"food_throw_str".into(), &JsValue::FALSE).unwrap();
    {
        let mut func = |_, _, _| {
            struct Foo {}
            impl Drop for Foo {
                fn drop(&mut self) {
                    Reflect::set(&global(), &"dropped_throw_str".into(), &JsValue::TRUE).unwrap();
                }
            }
            impl Foo {
                fn foo(&self) {
                    let _ = Reflect::set(&global(), &"food_throw_str".into(), &JsValue::TRUE);
                }
            }
            let foo = Foo {};
            if std::hint::black_box(true) {
                throw_str("THROW_STR");
            }
            foo.foo();
            Ok(true)
        };
        let closure = ScopedClosure::borrow_mut(&mut func);
        assert!(js_array![0]
            .try_every_result_closure_borrow(&closure)
            .is_err());
    }
    assert!(!Reflect::get(&global(), &"food_throw_str".into())
        .unwrap()
        .as_bool()
        .unwrap());
    assert!(Reflect::get(&global(), &"dropped_throw_str".into())
        .unwrap()
        .as_bool()
        .unwrap());
}

/// Rust function exported to JS that calls a throwing JS function.
/// The JS throw should trigger unwinding, running the Drop impl.
#[wasm_bindgen]
pub fn rust_call_throwing_js() {
    struct DropGuard;
    impl Drop for DropGuard {
        fn drop(&mut self) {
            // Set a global flag that we can check from JS
            Reflect::set(&global(), &"unwind_drop_ran".into(), &JsValue::TRUE).unwrap();
        }
    }

    let _guard = DropGuard;

    // This JS function throws - since there's no `catch` attribute,
    // it should trigger unwinding in Rust (when panic=unwind)
    js_throw_error();

    // This line should never be reached
    Reflect::set(
        &global(),
        &"unwind_continued_after_throw".into(),
        &JsValue::TRUE,
    )
    .unwrap();
}

/// Test that a JS throw from an import without `catch` triggers unwinding and runs Drop
#[cfg(panic = "unwind")]
#[wasm_bindgen_test]
fn js_throw_triggers_unwind_and_drop() {
    // Reset state
    js_reset_dropped();
    Reflect::set(&global(), &"unwind_drop_ran".into(), &JsValue::FALSE).unwrap();
    Reflect::set(
        &global(),
        &"unwind_continued_after_throw".into(),
        &JsValue::FALSE,
    )
    .unwrap();

    // JS will call rust_call_throwing_js(), which calls js_throw_error()
    // The throw should unwind, run Drop, and propagate as an error
    let result = js_trigger_unwind_test();
    assert!(result.is_err(), "JS throw should propagate as error");

    // Verify the drop ran during unwinding
    assert!(
        Reflect::get(&global(), &"unwind_drop_ran".into())
            .unwrap()
            .as_bool()
            .unwrap(),
        "Drop should have run during unwind"
    );

    // Verify we didn't continue past the throw
    assert!(
        !Reflect::get(&global(), &"unwind_continued_after_throw".into())
            .unwrap()
            .as_bool()
            .unwrap(),
        "Should not have continued after JS throw"
    );
}
