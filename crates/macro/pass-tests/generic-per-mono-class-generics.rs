// Compile-pass coverage for class-level generics under the experimental
// per-monomorphisation generic import path (`#[wasm_bindgen(generic_per_mono)]`).
//
// A class-level generic is a type or lifetime parameter of the function that
// also parameterises the receiver/return *class* type itself (e.g.
// `this: &Holder<T>`, or `-> Holder<T>` for a constructor / self-returning
// static method). This is the shape used throughout `js-sys` (`Array<T>`,
// `Map<K, V>`, `Promise<T>`, ...), and is supported by *hoisting* the
// function's own generic parameters that the class type's argument list uses
// onto the enclosing `impl` block's own generic header, reusing the same
// `get_fn_generics` hoisting analysis the type-erasure generic path already
// relies on for the same shape. See `try_to_tokens_generic` /
// `get_fn_generics` in `codegen.rs`.
//
// This only needs to *compile*; `crates/cli/tests/reference/generic-import.rs`
// pins the actual generated JS/Wasm output end-to-end.
//
// The `use_*` helpers below exist purely to *instantiate* each import, which is
// what forces per-monomorphisation codegen to run at all. Nothing calls them,
// and the imported types cannot be constructed here to call them with, so
// dead-code warnings are expected and suppressed.
#![allow(dead_code)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type Holder<T>;

    // Constructor returning the parameterised class: the generated `impl`
    // becomes `impl<T> Holder<T>` rather than a bare `impl Holder`, since `T`
    // is hoisted off the constructor's own parameter list.
    #[wasm_bindgen(constructor, generic_per_mono)]
    fn new<T>(value: T) -> Holder<T>;

    // Instance method: `T` is hoisted from the receiver `&Holder<T>` and
    // reused for the return type (mirrors `Array::at`/`Array::get`).
    #[wasm_bindgen(method, generic_per_mono)]
    fn get<T>(this: &Holder<T>) -> T;

    // Self-returning static method (mirrors `Array::of`): `class_return_path`
    // retargets the `impl` block to the return type's class the same way it
    // does for the constructor above.
    #[wasm_bindgen(static_method_of = Holder, generic_per_mono, js_name = of)]
    fn holder_of<T>(value: T) -> Holder<T>;

    // A hoisted class-level parameter (`T`) mixed with an additional,
    // non-hoisted function-only parameter (`U`): proves the split between
    // `class_generic_params` and `fn_generic_params` in `get_fn_generics`
    // works, and that both still end up correctly marshalled.
    #[wasm_bindgen(method, generic_per_mono)]
    fn combine<T, U>(this: &Holder<T>, other: U);
}

#[wasm_bindgen]
extern "C" {
    // A class type whose argument list is (partly or wholly) *concrete* rather
    // than a hoisted parameter of the function. `class_generic_exprs` carries
    // every type argument verbatim, so `class_impl_def` re-emits the list as
    // written; without that the arguments would be dropped and the wrapper
    // would land on the class's own parameter defaults (`impl Concrete`), a
    // receiver-type mismatch on the generated method.
    type Concrete<A, B>;

    // Mixed: `u32` is concrete, `T` is hoisted, so the header is `impl<T>
    // Concrete<u32, T>`.
    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn mixed_concrete_class_arg<T>(this: &Concrete<u32, T>) -> T;

    // Fully concrete: nothing is hoisted, so the header has no parameters of
    // its own and is simply `impl Concrete<u32, String>`. The function's own
    // `T` stays on the method, which is what makes this a per-mono import at
    // all.
    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn concrete_class_arg<T>(this: &Concrete<u32, String>, v: T);

    // The same, reached through the constructor route (`class_return_path`)
    // rather than a method receiver: a concrete argument list is vacuously
    // "constraining", so the return type is used as the class and its
    // arguments are re-emitted the same way.
    #[wasm_bindgen(constructor, generic_per_mono)]
    fn new_concrete<T>(v: T) -> Concrete<u32, String>;
}

#[wasm_bindgen]
extern "C" {
    // A class-level *lifetime* parameter, rather than a type parameter.
    type LifetimeHolder<'a>;

    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn get_lifetime<'a, T>(this: &'a LifetimeHolder<'a>) -> T;
}

