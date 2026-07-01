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
/// This reference tree is the canonical descriptor representation, but it
/// reaches `wasm-bindgen-cli` through two transports:
///
/// * **Macro-emitted descriptors** (regular shims, imported statics,
///   struct-field getters, raw `&dyn Fn` args) are serialised — at
///   const-eval time, by the [`schema`] module — into the
///   `__wasm_bindgen_descriptors` custom section
///   ([`wasm_bindgen_shared::DESCRIPTORS_SECTION_NAME`]). There each node
///   is identified by a 128-bit content hash ([`Schema::id`]) and
///   references its children — and its closure invoke shim
///   ([`Schema::invoke_name`]) — by id/name, so the section is free of
///   linker relocations.
/// * **Cast descriptors** (`wbg_cast::<From, To>`) cannot be emitted into
///   a `#[link_section]` from generic runtime code, so they keep the
///   data-segment [`DescriptorRecord`] transport and `wasm-bindgen-cli`
///   walks their `Schema` trees by relocated pointer.
///
/// Scalars (opcodes, lengths, holes, `nargs`) live in a node's
/// [`Schema::words`]; sub-descriptors live in its [`Schema::children`].
/// The cast transport carries the closure invoke shim address out-of-band
/// in [`Schema::invoke`], never inside the `words` run, so structure and
/// relocated pointers never alias.
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
    ///
    /// This field is used only by the **cast** transport, whose records
    /// still live in the data segment (a generic `wbg_cast::<From, To>`
    /// cannot emit `#[link_section]` bytes). Section-emitted descriptors
    /// reference their closure invoke shim by name via [`Schema::invoke_name`].
    invoke: *const (),
    // --- fields below are appended after `invoke` on purpose ---
    //
    // The CLI's `#[repr(C)]` cast reader only ever reads the six fields
    // above (`tag` .. `invoke`) out of the data segment, so appending
    // more fields — even a 16-byte-aligned `u128` — leaves those offsets
    // untouched. These fields are consumed at const-eval time by the
    // section serialiser below, never by memory-layout parsing.
    /// 128-bit content hash of this node (`H(tag, words, child ids)`),
    /// the node's identity in the `__wasm_bindgen_descriptors` section.
    /// Excludes [`Schema::invoke`] / [`Schema::invoke_name`] so
    /// structurally-identical types share an id.
    id: u128,
    /// Export name of this node's closure invoke shim, or `""` for a
    /// non-closure node. The section stores this string; the CLI resolves
    /// it to a function-table slot (no relocation needed, unlike the
    /// data-segment [`Schema::invoke`] pointer).
    invoke_name: &'static str,
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
            id: compute_id(tag, words, children, ""),
            invoke_name: "",
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
            id: compute_id(tag, words, children, ""),
            invoke_name: "",
        }
    }

    /// Like [`Schema::closure_node`], but records the invoke shim's
    /// export **name** instead of a relocated address. Used by the
    /// section transport for raw `&dyn Fn` / `&mut dyn FnMut` arguments:
    /// the macro emits and exports an `__wbg_invoke_<hash>` wrapper and
    /// stores its name here, which the CLI resolves to a function-table
    /// slot without any linker relocation.
    pub const fn closure_node_named(
        tag: SchemaTag,
        words: &'static [u32],
        children: &'static [&'static Schema],
        invoke_name: &'static str,
    ) -> Schema {
        Schema {
            tag,
            words: words.as_ptr(),
            words_len: words.len(),
            children: children.as_ptr(),
            children_len: children.len(),
            invoke: core::ptr::null(),
            id: compute_id(tag, words, children, invoke_name),
            invoke_name,
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
            // The content id is structural, so it is preserved verbatim:
            // attaching a different invoke shim does not change the type.
            id: base.id,
            invoke_name: base.invoke_name,
        }
    }

    /// A leaf node: `words` only, no children.
    pub const fn leaf(words: &'static [u32]) -> Schema {
        Schema::node(SchemaTag::Leaf, words, &[])
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

    /// This node's 128-bit content-hash id — its identity in the
    /// `__wasm_bindgen_descriptors` section.
    pub const fn id(&self) -> u128 {
        self.id
    }

    /// This node's closure invoke shim export name, or `""` if it is not
    /// a section-emitted closure node.
    pub const fn invoke_name(&self) -> &'static str {
        self.invoke_name
    }
}

