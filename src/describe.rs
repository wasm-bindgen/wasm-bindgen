//! This is an internal module, no stability guarantees are provided. Use at
//! your own risk.

#![doc(hidden)]

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::panic::AssertUnwindSafe;
use core::{mem::MaybeUninit, ptr::NonNull};

use crate::{Clamped, JsError, JsValue};
use cfg_if::cfg_if;

pub use wasm_bindgen_shared::tys::*;

/// A node in the reference-based wasm-bindgen type schema tree.
///
/// Each `WasmDescribe` impl exposes its schema as a single
/// `&'static Schema`. Wrapper types (`Option<T>`, `Vec<T>`, `&T`,
/// closures, ...) compose **by reference**: they point at the inner
/// type's already-built `Schema` rather than copying its opcodes into
/// a buffer. Because no array length ever depends on a generic
/// parameter, this sidesteps the `generic_const_exprs` wall
/// (rust-lang/rust#76560) without the old fixed-size `[u32; SCHEMA_MAX]`
/// buffer (and its hard 256-word cap).
///
/// `#[repr(C)]` so `wasm-bindgen-cli-support` can parse the node out of
/// the linked module's data segment deterministically. The structural
/// fields use thin base pointers plus explicit lengths (rather than
/// `&'static [T]` fat-pointer slices, whose field order is not
/// `#[repr(C)]`-stable) so the layout is fully determined for the CLI
/// parser.
///
/// Each base is a raw `*const` taken via `<[_]>::as_ptr`, so it carries
/// provenance over the **whole** run — reconstructing the slice in
/// [`Schema::words`] / [`Schema::children`] is therefore sound at
/// runtime as well as in const-eval (a reference to element 0, by
/// contrast, only has provenance for one element and would be UB to read
/// past under Stacked/Tree Borrows). For an *empty* run the base is a
/// dangling-but-aligned pointer and `*_len == 0`; every reader must
/// check the length before touching the pointer (the accessors below and
/// the CLI's tree reader both do). `as_ptr` does not reference a
/// `static`, so this stays valid on old compilers (it keeps the MSRV
/// low — `const_refs_to_static` was unstable < 1.83).
///
/// This reference tree is the **sole canonical descriptor ABI**: the
/// `#[wasm_bindgen]` macro and runtime emit a [`DescriptorRecord`] (in
/// the data segment) per shim/cast pointing at its `Schema` root, and
/// `wasm-bindgen-cli` walks the tree structurally — there is no flat
/// `u32` opcode stream and no custom section. Scalars (opcodes, lengths,
/// holes, `nargs`) live in a node's [`Schema::words`]; sub-descriptors
/// live in its [`Schema::children`]. Closure invoke shim addresses are
/// carried out-of-band in [`Schema::invoke`] (see below), never inside
/// the `words` run, so structure and relocated pointers never alias.
#[repr(C)]
pub struct Schema {
    /// Structural tag (`SchemaTag::Leaf` / `Wrap`).
    /// Informational; the CLI uses it as a validation/documentation aid.
    pub tag: SchemaTag,
    /// Base of a `words_len`-long run of opcode words. Dangling (but
    /// aligned) when `words_len == 0`.
    words: *const u32,
    words_len: usize,
    /// Base of a `children_len`-long run of child schema references.
    /// Dangling (but aligned) when `children_len == 0`.
    children: *const &'static Schema,
    children_len: usize,
    /// Closure invoke shim address for a closure-bearing `FUNCTION`
    /// node, stored as `invoke_fn as *const ()`. The linker lowers it to
    /// a function-table-index relocation that the CLI reads back from the
    /// data segment and uses as the `FUNCTION`'s `shim_idx`. Null for
    /// every non-closure node (the common case). Kept as an out-of-band
    /// field — rather than packed into `words` — because a relocated
    /// pointer cannot live in the plain `u32` data stream, and because
    /// const-eval on the MSRV cannot reconstruct a contiguous nested
    /// record by value.
    invoke: *const (),
}

// SAFETY: a `Schema` is only ever constructed as an immutable, promoted
// `'static` whose raw pointers address other `'static`s; it is never
// mutated, so sharing it across threads is sound despite the raw
// pointers (which are what cost the auto `Sync` impl).
unsafe impl Sync for Schema {}