// An explicit static class path carries the arguments needed to form the
// inherent impl even when the imported type belongs to a different extern block.
#[wasm_bindgen]
extern "C" {
    type ExternalLifetimeHolder<'a>;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        static_method_of = ExternalLifetimeHolder<'a>,
        generic_per_mono,
        js_name = create
    )]
    fn external_lifetime_create<'a, T>(value: T);
}

#[wasm_bindgen]
extern "C" {
    // A class lifetime not literally named `'a`. The reference-conversion
    // impls re-emit the type's full argument list, so they have to *declare*
    // the type's own lifetime params rather than assuming a single `'a`;
    // otherwise this is an undeclared lifetime (E0261) against generated code.
    type Tagged<'x>;

    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn tagged_get<'x, T>(this: &'x Tagged<'x>) -> T;
}

#[wasm_bindgen]
extern "C" {
    // More than one class lifetime argument. This is where the impl header's
    // deduplicated `class_lifetime_params` and the positional
    // `class_lifetime_args` passed to the self type genuinely diverge — the
    // declaration order below is deliberately reversed relative to the
    // receiver's argument list, and `TwoLifetimes<'b, 'a>` must not come back
    // out as `TwoLifetimes<'a, 'b>`.
    type TwoLifetimes<'a, 'b>;

    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn two_lifetimes_get<'a, 'b, T>(this: &'a TwoLifetimes<'b, 'a>) -> T;

    // The same lifetime used for both arguments: the header binds it once
    // while the self type still takes two arguments.
    #[wasm_bindgen(method, generic_per_mono, js_name = both)]
    fn two_lifetimes_same<'a, T>(this: &'a TwoLifetimes<'a, 'a>) -> T;
}

#[wasm_bindgen]
extern "C" {
    // A class type parameterised by *both* a lifetime and a type parameter of
    // the function, with both hoisted onto the same `impl` header. This used
    // to be rejected: the `&'a #rust_name` reference-conversion impls
    // (`IntoWasmAbi`, `OptionIntoWasmAbi`) reused the same `'a` for both the
    // reference lifetime and the class's own lifetime, so the two were forced
    // to unify — and once a hoisted type parameter also had to resolve
    // through that impl, the unification forced the class's `'a` to outlive
    // `'static` (E0521 against generated code). Fixed by declaring the type's
    // own lifetimes separately from a fresh, never-colliding reference
    // lifetime in the generated conversion impls.
    type LtHolder<'a, T>;

    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn class_lifetime_and_type_param<'a, T>(this: &'a LtHolder<'a, T>) -> T;

    // Same shape reached through the constructor route (`class_return_path`)
    // rather than a method receiver.
    #[wasm_bindgen(constructor, generic_per_mono)]
    fn new_class_lifetime_and_type_param<'a, T>(v: &'a T) -> LtHolder<'a, T>;

    // A concrete `'static` class argument does not need a lifetime declaration
    // on the generated `impl` header. It must survive in the self type for all
    // three class-wrapper routes.
    #[wasm_bindgen(method, generic_per_mono, js_name = get_static)]
    fn static_lifetime_get<T>(this: &LtHolder<'static, T>) -> T;

    #[wasm_bindgen(constructor, generic_per_mono)]
    fn new_static_lifetime<T>(v: T) -> LtHolder<'static, T>;

    #[wasm_bindgen(static_method_of = LtHolder, generic_per_mono, js_name = of_static)]
    fn static_lifetime_of<T>(v: T) -> LtHolder<'static, T>;
}

#[wasm_bindgen]
extern "C" {
    type Pair2<A, B>;

    // A lifetime nested *inside* a class type argument (rather than a direct
    // class-lifetime argument) lands in the bound-only-lifetime bucket
    // instead, but is declared on the same `impl` header alongside the
    // hoisted type parameter and previously hit the same hazard.
    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn nested_class_lifetime_and_type_param<'a, T>(this: &Pair2<T, &'a u32>) -> u32;
}

