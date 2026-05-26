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

#[inline(always)] // see the wasm-interpreter module
#[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
pub fn inform(a: u32) {
    unsafe { super::__wbindgen_describe(a) }
}

/// Describes the wasm-bindgen type schema for a type.
///
/// Two parallel descriptions exist on this trait:
///
/// 1. [`Self::SCHEMA`]: the primary mechanism. A `&'static [u32]`
///    slice of opcodes, available at `const` time and used by the
///    `#[wasm_bindgen]` macro to write descriptors directly into the
///    `__wasm_bindgen_descriptors` custom section
///    ([`wasm_bindgen_shared::DESCRIPTORS_SECTION_NAME`]).
/// 2. [`Self::describe`]: the legacy mechanism. Calls [`inform`] for
///    each opcode, which would historically be recovered by a wasm
///    interpreter executing a synthetic `__wbindgen_describe_<name>`
///    export function. The macro no longer emits those exports;
///    `describe()` remains live only for closure-cast descriptor
///    recovery (see `crates/cli-support/src/interpreter/`).
///
/// Implementing types should provide `SCHEMA` whenever it can be
/// expressed as a non-generic-length const. Generic wrapper impls
/// (`&T`, `Option<T>`, `Vec<T>`, `Result<T, E>`, etc.) cannot — their
/// schema length depends on `T::SCHEMA.len()`, which requires
/// `feature(generic_const_exprs)` (rust-lang/rust#76560, nightly
/// only) in array-length position. Those impls leave `SCHEMA` at the
/// default empty slice. The macro pattern-matches those wrapper
/// shapes syntactically and synthesises the schema from the inner
/// type's `SCHEMA` at each call site, where the lengths are concrete.
///
/// When `generic_const_exprs` stabilises, the wrapper impls below
/// gain proper `SCHEMA` consts and the macro's syntactic dispatch in
/// `schema_parts_for_type` simplifies.
pub trait WasmDescribe {
    fn describe();

    /// Schema opcode stream for this type. Defaults to an empty
    /// slice; see the trait docs for how `SCHEMA` and `describe`
    /// relate and why some impls leave it empty.
    const SCHEMA: &'static [u32] = &[];
}

/// Trait for element types to implement `WasmDescribe` for vectors
/// of themselves.
pub trait WasmDescribeVector {
    fn describe_vector();

    /// Section-transport schema for `Vec<Self>` / `Box<[Self]>`:
    /// `[VECTOR, ...Self::SCHEMA]` for most element types,
    /// `[VECTOR, NAMED_EXTERNREF, name_len, ...name chars]` for
    /// user-defined types that cross the boundary as JS handles.
    ///
    /// Default is `&[]`. Concrete impls set this per element type
    /// (see explicit impls for `JsValue` / `JsError` /
    /// `MaybeUninit<T>` etc. below, plus macro-emitted impls for user
    /// structs and ImportTypes). The repetition is a workaround for
    /// the `generic_const_exprs` wall — a single blanket impl would
    /// suffice if `[u32; T::SCHEMA.len() + 1]` were expressible on
    /// stable.
    const VECTOR_SCHEMA: &'static [u32] = &[];
}

macro_rules! simple {
    ($($t:ident => $d:ident)*) => ($(
        impl WasmDescribe for $t {
            const SCHEMA: &'static [u32] = &[$d];
            #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
            fn describe() { inform($d) }
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
    const SCHEMA: &'static [u32] = &[RAW_POINTER];
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(RAW_POINTER)
    }
}

impl<T> WasmDescribe for *mut T {
    const SCHEMA: &'static [u32] = &[RAW_POINTER];
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(RAW_POINTER)
    }
}

impl<T> WasmDescribe for NonNull<T> {
    const SCHEMA: &'static [u32] = &[NONNULL];
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(NONNULL)
    }
}

impl<T: WasmDescribe> WasmDescribe for [T] {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(SLICE);
        T::describe();
    }
}

impl<T: WasmDescribe + ?Sized> WasmDescribe for &T {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(REF);
        T::describe();
    }
}

