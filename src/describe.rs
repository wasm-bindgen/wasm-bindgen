//! This is an internal module, no stability guarantees are provided. Use at
//! your own risk.

#![doc(hidden)]

use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::panic::AssertUnwindSafe;
use core::{mem::MaybeUninit, ptr::NonNull};

use crate::{__rt::marker::ErasableGeneric, Clamped, JsError, JsValue, Nullable};
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
    /// Whether a `[Self]` crosses in place, as the buffer it already is.
    ///
    /// A `Vec<Self>`, `Box<[Self]>` or `&[Self]` crosses as a buffer of each
    /// element's ABI, unless this is set, when its own memory is passed
    /// instead.
    #[doc(hidden)]
    const ABI_TRANSPARENT: AbiTransparent<Self> = AbiTransparent::NO;

    fn describe();
}

/// Whether a `[T]` crosses in place. It names `T`, invariantly, so that one
/// type's can't stand in for another's, a subtype's included, and is set
/// only through [`AbiTransparent::yes`].
#[doc(hidden)]
pub struct AbiTransparent<T: ?Sized>(bool, PhantomData<fn(*const T) -> *const T>);

impl<T: ?Sized> AbiTransparent<T> {
    pub const NO: Self = Self(false, PhantomData);

    /// # Safety
    ///
    /// This is a layout promise: `T` is, in memory, the element JS reads in
    /// place for its descriptor, a number of the descriptor's width or a JS
    /// handle's index, and every such element JS writes is a valid `T`.
    pub const unsafe fn yes() -> Self {
        Self(true, PhantomData)
    }

    #[inline]
    pub(crate) const fn get(self) -> bool {
        self.0
    }
}

/// Trait for element types to implement WasmDescribe for vectors of
/// themselves.
///
/// Soft-deprecated and unused: a vector is described by its element's
/// [`WasmDescribe`]. It will be removed in 0.3.0.
pub trait WasmDescribeVector {
    fn describe_vector();
}

macro_rules! simple {
    ($($t:ident => $d:ident)*) => (simple!(@false $($t => $d)*););
    (@$transparent:literal $($t:ident => $d:ident)*) => ($(
        impl WasmDescribe for $t {
            const ABI_TRANSPARENT: AbiTransparent<Self> = AbiTransparent($transparent, PhantomData);

            #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
            fn describe() { inform($d) }
        }
    )*)
}

// A typed array's element, or a handle's index.
simple! {
    @true
    i8 => I8
    u8 => U8
    i16 => I16
    u16 => U16
    i32 => I32
    u32 => U32
    i64 => I64
    u64 => U64
    f32 => F32
    f64 => F64
    JsValue => EXTERNREF
}

simple! {
    i128 => I128
    u128 => U128
    bool => BOOLEAN
    char => CHAR
}

// isize/usize map to I32/U32 on wasm32 and direct *_AS_F64 descriptors on wasm64
cfg_if! {
    if #[cfg(target_arch = "wasm64")] {
        simple! {
            @true
            isize => I64_AS_F64
            usize => U64_AS_F64
        }
    } else {
        simple! {
            @true
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

/// An element that isn't in place crosses as its ABI, which `ABI_CELL` tells
/// the CLI, so it never reads an element's memory as its descriptor's.
impl<T: WasmDescribe> WasmDescribe for [T] {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(SLICE);
        if !T::ABI_TRANSPARENT.get() {
            inform(ABI_CELL);
        }
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

// Pointees for `&Box<str>` and `&Cow<str>`, described like `str`.
impl WasmDescribe for Box<str> {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        str::describe();
    }
}

impl WasmDescribe for Cow<'_, str> {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        str::describe();
    }
}

impl<T: ErasableGeneric<Repr = JsValue> + WasmDescribe> WasmDescribeVector for T {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe_vector() {
        inform(VECTOR);
        T::describe();
    }
}

impl<T: WasmDescribe> WasmDescribe for Box<[T]> {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(VECTOR);
        if !T::ABI_TRANSPARENT.get() {
            inform(ABI_CELL);
        }
        T::describe();
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

impl<T: WasmDescribe> WasmDescribe for Nullable<T> {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(NULLABLE);
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
    const ABI_TRANSPARENT: AbiTransparent<Self> = AbiTransparent(T::ABI_TRANSPARENT.0, PhantomData);

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
    const ABI_TRANSPARENT: AbiTransparent<Self> = AbiTransparent(true, PhantomData);

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

/// Describes an import argument whose wire is described by `T` under a
/// `slice = ..` or `vec = ..` mode word (see `SEQ_MODE_*`). The CLI applies
/// the mode to sequences of its key's shape with typed elements. Prefixed
/// because generated describe code glob-imports this module, where a plain
/// `SeqMode` would shadow a user type of that name.
pub struct WasmDescribeSeqMode<T: ?Sized, const MODE: u32>(core::marker::PhantomData<T>);

impl<T: WasmDescribe + ?Sized, const MODE: u32> WasmDescribe for WasmDescribeSeqMode<T, MODE> {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(SEQ_MODE);
        inform(MODE);
        T::describe();
    }
}