#[wasm_bindgen]
extern "C" {
    // Two class-level generic parameters (mirrors `Map<K, V>`).
    type Pair<K, V>;

    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn pair_get<K, V>(this: &Pair<K, V>) -> V;

    // Declaration order deliberately non-alphabetical and reversed relative to
    // the class argument list. `class_generic_params` is a `BTreeSet`, so it is
    // ordered alphabetically, while the self type's arguments come from the
    // declaration-ordered `class_generic_exprs`; this pins that the two cannot
    // desync into `impl<K, V> Pair<V, K>`-style mismatches.
    #[wasm_bindgen(method, generic_per_mono, js_name = swap)]
    fn pair_swap<V, K>(this: &Pair<V, K>) -> K;

    // The same parameter used twice: `class_generic_params` dedups to one entry
    // while `class_generic_exprs` stays two elements long.
    #[wasm_bindgen(method, generic_per_mono, js_name = both)]
    fn pair_both<T>(this: &Pair<T, T>) -> T;
}

#[wasm_bindgen]
extern "C" {
    type Boxed<T>;

    // A hoisted class-level parameter that appears in no argument or return
    // position, so the self type is the only thing constraining it.
    #[wasm_bindgen(method, generic_per_mono, js_name = tag)]
    fn boxed_tag<T>(this: &Boxed<T>) -> u32;

    // A *composed* class argument rather than a bare parameter: `T` is still
    // determined by the self type, so it can be hoisted.
    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn boxed_nested_get<T>(this: &Boxed<Option<T>>) -> u32;

    // An explicit `where`-clause bound on a hoisted parameter. This is the
    // exact shape that requires the bound to be re-emitted as a predicate on
    // the generated `impl` header: left on the wrapper method's own `where`
    // clause it would not constrain the impl's parameter (RFC 447), and any
    // parameter hoisted transitively out of such a bound would be an
    // unconstrained impl parameter (E0207).
    #[wasm_bindgen(method, generic_per_mono, js_name = dup)]
    fn boxed_where_bound<T>(this: &Boxed<T>) -> u32
    where
        T: Clone;

    // The shape that makes the hoisted bound *load-bearing on the shim*: the
    // shim's ABI signature projects an associated type off the hoisted
    // parameter (`<T::Item as IntoWasmAbi>::Abi`), which only resolves with
    // `T: IntoIterator` in scope. The shim is a nested item and inherits
    // nothing from the `impl` header the bound was hoisted onto, so it has to
    // be restated on the shim's own `where` clause; without that this is
    // `E0220` against generated code. `boxed_where_bound` above does not catch
    // it, because its hoisted parameter never reaches the shim's signature.
    #[wasm_bindgen(method, generic_per_mono, js_name = first)]
    fn boxed_where_projection<T>(this: &Boxed<T>, v: T::Item) -> u32
    where
        T: IntoIterator;

    // The same bound written inline. This already worked (the shim's own
    // parameter list re-emits inline bounds via `type_params_with_bounds`), and
    // pins that the two spellings stay equivalent.
    #[wasm_bindgen(method, generic_per_mono, js_name = first)]
    fn boxed_inline_projection<T: IntoIterator>(this: &Boxed<T>, v: T::Item) -> u32;
}

#[wasm_bindgen]
extern "C" {
    type Defaulted<T>;

    // A constructor whose return type's argument does not *determine* the
    // parameter (`T::Item`). `class_return_path` declines to hoist here, so the
    // arguments are stripped and the method hangs off the class's own defaults
    // (`impl Defaulted`). That is the established behaviour shared with the
    // type-erasure path, and is relied on by real imports such as
    // `js_sys::Promise::new_typed<T: Promising>(..) -> Promise<<T as
    // Promising>::Resolution>` — so it must keep compiling rather than being
    // rejected as an unhoistable class argument list.
    #[wasm_bindgen(constructor, generic_per_mono)]
    fn new_defaulted<T: IntoIterator>(v: u32) -> Defaulted<T::Item>;

    // A non-constraining constructor return is deliberately left on the
    // imported type's defaults. Its declaration bound must use those defaults
    // too, rather than the discarded `T::Item` return argument, which would
    // otherwise leave `T` undeclared on the generated inherent impl.
    type DefaultedBounded<T: Clone>;

    #[wasm_bindgen(constructor, generic_per_mono)]
    fn new_defaulted_bounded<T: IntoIterator>(v: u32) -> DefaultedBounded<T::Item>
    where
        T::Item: Clone;
}

