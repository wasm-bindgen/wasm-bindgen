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
use core::pin::Pin;
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
unsafe impl SerializedDescriptor for char {}

pub struct FnPtr(pub *const ());

unsafe impl SerializedDescriptor for FnPtr {}

unsafe impl Sync for FnPtr {}

unsafe impl<const N: usize, T: SerializedDescriptor> SerializedDescriptor for [T; N] {}

macro_rules! compose_serialized_descriptor {
    ($name:ident $( [$($decl_header:tt)*] [$($usage_header:tt)*] )? $(where [ $($where:tt)* ])? { $($field:ident : $ty:ty),* $(,)? } $impl:tt) => {
        #[repr(C)]
        pub struct $name $(<$($decl_header)*>)? $(where $($where)*)? {
            $($field: $ty),*
        }

        unsafe impl $(<$($decl_header)*>)? SerializedDescriptor for $name $(<$($usage_header)*>)?
        where
            // $($ty: SerializedDescriptor,)*
            $($($where)*)?
        {}

        impl $(<$($decl_header)*>)? $name $(<$($usage_header)*>)? $(where $($where)*)? $impl
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
    TaggedDescriptor
    [T: ?Sized + WasmDescribe]
    [T]
    {
        tag: u32,
        descriptor: T::Descriptor,
    }
    {
        pub const fn new(tag: u32) -> Self {
            Self {
                tag,
                descriptor: T::DESCRIPTOR,
            }
        }
    }
);

impl<T: WasmDescribe> WasmDescribe for [T] {
    type Descriptor = TaggedDescriptor<T>;

    const DESCRIPTOR: Self::Descriptor = Self::Descriptor::new(SLICE);
}

impl<T: WasmDescribe + ?Sized> WasmDescribe for &T {
    type Descriptor = TaggedDescriptor<T>;

    const DESCRIPTOR: Self::Descriptor = Self::Descriptor::new(REF);
}

impl<T: WasmDescribe + ?Sized> WasmDescribe for Pin<&T> {
    type Descriptor = TaggedDescriptor<T>;

    const DESCRIPTOR: Self::Descriptor = Self::Descriptor::new(LONGREF);
}

impl<T: WasmDescribe + ?Sized> WasmDescribe for &mut T {
    type Descriptor = TaggedDescriptor<T>;

    const DESCRIPTOR: Self::Descriptor = Self::Descriptor::new(REFMUT);
}

impl WasmDescribeVector for JsValue {
    type VectorDescriptor = TaggedDescriptor<JsValue>;

    const VECTOR_DESCRIPTOR: Self::VectorDescriptor = Self::VectorDescriptor::new(VECTOR);
}

impl<T: JsObject> WasmDescribeVector for T {
    type VectorDescriptor = TaggedDescriptor<T>;

    const VECTOR_DESCRIPTOR: Self::VectorDescriptor = Self::VectorDescriptor::new(VECTOR);
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

    const DESCRIPTOR: Self::Descriptor = Self::Descriptor::new(OPTIONAL);
}

impl<T: WasmDescribe, E: Into<JsValue>> WasmDescribe for Result<T, E> {
    type Descriptor = TaggedDescriptor<T>;

    const DESCRIPTOR: Self::Descriptor = Self::Descriptor::new(RESULT);
}

impl<T: WasmDescribe> WasmDescribe for MaybeUninit<T> {
    type Descriptor = T::Descriptor;

    const DESCRIPTOR: Self::Descriptor = T::DESCRIPTOR;
}

impl<T: WasmDescribe> WasmDescribe for Clamped<T> {
    type Descriptor = TaggedDescriptor<T>;

    const DESCRIPTOR: Self::Descriptor = Self::Descriptor::new(CLAMPED);
}

impl WasmDescribe for JsError {
    type Descriptor = <JsValue as WasmDescribe>::Descriptor;

    const DESCRIPTOR: Self::Descriptor = <JsValue as WasmDescribe>::DESCRIPTOR;
}

pub trait ArgsDescriptor: SerializedDescriptor {
    const COUNT: u32;
    const VALUE: Self;
}

compose_serialized_descriptor!(
    FuncDescriptor
    [F: ArgsDescriptor, R: WasmDescribe, RetInner: ReturnWasmAbi]
    [F, R, RetInner]
    {
        tag: u32,
        invoke_fn: FnPtr,
        count: u32,
        args: F,
        ret: R::Descriptor,
        ret_inner: RetInner::Descriptor,
    }
    {
        pub const fn new(invoke_fn: *const ()) -> Self {
            Self {
                tag: FUNCTION,
                invoke_fn: FnPtr(invoke_fn),
                count: F::COUNT,
                args: F::VALUE,
                ret: R::DESCRIPTOR,
                ret_inner: RetInner::DESCRIPTOR,
            }
        }

        pub const SHIM: Self = Self::new(std::ptr::null());
    }
);

compose_serialized_descriptor!(
    ClosureDescriptor
    [F: ?Sized + WasmClosure]
    [F]
    {
        tag: u32,
        dtor_fn: FnPtr,
        func: F::Descriptor,
        mutable: u32,
    }
    {
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
            dtor_fn: FnPtr(Self::destroy as _),
            func: F::DESCRIPTOR,
            mutable: F::IS_MUT as u32,
        };
    }
);

compose_serialized_descriptor!(
    SerializedName
    [const N: usize]
    [N]
    {
        len: u32,
        name: [char; N],
    }
    {
        pub const fn new(name: [char; N]) -> Self {
            Self { len: N as u32, name }
        }
    }
);

compose_serialized_descriptor!(
    ExportedDescriptor
    [const N: usize, T]
    [N, T]
    {
        name: SerializedName<N>,
        descriptor: T,
    }
    {
        pub const fn new(name: [char; N], descriptor: T) -> Self {
            Self {
                name: SerializedName::new(name),
                descriptor,
            }
        }
    }
);

compose_serialized_descriptor!(
    NamedExternRef
    [const N: usize]
    [N]
    {
        tag: u32,
        name: SerializedName<N>,
    }
    {
        pub const fn new(name: [char; N]) -> Self {
            Self {
                tag: EXTERNREF,
                name: SerializedName::new(name),
            }
        }
    }
);

compose_serialized_descriptor!(
    RustStruct
    [const N: usize]
    [N]
    {
        tag: u32,
        name: SerializedName<N>,
    }
    {
        pub const fn new(name: [char; N]) -> Self {
            Self {
                tag: RUST_STRUCT,
                name: SerializedName::new(name),
            }
        }
    }
);

compose_serialized_descriptor!(
    StringEnum
    [const N: usize]
    [N]
    {
        tag: u32,
        name: SerializedName<N>,
        invalid: u32,
        hole: u32,
    }
    {
        pub const fn new(name: [char; N], invalid: u32, hole: u32) -> Self {
            Self {
                tag: STRING_ENUM,
                name: SerializedName::new(name),
                invalid,
                hole,
            }
        }
    }
);

compose_serialized_descriptor!(
    Enum
    [const N: usize]
    [N]
    {
        tag: u32,
        name: SerializedName<N>,
        hole: u32,
    }
    {
        pub const fn new(name: [char; N], hole: u32) -> Self {
            Self {
                tag: ENUM,
                name: SerializedName::new(name),
                hole,
            }
        }
    }
);