impl Schema {
    /// General `Schema` builder.
    ///
    /// `words` and `children` must be promotable to `'static` at the
    /// **call site** — write them as array literals in a `const`
    /// initializer (e.g. `&[OPTIONAL]`, `&[T::SCHEMA]`). Building the
    /// `children` run inside a `const fn` does not work: rvalue static
    /// promotion only happens at the call site.
    pub const fn node(
        tag: SchemaTag,
        words: &'static [u32],
        children: &'static [&'static Schema],
    ) -> Schema {
        Schema {
            tag,
            // `as_ptr` gives whole-run provenance (unlike `first()`,
            // which only covers element 0) and references no `static`,
            // keeping the MSRV low. Empty runs yield a dangling pointer
            // that readers must guard with `*_len == 0`.
            words: words.as_ptr(),
            words_len: words.len(),
            children: children.as_ptr(),
            children_len: children.len(),
            invoke: core::ptr::null(),
        }
    }

    /// Like [`Schema::node`], but also records a closure invoke shim
    /// address in [`Schema::invoke`]. Used for closure-bearing
    /// `FUNCTION` nodes (raw `&dyn Fn` arguments and closure casts);
    /// the CLI reads the relocated value as the node's `shim_idx`.
    pub const fn closure_node(
        tag: SchemaTag,
        words: &'static [u32],
        children: &'static [&'static Schema],
        invoke: *const (),
    ) -> Schema {
        Schema {
            tag,
            words: words.as_ptr(),
            words_len: words.len(),
            children: children.as_ptr(),
            children_len: children.len(),
            invoke,
        }
    }

    /// Copy `base`'s structure but attach a closure invoke shim address.
    ///
    /// Used by the closure-cast path, which composes its `From` schema
    /// out of a shared `<T as WasmDescribe>::SCHEMA` (whose `invoke` is
    /// null) and the per-`(T, UNWIND_SAFE)` invoke shim chosen at the
    /// cast site. The raw structural pointers are copied verbatim (same
    /// provenance); only `invoke` is overwritten.
    pub const fn with_invoke(base: &'static Schema, invoke: *const ()) -> Schema {
        Schema {
            tag: base.tag,
            words: base.words,
            words_len: base.words_len,
            children: base.children,
            children_len: base.children_len,
            invoke,
        }
    }

    /// A leaf node: `words` only, no children.
    pub const fn leaf(words: &'static [u32]) -> Schema {
        Schema::node(SchemaTag::Leaf, words, &[])
    }

    /// A generic type-parameter hole node used inside a generic import's
    /// signature *template*. `idx_word` must be a single-element slice
    /// holding the zero-based parameter index (write it as a literal at the
    /// call site so it promotes to `'static`, e.g. `&[0u32]`). The CLI
    /// splices the concrete `fills[idx]` schema in place of this node.
    pub const fn type_param(idx_word: &'static [u32]) -> Schema {
        Schema::node(SchemaTag::TypeParam, idx_word, &[])
    }

    /// This node's run of opcode words as a slice.
    pub const fn words(&self) -> &[u32] {
        if self.words_len == 0 {
            // The base is dangling for an empty run; never form a slice
            // from it.
            &[]
        } else {
            // SAFETY: `words` has whole-run provenance over exactly
            // `words_len` contiguous `'static` `u32`s (taken via
            // `<[u32]>::as_ptr`).
            unsafe { core::slice::from_raw_parts(self.words, self.words_len) }
        }
    }

    /// This node's run of child schema references as a slice. See
    /// [`Schema::words`].
    pub const fn children(&self) -> &[&'static Schema] {
        if self.children_len == 0 {
            &[]
        } else {
            // SAFETY: `children` has whole-run provenance over exactly
            // `children_len` contiguous `'static` `&Schema`s (taken via
            // `<[_]>::as_ptr`).
            unsafe { core::slice::from_raw_parts(self.children, self.children_len) }
        }
    }
}