#[wasm_bindgen]
extern "C" {
    type Fallible<T>;

    // `catch` + constructor + class generics: the `Result` is unwrapped at
    // parse time, so `class_return_path` sees `Fallible<T>` through a different
    // route than the plain constructor above.
    #[wasm_bindgen(constructor, generic_per_mono, catch)]
    fn new_fallible<T>(value: T) -> Result<Fallible<T>, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    type Bounded;

    // An inline lifetime bound (`'a: 'b`) relating two of the function's own
    // lifetimes. The generated shim redeclares its lifetimes with bounds
    // intact, so the wrapper has to carry the bound too or its declaration is
    // strictly weaker than the shim it calls, and the call fails with
    // "lifetime may not live long enough" against generated code. Inline
    // bounds have no parameter-list slot once a parameter may be hoisted onto
    // the `impl` header, so they are reified into `where` predicates by
    // `generics::generic_bounds`.
    //
    // Note the bound only becomes load-bearing when the lifetimes reach the
    // shim through a *generic* projection (`&'a T`): with a concrete `&'a
    // JsValue` the implied bounds of the argument position are enough to prove
    // the predicate on their own, and dropping `'a: 'b` goes unnoticed.
    #[wasm_bindgen(method, generic_per_mono, js_name = take)]
    fn take_bounded<'a: 'b, 'b, T: AsRef<JsValue>>(this: &Bounded, a: &'a T, b: &'b T);
}

trait HasAssociatedType {
    type Output;
}

trait TakesType<T> {}

trait Relates<V> {
    type Output;
}

trait Output {
    type Item;
}

trait HasItem {
    type Item;
}

trait HasReferenceOutput {
    type Output;
}

#[wasm_bindgen]
extern "C" {
    // Bounds from the imported type declaration must be reinstated on a
    // generated inherent impl, even when the method does not repeat them.
    type BoundedHolder<T: Clone>;

    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn bounded_holder_get<T>(this: &BoundedHolder<T>) -> T;

    // The same requirement applies to declaration-level lifetime bounds.
    type LifetimeBoundedHolder<'a: 'b, 'b, T>;

    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn lifetime_bounded_holder_get<'a, 'b, T>(
        this: &LifetimeBoundedHolder<'a, 'b, T>,
    ) -> T;

    type GenericBoundHolder<F>;

    // `U` in an ordinary trait argument is not constrained by the class's
    // self type and must remain a function parameter, while an associated-type
    // equality does constrain `U` and belongs on the impl.
    #[wasm_bindgen(method, generic_per_mono, js_name = assoc)]
    fn associated_type_bound<F, U>(this: &GenericBoundHolder<F>, value: U)
    where
        F: HasAssociatedType<Output = U>;

    #[wasm_bindgen(method, generic_per_mono, js_name = ordinary)]
    fn ordinary_trait_argument<F, U>(this: &GenericBoundHolder<F>, value: U)
    where
        F: TakesType<U>;

    // An equality RHS is not safe to hoist when its predicate also has an
    // ordinary function-level trait argument.
    #[wasm_bindgen(method, generic_per_mono, js_name = mixed)]
    fn mixed_associated_type_bound<F, U, V>(this: &GenericBoundHolder<F>, value: V)
    where
        F: Relates<V, Output = U>;

    // A projection in an equality RHS does not determine its base parameter.
    #[wasm_bindgen(method, generic_per_mono, js_name = projected)]
    fn projected_associated_type_bound<F, U>(this: &GenericBoundHolder<F>)
    where
        F: Output<Item = U::Item>,
        U: HasItem;

    // An equality RHS that depends on a function lifetime stays on the method:
    // moving it to the impl loses the argument's implied `U: 'a` bound.
    #[wasm_bindgen(method, generic_per_mono, js_name = reference)]
    fn reference_associated_type_bound<'a, F, U>(
        this: &GenericBoundHolder<F>,
        value: &'a U,
    ) where
        F: HasReferenceOutput<Output = &'a U>;

    // Omitted imported-type defaults are substituted through earlier arguments
    // before their declaration bounds are emitted on the generated impl.
    type DefaultedBoundedHolder<T: Clone, U: Clone = Vec<T>>;

    #[wasm_bindgen(method, generic_per_mono, js_name = defaulted)]
    fn defaulted_bounded_holder<X>(this: &DefaultedBoundedHolder<u32>, value: X);

    // Substitution is simultaneous: swapping class arguments must not recurse
    // indefinitely while propagating the declaration's `K: Clone` bound.
    type SwappedBoundedPair<K: Clone, V>;

    #[wasm_bindgen(method, generic_per_mono, js_name = swap)]
    fn swapped_bounded_pair<V, K>(this: &SwappedBoundedPair<V, K>, value: u32);

    // The lookup for declaration bounds uses the Rust receiver type, not its
    // independent JS class name.
    type RenamedBoundedHolder<T: Clone>;

    #[wasm_bindgen(method, js_class = RenamedHolder, generic_per_mono, js_name = get)]
    fn renamed_bounded_holder_get<T>(this: &RenamedBoundedHolder<T>) -> T;

    // Class-return matching uses the Rust type identity, not the JS class name.
    type RenamedConstructed<T>;

    #[wasm_bindgen(constructor, js_class = RenamedJs, generic_per_mono)]
    fn renamed_new<T>(value: T) -> RenamedConstructed<T>;

    #[wasm_bindgen(
        static_method_of = RenamedConstructed,
        js_class = RenamedJs,
        generic_per_mono,
        js_name = of
    )]
    fn renamed_of<T>(value: T) -> RenamedConstructed<T>;

    // A qualified path to this module's imported type retains its declaration
    // bounds without matching unrelated `module::Type` names.
    type CrateBoundedHolder<T: Clone>;

    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn crate_bounded_holder_get<T>(this: &self::CrateBoundedHolder<T>) -> T;

}

