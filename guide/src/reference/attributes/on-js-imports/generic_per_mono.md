# `generic_per_mono`

> **⚠️ Experimental.** This attribute is experimental and its behaviour may
> change, or it may be removed, in any release. The set of supported signature
> shapes in particular is expected to grow, and the internal names of the
> generated JS bindings (`__wbindgen_generic_N`) are not stable. The
> type-erasure path described in
> [Working with wasm-bindgen Generics](../../working-with-generics.md) is the
> supported way to write a generic import.

By default a generic imported function has its type parameters **erased**: every
`T` is passed across the ABI as a `JsValue`, and the single JS binding that is
generated works for all instantiations. That is described in
[Working with wasm-bindgen Generics](../../working-with-generics.md), and it is
why `T` normally has to be a JS type (`JsGeneric`) rather than a Rust one.

`generic_per_mono` opts a single import out of erasure. Instead of one erased
binding, `wasm-bindgen` generates **one binding per monomorphisation**, each with
its own descriptor, so arguments and return values are marshalled at their
concrete types:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, generic_per_mono)]
    fn log<T>(value: T);
}

log(42u32);        // crosses as a number
log("hello");      // crosses as a string
log(true);         // crosses as a boolean
```

Each of those three calls gets its own JS shim. Because nothing is boxed into a
`JsValue`, `T` can be an ordinary Rust type — `u32`, `f64`, `bool`, `String` —
which the erasure path does not allow.

## When to use it

Reach for `generic_per_mono` when you want one Rust signature to serve several
*Rust* types and you care about how they marshal. Reach for the default erasure
path when you are modelling JS generics (`Array<T>`, `Promise<T>`) and want a
single binding for all of them.

The trade-off is code size: one JS shim and one descriptor per instantiation. A
generic import instantiated at a dozen types produces a dozen shims, so prefer
erasure when the concrete marshalling does not matter.

## Trait bounds

Bounds you declare are part of the import's contract. They are carried through to
the generated wrapper, so callers must satisfy them, and they also reach the
generated shim — which means a shim signature may project an associated type off
a bounded parameter. Inline bounds, `where` predicates, and higher-ranked
predicates all work:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(generic_per_mono)]
    fn sum_items<T>(items: T) -> f64
    where
        T: IntoIterator<Item = u32>;

    #[wasm_bindgen(generic_per_mono)]
    fn double<T>(value: T) -> T
    where
        for<'a> &'a T: core::ops::Add<&'a T, Output = T>;
}
```

Note that a bound only constrains which types the import can be *called* with; it
cannot make a type marshallable. Combining a higher-ranked bound with a `&T`
argument is a common way to write a declaration that compiles but can never be
called: `&T` additionally requires `for<'a> &'a T: IntoWasmAbi`, which only
holds for a fixed set of types (the primitive scalars, `JsValue`, imported JS
handle types, `&str` and `&[T]`), so a bound such as
`for<'a> &'a T: IntoIterator<Item = &'a u32>` leaves no type that satisfies both.
See [Note on `&T` arguments](#note-on-t-arguments).

Relaxed bounds are the exception: `T: ?Sized` is not supported on any
`wasm-bindgen` generic — erased or per-monomorphisation — and is reported as
`unsupported in wasm-bindgen generics`.

## Lifetime parameters

Lifetime parameters on the function are supported, including lifetime bounds
(`T: 'a`) and lifetime-outlives predicates (`'a: 'b`):

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(generic_per_mono)]
    fn log_ref<'a, T>(value: &'a T);
}
```

A lifetime on a method's receiver works too, and ties the borrow of the
receiver to the rest of the call:

```rust
#[wasm_bindgen]
extern "C" {
    type Widget;

    #[wasm_bindgen(method, generic_per_mono)]
    fn set<'a, T>(this: &'a Widget, value: &'a T);
}
```

Lifetimes carry no runtime information — they are erased before values cross
the wasm ABI — so this imposes no restriction beyond what plain Rust already
requires of the signature.

## Class-level generics

A method (or constructor, or self-returning static method) on an imported type
that is itself generic — `this: &Holder<T>`, the shape `js-sys` types like
`Array<T>` and `Iterator<T>` use — is also supported. The type parameter is
hoisted out of the wrapper function's own generic parameter list and onto the
enclosing `impl` block instead, so a declaration like:

```rust
#[wasm_bindgen]
extern "C" {
    type Holder<T>;

    #[wasm_bindgen(method, generic_per_mono)]
    fn get<T>(this: &Holder<T>) -> T;
}
```

expands to `impl<T> Holder<T> { fn get(&self) -> T { .. } }` rather than a
generic method on a non-generic `impl Holder`.

This also applies to a constructor, or a static method whose return type is
the class (e.g. `fn of<T>(value: T) -> Container<T>`), since those are impl'd
on the *return* type's class the same way a method is impl'd on its
receiver's class.

A lifetime belonging to the class itself (`type Holder<'a>`, used as
`this: &Holder<'a>`) is supported the same way.

