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
/// parser. Empty runs use `*_len == 0` with a `'static` sentinel base.
///
/// The flat `u32` opcode stream a schema represents is its `words`
/// followed by each child's flattened stream, in order (see
/// [`flatten_len`] / [`flatten_into`]); this is byte-identical to the
/// stream the previous fixed-buffer scheme produced, so the shipped
/// `__wasm_bindgen_descriptors` wire format and `Descriptor::decode`
/// are unchanged.
#[repr(C)]
pub struct Schema {
    /// Structural tag: `SCHEMA_NODE_LEAF` / `SCHEMA_NODE_WRAP` /
    /// `SCHEMA_NODE_CAT`. Informational for flattening (which is
    /// uniform); used by the CLI as a validation/documentation aid.
    pub tag: u32,
    /// Base of a `words_len`-long run of opcode words.
    pub words: &'static u32,
    pub words_len: usize,
    /// Base of a `children_len`-long run of child schema references.
    pub children: &'static &'static Schema,
    pub children_len: usize,
}

// `Schema` contains only shared references, so it is automatically
// `Sync`; the explicit empty sentinels below rely on that.

/// Sentinel for empty `words` runs (`words_len == 0`); never read.
static EMPTY_WORD: u32 = 0;
/// Sentinel pointee for empty `children` runs (`children_len == 0`).
static EMPTY_CHILD: Schema = Schema {
    tag: SCHEMA_NODE_LEAF,
    words: &EMPTY_WORD,
    words_len: 0,
    children: &EMPTY_CHILD_PTR,
    children_len: 0,
};
/// Sentinel base pointer for empty `children` runs; never read.
static EMPTY_CHILD_PTR: &Schema = &EMPTY_CHILD;

impl Schema {
    /// General `Schema` builder.
    ///
    /// `words` and `children` must be promotable to `'static` at the
    /// **call site** — write them as array literals in a `const`
    /// initializer (e.g. `&[OPTIONAL]`, `&[T::SCHEMA]`). Building the
    /// `children` run inside a `const fn` does not work: rvalue static
    /// promotion only happens at the call site.
    pub const fn node(
        tag: u32,
        words: &'static [u32],
        children: &'static [&'static Schema],
    ) -> Schema {
        Schema {
            tag,
            words: if words.is_empty() {
                &EMPTY_WORD
            } else {
                &words[0]
            },
            words_len: words.len(),
            children: if children.is_empty() {
                &EMPTY_CHILD_PTR
            } else {
                &children[0]
            },
            children_len: children.len(),
        }
    }

    /// A leaf node: `words` only, no children.
    pub const fn leaf(words: &'static [u32]) -> Schema {
        Schema::node(SCHEMA_NODE_LEAF, words, &[])
    }
}

/// Reconstruct a node's `words` run as a slice. Const-safe: the base
/// pointer was taken from element 0 of a `'static` array of exactly
/// `words_len` elements, so the provenance covers the whole run.
const fn words_slice(s: &Schema) -> &[u32] {
    // SAFETY: `s.words` points at the first of `s.words_len` contiguous
    // `'static` `u32`s (or the empty sentinel when `words_len == 0`).
    unsafe { core::slice::from_raw_parts(s.words as *const u32, s.words_len) }
}

