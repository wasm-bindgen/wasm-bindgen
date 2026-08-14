//! Runtime coverage for `#[wasm_bindgen(generic_per_mono)]` on an imported
//! type that is itself generic (`Container<T>`), the shape `js-sys` types like
//! `Array<T>` use. This is distinct from `generic_import_methods.rs`, where
//! the *function* has its own type parameter but the receiver (`Widget`) is
//! not itself generic.
//!
//! Here the type parameter lives on the receiver/return *class*
//! (`this: &Container<T>`), which the per-mono codegen hoists out of the
//! wrapper `fn`'s own generic parameter list and onto the enclosing `impl`
//! block (`impl<T> Container<T> { .. }`) via the same `get_fn_generics`
//! machinery the type-erasure generic-import path uses.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/generic_import_class_generics.js")]
extern "C" {
    type Container<T = JsValue>;

    // Constructor: the return type `Container<T>` carries the class-level
    // parameter, so the generated `impl` block is `impl<T> Container<T>`
    // rather than a generic method on a non-generic `impl Container`.
    #[wasm_bindgen(constructor, generic_per_mono)]
    fn new<T>(value: T) -> Container<T>;

    // Self-returning static method: same class-level hoisting as the
    // constructor, driven by `class_return_path()` instead of `constructor`.
    #[wasm_bindgen(static_method_of = Container, generic_per_mono, js_name = of)]
    fn of<T>(value: T) -> Container<T>;

    // Getter/setter with a generic return/argument, on a class-generic
    // receiver: two sources of "genericness" (the class's own `T`, threaded
    // through the receiver) rather than one.
    #[wasm_bindgen(method, getter, generic_per_mono, js_name = value)]
    fn value<T>(this: &Container<T>) -> T;

    #[wasm_bindgen(method, setter, generic_per_mono, js_name = value)]
    fn set_value<T>(this: &Container<T>, v: T);

    // A bare shared reference to the class's own type parameter
    // (`item: &T`), on a class-generic receiver. This exercises both
    // hoisting *and* the "bare shared ref" support together.
    #[wasm_bindgen(method, generic_per_mono)]
    fn push<T>(this: &Container<T>, item: &T) -> u32;

    // Plain (non-`generic_per_mono`) inspection helpers, so a bug in the
    // per-mono path being tested cannot also corrupt the assertions
    // themselves. These go through the existing type-erasure generic-import
    // path, which already supports a class-generic receiver.
    #[wasm_bindgen(js_name = containerValue)]
    fn container_value<T>(c: &Container<T>) -> JsValue;
    #[wasm_bindgen(js_name = containerItems)]
    fn container_items<T>(c: &Container<T>) -> String;
}

#[wasm_bindgen_test]
fn generic_per_mono_class_generic_constructor() {
    let a = Container::<u32>::new(7u32);
    assert_eq!(container_value(&a), JsValue::from(7u32));

    // A second monomorphisation must produce a distinct shim, not reuse the
    // first (and not conflict with it: both `impl<T> Container<T>` methods
    // are monomorphised independently per `T`).
    let b = Container::<String>::new(String::from("hi"));
    assert_eq!(container_value(&b), JsValue::from("hi"));
}

#[wasm_bindgen_test]
fn generic_per_mono_class_generic_static_method() {
    let a = Container::<u32>::of(3u32);
    assert_eq!(container_value(&a), JsValue::from(3u32));

    let b = Container::<String>::of(String::from("s"));
    assert_eq!(container_value(&b), JsValue::from("s"));
}

#[wasm_bindgen_test]
fn generic_per_mono_class_generic_getter_and_setter() {
    let c = Container::<u32>::new(1u32);
    assert_eq!(c.value(), 1u32);

    c.set_value(11u32);
    assert_eq!(c.value(), 11u32);
    assert_eq!(container_value(&c), JsValue::from(11u32));
}

#[wasm_bindgen_test]
fn generic_per_mono_class_generic_method_with_ref_arg() {
    // `&T` is only supported when `T` monomorphises to a copyable value type
    // or a JS handle (see `generic_import_ref.rs`), not e.g. `String`.
    let c = Container::<u32>::new(0u32);

    let len = c.push(&1u32);
    assert_eq!(len, 1);
    let len = c.push(&2u32);
    assert_eq!(len, 2);

    assert_eq!(container_items(&c), "1,2");
}
