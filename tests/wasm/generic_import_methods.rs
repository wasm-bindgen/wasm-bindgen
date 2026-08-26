//! Runtime coverage for `#[wasm_bindgen(experimental_generic_mono)]` on every
//! *method-shaped* import: constructors, instance methods, static methods,
//! getters and setters (plain, `structural` and `final`), and the `indexing_*`
//! operations.
//!
//! `crates/cli/tests/reference/generic-import.rs` pins the JS these emit, but a
//! snapshot cannot catch a binding that is internally consistent and still
//! wrong — a swapped receiver, a getter resolved off the wrong prototype, or an
//! index written to the wrong key all produce perfectly plausible glue. These
//! tests execute each shape and assert on observable state.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/generic_import_methods.js")]
extern "C" {
    type Widget;

    // `constructor` + `experimental_generic_mono`: `class_return_path()` retargets the
    // generated `impl` block to the *return* type's class, so the manufactured
    // binding attaches to `new Widget(..)`.
    #[wasm_bindgen(constructor, experimental_generic_mono)]
    fn new<T>(value: T) -> Widget;

    // Instance method. Two instantiations prove distinct per-mono shims on the
    // method path.
    #[wasm_bindgen(method, experimental_generic_mono, js_name = set)]
    fn set<T>(this: &Widget, value: T);

    // `&T` in a method argument, monomorphised at a JS-handle type, so the
    // handle's `IntoWasmAbi for &Widget` impl is what marshals it.
    #[wasm_bindgen(method, experimental_generic_mono, js_name = attach)]
    fn attach<T>(this: &Widget, other: &T);

    // Static method.
    #[wasm_bindgen(static_method_of = Widget, experimental_generic_mono, js_name = of)]
    fn of<T>(value: T) -> Widget;

    // Getter with a generic *return*: every monomorphisation reads the same JS
    // property but marshals the result at its own concrete type.
    #[wasm_bindgen(method, getter, experimental_generic_mono, js_name = value)]
    fn value<T>(this: &Widget) -> T;

    #[wasm_bindgen(method, setter, experimental_generic_mono, js_name = value)]
    fn set_value<T>(this: &Widget, v: T);

    // `structural` accessors read/write the property directly on the receiver
    // rather than through a captured descriptor.
    #[wasm_bindgen(method, getter, structural, experimental_generic_mono, js_name = tag)]
    fn tag<T>(this: &Widget) -> T;

    #[wasm_bindgen(method, setter, structural, experimental_generic_mono, js_name = tag)]
    fn set_tag<T>(this: &Widget, v: T);

    // `final` captures the property descriptor from `Widget.prototype` once, at
    // instantiation time, and invokes it with `.call(receiver)`. Getting the
    // receiver wrong here is exactly the silent failure a snapshot cannot see.
    #[wasm_bindgen(method, getter, final, experimental_generic_mono, js_name = kind)]
    fn kind<T>(this: &Widget) -> T;

    // `indexing_*` emit `obj[prop]`, `obj[prop] = val` and `delete obj[prop]`,
    // and always require `structural` + `method`.
    #[wasm_bindgen(method, structural, indexing_getter, experimental_generic_mono)]
    fn get<T>(this: &Widget, prop: &str) -> T;

    #[wasm_bindgen(method, structural, indexing_setter, experimental_generic_mono)]
    fn set_indexed<T>(this: &Widget, prop: &str, val: T);

    // The deleter has no value parameter, so its type parameter appears only in
    // the *index* position.
    #[wasm_bindgen(method, structural, indexing_deleter, experimental_generic_mono)]
    fn delete_indexed<T>(this: &Widget, prop: T);

    #[wasm_bindgen(js_name = widgetValue)]
    fn widget_value(w: &Widget) -> JsValue;
    #[wasm_bindgen(js_name = widgetTag)]
    fn widget_tag(w: &Widget) -> JsValue;
    #[wasm_bindgen(js_name = widgetReceived)]
    fn widget_received(w: &Widget) -> String;
    #[wasm_bindgen(js_name = widgetAttachedValue)]
    fn widget_attached_value(w: &Widget) -> String;
    #[wasm_bindgen(js_name = widgetHasProp)]
    fn widget_has_prop(w: &Widget, prop: &str) -> bool;
    #[wasm_bindgen(js_name = widgetSetProp)]
    fn widget_set_prop(w: &Widget, prop: &str, value: &JsValue);
}

