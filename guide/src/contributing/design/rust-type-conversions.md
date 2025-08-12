# Rust Type conversions

Previously we've been seeing mostly abridged versions of type conversions when
values enter Rust. Here we'll go into some more depth about how this is
implemented. There are two categories of traits for converting values, traits
for converting values from Rust to JS and traits for the other way around.

## From Rust to JS

First up let's take a look at going from Rust to JS:

```rust
pub trait IntoWasmAbi: WasmDescribe {
    type Abi: WasmAbi;
    fn into_abi(self) -> Self::Abi;
}
```

And that's it! This is actually the only trait needed currently for translating
a Rust value to a JS one. There's a few points here:

* We'll get to `WasmDescribe` later in this section.

* The associated type `Abi` is the type of the raw data that we actually want to pass to JS.
  The bound `WasmAbi` is implemented for primitive types like `u32` and `f64`,
  which can be represented directly as WebAssembly values, as well of a couple
  of other types like `WasmSlice`:

  ```rust
  pub struct WasmSlice {
      pub ptr: u32,
      pub len: u32,
  }
  ```

  This struct, which is how things like strings are represented in FFI, isn't
  a WebAssembly primitive type, and so it can't be mapped directly to a
  WebAssembly parameter / return value. This is why `WasmAbi` lets types specify
  how they can be split up into multiple WebAssembly parameters:

  ```rust
  impl WasmAbi for WasmSlice {
      fn split(self) -> (u32, u32, (), ()) {
          (self.ptr, self.len, (), ())
      }

      // some other details to specify return type of `split`, go in the other direction
  }
  ```

  This means that a `WasmSlice` gets split up into two `u32` parameters.
  The extra unit types on the end are there because Rust doesn't let us make
  `WasmAbi` generic over variable-length tuples, so we just take tuples of 4
  elements. The unit types still end up getting passed to/from JS, but the C ABI
  just completely ignores them and doesn't generate any arguments.

  Since we can't return multiple values, when returning a `WasmSlice` we instead
  put the two `u32`s into a `#[repr(C)]` struct and return that.

* And finally we have the `into_abi` function, returning the `Abi` associated
  type which will be actually passed to JS.

This trait is implemented for all types that can be converted to JS and is
unconditionally used during codegen. For example you'll often see `IntoWasmAbi
for Foo` but also `IntoWasmAbi for &'a Foo`.

The `IntoWasmAbi` trait is used in two locations. First it's used to convert
return values of Rust exported functions to JS. Second it's used to convert the
Rust arguments of JS functions imported to Rust.

## From JS to Rust

Unfortunately the opposite direction from above, going from JS to Rust, is a bit
more complicated. Here we've got three traits:

```rust
pub trait FromWasmAbi: WasmDescribe {
    type Abi: WasmAbi;
    unsafe fn from_abi(js: Self::Abi) -> Self;
}

pub trait ArgFromWasmAbi: WasmDescribe {
    type Abi: WasmAbi;
    type Anchor: Deref<Target=Self>;
    unsafe fn arg_from_abi(js: Self::Abi) -> Self::Anchor;
}

pub trait RefMutFromWasmAbi: WasmDescribe {
    type Abi: WasmAbi;
    type Anchor: DerefMut<Target=Self>;
    unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor;
}
```

The `FromWasmAbi` is relatively straightforward, basically the opposite of
`IntoWasmAbi`. It takes the ABI argument (typically the same as
`IntoWasmAbi::Abi`) to produce an instance of
`Self`. This trait is implemented primarily for types that *don't* have internal
lifetimes or are references.

The latter two traits here are mostly the same, and are intended for generating
references (both shared and mutable references). They look almost the same as
`FromWasmAbi` except that they return an `Anchor` type which implements a
`Deref` trait rather than `Self`.

The `Ref*` traits allow having arguments in functions that are references rather
than bare types, for example `&str`, `&JsValue`, or `&[u8]`. The `Anchor` here
is required to ensure that the lifetimes don't persist beyond one function call
and remain anonymous.

The `From*` family of traits are used for converting the Rust arguments in Rust
exported functions to JS. They are also used for the return value in JS
functions imported into Rust.
