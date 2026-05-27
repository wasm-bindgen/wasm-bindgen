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

/// Maximum number of `u32` schema opcodes any single type's schema
/// may occupy. The descriptor system carries every schema as a fixed-
/// size `[u32; SCHEMA_MAX]` buffer paired with a length so generic
/// wrapper types (`Option<T>`, `Vec<T>`, `dyn Fn(A, B) -> R`, etc.)
/// can compose their schemas in `const` context without requiring
/// `feature(generic_const_exprs)` for array-length expressions.
///
/// 256 words = 1 KB per impl. `wasm-bindgen-cli` strips these
/// statics from the final wasm output after reading the descriptors
/// section, so per-impl size only affects pre-bindgen wasm. A 256-
/// word ceiling is comfortably above any closure / nested-generic
/// schema produced by real wasm-bindgen code; if a future schema
/// shape outgrows it, the const-fn assemblers below `assert!`-panic
/// at compile time with a clear message.
pub const SCHEMA_MAX: usize = 256;

/// Describes the wasm-bindgen type schema for a type.
///
/// The schema is carried as a pair of associated consts:
///
/// * [`Self::SCHEMA_LEN`] — the number of meaningful `u32` words in
///   the schema.
/// * [`Self::SCHEMA_BUF`] — a fixed-size `[u32; SCHEMA_MAX]` buffer
///   whose first `SCHEMA_LEN` words are the schema opcodes;
///   trailing words are zero-padded.
///
/// Wrapper impls (`Option<T>`, `Vec<T>`, `Result<T, E>`, closure
/// trait objects) compose their schema in a `const fn` that reads
/// the inner type's `(SCHEMA_LEN, SCHEMA_BUF)` pair and writes a
/// composed buffer. The fixed buffer size sidesteps the
/// `generic_const_exprs` wall (rust-lang/rust#76560) — we don't need
/// generic-length array expressions because every schema lives in
/// the same-sized buffer.
///
/// The `#[wasm_bindgen]` macro writes the meaningful prefix of
/// `SCHEMA_BUF` into the `__wasm_bindgen_descriptors` custom section
/// ([`wasm_bindgen_shared::DESCRIPTORS_SECTION_NAME`]) at compile
/// time. `wasm-bindgen-cli` reads those bytes structurally — no
/// wasm interpretation involved.
pub trait WasmDescribe {
    /// Number of meaningful `u32` words at the start of [`Self::SCHEMA_BUF`].
    const SCHEMA_LEN: usize;

    /// Fixed-size schema buffer. Only the first [`Self::SCHEMA_LEN`]
    /// words are meaningful; the rest is zero-padded.
    const SCHEMA_BUF: [u32; SCHEMA_MAX];
}

/// Trait for element types to implement `WasmDescribe` for vectors
/// of themselves. Same `(LEN, BUF)` shape as [`WasmDescribe`] but
/// describes the `Vec<Self>` / `Box<[Self]>` schema rather than
/// `Self`'s.
pub trait WasmDescribeVector {
    /// Number of meaningful `u32` words at the start of [`Self::VECTOR_SCHEMA_BUF`].
    const VECTOR_SCHEMA_LEN: usize;

    /// Fixed-size schema buffer for `Vec<Self>` / `Box<[Self]>`.
    /// Only the first [`Self::VECTOR_SCHEMA_LEN`] words are meaningful.
    const VECTOR_SCHEMA_BUF: [u32; SCHEMA_MAX];
}

/// Build a `SCHEMA_BUF` from a single opcode (for leaf types like
/// `i32`, `JsValue`, etc.).
pub const fn leaf_schema(op: u32) -> [u32; SCHEMA_MAX] {
    let mut buf = [0u32; SCHEMA_MAX];
    buf[0] = op;
    buf
}

