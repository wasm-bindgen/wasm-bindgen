// Reference output for the experimental per-monomorphisation generic import
// path (`#[wasm_bindgen(generic_per_mono)]`). Each concrete instantiation is
// discovered by the descriptor interpreter and bound to its own manufactured
// `__wbindgen_generic_*` JS shim, rather than erasing type parameters to
// `JsValue`.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Free function, generic owned argument, unit return. Two instantiations
    // (`u32` and `f64`) each get their own shim with the correct marshalling.
    #[wasm_bindgen(generic_per_mono, js_name = log)]
    fn log_generic<T>(x: T);

    // Generic pass-through return `-> T`.
    #[wasm_bindgen(generic_per_mono, js_name = identity)]
    fn identity<T>(x: T) -> T;

    // A concrete argument mixed with a generic one. Two instantiations exercise
    // per-mono shim manufacture for a mixed signature.
    #[wasm_bindgen(generic_per_mono, js_name = mix)]
    fn mix<T>(label: u32, value: T);

    // Multiple generic type parameters in a single import.
    #[wasm_bindgen(generic_per_mono, js_name = pair)]
    fn pair<T, U>(a: T, b: U);

    // A trait-bounded generic import. The bound has to be re-emitted onto the
    // manufactured shim for `T::Item` to resolve, which is the most intricate
    // part of the per-monomorphisation codegen and is otherwise pinned only by
    // the wasm test suite, where a regression shows up as a runtime failure with
    // no diff to read.
    #[wasm_bindgen(generic_per_mono, js_name = sumItems)]
    fn sum_items<T: IntoIterator<Item = u32>>(xs: T) -> f64;

    // Declared but never instantiated. A `generic_per_mono` import with no
    // monomorphisations is what any library crate hits for the imports its
    // consumers do not call, and it takes a different path: the binding table
    // gains an entry with no matching descriptor, and the anchoring descriptor
    // export has to be interpreted and deleted rather than leaking into the
    // output. Nothing should be emitted for it below.
    #[wasm_bindgen(generic_per_mono, js_name = neverCalled)]
    fn never_called<T>(x: T);

    // A bare shared reference to a generic type parameter (`&T`). Copyable
    // referents (`&u32`, `&f64`) marshal by value; `&JsValue` marshals as an
    // externref. Each instantiation gets its own per-mono shim.
    #[wasm_bindgen(generic_per_mono, js_name = logRef)]
    fn log_ref<T>(x: &T);

    // A shared slice with a *generic* element type. `&[T]` takes the same
    // route as `&T` above (an HRTB `for<'a> &'a [T]: IntoWasmAbi` bound), so
    // each element type marshals as its own typed-array view.
    #[wasm_bindgen(generic_per_mono, js_name = logGenericSlice)]
    fn log_generic_slice<T>(xs: &[T]);

    // ...including as the `variadic` argument, which the non-sequence variadic
    // diagnostic explicitly recommends.
    #[wasm_bindgen(generic_per_mono, variadic, js_name = spreadGeneric)]
    fn spread_generic<T>(first: u32, rest: &[T]);

    // `catch` produces a `handleError`-wrapped shim.
    #[wasm_bindgen(generic_per_mono, catch, js_name = tryLog)]
    fn try_log<T>(x: T) -> Result<(), JsValue>;

    // `variadic` spreads the final argument. The variadic argument must be a
    // concrete iterable (here `Vec<T>`), which marshals to a spreadable JS
    // array; a bare generic `T` is rejected because it may monomorphise to a
    // non-iterable scalar.
    #[wasm_bindgen(generic_per_mono, variadic, js_name = variadicLog)]
    fn variadic_log<T>(first: u32, rest: Vec<T>);

    // `slice_to_array` hands JS a plain `Array` it owns rather than a
    // typed-array view into wasm memory. The slice element type must be
    // concrete, so the rewrite is independent of which monomorphisation is
    // being generated.
    #[wasm_bindgen(generic_per_mono, slice_to_array, js_name = logSlice)]
    fn log_slice<T>(xs: &[u16], other: T);

    #[wasm_bindgen(generic_per_mono, slice_to_array, js_name = logOptSlice)]
    fn log_opt_slice<T>(xs: Option<&[u16]>, other: T);

    // A `String` element type takes the *other* ownership path: JS receives a
    // freshly allocated index buffer that it must free, unlike the primitive
    // case above which borrows the caller's slice.
    #[wasm_bindgen(generic_per_mono, slice_to_array, js_name = logStrSlice)]
    fn log_str_slice<T>(xs: &[String], other: T);

    #[wasm_bindgen(generic_per_mono, slice_to_array, js_name = logOptStrSlice)]
    fn log_opt_str_slice<T>(xs: Option<&[String]>, other: T);

    // `async` imports return a `Promise` across the ABI whatever they resolve
    // to, so the descriptor is an externref and the resolved value is converted
    // separately inside `JsFuture<T>`. That makes a monomorphised `-> T` work,
    // including for a `T` that is not itself handle-shaped.
    #[wasm_bindgen(generic_per_mono, js_name = asyncIdentity)]
    async fn async_identity<T>(x: T) -> T;

    // Same, but resolving to a concrete non-handle type.
    #[wasm_bindgen(generic_per_mono, js_name = asyncCount)]
    async fn async_count<T>(x: T) -> u32;

    // And through the `Ok` type of a `catch` import.
    #[wasm_bindgen(generic_per_mono, catch, js_name = asyncTry)]
    async fn async_try<T>(x: T) -> Result<T, JsValue>;

    // Non-async `catch` with a *generic* `Ok` type. Unlike `try_log` above
    // (whose `Ok` is `()`) the success value has to be marshalled at each
    // monomorphisation's concrete type, while the error stays `JsValue`.
    #[wasm_bindgen(generic_per_mono, catch, js_name = tryGet)]
    fn try_get<T>(key: u32) -> Result<T, JsValue>;

    // `&mut` to a *concrete* type inside a `generic_per_mono` import. These bind
    // exactly as they do on the non-generic import path: a `&mut [u16]` is a
    // mutable typed-array view into wasm memory that is written back, and a
    // `&mut dyn FnMut` gets a reentrancy-guarded JS wrapper. Only a `&mut` whose
    // referent mentions a type parameter is rejected.
    #[wasm_bindgen(generic_per_mono, js_name = fillSlice)]
    fn fill_slice<T>(xs: &mut [u16], other: T);

    #[wasm_bindgen(generic_per_mono, js_name = withCallback)]
    fn with_callback<T>(f: &mut dyn FnMut(u32), other: T);
}