fn use_holder(holder_u32: &Holder<u32>, holder_string: &Holder<String>) {
    // Two distinct instantiations of `get` prove two distinct manufactured
    // shims are generated and both marshal correctly.
    let _: u32 = holder_u32.get();
    let _: String = holder_string.get();

    // Two distinct instantiations of the constructor.
    let _: Holder<u32> = Holder::new(1u32);
    let _: Holder<String> = Holder::new(String::from("hi"));

    // Two distinct instantiations of the self-returning static method.
    let _: Holder<u32> = Holder::holder_of(2u32);
    let _: Holder<String> = Holder::holder_of(String::from("bye"));

    // Mixed hoisted (`T`) / non-hoisted (`U`) parameters, called with
    // distinct `T`/`U` combinations.
    holder_u32.combine(3.0f64);
    holder_string.combine(4u32);
}

fn use_concrete(mixed: &Concrete<u32, String>) {
    let _: String = mixed.mixed_concrete_class_arg();
    mixed.concrete_class_arg(1u32);
    mixed.concrete_class_arg(String::from("hi"));

    let _: Concrete<u32, String> = Concrete::new_concrete(2u32);
    let _: Concrete<u32, String> = Concrete::new_concrete(String::from("bye"));
}

fn use_lifetime_holder<'a>(holder: &'a LifetimeHolder<'a>) {
    let _: u32 = holder.get_lifetime();
    let _: String = holder.get_lifetime();
}

fn use_tagged<'x>(tagged: &'x Tagged<'x>) {
    let _: u32 = tagged.tagged_get();
    let _: String = tagged.tagged_get();
}

fn use_two_lifetimes<'a, 'b>(two: &'a TwoLifetimes<'b, 'a>, same: &'a TwoLifetimes<'a, 'a>) {
    let _: u32 = two.two_lifetimes_get();
    let _: String = same.two_lifetimes_same();
}

fn use_lt_holder<'a>(
    holder_u32: &'a LtHolder<'a, u32>,
    holder_string: &'a LtHolder<'a, String>,
    static_holder: &LtHolder<'static, u32>,
    value: &'a JsValue,
) {
    let _: u32 = holder_u32.class_lifetime_and_type_param();
    let _: String = holder_string.class_lifetime_and_type_param();

    let _: LtHolder<'a, JsValue> = LtHolder::new_class_lifetime_and_type_param(value);
    let _: u32 = static_holder.static_lifetime_get();
    let _: LtHolder<'static, u32> = LtHolder::new_static_lifetime(1u32);
    let _: LtHolder<'static, String> = LtHolder::static_lifetime_of(String::from("static"));
}

