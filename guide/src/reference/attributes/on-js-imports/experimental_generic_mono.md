# `experimental_generic_mono`

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

`experimental_generic_mono` opts a single import out of erasure. Instead of one erased
binding, `wasm-bindgen` generates **one binding per monomorphisation**, each with
its own descriptor, so arguments and return values are marshalled at their
concrete types:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, experimental_generic_mono)]
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

Reach for `experimental_generic_mono` when you want one Rust signature to serve several
*Rust* types and you care about how they marshal. Reach for the default erasure
path when you are modelling a JS generic *container* (`Array<T>`, `Promise<T>`)
and want a single binding shared by every element type.

Note that this is a choice about the *element* marshalling, not about whether the
class itself may be generic: `experimental_generic_mono` does support a generic imported
type, including as a method receiver or constructor return
(see [Class-level generics](#class-level-generics)). The question is whether you
want one shim per element type (`experimental_generic_mono`) or one shim for all of them
(erasure).

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
    #[wasm_bindgen(experimental_generic_mono)]
    fn sum_items<T>(items: T) -> f64
    where
        T: IntoIterator<Item = u32>;

    #[wasm_bindgen(experimental_generic_mono)]
    fn double<T>(value: T) -> T
    where
        for<'a> &'a T: core::ops::Add<&'a T, Output = T>;
}
```

Note that a bound only constrains which types the import can be *called* with; it
cannot make a type marshallable. Combining a higher-ranked bound with a `&T`
argument is a common way to write a declaration that compiles but can never be
called: `&T` additionally requires an `IntoWasmAbi` impl for the reference,
which exists only for `JsValue` and imported JS types, so a bound such as
`for<'a> &'a T: IntoIterator<Item = &'a u32>` leaves no type that satisfies both.
See [Note on `&T` arguments](#note-on-t-arguments).

Relaxed bounds are the exception: `T: ?Sized` is not supported on any
`wasm-bindgen` generic — erased or per-monomorphisation — and is reported as
`unsupported in wasm-bindgen generics`.

## `impl Trait` arguments

Argument-position `impl Trait` is supported. It is desugared into a synthesized
named type parameter with the same bound before any other codegen runs, so it
is monomorphised exactly like a type parameter you named yourself — the two are
interchangeable:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, experimental_generic_mono)]
    fn log(value: impl core::fmt::Debug);
}

log(42u32);
log("hello");
```

is equivalent to writing `fn log<T: core::fmt::Debug>(value: T)`. This also
means a function can have a type parameter without appearing to: `impl Trait`
counts towards the "at least one type parameter" requirement even though it
never appears in the function's own generic parameter list.

`impl Trait` can be mixed with named type parameters, nested inside another
type (`Vec<impl Trait>`), and repeated — each occurrence gets its own
synthesized parameter:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(experimental_generic_mono)]
    fn mix<T>(label: T, values: Vec<impl Clone>);
}
```

Bounds on a synthesized parameter are enforced exactly as if you had written
them out by hand: a caller that violates one gets a diagnostic pointing at the
`impl Trait` in the declaration, the same as it would for an explicit bound.

## Lifetime parameters

Lifetime parameters on the function are supported, including lifetime bounds
(`T: 'a`) and lifetime-outlives predicates (`'a: 'b`):

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(experimental_generic_mono)]
    fn log_ref<'a, T>(value: &'a T);
}
```

A lifetime on a method's receiver works too, and ties the borrow of the
receiver to the rest of the call:

```rust
#[wasm_bindgen]
extern "C" {
    type Widget;

    #[wasm_bindgen(method, experimental_generic_mono)]
    fn set<'a, T>(this: &'a Widget, value: &'a T);
}
```

Lifetimes carry no runtime information — they are erased before values cross
the wasm ABI — so this imposes no restriction beyond what plain Rust already
requires of the signature. A lifetime belonging to the **class** itself works
too; see [Class-level generics](#class-level-generics).

## Class-level generics

A *class-level* generic is a type or lifetime parameter of the function that
also parameterises the receiver/return **class** type itself, rather than only
appearing in an ordinary argument or return position. This is the shape used
throughout `js-sys` (`Array<T>`, `Map<K, V>`, `Promise<T>`, ...): the imported
*type* is declared with its own generic parameter, and a method, constructor,
or static method that returns the class ties one of its own generics to it:

```rust
#[wasm_bindgen]
extern "C" {
    type Holder<T>;

    #[wasm_bindgen(constructor, experimental_generic_mono)]
    fn new<T>(value: T) -> Holder<T>;

    #[wasm_bindgen(method, experimental_generic_mono)]
    fn get<T>(this: &Holder<T>) -> T;
}