/// One descriptor entry, referenced (by pointer) from a marker carrier
/// function's call to the `__wbindgen_descriptor_marker` import. This is
/// the single, unified descriptor transport: regular shims, imported
/// statics, casts, and generic-import monomorphizations all emit one of
/// these into the data segment.
///
/// `#[repr(C)]` so `wasm-bindgen-cli-support` can parse it out of the
/// linked module's data segment. All pointer fields are relocated by the
/// linker and read back by the CLI through the same data-segment view it
/// uses for the `Schema` trees themselves.
#[repr(C)]
pub struct DescriptorRecord {
    /// Format version. The CLI skips (and logs) records whose version it
    /// does not recognise, mirroring the old per-entry forward-compat.
    pub version: u32,
    /// One of `DESCRIPTOR_KIND_REGULAR` / `_STATIC` / `_CAST` /
    /// `_GENERIC_IMPORT`.
    pub kind: u32,
    /// Pointer to the shim name's UTF-8 bytes, or null for casts (which
    /// the CLI matches by trampoline identity, not by name). For
    /// `GENERIC_IMPORT` this is the generic import's shim name.
    pub name: *const u8,
    /// Length of the shim name in bytes; `0` for casts.
    pub name_len: usize,
    /// Schema root. For `REGULAR` it is the `FUNCTION` node; for `STATIC`
    /// the bare type node; for `CAST` the `From` type node; for
    /// `GENERIC_IMPORT` the `FUNCTION` *template* node (whose
    /// generic-parameter positions are `SchemaTag::TypeParam` holes).
    pub root: &'static Schema,
    /// `CAST` only: the `To` type node. Null for `REGULAR` / `STATIC` /
    /// `GENERIC_IMPORT`.
    pub to_root: *const Schema,
    /// `GENERIC_IMPORT` only: base of a `fills_len`-long run of concrete
    /// per-type-parameter `Schema`s, indexed by parameter index (deduped:
    /// one entry per distinct parameter). The CLI splices `fills[idx]` into
    /// each `TypeParam(idx)` hole in `root`. Closure-typed parameters carry
    /// their per-monomorphization invoke-shim address in the fill node's
    /// `Schema::invoke`. Dangling-but-aligned when `fills_len == 0` (every
    /// non-generic record); readers must guard on the length.
    pub fills: *const &'static Schema,
    /// Number of fills; `0` for non-generic records.
    pub fills_len: usize,
}

// `DescriptorRecord` holds raw pointers, so it is not `Sync` by default;
// it is only ever shared immutably as a promoted `'static`, so asserting
// `Sync` is sound.
unsafe impl Sync for DescriptorRecord {}

/// Describes the wasm-bindgen type schema for a type as a single
/// `&'static Schema` tree (see [`Schema`]).
///
/// The `#[wasm_bindgen]` macro references this tree from a
/// [`DescriptorRecord`] it emits into the data segment.
/// `wasm-bindgen-cli` walks the tree structurally — no wasm
/// interpretation and no flat opcode stream involved.
pub trait WasmDescribe {
    /// This type's schema tree.
    const SCHEMA: &'static Schema;
}

/// Trait for element types to implement `WasmDescribe` for vectors of
/// themselves. Same shape as [`WasmDescribe`] but describes the
/// `Vec<Self>` / `Box<[Self]>` schema rather than `Self`'s.
pub trait WasmDescribeVector {
    /// The `Vec<Self>` / `Box<[Self]>` schema tree.
    const VECTOR_SCHEMA: &'static Schema;
}

macro_rules! simple {
    ($($t:ident => $d:ident)*) => ($(
        impl WasmDescribe for $t {
            const SCHEMA: &'static Schema = &Schema::leaf(&[$d]);
        }
    )*)
}

simple! {
    i8 => I8
    u8 => U8
    i16 => I16
    u16 => U16
    i32 => I32
    u32 => U32
    i64 => I64
    u64 => U64
    i128 => I128
    u128 => U128
    f32 => F32
    f64 => F64
    bool => BOOLEAN
    char => CHAR
    JsValue => EXTERNREF
}

// isize/usize map to I32/U32 on wasm32 and direct *_AS_F64 descriptors on wasm64
cfg_if! {
    if #[cfg(target_arch = "wasm64")] {
        simple! {
            isize => I64_AS_F64
            usize => U64_AS_F64
        }
    } else {
        simple! {
            isize => I32
            usize => U32
        }
    }
}

cfg_if! {
    if #[cfg(feature = "enable-interning")] {
        simple! {
            str => CACHED_STRING
        }

    } else {
        simple! {
            str => STRING
        }
    }
}