fn use_pair2<'a>(pair: &Pair2<u32, &'a u32>) {
    let _: u32 = pair.nested_class_lifetime_and_type_param();
}

fn use_pair(pair: &Pair<u32, String>, flipped: &Pair<String, u32>, same: &Pair<u32, u32>) {
    let _: String = pair.pair_get();
    let _: u32 = flipped.pair_swap();
    let _: u32 = same.pair_both();
}

fn use_boxed(boxed: &Boxed<u32>, nested: &Boxed<Option<u32>>, projected: &Boxed<Vec<String>>) {
    let _: u32 = boxed.boxed_tag();
    let _: u32 = nested.boxed_nested_get();
    let _: u32 = boxed.boxed_where_bound();
    let _: u32 = projected.boxed_where_projection(String::from("hi"));
    let _: u32 = projected.boxed_inline_projection(String::from("hi"));
}

fn use_fallible() -> Result<(), JsValue> {
    let _: Fallible<u32> = Fallible::new_fallible(1u32)?;
    let _: Fallible<String> = Fallible::new_fallible(String::from("hi"))?;
    Ok(())
}

fn use_bounded<'a: 'b, 'b>(bounded: &Bounded, a: &'a JsValue, b: &'b JsValue) {
    bounded.take_bounded(a, b);
}

fn use_renamed_bounded(holder: &RenamedBoundedHolder<u32>) {
    let _: u32 = holder.renamed_bounded_holder_get();
    let _: RenamedConstructed<u32> = RenamedConstructed::renamed_new(1u32);
    let _: RenamedConstructed<String> = RenamedConstructed::renamed_of(String::from("renamed"));
}

trait HasReturn {
    type Return;
}

#[wasm_bindgen]
extern "C" {
    // Inline bounds on the imported type declaration itself must be normalized
    // into predicates for every generated conversion impl, not only per-mono
    // shims.
    type TypeLifetimeBounds<'a: 'b, 'b, T>;

    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn type_lifetime_bounds_get<'a: 'b, 'b, T>(
        this: &'a TypeLifetimeBounds<'a, 'b, T>,
    ) -> T;

    // The generated borrow lifetime must be fresh even when a user deliberately
    // chooses the old internal spelling.
    type RefNamed<'__wbg_ref, T>;

    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn ref_named_get<'__wbg_ref, T>(
        this: &'__wbg_ref RefNamed<'__wbg_ref, T>,
    ) -> T;

    type LifetimeBounded<'a>;

    // `'a` is hoisted with the class type. Its outlives predicate must follow
    // it to the generated impl, along with the related `'b` lifetime.
    #[wasm_bindgen(method, generic_per_mono, js_name = take)]
    fn take_class_lifetime_bound<'a: 'b, 'b, T: AsRef<JsValue>>(
        this: &'a LifetimeBounded<'a>,
        value: &'b T,
    );

    type TransitiveBounded<F>;

    // `R` is not in the class type directly; it must be hoisted because the
    // bound on class parameter `F` uses it as an associated-type binding.
    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn transitive_bound<F, R>(this: &TransitiveBounded<F>) -> R
    where
        F: HasReturn<Return = R>;
}

fn use_class_lifetime_bound<'a: 'b, 'b>(
    holder: &'a LifetimeBounded<'a>,
    value: &'b JsValue,
) {
    holder.take_class_lifetime_bound(value);
}

fn use_type_lifetime_bounds<'a: 'b, 'b>(
    holder: &'a TypeLifetimeBounds<'a, 'b, u32>,
    named: &'a RefNamed<'a, String>,
) {
    let _: u32 = holder.type_lifetime_bounds_get();
    let _: String = named.ref_named_get();
}

fn use_transitive_bound<F, R>(holder: &TransitiveBounded<F>) -> R
where
    F: HasReturn<Return = R>,
    R: wasm_bindgen::convert::FromWasmAbi,
{
    holder.transitive_bound()
}

fn main() {}