let holder = Holder::new(42u32);
let value: u32 = holder.get();
```

The function's own type parameter that the class type's argument list uses
(`T` in `Holder<T>` above) is *hoisted* off the wrapper function's own
parameter list and onto the generated `impl` block's header instead — the
`impl` above the constructor and `get` becomes `impl<T> Holder<T>`, rather
than a bare `impl Holder`. A function parameter that is not part of the class's
own argument list (an ordinary, non-hoisted type parameter, or one used only in
an argument/return position) stays on the function as usual, so the two kinds
compose in a single signature:

```rust
#[wasm_bindgen]
extern "C" {
    type Holder<T>;

    // `T` is hoisted (it parameterises the receiver); `U` stays on `combine`.
    #[wasm_bindgen(method, experimental_generic_mono)]
    fn combine<T, U>(this: &Holder<T>, other: U);
}
```

A lifetime belonging to the class works the same way, on its own or alongside a
hoisted type parameter:

```rust
#[wasm_bindgen]
extern "C" {
    type LifetimeHolder<'a>;

    #[wasm_bindgen(method, experimental_generic_mono)]
    fn get<'a, T>(this: &'a LifetimeHolder<'a>) -> T;

    type LtHolder<'a, T>;

    #[wasm_bindgen(method, experimental_generic_mono, js_name = get)]
    fn lt_get<'a, T>(this: &'a LtHolder<'a, T>) -> T;
}
```

This composes with the constructor and self-returning static method (e.g.
`static_method_of = Holder`) shapes the same way it composes with an ordinary
instance method, mirroring how `Array::new`/`Array::of` return `Array<T>` in
`js-sys`.

A static method that is *not* the constructor and does not return the class is
not tied to the class's parameters at all, so it binds against the class's own
parameter defaults, exactly as it does on the erasure path.

### What can be hoisted

The function's generics are *hoisted* onto the generated `impl` block's own
header, so each generic argument of the class type has to be something that
header can name and that the self type can then determine. Each argument may
be:

* a generic parameter of the function, either bare (`&Holder<T>`) or composed in
  a way that still determines it (`&Holder<Option<T>>`);
* a lifetime parameter of the function (`&'a Holder<'a, T>`);
* concrete (`&Holder<u32>`). There is nothing to hoist for it — it is re-emitted
  as written, so the method lands on `impl Holder<u32>` and exists only for that
  instantiation of the class.

These mix freely within one argument list: `&Pair<u32, T>` gives
`impl<T> Pair<u32, T>`.

The following are rejected up front, rather than left to fail as a confusing
rustc error against generated code:

* **An impl class argument that mentions a parameter without determining it**
  (`&Holder<T::Assoc>`, or `static_method_of = Holder<T::Assoc>`). Hoisting `T`
  would leave it unconstrained by the self type.
* **An elided lifetime argument** (`&Holder<'_, T>`), which cannot be declared
  on the generated `impl` header. A concrete `'static` lifetime is supported;
  name any other lifetime as a parameter of the function instead
  (`fn f<'a, T>(this: &'a Holder<'a, T>)`).

A constructor or inferred self-returning static method whose return is
`Holder<T::Assoc>` is not rejected. Its return arguments cannot determine `T`,
so the method remains on the imported class's default specialization instead
of being hoisted.

## Callback arguments

A top-level raw `&dyn Fn(...)` or `&mut dyn FnMut(...)` callback may use type
parameters in its own inputs and return type. This is the shape used by typed
`js-sys` APIs such as `Array::map`:

```rust
#[wasm_bindgen]
extern "C" {
    type Array<T>;

    #[wasm_bindgen(method, experimental_generic_mono)]
    fn map<T, U>(
        this: &Array<T>,
        callback: &mut dyn FnMut(T, u32, Array<T>) -> U,
    ) -> Array<U>;
}
```

The public wrapper accepts an ordinary Rust closure through `impl Fn` or
`impl FnMut`, as on the non-generic import path. Each monomorphization describes
the callback's concrete ABI argument and return types.

Generic callback inputs and returns must be owned types. Borrowed forms such as
`FnMut(&T)` and higher-ranked forms such as `for<'a> FnMut(&'a T)` are rejected.
The callback must also be the top-level argument shape shown above; callback
trait objects nested in another type do not receive this lowering.

## Other attributes

`experimental_generic_mono` composes with the usual import attributes — `method`,
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
    #[wasm_bindgen(experimental_generic_mono)]
    async fn round_trip<T>(value: T) -> T;
}
```

## Applying it to a whole block

`experimental_generic_mono` can also go on the `extern "C"` block, which every function in
the block then inherits:

```rust
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    fn log_one<T>(x: T);
    fn log_two<T>(a: u32, b: T) -> T;
}
```

The block flag only applies where it can: the per-monomorphisation path needs at
least one type parameter, so a non-generic import in the block is left alone and
binds through the ordinary single shim. A block can therefore mix the two freely:

```rust
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    fn log_generic<T>(x: T); // per-monomorphisation
    fn log_u32(x: u32);      // ordinary import, unaffected
}
```

Writing `experimental_generic_mono` directly on a non-generic function is still an error,
since there you asked for something that cannot be done.

## Unsupported shapes

These are rejected at compile time with a diagnostic pointing at the offending
declaration. Each generally keeps working on the type-erasure path, so the fix is
usually to drop `experimental_generic_mono`:

* **A mutable reference to a type parameter** (`&mut T`, or `&mut Vec<T>`, or
  any other `&mut` whose referent mentions a type parameter), and a reference to
  a type parameter **nested inside another type** (e.g. `Option<&T>`). A bare
  `&T` *is* supported, and mutable references to *concrete* types (e.g.
  `&mut [u16]`, `&mut dyn FnMut(u32)`) bind exactly as they do on the
  non-generic import path — the restriction is only about references to type
  parameters. Top-level callbacks whose owned input or return types are generic
  are supported separately as described in [Callback arguments](#callback-arguments).
* **Borrowed or higher-ranked generic callback inputs and returns**
  (`FnMut(&T)`, `for<'a> FnMut(&'a T)`).
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
  `unsupported pattern in experimental_generic_mono imported function`. The generated
  per-monomorphisation shim has to forward each argument by name, so it needs a
  binding it can name. Give the argument a single identifier instead.

**Const generic parameters** are also rejected, but not by `experimental_generic_mono`:
`wasm-bindgen` does not support them on *any* generic import, erased or not, and
reports `unsupported in wasm-bindgen generics`. Dropping `experimental_generic_mono` will
not help.

**Type-parameter defaults** (`fn f<T = JsValue>(x: T)`) are rejected with
`defaults for generic parameters are not allowed here` — the same diagnostic
rustc gives for a default on any ordinary function. On the erasure path a default
is meaningful, because it picks the single concrete type the one shared binding is
generated for; under `experimental_generic_mono` there is no single instantiation to pick,
since every instantiation gets its own shim. The error is reported by
`wasm-bindgen` rather than rustc only because nothing of the original signature
survives macro expansion, so rustc's own deny-by-default
`invalid_type_param_default` lint never sees it. Drop the default, or use the
erasure path if you wanted it to mean something.

### Colliding imports

Everything above is reported by the macro, at compile time. One failure is
reported later, by the `wasm-bindgen` CLI, because it cannot be detected until
the whole module is linked: two `experimental_generic_mono` imports that agree on every
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
`wasm-bindgen` needs to marshal it — `JsValue` or a JS handle type. Passing
`&SomeStruct` for a plain Rust struct, or `&u32` for a scalar, is rejected,
since there is no `&T` ABI representation for them; take the value by value, or
pass a JS type.