/// Reconstruct a node's `children` run as a slice. See [`words_slice`].
const fn children_slice(s: &Schema) -> &[&'static Schema] {
    // SAFETY: `s.children` points at the first of `s.children_len`
    // contiguous `'static` `&Schema`s (or the empty sentinel).
    unsafe {
        core::slice::from_raw_parts(s.children as *const &'static Schema, s.children_len)
    }
}

/// Number of `u32` words the flattened schema occupies: this node's
/// `words` plus every child's flattened length, recursively.
pub const fn flatten_len(s: &Schema) -> usize {
    let mut total = s.words_len;
    let kids = children_slice(s);
    let mut i = 0;
    while i < kids.len() {
        total += flatten_len(kids[i]);
        i += 1;
    }
    total
}

/// Flatten a schema tree into the flat `u32` opcode stream (pre-order:
/// each node's `words` then its children's flattened streams).
///
/// `N` must equal [`flatten_len`] for `s`; the macro picks it at the
/// concrete call site (legal on stable — no generic-length arrays).
pub const fn flatten_into<const N: usize>(s: &Schema) -> [u32; N] {
    let mut out = [0u32; N];
    let written = write_flat(s, &mut out, 0);
    assert!(
        written == N,
        "flatten_into: N does not match flatten_len(schema)"
    );
    out
}

const fn write_flat(s: &Schema, out: &mut [u32], mut idx: usize) -> usize {
    let words = words_slice(s);
    let mut i = 0;
    while i < words.len() {
        out[idx] = words[i];
        idx += 1;
        i += 1;
    }
    let kids = children_slice(s);
    let mut k = 0;
    while k < kids.len() {
        idx = write_flat(kids[k], out, idx);
        k += 1;
    }
    idx
}

/// One closure/plain cast's descriptor, referenced (by pointer) from a
/// cast trampoline's call to the `__wbindgen_cast_marker` import.
///
/// `#[repr(C)]` so `wasm-bindgen-cli-support` can parse it out of the
/// linked module's data segment. `invoke` is stored as a **pointer**
/// (not a const integer): for closure casts it is the invoke shim's
/// function-item address (`fn as *const ()`), which the linker lowers
/// to a function-table-index relocation the CLI reads back from the
/// data segment; for plain casts it is null.
#[repr(C)]
pub struct CastRecord {
    pub from: &'static Schema,
    pub to: &'static Schema,
    pub invoke: *const (),
}

// `CastRecord` holds a raw pointer (`invoke`), so it is not `Sync` by
// default; it is only ever shared immutably as a promoted `'static`, so
// asserting `Sync` is sound.
unsafe impl Sync for CastRecord {}

/// Describes the wasm-bindgen type schema for a type as a single
/// `&'static Schema` tree (see [`Schema`]).
///
/// The `#[wasm_bindgen]` macro flattens this tree at compile time (via
/// [`flatten_into`]) and writes the resulting words into the
/// `__wasm_bindgen_descriptors` custom section
/// ([`wasm_bindgen_shared::DESCRIPTORS_SECTION_NAME`]).
/// `wasm-bindgen-cli` reads those bytes structurally — no wasm
/// interpretation involved.
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
    const SCHEMA: &'static Schema = &Schema::node(SCHEMA_NODE_WRAP, &[SLICE], &[T::SCHEMA]);
}

impl<T: WasmDescribe + ?Sized> WasmDescribe for &T {
    const SCHEMA: &'static Schema = &Schema::node(SCHEMA_NODE_WRAP, &[REF], &[T::SCHEMA]);
}

impl<T: WasmDescribe + ?Sized> WasmDescribe for &mut T {
    const SCHEMA: &'static Schema = &Schema::node(SCHEMA_NODE_WRAP, &[REFMUT], &[T::SCHEMA]);
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
    const VECTOR_SCHEMA: &'static Schema = &Schema::leaf(&[VECTOR, EXTERNREF]);
}

impl WasmDescribeVector for JsError {
    const VECTOR_SCHEMA: &'static Schema = &Schema::leaf(&[VECTOR, EXTERNREF]);
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
    const SCHEMA: &'static Schema = &Schema::node(SCHEMA_NODE_WRAP, &[OPTIONAL], &[T::SCHEMA]);
}

impl WasmDescribe for () {
    const SCHEMA: &'static Schema = &Schema::leaf(&[UNIT]);
}

impl<T: WasmDescribe, E: Into<JsValue>> WasmDescribe for Result<T, E> {
    const SCHEMA: &'static Schema = &Schema::node(SCHEMA_NODE_WRAP, &[RESULT], &[T::SCHEMA]);
}

impl<T: WasmDescribe> WasmDescribe for MaybeUninit<T> {
    // MaybeUninit<T> is transparent for descriptor purposes: it
    // crosses the boundary as exactly the same shape as T.
    const SCHEMA: &'static Schema = T::SCHEMA;
}

impl<T: WasmDescribe> WasmDescribe for Clamped<T> {
    const SCHEMA: &'static Schema = &Schema::node(SCHEMA_NODE_WRAP, &[CLAMPED], &[T::SCHEMA]);
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

/// Const-eval helpers used by the `#[wasm_bindgen]` macro when it emits
/// static descriptor entries into the `__wasm_bindgen_descriptors` custom
/// section.
///
/// All items here are `pub` because the macro expands code into the user's
/// crate that refers to them by absolute path. They are not part of the
/// public API.
pub mod schema {
    use wasm_bindgen_shared::{
        DESCRIPTOR_FORMAT_VERSION, DESCRIPTOR_KIND_CAST, DESCRIPTOR_KIND_REGULAR,
        DESCRIPTOR_KIND_STATIC,
    };