/// Compute a node's 128-bit content-hash id from its structural content
/// (tag, opcode words, its children's already-computed ids) plus its
/// closure invoke reference, using the shared FNV-1a-128 primitives so
/// producer and consumer agree.
///
/// The invoke name is folded in so a closure-bearing `FUNCTION` node is
/// never conflated with a structurally-identical plain-function node
/// (which carries no invoke): e.g. the `FUNCTION` node for `&dyn
/// Fn(u32, u32)` must not dedup against the top-level node for
/// `fn(u32, u32)`, or the closure's `shim_idx` would be lost when the
/// consumer unions nodes by id. It also keeps distinct invoke shims
/// distinct.
const fn compute_id(
    tag: SchemaTag,
    words: &[u32],
    children: &[&'static Schema],
    invoke_name: &str,
) -> u128 {
    use wasm_bindgen_shared::descriptor_hash as h;
    let mut hash = h::init();
    hash = h::update_u32(hash, tag as u32);
    hash = h::update_u32(hash, words.len() as u32);
    let mut i = 0;
    while i < words.len() {
        hash = h::update_u32(hash, words[i]);
        i += 1;
    }
    hash = h::update_u32(hash, children.len() as u32);
    let mut j = 0;
    while j < children.len() {
        hash = h::update_u128(hash, children[j].id);
        j += 1;
    }
    hash = h::update_bytes(hash, invoke_name.as_bytes());
    hash
}

/// Const-fn serialisation of a schema tree into a
/// `__wasm_bindgen_descriptors` section entry.
///
/// The `#[wasm_bindgen]` macro emits one `#[link_section]` byte array per
/// descriptor by calling [`pack_entry`] (sized via [`entry_byte_len`]) at
/// const-eval time. Every reachable node is written pre-order (no
/// in-entry dedup — the consumer unions all nodes across all entries into
/// one `id -> node` map, so duplicates are harmless and collapse for
/// free). Children and roots are referenced only by 128-bit id, so the
/// section carries no linker relocations.
///
/// ## Wire format
///
/// One entry (`#[link_section]` static), little-endian throughout:
///
/// ```text
/// u8        format_version
/// u32       body_len            (bytes after this field; lets an older
///                                CLI skip an unknown-version entry)
/// -- body --
/// u8        kind                (REGULAR / STATIC)
/// u32       name_len
/// [u8]      name                (shim name, UTF-8)
/// u128      root_id
/// u32       node_count
/// node_count x Node:
///   u128    id
///   u32     n_words
///   [u32]   words
///   u32     n_children
///   [u128]  child_ids
///   u32     invoke_name_len
///   [u8]    invoke_name         (closure invoke shim export name; empty
///                                for the overwhelmingly common case)
/// ```
pub mod schema {
    use super::Schema;
    use wasm_bindgen_shared::DESCRIPTOR_FORMAT_VERSION;

    /// Number of nodes written for `s` (pre-order, no dedup).
    pub const fn node_count(s: &Schema) -> usize {
        let mut count = 1;
        let kids = s.children();
        let mut i = 0;
        while i < kids.len() {
            count += node_count(kids[i]);
            i += 1;
        }
        count
    }

    /// Bytes occupied by a single node record.
    const fn single_node_byte_len(s: &Schema) -> usize {
        16 // id
            + 4 // n_words
            + 4 * s.words().len()
            + 4 // n_children
            + 16 * s.children().len()
            + 4 // invoke_name_len
            + s.invoke_name().len()
    }

    /// Bytes occupied by `s` and all its descendants (pre-order).
    const fn nodes_byte_len(s: &Schema) -> usize {
        let mut total = single_node_byte_len(s);
        let kids = s.children();
        let mut i = 0;
        while i < kids.len() {
            total += nodes_byte_len(kids[i]);
            i += 1;
        }
        total
    }

    /// Total byte length of an entry for a shim `name` of `name_len`
    /// bytes whose descriptor root is `root`. The macro passes the result
    /// as the `[u8; N]` array length at the emission site.
    pub const fn entry_byte_len(name_len: usize, root: &Schema) -> usize {
        1  // format_version
            + 4  // body_len
            + 1  // kind
            + 4  // name_len
            + name_len
            + 16 // root_id
            + 4  // node_count
            + nodes_byte_len(root)
    }

    const fn write_u8<const B: usize>(mut buf: [u8; B], idx: usize, v: u8) -> ([u8; B], usize) {
        buf[idx] = v;
        (buf, idx + 1)
    }

    const fn write_u32<const B: usize>(
        mut buf: [u8; B],
        mut idx: usize,
        v: u32,
    ) -> ([u8; B], usize) {
        let bytes = v.to_le_bytes();
        let mut i = 0;
        while i < bytes.len() {
            buf[idx] = bytes[i];
            idx += 1;
            i += 1;
        }
        (buf, idx)
    }

    const fn write_u128<const B: usize>(
        mut buf: [u8; B],
        mut idx: usize,
        v: u128,
    ) -> ([u8; B], usize) {
        let bytes = v.to_le_bytes();
        let mut i = 0;
        while i < bytes.len() {
            buf[idx] = bytes[i];
            idx += 1;
            i += 1;
        }
        (buf, idx)
    }

    const fn write_bytes<const B: usize>(
        mut buf: [u8; B],
        mut idx: usize,
        data: &[u8],
    ) -> ([u8; B], usize) {
        let mut i = 0;
        while i < data.len() {
            buf[idx] = data[i];
            idx += 1;
            i += 1;
        }
        (buf, idx)
    }

    /// Write one node record (no recursion).
    const fn write_single_node<const B: usize>(
        buf: [u8; B],
        idx: usize,
        s: &Schema,
    ) -> ([u8; B], usize) {
        let (buf, idx) = write_u128(buf, idx, s.id());
        let words = s.words();
        let (mut buf, mut idx) = write_u32(buf, idx, words.len() as u32);
        let mut i = 0;
        while i < words.len() {
            let (b, n) = write_u32(buf, idx, words[i]);
            buf = b;
            idx = n;
            i += 1;
        }
        let kids = s.children();
        let (mut buf, mut idx) = write_u32(buf, idx, kids.len() as u32);
        let mut k = 0;
        while k < kids.len() {
            let (b, n) = write_u128(buf, idx, kids[k].id());
            buf = b;
            idx = n;
            k += 1;
        }
        let inv = s.invoke_name().as_bytes();
        let (buf, idx) = write_u32(buf, idx, inv.len() as u32);
        write_bytes(buf, idx, inv)
    }

    /// Write `s` and all descendants pre-order.
    const fn write_nodes<const B: usize>(
        buf: [u8; B],
        idx: usize,
        s: &Schema,
    ) -> ([u8; B], usize) {
        let (mut buf, mut idx) = write_single_node(buf, idx, s);
        let kids = s.children();
        let mut i = 0;
        while i < kids.len() {
            let (b, n) = write_nodes(buf, idx, kids[i]);
            buf = b;
            idx = n;
            i += 1;
        }
        (buf, idx)
    }

    /// Pack a full section entry into a `[u8; B]`. `B` must equal
    /// [`entry_byte_len`]`(name.len(), root)`; the macro guarantees this
    /// at the call site (a mismatch fails the trailing `assert!`).
    pub const fn pack_entry<const B: usize>(name: &[u8], kind: u32, root: &Schema) -> [u8; B] {
        let buf = [0u8; B];
        let (buf, idx) = write_u8(buf, 0, DESCRIPTOR_FORMAT_VERSION as u8);
        // body_len = everything after the 5-byte (version + len) header.
        let (buf, idx) = write_u32(buf, idx, (B - 5) as u32);
        let (buf, idx) = write_u8(buf, idx, kind as u8);
        let (buf, idx) = write_u32(buf, idx, name.len() as u32);
        let (buf, idx) = write_bytes(buf, idx, name);
        let (buf, idx) = write_u128(buf, idx, root.id());
        let (buf, idx) = write_u32(buf, idx, node_count(root) as u32);
        let (buf, idx) = write_nodes(buf, idx, root);
        assert!(idx == B, "pack_entry: computed size does not match B");
        buf
    }
}

/// One descriptor entry, referenced (by pointer) from a marker carrier
/// function's call to the `__wbindgen_descriptor_marker` import. This is
/// the single, unified descriptor transport: regular shims, imported
/// statics, and casts all emit one of these into the data segment.
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
    /// One of `DESCRIPTOR_KIND_REGULAR` / `_STATIC` / `_CAST`.
    pub kind: u32,
    /// Pointer to the shim name's UTF-8 bytes, or null for casts (which
    /// the CLI matches by trampoline identity, not by name).
    pub name: *const u8,
    /// Length of the shim name in bytes; `0` for casts.
    pub name_len: usize,
    /// Schema root. For `REGULAR` it is the `FUNCTION` node; for `STATIC`
    /// the bare type node; for `CAST` the `From` type node.
    pub root: &'static Schema,
    /// `CAST` only: the `To` type node. Null for `REGULAR` / `STATIC`.
    pub to_root: Option<&'static Schema>,
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
    const VECTOR_SCHEMA: &'static Schema = &Schema::node(
        SchemaTag::Wrap,
        &[VECTOR],
        &[<JsValue as WasmDescribe>::SCHEMA],
    );
}

impl WasmDescribeVector for JsError {
    const VECTOR_SCHEMA: &'static Schema = &Schema::node(
        SchemaTag::Wrap,
        &[VECTOR],
        &[<JsValue as WasmDescribe>::SCHEMA],
    );
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