/// Concatenate a header (in-line opcodes) with an inner schema's
/// meaningful prefix. Used by wrapper impls (`REF`, `OPTIONAL`,
/// `VECTOR`, etc.) to compose their schema from the inner type's
/// `(SCHEMA_LEN, SCHEMA_BUF)`.
pub const fn wrap_schema(
    header: &[u32],
    inner_buf: &[u32; SCHEMA_MAX],
    inner_len: usize,
) -> [u32; SCHEMA_MAX] {
    let total = header.len() + inner_len;
    assert!(
        total <= SCHEMA_MAX,
        "wasm-bindgen schema buffer overflow: a single type's schema exceeds \
         SCHEMA_MAX words. If you hit this, please file an issue describing \
         the type; the bound is generous (256 words) but not infinite."
    );
    let mut buf = [0u32; SCHEMA_MAX];
    let mut i = 0;
    while i < header.len() {
        buf[i] = header[i];
        i += 1;
    }
    let mut j = 0;
    while j < inner_len {
        buf[header.len() + j] = inner_buf[j];
        j += 1;
    }
    buf
}

/// Concatenate any number of meaningful schema slices (each carried
/// as a `(buf, len)` pair) into a single `SCHEMA_BUF`. Used by
/// closure-trait-object impls whose schema is built from a variadic
/// list of argument schemas plus duplicated return-type schemas.
pub const fn cat_schema(parts: &[(&[u32; SCHEMA_MAX], usize)]) -> [u32; SCHEMA_MAX] {
    let mut buf = [0u32; SCHEMA_MAX];
    let mut idx = 0;
    let mut p = 0;
    while p < parts.len() {
        let (b, l) = parts[p];
        let mut i = 0;
        while i < l {
            assert!(
                idx < SCHEMA_MAX,
                "wasm-bindgen schema buffer overflow: composed schema exceeds \
                 SCHEMA_MAX words. If you hit this, please file an issue."
            );
            buf[idx] = b[i];
            idx += 1;
            i += 1;
        }
        p += 1;
    }
    buf
}

/// Build a SCHEMA_BUF from a flat slice of opcode words. Used by the
/// `#[wasm_bindgen]` macro for emitted types whose schema is fully
/// known at macro-expansion time (struct, enum, string-enum, etc.).
pub const fn schema_from_slice(words: &[u32]) -> [u32; SCHEMA_MAX] {
    assert!(
        words.len() <= SCHEMA_MAX,
        "wasm-bindgen schema buffer overflow: literal schema exceeds SCHEMA_MAX words."
    );
    let mut buf = [0u32; SCHEMA_MAX];
    let mut i = 0;
    while i < words.len() {
        buf[i] = words[i];
        i += 1;
    }
    buf
}

