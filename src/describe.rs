//! This is an internal module, no stability guarantees are provided. Use at
//! your own risk.

#![doc(hidden)]

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::{mem::MaybeUninit, ptr::NonNull};

use crate::convert::ReturnWasmAbi;
use crate::{Clamped, JsError, JsObject, JsValue};

use crate::closure::WasmClosure;
pub use wasm_bindgen_shared::tys::*;

const MAYBE_CACHED_STRING: u32 = if cfg!(feature = "enable-interning") {
    CACHED_STRING
} else {
    STRING
};

/// # Safety
///
/// Implementors of this trait must have a memory layout that is compatible
/// with `[u32; N]` for some value of N. This means the type must be
/// representable as a contiguous array of `u32` values with no padding
/// or other layout differences.
pub unsafe trait SerializedDescriptor {}

unsafe impl SerializedDescriptor for u32 {}

unsafe impl SerializedDescriptor for *const () {}

unsafe impl<const N: usize> SerializedDescriptor for [u32; N] {}

macro_rules! compose_serialized_descriptor {
    ([ $name:ident $($header:tt)* ] [ $($where:tt)* ] { $($field:ident : $ty:ty),* $(,)? }) => {
        #[repr(C)]
        pub struct $name $($header)* where $($where)* {
            $($field: $ty),*
        }

        unsafe impl $($header)* SerializedDescriptor for $name $($header)*
        where
            $($ty: SerializedDescriptor,)*
            $($where)*
        {}
    };
}

pub trait WasmDescribe {
    type Descriptor: SerializedDescriptor;

    const DESCRIPTOR: Self::Descriptor;
}

/// Trait for element types to implement WasmDescribe for vectors of
/// themselves.
pub trait WasmDescribeVector {
    type VectorDescriptor: SerializedDescriptor;

    const VECTOR_DESCRIPTOR: Self::VectorDescriptor;
}

macro_rules! simple {
    ($($t:ty => $d:ident)*) => ($(
        impl WasmDescribe for $t {
            type Descriptor = u32;

            const DESCRIPTOR: Self::Descriptor = $d;
        }
    )*)
}

simple! {
    () => UNIT
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
    isize => I32
    usize => U32
    f32 => F32
    f64 => F64
    bool => BOOLEAN
    char => CHAR
    JsValue => EXTERNREF
    str => MAYBE_CACHED_STRING
    String => MAYBE_CACHED_STRING
}

impl<T> WasmDescribe for *const T {
    type Descriptor = u32;

    const DESCRIPTOR: Self::Descriptor = U32;
}

impl<T> WasmDescribe for *mut T {
    type Descriptor = u32;

    const DESCRIPTOR: Self::Descriptor = U32;
}

impl<T> WasmDescribe for NonNull<T> {
    type Descriptor = u32;

    const DESCRIPTOR: Self::Descriptor = U32;
}

compose_serialized_descriptor!(
    [TaggedDescriptor<T>]
    [T: ?Sized + WasmDescribe]
    {
        tag: u32,
        inner: T::Descriptor,
    }
);

impl<T: ?Sized + WasmDescribe> TaggedDescriptor<T> {
    pub const fn new(tag: u32) -> Self {
        Self {
            tag,
            inner: T::DESCRIPTOR,
        }
    }
}

impl<T: WasmDescribe> WasmDescribe for [T] {
    type Descriptor = TaggedDescriptor<T>;

    const DESCRIPTOR: Self::Descriptor = TaggedDescriptor::new(SLICE);
}

impl<T: WasmDescribe + ?Sized> WasmDescribe for &T {
    type Descriptor = TaggedDescriptor<T>;

    const DESCRIPTOR: Self::Descriptor = TaggedDescriptor::new(REF);
}

impl<T: WasmDescribe + ?Sized> WasmDescribe for &mut T {
    type Descriptor = TaggedDescriptor<T>;

    const DESCRIPTOR: Self::Descriptor = TaggedDescriptor::new(REFMUT);
}

