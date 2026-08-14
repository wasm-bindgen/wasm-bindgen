//! Runtime coverage for `#[wasm_bindgen(generic_per_mono)]` on a raw `&dyn
//! Fn(...)` / `&mut dyn FnMut(...)` trait-object argument whose own call
//! signature — not just the rest of the import's signature — mentions a type
//! parameter, e.g. `predicate: &mut dyn FnMut(T, u32, Array<T>) -> bool`. This
//! is the `Array::for_each`/`Array::every` shape in `js-sys`.
//!
//! This is distinct from `generic_import_args.rs`'s `with_callback`, which
//! covers a *concrete* `&mut dyn FnMut(u32)` argument alongside an unrelated
//! `T` elsewhere in the signature: there, the closure itself never mentions
//! `T`, so it goes through the same codegen a non-generic import would use.
//! Here `T` (and sometimes a second parameter `U`) lives inside the closure's
//! own argument list or return type, which needs the monomorphised shim to
//! describe the closure at its *concrete* signature per instantiation.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/generic_import_closures.js")]
extern "C" {
    // Regression/sanity: the closure itself is fully concrete (`u32`); only
    // the unrelated `times` argument is generic. This must keep going through
    // the *existing* concrete `&dyn Fn`/`&mut dyn FnMut` codegen rather than
    // the new per-monomorphisation closure path, since there is nothing about
    // the closure to monomorphise.
    #[wasm_bindgen(generic_per_mono, js_name = concreteCallback)]
    fn concrete_callback<T>(times: T, f: &mut dyn FnMut(u32));

    // The closure's *argument* mentions the type parameter.
    #[wasm_bindgen(generic_per_mono, js_name = forEachOwned)]
    fn for_each_owned<T>(xs: Vec<T>, f: &mut dyn FnMut(T));

    // The closure's *return* mentions the type parameter (and nothing else in
    // the signature does).
    #[wasm_bindgen(generic_per_mono, js_name = buildValue)]
    fn build_value<T>(f: &mut dyn FnMut(u32) -> T) -> T;

    // A shared (non-`mut`) `&dyn Fn`, with two distinct type parameters: one
    // in the closure's argument, a different one in its return.
    //
    // The outer return here is deliberately a bare `U`, not `Vec<U>`: a
    // `Vec<U>` return whose `U` differs from another type parameter used
    // elsewhere in the signature (e.g. the `T` in `xs: Vec<T>` below) trips a
    // pre-existing bug in `generic_per_mono`'s general return-type handling
    // that has nothing to do with closures — reproducible with no closure
    // argument at all. That is out of scope for this change; `fold_values`
    // below covers the realistic "map over a `Vec<T>`" shape without hitting
    // it, by folding down to a single `U` instead of collecting a `Vec<U>`.
    #[wasm_bindgen(generic_per_mono, js_name = transformValue)]
    fn transform_value<T, U>(x: T, f: &dyn Fn(T) -> U) -> U;

    // Multiple closures in one signature, each independently monomorphised:
    // a `Fn` mapper and a separate `FnMut` reducer, folding over a `Vec<T>`.
    #[wasm_bindgen(generic_per_mono, js_name = foldValues)]
    fn fold_values<T, U>(
        xs: Vec<T>,
        init: U,
        mapper: &dyn Fn(T) -> U,
        reducer: &mut dyn FnMut(U, U) -> U,
    ) -> U;

    // The `Array<T>::for_each`/`Array<T>::every` shape itself: a closure
    // argument mentioning the *class's* hoisted type parameter, composing
    // class-level generics (see `generic_import_class_generics.rs`) with a
    // generic closure signature.
    type Bucket<T = JsValue>;

    #[wasm_bindgen(constructor, generic_per_mono)]
    fn new<T>(items: Vec<T>) -> Bucket<T>;

    #[wasm_bindgen(method, generic_per_mono, js_name = forEach)]
    fn for_each<T>(this: &Bucket<T>, f: &mut dyn FnMut(T, u32, Bucket<T>));

    #[wasm_bindgen(method, generic_per_mono)]
    fn every<T>(this: &Bucket<T>, predicate: &mut dyn FnMut(T, u32, Bucket<T>) -> bool) -> bool;

    // Plain (non-`generic_per_mono`) inspection helper for the third
    // (container) argument the closures above receive, so a bug in the
    // per-mono path under test cannot also corrupt the assertion.
    #[wasm_bindgen(js_name = bucketLen)]
    fn bucket_len<T>(b: &Bucket<T>) -> u32;
}