// The concrete `impl IntoWasmAbi for &$t` impls generated by
// `ref_into_wasm_abi_for_scalar!` enable `&scalar` arguments for *every* import,
// not just `generic_per_mono` ones. This block is an ordinary non-generic
// import, so it binds through one named `__wbg_*` shim and pins the wire form
// of each scalar passed by reference.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = takeScalarRefs)]
    fn take_scalar_refs(a: &u32, b: &i8, c: &bool, d: &char, e: &f64, f: &i64, g: &isize);

    // `i128`/`u128` span multiple ABI prims, so `Ref(..)` of them is the case
    // most likely to be marshalled wrongly, and `f32` is the remaining float
    // width. `scalar_by_shared_ref_set_is_exactly_the_scalars` guards which
    // types are in the set, not how they cross.
    #[wasm_bindgen(js_name = takeWideScalarRefs)]
    fn take_wide_scalar_refs(a: &i128, b: &u128, c: &f32);
}

// `slice_to_array` is inheritable from the enclosing block and applies to every
// slice-shaped argument of every function it covers, `generic_per_mono` included.
#[wasm_bindgen(slice_to_array)]
extern "C" {
    #[wasm_bindgen(generic_per_mono, js_name = logBlockSlice)]
    fn log_block_slice<T>(xs: &[u16], other: T);
}