The receiver argument itself needs no additional trait bound for this: an
imported type implements `IntoWasmAbi`/`WasmDescribe` unconditionally over its
type parameter(s), regardless of what `T` is.

## Closure arguments

A raw `&dyn Fn(...)`/`&mut dyn FnMut(...)` trait-object argument is supported,
including when its own call signature — not just the rest of the import's
signature — mentions a type parameter. This is the shape `js-sys` uses for
`Array::for_each`/`Array::every`:

```rust
#[wasm_bindgen]
extern "C" {
    type Array<T>;

    #[wasm_bindgen(method, js_name = forEach, generic_per_mono)]
    fn for_each<T>(this: &Array<T>, callback: &mut dyn FnMut(T, u32, Array<T>));

    #[wasm_bindgen(method, generic_per_mono)]
    fn every<T>(this: &Array<T>, predicate: &mut dyn FnMut(T, u32, Array<T>) -> bool) -> bool;
}
```

The wrapper parameter becomes `&(impl Fn(..) + MaybeUnwindSafe)` /
`&mut (impl FnMut(..) + MaybeUnwindSafe)`, exactly as it does for a *concrete*
closure argument on the non-generic import path — an ordinary Rust closure is
all a caller needs to write:

```rust
arr.for_each(&mut |value, index, arr| {
    // `value: T`, `index: u32`, `arr: Array<T>` — all monomorphised at
    // whatever `T` this particular `Array<T>` was.
});
```

Each monomorphisation gets its own copy of the closure-invoke machinery,
describing the closure's *concrete* argument and return types, the same way it
already does for every other generic argument on this path. The closure's own
argument types and return type may independently mention a type parameter —
including one hoisted from the class, as `T`/`Array<T>` are above — and
multiple closure arguments in one signature are each handled on their own.

