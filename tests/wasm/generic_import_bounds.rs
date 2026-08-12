//! Runtime coverage for user-written trait bounds on a per-monomorphisation
//! generic import (`#[wasm_bindgen(generic_per_mono)]`).
//!
//! Bounds are part of the declared signature's contract, so they must be carried
//! through codegen rather than dropped. Two routes are exercised, because they
//! reach the generated wrapper differently:
//! - inline bounds (`fn f<T: Trait>`), which travel with the parameter list;
//! - `where` predicates, which have no such carrier and must be re-emitted.
//!
//! A bound must also reach the monomorphised shim, whose ABI signature can
//! project an associated type off a bounded parameter (`T::Wire`); without the
//! bound in scope there, that projection does not resolve. `compile-fail`
//! coverage for callers that violate a bound lives in
//! `crates/macro/ui-tests/generic-per-mono-bounds.rs`.
//!
//! Alongside the purpose-built `Wire` trait, real library traits are exercised
//! (`Iterator`, `IntoIterator`, `ExactSizeIterator`), because they have shapes a
//! hand-rolled trait does not: their bound paths carry generic arguments
//! (`Iterator<Item = u32>`, `AsRef<[u32]>`), and a projection off them can
//! resolve through a supertrait rather than the named trait itself. Those
//! arguments must survive verbatim into both the wrapper and the shim.

use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

/// Projects a type parameter onto the type that actually crosses the ABI.
trait Wire {
    type Wire;
    fn lift(wire: Self::Wire) -> Self;
}

impl Wire for u32 {
    type Wire = u32;
    fn lift(wire: u32) -> u32 {
        wire
    }
}

impl Wire for String {
    type Wire = String;
    fn lift(wire: String) -> String {
        wire
    }
}

#[wasm_bindgen(module = "tests/wasm/generic_import_bounds.js")]
extern "C" {
    #[wasm_bindgen(js_name = takeLog)]
    fn take_log() -> String;

    // Inline bound.
    #[wasm_bindgen(generic_per_mono, js_name = record)]
    fn record_inline<T: Copy + Clone>(x: T);

    // `where` predicate.
    #[wasm_bindgen(generic_per_mono, js_name = record)]
    fn record_where<T>(x: T)
    where
        T: Clone;

    // Inline and `where` bounds on the same signature, across two parameters.
    #[wasm_bindgen(generic_per_mono, js_name = sum)]
    fn sum_bounded<T: Copy, U>(a: T, b: U) -> f64
    where
        U: Copy;

    // Argument-position `impl Trait`: desugars into a synthesized named type
    // parameter with the same bound, so it exercises the identical
    // bound-carrying machinery as `record_inline` above, just declared the
    // sugared way with no named type parameter of its own.
    #[wasm_bindgen(generic_per_mono, js_name = record)]
    fn record_impl_trait(x: impl Copy + Clone);

    // `impl Trait` mixed with a real, named, bounded type parameter in the
    // same signature.
    #[wasm_bindgen(generic_per_mono, js_name = sum)]
    fn sum_impl_trait_mixed<U: Copy>(a: impl Copy, b: U) -> f64;

    // The bound's associated type appears in argument and return position, so
    // the bound has to reach the shim for `T::Wire`'s ABI to resolve.
    #[wasm_bindgen(generic_per_mono, js_name = echo)]
    fn echo_wire<T>(x: T::Wire) -> T::Wire
    where
        T: Wire;

    // Higher-ranked `where` predicate.
    #[wasm_bindgen(generic_per_mono, js_name = echo)]
    fn echo_hrtb<T>(x: T) -> T
    where
        for<'a> &'a T: Clone,
        T: Clone;

    // --- Real library traits ---

    // Inline bound on a real trait whose path carries an associated-type
    // binding. `T` is named nowhere else in the signature, so callers turbofish
    // and the binding is what makes the argument's type concrete.
    #[wasm_bindgen(generic_per_mono, js_name = record)]
    fn record_iter_item<T: Iterator<Item = u32>>(x: T::Item);

    // The same trait as a `where` predicate, with the projection left open in
    // both argument and return position: the bound has to reach the shim for
    // `T::Item`'s ABI to resolve there.
    #[wasm_bindgen(generic_per_mono, js_name = echo)]
    fn echo_iter_item<T>(x: T::Item) -> T::Item
    where
        T: Iterator;

    // Only `ExactSizeIterator` is named, so `T::Item` resolves through its
    // `Iterator` supertrait rather than the bound's own trait.
    #[wasm_bindgen(generic_per_mono, js_name = echo)]
    fn echo_exact_size_item<T>(x: T::Item) -> T::Item
    where
        T: ExactSizeIterator;

    // A predicate whose bounded type is a projection rather than a bare
    // parameter, next to the bound that makes the projection nameable.
    #[wasm_bindgen(generic_per_mono, js_name = echo)]
    fn echo_cloneable_item<T>(x: T::Item) -> T::Item
    where
        T: Iterator,
        T::Item: Clone;

    // Here the bounded parameter is itself the value crossing the ABI, so the
    // user's bounds and the synthesized `IntoWasmAbi`/`WasmDescribe` ones land
    // on the same parameter. `AsRef<[u32]>` also puts a plain type argument
    // (rather than an associated-type binding) in a bound path.
    #[wasm_bindgen(generic_per_mono, js_name = sumAll)]
    fn sum_all<T: IntoIterator<Item = u32> + AsRef<[u32]>>(xs: T) -> f64;

    // Higher-ranked predicate over a real trait: it is the *reference* that must
    // be iterable, which `T: IntoIterator` alone would not give.
    #[wasm_bindgen(generic_per_mono, js_name = sumAll)]
    fn sum_all_by_ref<T>(xs: T) -> f64
    where
        for<'a> &'a T: IntoIterator;
}