#[wasm_bindgen_test]
fn experimental_generic_mono_constructor() {
    let a = Widget::new(7u32);
    assert_eq!(widget_value(&a), JsValue::from(7u32));
    // The constructor ran, rather than the argument being handed to some other
    // slot: `_kind` is derived from the argument's JS type.
    assert_eq!(a.kind::<String>(), "widget:number");

    // A second monomorphisation must produce a distinct shim, not reuse the first.
    let b = Widget::new(String::from("hi"));
    assert_eq!(widget_value(&b), JsValue::from("hi"));
    assert_eq!(b.kind::<String>(), "widget:string");
}

#[wasm_bindgen_test]
fn experimental_generic_mono_method_receives_correct_receiver_and_value() {
    let a = Widget::new(0u32);
    let b = Widget::new(0u32);

    a.set(1u32);
    a.set(2.5f64);
    a.set(String::from("x"));
    b.set(9u32);

    // Values arrive in order on the right receiver. A binding that passed the
    // receiver as the first argument, or crossed the two handles, fails here.
    assert_eq!(widget_received(&a), "1,2.5,x");
    assert_eq!(widget_received(&b), "9");
}

#[wasm_bindgen_test]
fn experimental_generic_mono_method_with_handle_ref_arg() {
    let host = Widget::new(0u32);
    let guest = Widget::new(42u32);

    host.attach(&guest);

    // The *right* handle crossed, not merely some handle.
    assert_eq!(widget_attached_value(&host), "42");
    assert_eq!(widget_attached_value(&guest), "none");
}

#[wasm_bindgen_test]
fn experimental_generic_mono_static_method() {
    let a = Widget::of(5u32);
    assert_eq!(widget_value(&a), JsValue::from(5u32));
    // `of` sets a distinct `_kind`, so this fails if the call was routed to the
    // constructor instead.
    assert_eq!(a.kind::<String>(), "static:number");

    let b = Widget::of(String::from("s"));
    assert_eq!(b.kind::<String>(), "static:string");
}

#[wasm_bindgen_test]
fn experimental_generic_mono_getter_and_setter() {
    let w = Widget::new(1u32);

    // Generic return, marshalled at each concrete type.
    assert_eq!(w.value::<u32>(), 1);

    w.set_value(11u32);
    assert_eq!(w.value::<u32>(), 11);
    assert_eq!(widget_value(&w), JsValue::from(11u32));

    w.set_value(String::from("str"));
    assert_eq!(w.value::<String>(), "str");

    w.set_value(2.5f64);
    assert_eq!(w.value::<f64>(), 2.5);
}

#[wasm_bindgen_test]
fn experimental_generic_mono_structural_getter_and_setter() {
    let w = Widget::new(0u32);

    w.set_tag(3u32);
    assert_eq!(w.tag::<u32>(), 3);
    assert_eq!(widget_tag(&w), JsValue::from(3u32));

    w.set_tag(String::from("t"));
    assert_eq!(w.tag::<String>(), "t");
}

#[wasm_bindgen_test]
fn experimental_generic_mono_final_getter_resolves_against_the_receiver() {
    // `final` captures `Widget.prototype`'s descriptor once and invokes it with
    // `.call(receiver)`. Two receivers with different `_kind` values prove the
    // captured getter is applied to the argument rather than to a fixed object.
    let a = Widget::new(1u32);
    let b = Widget::new(String::from("s"));

    assert_eq!(a.kind::<String>(), "widget:number");
    assert_eq!(b.kind::<String>(), "widget:string");
}

#[wasm_bindgen_test]
fn experimental_generic_mono_indexing_getter_and_setter() {
    let w = Widget::new(0u32);

    w.set_indexed("a", 1u32);
    w.set_indexed("b", String::from("two"));

    assert_eq!(w.get::<u32>("a"), 1);
    assert_eq!(w.get::<String>("b"), "two");

    // Written to the named key, not to an adjacent or numeric one.
    assert!(widget_has_prop(&w, "a"));
    assert!(widget_has_prop(&w, "b"));
    assert!(!widget_has_prop(&w, "c"));
}

#[wasm_bindgen_test]
fn experimental_generic_mono_indexing_deleter() {
    let w = Widget::new(0u32);

    widget_set_prop(&w, "gone", &JsValue::from(1u32));
    widget_set_prop(&w, "kept", &JsValue::from(2u32));
    assert!(widget_has_prop(&w, "gone"));

    // The type parameter is in the *index* position here.
    w.delete_indexed(String::from("gone"));

    assert!(!widget_has_prop(&w, "gone"));
    assert!(widget_has_prop(&w, "kept"));
}