impl WasmDescribeVector for JsValue {
    type VectorDescriptor = TaggedDescriptor<JsValue>;

    const VECTOR_DESCRIPTOR: Self::VectorDescriptor = TaggedDescriptor::new(VECTOR);
}

impl<T: JsObject> WasmDescribeVector for T {
    type VectorDescriptor = TaggedDescriptor<T>;

    const VECTOR_DESCRIPTOR: Self::VectorDescriptor = TaggedDescriptor::new(VECTOR);
}

impl<T: WasmDescribeVector> WasmDescribe for Box<[T]> {
    type Descriptor = T::VectorDescriptor;

    const DESCRIPTOR: Self::Descriptor = T::VECTOR_DESCRIPTOR;
}

impl<T> WasmDescribe for Vec<T>
where
    Box<[T]>: WasmDescribe,
{
    type Descriptor = <Box<[T]> as WasmDescribe>::Descriptor;

    const DESCRIPTOR: Self::Descriptor = <Box<[T]> as WasmDescribe>::DESCRIPTOR;
}

impl<T: WasmDescribe> WasmDescribe for Option<T> {
    type Descriptor = TaggedDescriptor<T>;

    const DESCRIPTOR: Self::Descriptor = TaggedDescriptor::new(OPTIONAL);
}

impl<T: WasmDescribe, E: Into<JsValue>> WasmDescribe for Result<T, E> {
    type Descriptor = TaggedDescriptor<T>;

    const DESCRIPTOR: Self::Descriptor = TaggedDescriptor::new(RESULT);
}

impl<T: WasmDescribe> WasmDescribe for MaybeUninit<T> {
    type Descriptor = T::Descriptor;

    const DESCRIPTOR: Self::Descriptor = T::DESCRIPTOR;
}

impl<T: WasmDescribe> WasmDescribe for Clamped<T> {
    type Descriptor = TaggedDescriptor<T>;

    const DESCRIPTOR: Self::Descriptor = TaggedDescriptor::new(CLAMPED);
}

impl WasmDescribe for JsError {
    type Descriptor = <JsValue as WasmDescribe>::Descriptor;

    const DESCRIPTOR: Self::Descriptor = <JsValue as WasmDescribe>::DESCRIPTOR;
}

compose_serialized_descriptor!(
    [FuncDescriptor<A, R, RetInner>]
    [A: SerializedDescriptor, R: WasmDescribe, RetInner: ReturnWasmAbi]
    {
        tag: u32,
        invoke_fn: *const (),
        args: A,
        ret: R::Descriptor,
        inner: RetInner::Descriptor,
    }
);

impl<A: SerializedDescriptor, R: WasmDescribe, RetInner: ReturnWasmAbi>
    FuncDescriptor<A, R, RetInner>
{
    pub const fn new(invoke_fn: *const (), args: A) -> Self {
        Self {
            tag: FUNCTION,
            invoke_fn,
            args,
            ret: R::DESCRIPTOR,
            inner: RetInner::DESCRIPTOR,
        }
    }
}

compose_serialized_descriptor!(
    [ClosureDescriptor<F>]
    [F: ?Sized + WasmClosure]
    {
        tag: u32,
        dtor_fn: *const (),
        func: F::Descriptor,
        mutable: u32,
    }
);

impl<F: ?Sized + WasmClosure> ClosureDescriptor<F> {
    unsafe extern "C" fn destroy(a: usize, b: usize) {
        // This can be called by the JS glue in erroneous situations
        // such as when the closure has already been destroyed. If
        // that's the case let's not make things worse by
        // segfaulting and/or asserting, so just ignore null
        // pointers.
        if a == 0 {
            return;
        }
        drop(Box::from_raw(core::mem::transmute_copy::<_, *mut F>(&(
            a, b,
        ))));
    }

    pub const VALUE: Self = Self {
        tag: CLOSURE,
        dtor_fn: Self::destroy as *const (),
        func: F::DESCRIPTOR,
        mutable: F::IS_MUT as u32,
    };
}