#[wasm_bindgen_test]
fn generic_per_mono_concrete_closure_regression() {
    let mut seen = Vec::new();
    {
        let mut push = |v: u32| seen.push(v);
        concrete_callback(3u32, &mut push);
    }
    assert_eq!(seen, vec![0, 1, 2]);

    // A second monomorphisation of the unrelated `T`, same concrete closure.
    let mut count = 0u32;
    {
        let mut bump = |_: u32| count += 1;
        concrete_callback(2.0f64, &mut bump);
    }
    assert_eq!(count, 2);
}

#[wasm_bindgen_test]
fn generic_per_mono_closure_argument_is_generic() {
    let mut seen: Vec<u32> = Vec::new();
    for_each_owned(vec![1u32, 2, 3], &mut |v| seen.push(v));
    assert_eq!(seen, vec![1, 2, 3]);

    // A second monomorphisation: the closure's own argument type changes.
    let mut seen_strings: Vec<String> = Vec::new();
    for_each_owned(vec![String::from("a"), String::from("b")], &mut |v| {
        seen_strings.push(v)
    });
    assert_eq!(seen_strings, vec!["a".to_string(), "b".to_string()]);
}

#[wasm_bindgen_test]
fn generic_per_mono_closure_return_is_generic() {
    // JS calls `f(1)` once and hands the result straight back.
    assert_eq!(build_value(&mut |x| x * 2), 2u32);

    // A second monomorphisation of the closure's return type.
    assert_eq!(build_value(&mut |x| format!("n{x}")), "n1".to_string());
}

#[wasm_bindgen_test]
fn generic_per_mono_shared_fn_with_two_generic_params() {
    let out = transform_value(21u32, &|x| x * 2);
    assert_eq!(out, 42u32);

    let out = transform_value(21u32, &|x| format!("v{x}"));
    assert_eq!(out, "v21".to_string());
}

#[wasm_bindgen_test]
fn generic_per_mono_multiple_generic_closures() {
    let total = fold_values(vec![1u32, 2, 3, 4], 0u32, &|x| x, &mut |a, b| a + b);
    assert_eq!(total, 10);

    // Different `T`/`U` monomorphisation: map to string length, reduce by max.
    let words = vec!["a".to_string(), "bcd".to_string(), "ef".to_string()];
    let longest = fold_values(words, 0u32, &|s| s.len() as u32, &mut |a, b| a.max(b));
    assert_eq!(longest, 3);
}

#[wasm_bindgen_test]
fn generic_per_mono_class_generic_for_each() {
    let bucket = Bucket::new(vec![10u32, 20, 30]);
    let mut seen: Vec<(u32, u32)> = Vec::new();
    bucket.for_each(&mut |value, index, container| {
        seen.push((value, index));
        assert_eq!(bucket_len(&container), 3);
    });
    assert_eq!(seen, vec![(10, 0), (20, 1), (30, 2)]);

    // A second monomorphisation of the class's own type parameter.
    let words = Bucket::new(vec!["x".to_string(), "y".to_string()]);
    let mut collected: Vec<String> = Vec::new();
    words.for_each(&mut |value, _index, _container| collected.push(value));
    assert_eq!(collected, vec!["x".to_string(), "y".to_string()]);
}

#[wasm_bindgen_test]
fn generic_per_mono_class_generic_every() {
    let bucket = Bucket::new(vec![2u32, 4, 6]);
    assert!(bucket.every(&mut |value, _index, _container| value % 2 == 0));

    let bucket = Bucket::new(vec![2u32, 3, 6]);
    // Must stop as soon as the predicate returns `false`, so only the first
    // two elements are visited.
    let mut visited = 0u32;
    let all_even = bucket.every(&mut |value, _index, _container| {
        visited += 1;
        value % 2 == 0
    });
    assert!(!all_even);
    assert_eq!(visited, 2);
}