/// A caller that must itself satisfy the import's bounds to name `T::Wire`.
fn wire_round_trip<T>(wire: T::Wire) -> T
where
    T: Wire,
    T::Wire: IntoWasmAbi + FromWasmAbi + WasmDescribe,
{
    T::lift(echo_wire::<T>(wire))
}

#[wasm_bindgen_test]
fn generic_import_inline_bound() {
    let _ = take_log();
    record_inline(1u32);
    record_inline(2.5f64);
    record_inline(true);
    assert_eq!(take_log(), "1,2.5,true");
}

#[wasm_bindgen_test]
fn generic_import_where_bound() {
    let _ = take_log();
    record_where(7u32);
    record_where(String::from("s"));
    assert_eq!(take_log(), "7,s");
}

#[wasm_bindgen_test]
fn generic_import_inline_and_where_bounds() {
    assert_eq!(sum_bounded(2u32, 3u32), 5.0);
    assert_eq!(sum_bounded(2.5f64, 4u8), 6.5);
}

#[wasm_bindgen_test]
fn generic_import_impl_trait_bound() {
    let _ = take_log();
    record_impl_trait(21u32);
    record_impl_trait(22.5f64);
    assert_eq!(take_log(), "21,22.5");
}

#[wasm_bindgen_test]
fn generic_import_impl_trait_mixed_with_named_bound() {
    assert_eq!(sum_impl_trait_mixed(2u32, 3u32), 5.0);
    assert_eq!(sum_impl_trait_mixed(2.5f64, 4u8), 6.5);
}

#[wasm_bindgen_test]
fn generic_import_associated_type_through_bound() {
    assert_eq!(wire_round_trip::<u32>(9u32), 9u32);
    assert_eq!(
        wire_round_trip::<String>(String::from("assoc")),
        String::from("assoc")
    );
}

#[wasm_bindgen_test]
fn generic_import_higher_ranked_bound() {
    assert_eq!(echo_hrtb(4u32), 4u32);
    assert_eq!(echo_hrtb(String::from("h")), String::from("h"));
}

#[wasm_bindgen_test]
fn generic_import_iterator_bound() {
    let _ = take_log();
    // `Range<u32>: Iterator<Item = u32>`, so the argument is a `u32`.
    record_iter_item::<core::ops::Range<u32>>(11);
    record_iter_item::<core::ops::Range<u32>>(12);
    assert_eq!(take_log(), "11,12");
}

#[wasm_bindgen_test]
fn generic_import_iterator_item_round_trip() {
    assert_eq!(echo_iter_item::<core::ops::Range<u32>>(13u32), 13u32);
    assert_eq!(
        echo_iter_item::<std::vec::IntoIter<String>>(String::from("item")),
        String::from("item")
    );
}

#[wasm_bindgen_test]
fn generic_import_supertrait_projection() {
    // `Range<u32>` is not `ExactSizeIterator` on a 32-bit target, so pick an
    // iterator that is one everywhere.
    assert_eq!(
        echo_exact_size_item::<std::vec::IntoIter<u32>>(14u32),
        14u32
    );
    assert_eq!(
        echo_exact_size_item::<std::vec::IntoIter<String>>(String::from("exact")),
        String::from("exact")
    );
}

#[wasm_bindgen_test]
fn generic_import_bound_on_projection() {
    assert_eq!(echo_cloneable_item::<core::ops::Range<u32>>(15u32), 15u32);
    assert_eq!(
        echo_cloneable_item::<std::vec::IntoIter<String>>(String::from("clone")),
        String::from("clone")
    );
}

#[wasm_bindgen_test]
fn generic_import_into_iterator_bound() {
    // `T` is inferred here, since the bounded parameter is the argument itself.
    assert_eq!(sum_all(vec![1u32, 2, 3]), 6.0);
    assert_eq!(sum_all(Vec::<u32>::new()), 0.0);
}

#[wasm_bindgen_test]
fn generic_import_higher_ranked_real_trait_bound() {
    assert_eq!(sum_all_by_ref(vec![4u32, 5]), 9.0);
}