    /// Length in bytes of the body half of one descriptor section entry,
    /// i.e. everything that follows the outer `format_version` byte and
    /// `entry_body_byte_len` u32. Matches the format documented next to
    /// [`wasm_bindgen_shared::DESCRIPTORS_SECTION_NAME`]:
    ///
    /// ```text
    /// body bytes:
    ///   1 (shim_name_len)
    ///   + name_len            // shim_name UTF-8 bytes
    ///   + 1 (kind)
    ///   + 4 (schema_word_count u32 LE)
    ///   + word_count * 4      // schema (u32 LE per word)
    /// ```
    pub const fn entry_body_byte_len(name_len: usize, word_count: usize) -> usize {
        1 + name_len + 1 + 4 + word_count * 4
    }

    /// Total length in bytes of a packed descriptor section entry,
    /// including the outer per-entry framing (version + body length).
    /// This is the size of the `[u8; N]` static the macro emits.
    pub const fn entry_byte_len(name_len: usize, word_count: usize) -> usize {
        // 1 byte format_version + 4 bytes body_byte_len + body
        1 + 4 + entry_body_byte_len(name_len, word_count)
    }

    /// Pack one full descriptor entry (including its per-entry framing)
    /// into a fixed-size byte array.
    ///
    /// `B` must equal [`entry_byte_len`] for the given `name` and
    /// `schema_words.len()`. The macro picks the right value at
    /// expansion time.
    ///
    /// `schema_words` is a type's flattened schema opcode stream
    /// (i.e. `&flatten_into::<N>(<T>::SCHEMA)` for some `WasmDescribe`
    /// impl, where `N == flatten_len(<T>::SCHEMA)`).
    ///
    /// Layout:
    ///
    /// ```text
    /// out[0]                       = DESCRIPTOR_FORMAT_VERSION
    /// out[1..5]                    = body_byte_len (u32 LE)
    /// out[5..5 + body_byte_len]    = body (see entry_body_byte_len)
    /// ```
    pub const fn pack_entry<const B: usize>(
        name: &[u8],
        kind: u8,
        schema_words: &[u32],
    ) -> [u8; B] {
        let mut out = [0u8; B];
        let mut i = 0;

        // ----- per-entry framing -----
        out[i] = DESCRIPTOR_FORMAT_VERSION;
        i += 1;

        let body_len = entry_body_byte_len(name.len(), schema_words.len()) as u32;
        let body_len_bytes = body_len.to_le_bytes();
        out[i] = body_len_bytes[0];
        out[i + 1] = body_len_bytes[1];
        out[i + 2] = body_len_bytes[2];
        out[i + 3] = body_len_bytes[3];
        i += 4;

        // ----- body -----

        // shim_name_len (u8). `name.len()` is bounded by 255 because the
        // wire format only allots one byte for it; this assert catches
        // accidental over-long names at compile time.
        assert!(
            name.len() <= 255,
            "descriptor shim name longer than 255 bytes"
        );
        out[i] = name.len() as u8;
        i += 1;

        // shim name bytes
        let mut j = 0;
        while j < name.len() {
            out[i] = name[j];
            i += 1;
            j += 1;
        }

        // kind byte (sanity: must be one of the known constants).
        assert!(
            kind == DESCRIPTOR_KIND_REGULAR
                || kind == DESCRIPTOR_KIND_CAST
                || kind == DESCRIPTOR_KIND_STATIC,
            "unknown descriptor entry kind"
        );
        out[i] = kind;
        i += 1;

        // schema word count (u32 LE)
        let wc = schema_words.len() as u32;
        let wc_bytes = wc.to_le_bytes();
        out[i] = wc_bytes[0];
        out[i + 1] = wc_bytes[1];
        out[i + 2] = wc_bytes[2];
        out[i + 3] = wc_bytes[3];
        i += 4;

        // schema words (each u32 LE)
        let mut k = 0;
        while k < schema_words.len() {
            let b = schema_words[k].to_le_bytes();
            out[i] = b[0];
            out[i + 1] = b[1];
            out[i + 2] = b[2];
            out[i + 3] = b[3];
            i += 4;
            k += 1;
        }

        // We must have used every byte of the array.
        assert!(
            i == B,
            "entry size mismatch: B is wrong for this name/schema"
        );
        out
    }
}