// `generic_per_mono` is itself inheritable from the enclosing block, so the
// generic functions here take the per-monomorphisation path without repeating the
// attribute — each instantiation still gets its own `__wbindgen_generic_*` shim
// rather than a single erased binding.
#[wasm_bindgen(generic_per_mono)]
extern "C" {
    #[wasm_bindgen(js_name = blockInherited)]
    fn block_inherited<T>(x: T);

    // A second function in the same block, to show the flag really is
    // block-scoped rather than attaching to the first item.
    #[wasm_bindgen(js_name = blockInheritedTwo)]
    fn block_inherited_two<T>(a: u32, b: T) -> T;

    // A non-generic function in the same block. The block flag cannot apply to
    // it, so it is left alone and binds through one ordinary named `__wbg_*`
    // shim rather than a manufactured `__wbindgen_generic_*` one. This is what
    // lets a block mix the two.
    #[wasm_bindgen(js_name = blockNotGeneric)]
    fn block_not_generic(x: u32);
}

// `js_namespace` resolves the JS import the same way it does for a non-generic
// import; the namespace lookup is descriptor-independent, so every
// monomorphisation binds through the same namespaced value.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, generic_per_mono, js_name = log)]
    fn ns_log<T>(x: T);

    #[wasm_bindgen(js_namespace = ["a", "b"], generic_per_mono, js_name = deepLog)]
    fn ns_deep_log<T>(x: T);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Widget)]
    type Widget;

    // `constructor` + `generic_per_mono`. This has dedicated codegen:
    // `class_return_path()` retargets the generated `impl` block to the *return*
    // type's class, so the manufactured binding attaches to `new Widget(..)`
    // rather than to the free-function path.
    #[wasm_bindgen(constructor, generic_per_mono, js_class = "Widget")]
    fn new<T>(value: T) -> Widget;

    // A generic method binds as an instance method call. Two instantiations
    // prove distinct per-mono shims for the method path.
    #[wasm_bindgen(method, generic_per_mono, js_class = "Widget", js_name = set)]
    fn set<T>(this: &Widget, value: T);

    // A generic method taking a bare shared reference `&T`. Here `T`
    // monomorphises to the JS-handle `Widget`, so `&Widget` marshals via the
    // handle's `IntoWasmAbi for &Widget` impl.
    #[wasm_bindgen(method, generic_per_mono, js_class = "Widget", js_name = attach)]
    fn attach<T>(this: &Widget, other: &T);

    // A generic static method. Two instantiations prove distinct per-mono shims
    // for the static path.
    #[wasm_bindgen(static_method_of = Widget, generic_per_mono, js_name = of)]
    fn of<T>(value: T) -> Widget;

    // A getter with a generic *return*: each monomorphisation reads the same JS
    // property but marshals the result at its concrete type.
    #[wasm_bindgen(method, getter, generic_per_mono, js_class = "Widget", js_name = value)]
    fn value<T>(this: &Widget) -> T;

    // A setter with a generic argument.
    #[wasm_bindgen(method, setter, generic_per_mono, js_class = "Widget", js_name = value)]
    fn set_value<T>(this: &Widget, v: T);

    // `structural` accessors go through a property access on the receiver rather
    // than a bound function reference, and compose with per-mono the same way.
    #[wasm_bindgen(
        method,
        getter,
        structural,
        generic_per_mono,
        js_class = "Widget",
        js_name = tag
    )]
    fn tag<T>(this: &Widget) -> T;

    #[wasm_bindgen(
        method,
        setter,
        structural,
        generic_per_mono,
        js_class = "Widget",
        js_name = tag
    )]
    fn set_tag<T>(this: &Widget, v: T);

    // `final` is the opposite of `structural`: the property is looked up once at
    // instantiation time rather than on every call.
    #[wasm_bindgen(method, getter, final, generic_per_mono, js_class = "Widget", js_name = kind)]
    fn kind<T>(this: &Widget) -> T;

    // The `indexing_*` operations emit `obj[prop]`, `obj[prop] = val` and
    // `delete obj[prop]`. They always require `structural` + `method`. Here the
    // index is concrete and the value generic, so each monomorphisation differs
    // only in how the value crosses.
    #[wasm_bindgen(method, structural, indexing_getter, generic_per_mono)]
    fn get<T>(this: &Widget, prop: &str) -> T;

    #[wasm_bindgen(method, structural, indexing_setter, generic_per_mono)]
    fn set_indexed<T>(this: &Widget, prop: &str, val: T);

    // A deleter has no value parameter, so its type parameter appears only in the
    // *index* position.
    #[wasm_bindgen(method, structural, indexing_deleter, generic_per_mono)]
    fn delete_indexed<T>(this: &Widget, prop: T);
}