impl<T> WasmDescribe for *const T {
    const SCHEMA: &'static Schema = &Schema::leaf(&[RAW_POINTER]);
}

impl<T> WasmDescribe for *mut T {
    const SCHEMA: &'static Schema = &Schema::leaf(&[RAW_POINTER]);
}

impl<T> WasmDescribe for NonNull<T> {
    const SCHEMA: &'static Schema = &Schema::leaf(&[NONNULL]);
}

impl<T: WasmDescribe> WasmDescribe for [T] {
    const SCHEMA: &'static Schema = &Schema::node(SchemaTag::Wrap, &[SLICE], &[T::SCHEMA]);
}

impl<T: WasmDescribe + ?Sized> WasmDescribe for &T {
    const SCHEMA: &'static Schema = &Schema::node(SchemaTag::Wrap, &[REF], &[T::SCHEMA]);
}

impl<T: WasmDescribe + ?Sized> WasmDescribe for &mut T {
    const SCHEMA: &'static Schema = &Schema::node(SchemaTag::Wrap, &[REFMUT], &[T::SCHEMA]);
}

cfg_if! {
    if #[cfg(feature = "enable-interning")] {
        simple! {
            String => CACHED_STRING
        }

    } else {
        simple! {
            String => STRING
        }
    }
}

// Concrete `WasmDescribeVector` impls for the JsValue-erased types
// previously covered by a blanket
// `impl<T: ErasableGeneric<Repr=JsValue>> WasmDescribeVector for T`.
//
// Retaining concrete impls keeps `Vec<UserStruct>`-style
// `VECTOR_SCHEMA` emissions inside the macro (which use
// `NAMED_EXTERNREF` rather than the struct's own `SCHEMA`)
// straightforward, and avoids unnecessary deep monomorphisation
// chains in the const evaluator.
impl WasmDescribeVector for JsValue {
    const VECTOR_SCHEMA: &'static Schema =
        &Schema::node(SchemaTag::Wrap, &[VECTOR], &[<JsValue as WasmDescribe>::SCHEMA]);
}

impl WasmDescribeVector for JsError {
    const VECTOR_SCHEMA: &'static Schema =
        &Schema::node(SchemaTag::Wrap, &[VECTOR], &[<JsValue as WasmDescribe>::SCHEMA]);
}

impl<T: WasmDescribeVector> WasmDescribe for Box<[T]> {
    const SCHEMA: &'static Schema = T::VECTOR_SCHEMA;
}

impl<T> WasmDescribe for Vec<T>
where
    Box<[T]>: WasmDescribe,
{
    const SCHEMA: &'static Schema = <Box<[T]> as WasmDescribe>::SCHEMA;
}

impl<T: WasmDescribe> WasmDescribe for Option<T> {
    const SCHEMA: &'static Schema = &Schema::node(SchemaTag::Wrap, &[OPTIONAL], &[T::SCHEMA]);
}

impl WasmDescribe for () {
    const SCHEMA: &'static Schema = &Schema::leaf(&[UNIT]);
}

impl<T: WasmDescribe, E: Into<JsValue>> WasmDescribe for Result<T, E> {
    const SCHEMA: &'static Schema = &Schema::node(SchemaTag::Wrap, &[RESULT], &[T::SCHEMA]);
}

impl<T: WasmDescribe> WasmDescribe for MaybeUninit<T> {
    // MaybeUninit<T> is transparent for descriptor purposes: it
    // crosses the boundary as exactly the same shape as T.
    const SCHEMA: &'static Schema = T::SCHEMA;
}

impl<T: WasmDescribe> WasmDescribe for Clamped<T> {
    const SCHEMA: &'static Schema = &Schema::node(SchemaTag::Wrap, &[CLAMPED], &[T::SCHEMA]);
}

impl WasmDescribe for JsError {
    // JsError's schema is the same as JsValue's: a single EXTERNREF.
    const SCHEMA: &'static Schema = <JsValue as WasmDescribe>::SCHEMA;
}

impl<T> WasmDescribe for AssertUnwindSafe<T>
where
    T: WasmDescribe,
{
    const SCHEMA: &'static Schema = T::SCHEMA;
}
