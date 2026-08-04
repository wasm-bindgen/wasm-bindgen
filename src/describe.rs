//! This is an internal module, no stability guarantees are provided. Use at
//! your own risk.

#![doc(hidden)]

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::panic::AssertUnwindSafe;
use core::{mem::MaybeUninit, ptr::NonNull};

use crate::{__rt::marker::ErasableGeneric, Clamped, JsError, JsValue};
use cfg_if::cfg_if;

pub use wasm_bindgen_shared::tys::*;

#[inline(always)] // see the wasm-interpreter module
#[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
pub fn inform(a: u32) {
    unsafe { super::__wbindgen_describe(a) }
}

/// Marker terminating a per-monomorphisation descriptor function. See
/// `__wbindgen_describe_generic_import` in the crate root. Used both for
/// generic imports and, with an empty shim key, for `__rt::wbg_cast` identity
/// adapters. `func` is the monomorphised shim's own pointer and `prims` points
/// at its ABI arguments; both exist only to keep inputs live across the opaque
/// FFI boundary so the descriptor survives to be interpreted by the CLI.
///
/// Must be `#[inline(always)]` (like [`inform`]) so the marker call lands
/// directly inside each monomorphised shim; the CLI's discovery pass scans for
/// functions that *directly* call the marker import.
///
/// # Safety
///
/// This is an FFI call into an import that the CLI replaces wholesale, so
/// nothing here is checked. The caller must guarantee all of:
///
/// * It is called from an `#[inline(never)]`, monomorphised descriptor shim
///   whose body is a descriptor stream, and exactly once in that shim. If the
///   call is inlined into a caller, or two monomorphisations end up in one Wasm
///   function, the CLI's discovery pass either misses the descriptor or binds
///   only the first one and silently mis-binds the rest.
/// * `prims` points at the shim's own live ABI arguments (a
///   `*const (Prim1, ..)`), so that they are kept alive across this opaque
///   boundary and cannot be eliminated as dead code before the descriptor is
///   emitted.
/// * `func` is the monomorphised shim's own function pointer, again only to keep
///   it live.
/// * The returned pointer is *not* a valid pointer to dereference as such. It is
///   only ever valid to `core::ptr::read` it at exactly the `Abi` type the shim
///   declares as its return type — that is what the CLI's rewritten call site
///   provides. Reading it at any other type, or dereferencing it directly, is
///   undefined behaviour.
///
/// Note that unlike [`crate::convert`], this module does not
/// `#![allow(clippy::missing_safety_doc)]`, so this section is required.
#[inline(always)]
#[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
pub unsafe fn describe_generic_import(func: *const (), prims: *const ()) -> *const () {
    super::__wbindgen_describe_generic_import(func, prims)
}

pub trait WasmDescribe {
    fn describe();
}

/// Trait for element types to implement WasmDescribe for vectors of
/// themselves.
pub trait WasmDescribeVector {
    fn describe_vector();
}

macro_rules! simple {
    ($($t:ident => $d:ident)*) => ($(
        impl WasmDescribe for $t {
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
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(RAW_POINTER)
    }
}

impl<T> WasmDescribe for *mut T {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(RAW_POINTER)
    }
}

impl<T> WasmDescribe for NonNull<T> {
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

impl<T: ErasableGeneric<Repr = JsValue> + WasmDescribe> WasmDescribeVector for T {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe_vector() {
        inform(VECTOR);
        T::describe();
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