// The concrete `impl IntoWasmAbi for &$t` impls generated by
// `ref_into_wasm_abi_for_scalar!` also satisfy `ReturnWasmAbi`, so they widen
// what *exported* functions may return, not just what imports may take.
// Writing `-> &u32` directly is still rejected by the
// macro ("cannot return a borrowed ref with #[wasm_bindgen]"), but that guard is
// necessarily syntactic — a proc macro cannot resolve a type alias — so the
// alias form below reaches the ABI layer and returns the pointee by copy.
//
// Pinned here because it is reachable from ordinary user code and silent: there
// is no diagnostic either way, so a change in the emitted ABI would otherwise go
// unnoticed.
type ScalarRef = &'static u32;

#[wasm_bindgen]
pub fn return_scalar_ref_via_alias() -> ScalarRef {
    &42
}

#[wasm_bindgen]
pub async fn run(widget: &Widget) -> Result<(), JsValue> {
    log_generic(1u32);
    log_generic(2.0f64);
    log_generic(String::from("three"));
    // `&str` and `String` are distinct wire protocols behind the same JS
    // string: a borrowed (ptr, len) pair with no free vs an owned buffer the
    // shim frees.
    log_generic("four");

    let _ = identity(3u32);
    let _ = identity(4.0f64);

    mix(5, 6u32);
    mix(6, 7.0f64);

    pair(1u32, 2.0f64);

    log_ref(&13u32);
    log_ref(&14.0f64);
    log_ref(&JsValue::from("fifteen"));

    log_generic_slice(&[16u32, 17]);
    log_generic_slice(&[18.5f64]);
    spread_generic(19, &[20u32, 21]);

    try_log(7u32)?;

    variadic_log(8, vec![9u32, 10u32]);

    log_slice(&[1u16, 2u16], 9u32);
    log_slice(&[3u16, 4u16], 10.0f64);
    log_opt_slice(Some(&[5u16]), 11u32);
    log_opt_slice(None, 12u32);
    log_str_slice(&[String::from("a")], 13u32);
    log_opt_str_slice(Some(&[String::from("b")]), 18u32);
    log_opt_str_slice(None, 19u32);
    log_block_slice(&[6u16], 14u32);

    block_inherited(20u32);
    block_inherited(21.0f64);
    let _: f64 = block_inherited_two(22, 23.0f64);
    block_not_generic(24);

    ns_log(23u32);
    ns_deep_log(24.0f64);

    let _: u32 = async_identity(15u32).await;
    let _: String = async_identity(String::from("b")).await;
    let _: u32 = async_count(16u32).await;
    let _: u32 = async_try(17u32).await?;

    let _: u32 = try_get(30)?;
    let _: f64 = try_get(31)?;

    let mut buf = [1u16, 2u16];
    fill_slice(&mut buf, 28u32);
    let mut seen = 0u32;
    with_callback(&mut |v| seen += v, 29u32);

    take_scalar_refs(&1u32, &2i8, &true, &'c', &3.0f64, &4i64, &5isize);
    take_wide_scalar_refs(&1i128, &2u128, &3.0f32);

    let _ = sum_items(vec![1u32, 2u32]);

    // `never_called` is deliberately not instantiated here.

    // Two instantiations of the generic constructor.
    let _ = Widget::new(32u32);
    let _ = Widget::new(33.0f64);

    widget.set(10u32);
    widget.set(11.0f64);

    widget.attach(widget);

    let _ = Widget::of(11u32);
    let _ = Widget::of(12.0f64);

    let _: u32 = widget.value();
    let _: f64 = widget.value();
    widget.set_value(25u32);

    let _: u32 = widget.tag();
    widget.set_tag(26.0f64);

    let _: u32 = widget.kind();

    let _: u32 = widget.get("k");
    widget.set_indexed("k", 27u32);
    widget.delete_indexed("k");

    Ok(())
}