impl<T: WasmDescribe + ?Sized> WasmDescribe for &mut T {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(REFMUT);
        T::describe();
    }
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
// The blanket impl was removed because it could not populate
// `VECTOR_SCHEMA = &[VECTOR, T::SCHEMA...]` at const time — the
// length depends on `T::SCHEMA.len()`, an associated-const-derived
// array length that requires `feature(generic_const_exprs)`
// (rust-lang/rust#76560, nightly-only, no stable timeline). Each
// `ErasableGeneric<Repr=JsValue>` type now provides its own impl
// with a concrete `VECTOR_SCHEMA` so the section transport carries
// `Vec<Self>` arguments without falling back to the interpreter.
//
// When `generic_const_exprs` stabilises, the blanket impl can return
// and the per-type repetition below can collapse to a single
// generic line. The macro-emitted ImportType impls
// (`crates/macro-support/src/codegen.rs`) would also collapse.
impl WasmDescribeVector for JsValue {
    const VECTOR_SCHEMA: &'static [u32] = &[VECTOR, EXTERNREF];
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe_vector() {
        inform(VECTOR);
        JsValue::describe();
    }
}

impl WasmDescribeVector for JsError {
    const VECTOR_SCHEMA: &'static [u32] = &[VECTOR, EXTERNREF];
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe_vector() {
        inform(VECTOR);
        JsError::describe();
    }
}

impl<T: WasmDescribeVector> WasmDescribe for Box<[T]> {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        T::describe_vector();
    }
}

impl<T> WasmDescribe for Vec<T>
where
    Box<[T]>: WasmDescribe,
{
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        <Box<[T]>>::describe();
    }
}

impl<T: WasmDescribe> WasmDescribe for Option<T> {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(OPTIONAL);
        T::describe();
    }
}

impl WasmDescribe for () {
    const SCHEMA: &'static [u32] = &[UNIT];
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(UNIT)
    }
}

impl<T: WasmDescribe, E: Into<JsValue>> WasmDescribe for Result<T, E> {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(RESULT);
        T::describe();
    }
}

impl<T: WasmDescribe> WasmDescribe for MaybeUninit<T> {
    // MaybeUninit<T> is transparent for descriptor purposes: it
    // crosses the boundary as exactly the same shape as T. Mirror
    // that in both transports.
    const SCHEMA: &'static [u32] = T::SCHEMA;
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        T::describe();
    }
}

impl<T: WasmDescribe> WasmDescribe for Clamped<T> {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(CLAMPED);
        T::describe();
    }
}

impl WasmDescribe for JsError {
    // JsError's schema is the same as JsValue's: a single EXTERNREF.
    const SCHEMA: &'static [u32] = <JsValue as WasmDescribe>::SCHEMA;
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        JsValue::describe();
    }
}

impl<T> WasmDescribe for AssertUnwindSafe<T>
where
    T: WasmDescribe,
{
    fn describe() {
        T::describe();
    }
}

/// Const-eval helpers used by the `#[wasm_bindgen]` macro when it emits
/// static descriptor entries into the `__wasm_bindgen_descriptors` custom
/// section.
///
/// All items here are `pub` because the macro expands code into the user's
/// crate that refers to them by absolute path. They are not part of the
/// public API; treat as `doc(hidden)` even though we don't slap that on
/// every item.
pub mod schema {
    use wasm_bindgen_shared::{
        DESCRIPTOR_FORMAT_VERSION, DESCRIPTOR_KIND_CAST, DESCRIPTOR_KIND_REGULAR,
        DESCRIPTOR_KIND_STATIC,
    };

    /// Total length, in `u32` words, of a list of opcode slices when
    /// concatenated. Used by the macro to size the static array storing
    /// the final schema stream for one descriptor entry.
    pub const fn word_total(parts: &[&[u32]]) -> usize {
        let mut total = 0;
        let mut i = 0;
        while i < parts.len() {
            total += parts[i].len();
            i += 1;
        }
        total
    }

    /// Concatenate a list of opcode slices into a single fixed-size `u32`
    /// array. `N` must equal [`word_total`] of `parts` (the macro picks
    /// the right value because at expansion time the lengths are
    /// concrete).
    pub const fn concat_words<const N: usize>(parts: &[&[u32]]) -> [u32; N] {
        let mut out = [0u32; N];
        let mut idx = 0;
        let mut p = 0;
        while p < parts.len() {
            let part = parts[p];
            let mut j = 0;
            while j < part.len() {
                out[idx] = part[j];
                idx += 1;
                j += 1;
            }
            p += 1;
        }
        out
    }

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