The one restriction is that the closure has to be at the top level of the
argument: nested inside another type, e.g. `Option<&mut dyn FnMut(T)>` or
`Box<dyn FnMut(T)>`, it is not supported — see
[Unsupported shapes](#unsupported-shapes).

## Other attributes

`generic_per_mono` composes with the usual import attributes — `method`,
`static_method_of`, `constructor`, `getter`, `setter`, `structural`, `final`,
`indexing_getter`, `indexing_setter`, `indexing_deleter`, `js_namespace`,
`js_name`, `catch`, `variadic`, and `slice_to_array` — and the resulting JS
binding is shaped exactly as it would be for the equivalent non-generic import.

The two that do *not* compose are `assert_no_shim` and `reexport`; both are
rejected, see [Unsupported shapes](#unsupported-shapes).

`async` is supported, and returns a future in the usual way:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(generic_per_mono)]
    async fn round_trip<T>(value: T) -> T;
}
```

## Applying it to a whole block

`generic_per_mono` can also go on the `extern "C"` block, which every function in
the block then inherits:

```rust
#[wasm_bindgen(generic_per_mono)]
extern "C" {
    fn log_one<T>(x: T);
    fn log_two<T>(a: u32, b: T) -> T;
}
```

The block flag only applies where it can: the per-monomorphisation path needs at
least one type parameter, so a non-generic import in the block is left alone and
binds through the ordinary single shim. A block can therefore mix the two freely:

```rust
#[wasm_bindgen(generic_per_mono)]
extern "C" {
    fn log_generic<T>(x: T); // per-monomorphisation
    fn log_u32(x: u32);      // ordinary import, unaffected
}
```

Writing `generic_per_mono` directly on a non-generic function is still an error,
since there you asked for something that cannot be done.

## Unsupported shapes

These are rejected at compile time with a diagnostic pointing at the offending
declaration. Each generally keeps working on the type-erasure path, so the fix is
usually to drop `generic_per_mono`:

* **A mutable reference to a type parameter** (`&mut T`, or `&mut Vec<T>`, or
  any other `&mut` whose referent mentions a type parameter), and a reference to
  a type parameter **nested inside another type** (e.g. `Option<&T>`). A bare
  `&T` *is* supported, and mutable references to *concrete* types (e.g.
  `&mut [u16]`, `&mut dyn FnMut(u32)`) bind exactly as they do on the
  non-generic import path — the restriction is only about references to type
  parameters.
* **Returning a reference.**
* **A bare type parameter, or a reference to one (`&T`), as the `variadic`
  argument**, since it may monomorphise to a scalar, which is not spreadable.
* **A type parameter in the error position of a `catch` import**
  (`Result<T, E>` with generic `E`): only the `Ok` type is monomorphised, and the
  error type is always `JsValue`.
* **`slice_to_array` on a slice whose element type mentions a type parameter**
  (`&[T]`, `&[Vec<T>]`, `Option<&[T]>`). `VectorRefIntoWasmAbi` is implemented
  per concrete ABI shape, so no bound the caller can write makes an arbitrary `T`
  satisfy it; the element type must be concrete. See
  [`slice_to_array`](./slice_to_array.md).
* **`reexport`**, which has no well-defined target when one binding is
  manufactured per monomorphisation.
* **`assert_no_shim`**, which asserts that no shim function is generated for the
  import. Per-monomorphisation codegen can never satisfy that, because it
  manufactures one shim per instantiation by construction, so the combination is
  rejected rather than silently ignored.
* **An argument whose pattern is not a plain name or `_`** (for example a tuple
  pattern such as `fn f<T>((a, b): (u32, u32), x: T)`), reported as
  `unsupported pattern in generic_per_mono imported function`. The generated
  per-monomorphisation shim has to forward each argument by name, so it needs a
  binding it can name. Give the argument a single identifier instead.
* **A closure trait object mentioning a type parameter, nested inside another
  type** (`Option<&mut dyn FnMut(T)>`, `Box<dyn FnMut(T)>`, a tuple element,
  ...). See [Closure arguments](#closure-arguments): only a bare `&dyn
  Fn(...)`/`&mut dyn FnMut(...)` at the top of the argument is supported. Pull
  the closure out into its own argument, or give it a concrete (non-generic)
  signature.

**Const generic parameters** are also rejected, but not by `generic_per_mono`:
`wasm-bindgen` does not support them on *any* generic import, erased or not, and
reports `unsupported in wasm-bindgen generics`. Dropping `generic_per_mono` will
not help.

### Colliding imports

Everything above is reported by the macro, at compile time. One failure is
reported later, by the `wasm-bindgen` CLI, because it cannot be detected until
the whole module is linked: two `generic_per_mono` imports that agree on every
input to the shim key and differ *only* in an attribute that the key does not
hash.

A monomorphisation records only a shim key, which is a hash over the Rust
function name, the `js_name` (when one is given), the `js_namespace`, the
signature tokens, the `module`, and any `cfg` attributes. It does *not* cover
`catch`, `variadic`, `slice_to_array`, `structural`/`final`, or the
getter/setter accessor kind. Two imports differing only in one of those claim
the same key, and the CLI cannot tell which binding a given instantiation meant.
Rather than silently binding one of them, which would mis-bind every
monomorphisation of the other, it fails with an error naming both JS targets.

To fix it, make the two distinguishable on the Rust side: rename one of the Rust
functions, adding `js_name` to keep the JS-visible name unchanged, or give them
different signatures.

## Note on `&T` arguments

A bare `&T` argument is supported, and requires the referent to satisfy the bound
`wasm-bindgen` needs to marshal it — either one of the built-in scalar types
(`i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `i128`, `u128`, `isize`,
`usize`, `f32`, `f64`, `bool`, `char`) or a JS handle type. Passing `&SomeStruct`
for a plain Rust struct is rejected, since there is no ABI representation for it;
take it by value, or pass a JS type.