macro_rules! simple {
    ($($t:ident => $d:ident)*) => ($(
        impl WasmDescribe for $t {
            const SCHEMA_LEN: usize = 1;
            const SCHEMA_BUF: [u32; SCHEMA_MAX] = leaf_schema($d);
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
    const SCHEMA_LEN: usize = 1;
    const SCHEMA_BUF: [u32; SCHEMA_MAX] = leaf_schema(RAW_POINTER);
}

impl<T> WasmDescribe for *mut T {
    const SCHEMA_LEN: usize = 1;
    const SCHEMA_BUF: [u32; SCHEMA_MAX] = leaf_schema(RAW_POINTER);
}

impl<T> WasmDescribe for NonNull<T> {
    const SCHEMA_LEN: usize = 1;
    const SCHEMA_BUF: [u32; SCHEMA_MAX] = leaf_schema(NONNULL);
}

impl<T: WasmDescribe> WasmDescribe for [T] {
    const SCHEMA_LEN: usize = 1 + T::SCHEMA_LEN;
    const SCHEMA_BUF: [u32; SCHEMA_MAX] = wrap_schema(&[SLICE], &T::SCHEMA_BUF, T::SCHEMA_LEN);
}

impl<T: WasmDescribe + ?Sized> WasmDescribe for &T {
    const SCHEMA_LEN: usize = 1 + T::SCHEMA_LEN;
    const SCHEMA_BUF: [u32; SCHEMA_MAX] = wrap_schema(&[REF], &T::SCHEMA_BUF, T::SCHEMA_LEN);
}

impl<T: WasmDescribe + ?Sized> WasmDescribe for &mut T {
    const SCHEMA_LEN: usize = 1 + T::SCHEMA_LEN;
    const SCHEMA_BUF: [u32; SCHEMA_MAX] = wrap_schema(&[REFMUT], &T::SCHEMA_BUF, T::SCHEMA_LEN);
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
// Now that every `WasmDescribe` impl carries a fixed-size
// `SCHEMA_BUF`, a single blanket impl could in principle return.
// However, retaining concrete impls keeps `Vec<UserStruct>`-style
// VECTOR_SCHEMA emissions inside the macro (which use
// `NAMED_EXTERNREF` rather than the struct's own `SCHEMA_BUF`)
// straightforward, and avoids unnecessary deep monomorphisation
// chains in the const evaluator.
impl WasmDescribeVector for JsValue {
    const VECTOR_SCHEMA_LEN: usize = 2;
    const VECTOR_SCHEMA_BUF: [u32; SCHEMA_MAX] = {
        let mut b = leaf_schema(VECTOR);
        b[1] = EXTERNREF;
        b
    };
}

impl WasmDescribeVector for JsError {
    const VECTOR_SCHEMA_LEN: usize = 2;
    const VECTOR_SCHEMA_BUF: [u32; SCHEMA_MAX] = {
        let mut b = leaf_schema(VECTOR);
        b[1] = EXTERNREF;
        b
    };
}

impl<T: WasmDescribeVector> WasmDescribe for Box<[T]> {
    const SCHEMA_LEN: usize = T::VECTOR_SCHEMA_LEN;
    const SCHEMA_BUF: [u32; SCHEMA_MAX] = T::VECTOR_SCHEMA_BUF;
}

impl<T> WasmDescribe for Vec<T>
where
    Box<[T]>: WasmDescribe,
{
    const SCHEMA_LEN: usize = <Box<[T]> as WasmDescribe>::SCHEMA_LEN;
    const SCHEMA_BUF: [u32; SCHEMA_MAX] = <Box<[T]> as WasmDescribe>::SCHEMA_BUF;
}

impl<T: WasmDescribe> WasmDescribe for Option<T> {
    const SCHEMA_LEN: usize = 1 + T::SCHEMA_LEN;
    const SCHEMA_BUF: [u32; SCHEMA_MAX] = wrap_schema(&[OPTIONAL], &T::SCHEMA_BUF, T::SCHEMA_LEN);
}

impl WasmDescribe for () {
    const SCHEMA_LEN: usize = 1;
    const SCHEMA_BUF: [u32; SCHEMA_MAX] = leaf_schema(UNIT);
}

impl<T: WasmDescribe, E: Into<JsValue>> WasmDescribe for Result<T, E> {
    const SCHEMA_LEN: usize = 1 + T::SCHEMA_LEN;
    const SCHEMA_BUF: [u32; SCHEMA_MAX] = wrap_schema(&[RESULT], &T::SCHEMA_BUF, T::SCHEMA_LEN);
}

impl<T: WasmDescribe> WasmDescribe for MaybeUninit<T> {
    // MaybeUninit<T> is transparent for descriptor purposes: it
    // crosses the boundary as exactly the same shape as T.
    const SCHEMA_LEN: usize = T::SCHEMA_LEN;
    const SCHEMA_BUF: [u32; SCHEMA_MAX] = T::SCHEMA_BUF;
}

impl<T: WasmDescribe> WasmDescribe for Clamped<T> {
    const SCHEMA_LEN: usize = 1 + T::SCHEMA_LEN;
    const SCHEMA_BUF: [u32; SCHEMA_MAX] = wrap_schema(&[CLAMPED], &T::SCHEMA_BUF, T::SCHEMA_LEN);
}

impl WasmDescribe for JsError {
    // JsError's schema is the same as JsValue's: a single EXTERNREF.
    const SCHEMA_LEN: usize = <JsValue as WasmDescribe>::SCHEMA_LEN;
    const SCHEMA_BUF: [u32; SCHEMA_MAX] = <JsValue as WasmDescribe>::SCHEMA_BUF;
}

impl<T> WasmDescribe for AssertUnwindSafe<T>
where
    T: WasmDescribe,
{
    const SCHEMA_LEN: usize = T::SCHEMA_LEN;
    const SCHEMA_BUF: [u32; SCHEMA_MAX] = T::SCHEMA_BUF;
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
    /// `schema_words` is the meaningful prefix of a type's `SCHEMA_BUF`
    /// (i.e. `&SCHEMA_BUF[..SCHEMA_LEN]` for some `WasmDescribe` impl).
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
