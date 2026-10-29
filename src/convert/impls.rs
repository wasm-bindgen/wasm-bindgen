use alloc::boxed::Box;
use alloc::vec::Vec;
use core::char;
use core::mem::{self, ManuallyDrop};
use core::ptr::NonNull;

use crate::__rt::marker::ErasableGeneric;
use crate::__rt::{WasmSignedWordRepr, WasmWordRepr};
use crate::convert::traits::{WasmAbi, WasmPrimitive};
use crate::convert::{
    FromWasmAbi, IntoWasmAbi, LongRefFromWasmAbi, MaybeFromWasmAbi, MaybeIntoWasmAbi,
    NullableFromWasmAbi, NullableIntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi, RefFromWasmAbi,
    ReturnWasmAbi, TryFromJsValue, UpcastFrom,
};
use crate::sys::Promising;
use crate::sys::{JsNullable, JsOption, Undefined};
use crate::{throw_str, Clamped, JsError, JsValue, Nullable};

// Primitive types can always be passed over the ABI.
impl<T: WasmPrimitive> WasmAbi for T {
    type Prim1 = Self;
    type Prim2 = ();
    type Prim3 = ();
    type Prim4 = ();

    #[inline]
    fn split(self) -> (Self, (), (), ()) {
        (self, (), (), ())
    }

    #[inline]
    fn join(prim: Self, _: (), _: (), _: ()) -> Self {
        prim
    }
}

impl WasmAbi for i128 {
    type Prim1 = u64;
    type Prim2 = u64;
    type Prim3 = ();
    type Prim4 = ();

    #[inline]
    fn split(self) -> (u64, u64, (), ()) {
        let low = self as u64;
        let high = (self >> 64) as u64;
        (low, high, (), ())
    }

    #[inline]
    fn join(low: u64, high: u64, _: (), _: ()) -> Self {
        (((high as u128) << 64) | low as u128) as i128
    }
}
impl WasmAbi for u128 {
    type Prim1 = u64;
    type Prim2 = u64;
    type Prim3 = ();
    type Prim4 = ();

    #[inline]
    fn split(self) -> (u64, u64, (), ()) {
        let low = self as u64;
        let high = (self >> 64) as u64;
        (low, high, (), ())
    }

    #[inline]
    fn join(low: u64, high: u64, _: (), _: ()) -> Self {
        ((high as u128) << 64) | low as u128
    }
}

/// The ABI of `Option<T>`, `Nullable<T>` and `Option<Nullable<T>>` for a `T`
/// whose own ABI has no spare value: `tag` is `0` for `None`, `1` for a value
/// and `2` for `Null`.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct MaybeAbi<T> {
    pub tag: u32,
    pub value: T,
}

impl<T: WasmAbi<Prim4 = ()>> WasmAbi for MaybeAbi<T> {
    type Prim1 = u32;
    type Prim2 = T::Prim1;
    type Prim3 = T::Prim2;
    type Prim4 = T::Prim3;

    #[inline]
    fn split(self) -> (u32, T::Prim1, T::Prim2, T::Prim3) {
        let (prim1, prim2, prim3, ()) = self.value.split();
        (self.tag, prim1, prim2, prim3)
    }

    #[inline]
    fn join(tag: u32, prim1: T::Prim1, prim2: T::Prim2, prim3: T::Prim3) -> Self {
        MaybeAbi {
            tag,
            value: T::join(prim1, prim2, prim3, ()),
        }
    }
}

macro_rules! type_wasm_native {
    ($($t:tt as $c:tt)*) => ($(
        impl IntoWasmAbi for $t {
            type Abi = $c;

            #[inline]
            fn into_abi(self) -> $c { self as $c }
        }

        impl FromWasmAbi for $t {
            type Abi = $c;

            #[inline]
            unsafe fn from_abi(js: $c) -> Self { js as $t }
        }

        impl MaybeIntoWasmAbi for $t {
            type MaybeAbi = MaybeAbi<$c>;

            #[inline]
            fn into_maybe_abi(self) -> MaybeAbi<$c> {
                MaybeAbi { tag: 1, value: self as $c }
            }

            #[inline]
            fn none() -> MaybeAbi<$c> {
                MaybeAbi { tag: 0, value: 0 as $c }
            }
        }

        impl NullableIntoWasmAbi for $t {
            #[inline]
            fn null() -> MaybeAbi<$c> {
                MaybeAbi { tag: 2, value: 0 as $c }
            }
        }

        impl MaybeFromWasmAbi for $t {
            type MaybeAbi = MaybeAbi<$c>;

            #[inline]
            unsafe fn from_maybe_abi(js: MaybeAbi<$c>) -> Self {
                js.value as $t
            }

            #[inline]
            fn is_none(js: &MaybeAbi<$c>) -> bool {
                js.tag == 0
            }
        }

        impl NullableFromWasmAbi for $t {
            #[inline]
            fn is_null(js: &MaybeAbi<$c>) -> bool {
                js.tag == 2
            }
        }

        impl UpcastFrom<$t> for JsValue {}
        impl UpcastFrom<$t> for JsOption<JsValue> {}
        impl UpcastFrom<$t> for JsNullable<JsValue> {}
        impl UpcastFrom<$t> for $t {}
        impl<Target: UpcastFrom<$t>> UpcastFrom<$t> for Option<Target> {}
        impl<Target: UpcastFrom<$t>> UpcastFrom<$t> for Nullable<Target> {}
    )*)
}

type_wasm_native!(
    i64 as i64
    u64 as u64
    i128 as i128
    u128 as u128
    f64 as f64
);

impl UpcastFrom<u64> for u128 {}
impl UpcastFrom<u64> for JsOption<u128> {}
impl UpcastFrom<u64> for JsNullable<u128> {}
impl UpcastFrom<i64> for i128 {}
impl UpcastFrom<i64> for JsOption<i128> {}
impl UpcastFrom<i64> for JsNullable<i128> {}

/// Sentinel value used to encode `None` for optional pointer-sized and
/// 32-bit numeric values transferred over the JS `number` ABI.
///
/// `2^53 - 1` (`Number.MAX_SAFE_INTEGER`) is chosen because it is:
/// - exactly representable as an `f64` (so JS round-trips it losslessly),
/// - outside the range of any valid `i32`/`u32`/`f32` value (so it can't
///   collide with a real `Some(...)` payload from those types), and
/// - far above any plausible wasm64 pointer (which is bounded by the
///   memory-64 address space limit, well below `2^53`).
const F64_ABI_OPTION_SENTINEL: f64 = 9007199254740991_f64;

/// `Nullable::Null` over the JS `number` ABI: the safe integer below
/// [`F64_ABI_OPTION_SENTINEL`], outside every value range for the same
/// reasons.
#[doc(hidden)]
pub const F64_ABI_NULL_SENTINEL: f64 = 9007199254740990_f64;

macro_rules! type_wasm_native_f64_option {
    ($($t:tt as $c:tt)*) => ($(
        impl IntoWasmAbi for $t {
            type Abi = $c;

            #[inline]
            fn into_abi(self) -> $c { self as $c }
        }

        impl FromWasmAbi for $t {
            type Abi = $c;

            #[inline]
            unsafe fn from_abi(js: $c) -> Self { js as $t }
        }

        unsafe impl ErasableGeneric for $t {
            type Repr = $t;
        }

        impl Promising for $t {
            type Resolution = $t;
        }

        impl MaybeIntoWasmAbi for $t {
            type MaybeAbi = f64;

            #[inline]
            fn into_maybe_abi(self) -> f64 {
                self as $c as f64
            }

            #[inline]
            fn none() -> f64 {
                F64_ABI_OPTION_SENTINEL
            }
        }

        impl NullableIntoWasmAbi for $t {
            #[inline]
            fn null() -> f64 {
                F64_ABI_NULL_SENTINEL
            }
        }

        impl MaybeFromWasmAbi for $t {
            type MaybeAbi = f64;

            #[inline]
            unsafe fn from_maybe_abi(js: f64) -> Self {
                js as $c as $t
            }

            #[inline]
            fn is_none(js: &f64) -> bool {
                *js == F64_ABI_OPTION_SENTINEL
            }
        }

        impl NullableFromWasmAbi for $t {
            #[inline]
            fn is_null(js: &f64) -> bool {
                *js == F64_ABI_NULL_SENTINEL
            }
        }

        impl UpcastFrom<$t> for JsValue {}
        impl UpcastFrom<$t> for JsOption<JsValue> {}
        impl UpcastFrom<$t> for JsNullable<JsValue> {}
        impl UpcastFrom<$t> for $t {}
        impl<Target: UpcastFrom<$t>> UpcastFrom<$t> for Option<Target> {}
        impl<Target: UpcastFrom<$t>> UpcastFrom<$t> for Nullable<Target> {}
    )*)
}

type_wasm_native_f64_option!(
    i32 as i32
    u32 as u32
    f32 as f32
    isize as WasmSignedWordRepr
    usize as WasmWordRepr
);

#[cfg(target_pointer_width = "32")]
impl UpcastFrom<isize> for i32 {}
#[cfg(target_pointer_width = "32")]
impl UpcastFrom<isize> for JsOption<i32> {}
#[cfg(target_pointer_width = "32")]
impl UpcastFrom<isize> for JsNullable<i32> {}

impl UpcastFrom<isize> for i64 {}
impl UpcastFrom<isize> for JsOption<i64> {}
impl UpcastFrom<isize> for JsNullable<i64> {}
impl UpcastFrom<isize> for i128 {}
impl UpcastFrom<isize> for JsOption<i128> {}
impl UpcastFrom<isize> for JsNullable<i128> {}

impl UpcastFrom<i32> for isize {}
impl UpcastFrom<i32> for JsOption<isize> {}
impl UpcastFrom<i32> for JsNullable<isize> {}
impl UpcastFrom<i32> for i64 {}
impl UpcastFrom<i32> for JsOption<i64> {}
impl UpcastFrom<i32> for JsNullable<i64> {}
impl UpcastFrom<i32> for i128 {}
impl UpcastFrom<i32> for JsOption<i128> {}
impl UpcastFrom<i32> for JsNullable<i128> {}

impl UpcastFrom<u32> for usize {}
impl UpcastFrom<u32> for JsOption<usize> {}
impl UpcastFrom<u32> for JsNullable<usize> {}
impl UpcastFrom<u32> for u64 {}
impl UpcastFrom<u32> for JsOption<u64> {}
impl UpcastFrom<u32> for JsNullable<u64> {}
impl UpcastFrom<u32> for u128 {}
impl UpcastFrom<u32> for JsOption<u128> {}
impl UpcastFrom<u32> for JsNullable<u128> {}

#[cfg(target_pointer_width = "32")]
impl UpcastFrom<usize> for u32 {}
#[cfg(target_pointer_width = "32")]
impl UpcastFrom<usize> for JsOption<u32> {}
#[cfg(target_pointer_width = "32")]
impl UpcastFrom<usize> for JsNullable<u32> {}
impl UpcastFrom<usize> for u64 {}
impl UpcastFrom<usize> for JsOption<u64> {}
impl UpcastFrom<usize> for JsNullable<u64> {}
impl UpcastFrom<usize> for u128 {}
impl UpcastFrom<usize> for JsOption<u128> {}
impl UpcastFrom<usize> for JsNullable<u128> {}

impl UpcastFrom<f32> for f64 {}
impl UpcastFrom<f32> for JsOption<f64> {}
impl UpcastFrom<f32> for JsNullable<f64> {}

/// The sentinel value is 0xFF_FFFF for primitives with less than 32 bits.
///
/// This value is used, so all small primitive types (`bool`, `i8`, `u8`,
/// `i16`, `u16`, `char`) can use the same JS glue code. `char::MAX` is
/// 0x10_FFFF btw.
const U32_ABI_OPTION_SENTINEL: u32 = 0x00FF_FFFFu32;

/// `Nullable::Null` for the same types, below the `Option` sentinel.
const U32_ABI_NULL_SENTINEL: u32 = 0x00FF_FFFEu32;

macro_rules! type_abi_as_u32 {
    ($($t:tt)*) => ($(
        impl IntoWasmAbi for $t {
            type Abi = u32;

            #[inline]
            fn into_abi(self) -> u32 { self as u32 }
        }

        impl FromWasmAbi for $t {
            type Abi = u32;

            #[inline]
            unsafe fn from_abi(js: u32) -> Self { js as $t }
        }

        impl OptionIntoWasmAbi for $t {
            #[inline]
            fn none() -> u32 { U32_ABI_OPTION_SENTINEL }
        }

        impl OptionFromWasmAbi for $t {
            #[inline]
            fn is_none(js: &u32) -> bool { *js == U32_ABI_OPTION_SENTINEL }
        }

        impl_nullable_into_shared_abi!(null = U32_ABI_NULL_SENTINEL; [] $t);
        impl_nullable_from_shared_abi!(is_null = |js| *js == U32_ABI_NULL_SENTINEL; [] $t);

        unsafe impl ErasableGeneric for $t {
            type Repr = $t;
        }

        impl Promising for $t {
            type Resolution = $t;
        }

        impl UpcastFrom<$t> for JsValue {}
        impl UpcastFrom<$t> for JsOption<JsValue> {}
        impl UpcastFrom<$t> for JsNullable<JsValue> {}
        impl UpcastFrom<$t> for $t {}
        impl<Target: UpcastFrom<$t>> UpcastFrom<$t> for Option<Target> {}
        impl<Target: UpcastFrom<$t>> UpcastFrom<$t> for Nullable<Target> {}
    )*)
}

type_abi_as_u32!(i8 u8 i16 u16);

impl UpcastFrom<i8> for i16 {}
impl UpcastFrom<i8> for JsOption<i16> {}
impl UpcastFrom<i8> for JsNullable<i16> {}
impl UpcastFrom<i8> for i32 {}
impl UpcastFrom<i8> for JsOption<i32> {}
impl UpcastFrom<i8> for JsNullable<i32> {}
impl UpcastFrom<i8> for i64 {}
impl UpcastFrom<i8> for JsOption<i64> {}
impl UpcastFrom<i8> for JsNullable<i64> {}
impl UpcastFrom<i8> for i128 {}
impl UpcastFrom<i8> for JsOption<i128> {}
impl UpcastFrom<i8> for JsNullable<i128> {}

impl UpcastFrom<u8> for u16 {}
impl UpcastFrom<u8> for JsOption<u16> {}
impl UpcastFrom<u8> for JsNullable<u16> {}
impl UpcastFrom<u8> for u32 {}
impl UpcastFrom<u8> for JsOption<u32> {}
impl UpcastFrom<u8> for JsNullable<u32> {}
impl UpcastFrom<u8> for u64 {}
impl UpcastFrom<u8> for JsOption<u64> {}
impl UpcastFrom<u8> for JsNullable<u64> {}
impl UpcastFrom<u8> for u128 {}
impl UpcastFrom<u8> for JsOption<u128> {}
impl UpcastFrom<u8> for JsNullable<u128> {}

impl UpcastFrom<i16> for i32 {}
impl UpcastFrom<i16> for JsOption<i32> {}
impl UpcastFrom<i16> for JsNullable<i32> {}
impl UpcastFrom<i16> for i64 {}
impl UpcastFrom<i16> for JsOption<i64> {}
impl UpcastFrom<i16> for JsNullable<i64> {}
impl UpcastFrom<i16> for i128 {}
impl UpcastFrom<i16> for JsOption<i128> {}
impl UpcastFrom<i16> for JsNullable<i128> {}

impl UpcastFrom<u16> for u32 {}
impl UpcastFrom<u16> for JsOption<u32> {}
impl UpcastFrom<u16> for JsNullable<u32> {}
impl UpcastFrom<u16> for u64 {}
impl UpcastFrom<u16> for JsOption<u64> {}
impl UpcastFrom<u16> for JsNullable<u64> {}
impl UpcastFrom<u16> for u128 {}
impl UpcastFrom<u16> for JsOption<u128> {}
impl UpcastFrom<u16> for JsNullable<u128> {}

impl IntoWasmAbi for bool {
    type Abi = u32;

    #[inline]
    fn into_abi(self) -> u32 {
        self as u32
    }
}

impl FromWasmAbi for bool {
    type Abi = u32;

    #[inline]
    unsafe fn from_abi(js: u32) -> bool {
        js != 0
    }
}

impl OptionIntoWasmAbi for bool {
    #[inline]
    fn none() -> u32 {
        U32_ABI_OPTION_SENTINEL
    }
}

impl OptionFromWasmAbi for bool {
    #[inline]
    fn is_none(js: &u32) -> bool {
        *js == U32_ABI_OPTION_SENTINEL
    }
}

impl_nullable_into_shared_abi!(null = U32_ABI_NULL_SENTINEL; [] bool);
impl_nullable_from_shared_abi!(is_null = |js| *js == U32_ABI_NULL_SENTINEL; [] bool);

unsafe impl ErasableGeneric for bool {
    type Repr = bool;
}

impl Promising for bool {
    type Resolution = bool;
}

impl UpcastFrom<bool> for JsValue {}
impl UpcastFrom<bool> for JsOption<JsValue> {}
impl UpcastFrom<bool> for JsNullable<JsValue> {}
impl UpcastFrom<bool> for bool {}
impl<Target: UpcastFrom<bool>> UpcastFrom<bool> for Option<Target> {}
impl<Target: UpcastFrom<bool>> UpcastFrom<bool> for Nullable<Target> {}

impl IntoWasmAbi for char {
    type Abi = u32;

    #[inline]
    fn into_abi(self) -> u32 {
        self as u32
    }
}

impl FromWasmAbi for char {
    type Abi = u32;

    #[inline]
    unsafe fn from_abi(js: u32) -> char {
        // SAFETY: Checked in bindings.
        char::from_u32_unchecked(js)
    }
}

impl OptionIntoWasmAbi for char {
    #[inline]
    fn none() -> u32 {
        U32_ABI_OPTION_SENTINEL
    }
}

impl OptionFromWasmAbi for char {
    #[inline]
    fn is_none(js: &u32) -> bool {
        *js == U32_ABI_OPTION_SENTINEL
    }
}

impl_nullable_into_shared_abi!(null = U32_ABI_NULL_SENTINEL; [] char);
impl_nullable_from_shared_abi!(is_null = |js| *js == U32_ABI_NULL_SENTINEL; [] char);

unsafe impl ErasableGeneric for char {
    type Repr = char;
}

impl Promising for char {
    type Resolution = char;
}

impl UpcastFrom<char> for JsValue {}
impl UpcastFrom<char> for JsOption<JsValue> {}
impl UpcastFrom<char> for JsNullable<JsValue> {}
impl UpcastFrom<char> for char {}
impl<Target: UpcastFrom<char>> UpcastFrom<char> for Option<Target> {}
impl<Target: UpcastFrom<char>> UpcastFrom<char> for Nullable<Target> {}

impl<T> IntoWasmAbi for *const T {
    type Abi = WasmWordRepr;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self as usize as WasmWordRepr
    }
}

impl<T> FromWasmAbi for *const T {
    type Abi = WasmWordRepr;

    #[inline]
    unsafe fn from_abi(js: Self::Abi) -> *const T {
        js as usize as *const T
    }
}

unsafe impl<T: ErasableGeneric> ErasableGeneric for *const T {
    type Repr = *const T::Repr;
}

impl<T, Target> UpcastFrom<*const T> for *const Target where Target: UpcastFrom<T> {}
impl<T, Target> UpcastFrom<*const T> for JsOption<*const Target> where Target: UpcastFrom<T> {}
impl<T, Target> UpcastFrom<*const T> for JsNullable<*const Target> where Target: UpcastFrom<T> {}

unsafe impl<T: ErasableGeneric> ErasableGeneric for Option<T> {
    type Repr = Option<<T as ErasableGeneric>::Repr>;
}

impl<T, Target> UpcastFrom<Option<T>> for Option<Target> where Target: UpcastFrom<T> {}
impl<T, Target> UpcastFrom<Option<T>> for JsOption<Option<Target>> where Target: UpcastFrom<T> {}
impl<T, Target> UpcastFrom<Option<T>> for JsNullable<Option<Target>> where Target: UpcastFrom<T> {}
// `None` is `undefined`, a JS value, so an optional carrier upcasts to `JsValue`
// whenever its payload does, as each scalar and string carrier does.
impl<T> UpcastFrom<Option<T>> for JsValue where JsValue: UpcastFrom<T> {}

/// Pointers in a `Nullable` cross as JS numbers with the `f64` sentinels, as
/// they do in an `Option`.
macro_rules! nullable_ptr_abi {
    ($($t:ty),*) => ($(
        impl<T> MaybeIntoWasmAbi for $t {
            type MaybeAbi = f64;

            #[inline]
            fn into_maybe_abi(self) -> f64 {
                self as usize as f64
            }

            #[inline]
            fn none() -> f64 {
                F64_ABI_OPTION_SENTINEL
            }
        }

        impl<T> NullableIntoWasmAbi for $t {
            #[inline]
            fn null() -> f64 {
                F64_ABI_NULL_SENTINEL
            }
        }

        impl<T> MaybeFromWasmAbi for $t {
            type MaybeAbi = f64;

            #[inline]
            unsafe fn from_maybe_abi(js: f64) -> Self {
                js as usize as $t
            }

            #[inline]
            fn is_none(js: &f64) -> bool {
                *js == F64_ABI_OPTION_SENTINEL
            }
        }

        impl<T> NullableFromWasmAbi for $t {
            #[inline]
            fn is_null(js: &f64) -> bool {
                *js == F64_ABI_NULL_SENTINEL
            }
        }
    )*)
}

nullable_ptr_abi!(*const T, *mut T);

impl<T> IntoWasmAbi for *mut T {
    type Abi = WasmWordRepr;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self as usize as WasmWordRepr
    }
}

impl<T> FromWasmAbi for *mut T {
    type Abi = WasmWordRepr;

    #[inline]
    unsafe fn from_abi(js: Self::Abi) -> *mut T {
        js as usize as *mut T
    }
}

impl<T> IntoWasmAbi for NonNull<T> {
    type Abi = WasmWordRepr;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self.as_ptr() as usize as WasmWordRepr
    }
}

impl<T> OptionIntoWasmAbi for NonNull<T> {
    #[inline]
    fn none() -> Self::Abi {
        0 as WasmWordRepr
    }
}

impl<T> FromWasmAbi for NonNull<T> {
    type Abi = WasmWordRepr;

    #[inline]
    unsafe fn from_abi(js: Self::Abi) -> Self {
        // SAFETY: Checked in bindings.
        NonNull::new_unchecked(js as usize as *mut T)
    }
}

impl<T> OptionFromWasmAbi for NonNull<T> {
    #[inline]
    fn is_none(js: &Self::Abi) -> bool {
        *js == 0 as WasmWordRepr
    }
}

impl IntoWasmAbi for JsValue {
    type Abi = u32;

    #[inline]
    fn into_abi(self) -> u32 {
        let ret = self.idx;
        mem::forget(self);
        ret
    }
}

impl FromWasmAbi for JsValue {
    type Abi = u32;

    #[inline]
    unsafe fn from_abi(js: u32) -> JsValue {
        JsValue::_new(js)
    }
}

impl IntoWasmAbi for &JsValue {
    type Abi = u32;

    #[inline]
    fn into_abi(self) -> u32 {
        self.idx
    }
}

impl RefFromWasmAbi for JsValue {
    type Abi = u32;
    type Anchor = ManuallyDrop<JsValue>;

    #[inline]
    unsafe fn ref_from_abi(js: u32) -> Self::Anchor {
        ManuallyDrop::new(JsValue::_new(js))
    }
}

impl LongRefFromWasmAbi for JsValue {
    type Abi = u32;
    type Anchor = JsValue;

    #[inline]
    unsafe fn long_ref_from_abi(js: u32) -> Self::Anchor {
        Self::from_abi(js)
    }
}

impl OptionIntoWasmAbi for JsValue {
    #[inline]
    fn none() -> u32 {
        crate::__rt::JSIDX_UNDEFINED
    }
}

impl OptionIntoWasmAbi for &JsValue {
    #[inline]
    fn none() -> u32 {
        crate::__rt::JSIDX_UNDEFINED
    }
}

impl OptionFromWasmAbi for JsValue {
    #[inline]
    fn is_none(js: &u32) -> bool {
        unsafe { Self::ref_from_abi(*js) }.is_undefined()
    }
}

impl_nullable_into_shared_abi!(null = crate::__rt::JSIDX_NULL; [] JsValue);
impl_nullable_into_shared_abi!(null = crate::__rt::JSIDX_NULL; [] &JsValue);
// The glue hands `null` over as the fixed handle, so this is an index test
// rather than a call into JS; a `null` that did arrive in its own slot is an
// owned `Value` and is dropped as one.
impl_nullable_from_shared_abi!(is_null = |js| *js == crate::__rt::JSIDX_NULL; [] JsValue);

/// A spare value in `T`'s own ABI is the common case, so `OptionIntoWasmAbi`
/// supplies the `Option<T>` ABI for every such type; a type without one
/// implements `MaybeIntoWasmAbi` itself with a wider `MaybeAbi`.
impl<T: OptionIntoWasmAbi> MaybeIntoWasmAbi for T {
    type MaybeAbi = T::Abi;

    #[inline]
    fn into_maybe_abi(self) -> T::Abi {
        self.into_abi()
    }

    #[inline]
    fn none() -> T::Abi {
        <T as OptionIntoWasmAbi>::none()
    }
}

impl<T: OptionFromWasmAbi> MaybeFromWasmAbi for T {
    type MaybeAbi = T::Abi;

    #[inline]
    unsafe fn from_maybe_abi(js: T::Abi) -> Self {
        T::from_abi(js)
    }

    #[inline]
    unsafe fn try_from_maybe_abi(js: T::Abi) -> Result<Self, &'static str> {
        T::try_from_abi(js)
    }

    #[inline]
    fn is_none(js: &T::Abi) -> bool {
        <T as OptionFromWasmAbi>::is_none(js)
    }
}

impl<T: MaybeIntoWasmAbi> IntoWasmAbi for Option<T> {
    type Abi = T::MaybeAbi;

    #[inline]
    fn into_abi(self) -> T::MaybeAbi {
        match self {
            None => T::none(),
            Some(me) => me.into_maybe_abi(),
        }
    }
}

impl<T: MaybeFromWasmAbi> FromWasmAbi for Option<T> {
    type Abi = T::MaybeAbi;

    #[inline]
    unsafe fn from_abi(js: T::MaybeAbi) -> Self {
        if T::is_none(&js) {
            None
        } else {
            Some(T::from_maybe_abi(js))
        }
    }

    #[inline]
    unsafe fn try_from_abi(js: T::MaybeAbi) -> Result<Self, &'static str> {
        if T::is_none(&js) {
            Ok(None)
        } else {
            T::try_from_maybe_abi(js).map(Some)
        }
    }
}

impl<T: NullableIntoWasmAbi> IntoWasmAbi for Nullable<T> {
    type Abi = T::MaybeAbi;

    #[inline]
    fn into_abi(self) -> T::MaybeAbi {
        match self {
            Nullable::Null => T::null(),
            Nullable::Value(me) => me.into_maybe_abi(),
        }
    }
}

/// `Option<Nullable<T>>` is `None` for `T`'s `undefined` ABI value.
impl<T: NullableIntoWasmAbi> OptionIntoWasmAbi for Nullable<T> {
    #[inline]
    fn none() -> T::MaybeAbi {
        T::none()
    }
}

impl<T: NullableFromWasmAbi> FromWasmAbi for Nullable<T> {
    type Abi = T::MaybeAbi;

    #[inline]
    unsafe fn from_abi(js: T::MaybeAbi) -> Self {
        if T::is_null(&js) {
            Nullable::Null
        } else {
            Nullable::Value(T::from_maybe_abi(js))
        }
    }

    #[inline]
    unsafe fn try_from_abi(js: T::MaybeAbi) -> Result<Self, &'static str> {
        if T::is_null(&js) {
            Ok(Nullable::Null)
        } else {
            T::try_from_maybe_abi(js).map(Nullable::Value)
        }
    }
}

/// `T`'s `undefined` test may cover its `null` sentinel too, as a slice's
/// does with the null pointer, so `Null` is ruled out first.
impl<T: NullableFromWasmAbi> OptionFromWasmAbi for Nullable<T> {
    #[inline]
    fn is_none(js: &T::MaybeAbi) -> bool {
        !T::is_null(js) && T::is_none(js)
    }
}

unsafe impl<T: ErasableGeneric> ErasableGeneric for Nullable<T> {
    type Repr = Nullable<<T as ErasableGeneric>::Repr>;
}

impl<T, Target> UpcastFrom<Nullable<T>> for Nullable<Target> where Target: UpcastFrom<T> {}
// An `Option<T>` reads `null` as `None`, so every value of a `Nullable<T>` is one
// of an `Option<T>`. This is the only `Nullable` to `Option` rule coherence
// allows, so `Nullable<T>` reaches `Option<Nullable<T>>` through it once `T`
// wraps into `Nullable`.
impl<T, Target> UpcastFrom<Nullable<T>> for Option<Target> where Target: UpcastFrom<T> {}
impl<T, Target> UpcastFrom<Nullable<T>> for JsOption<Nullable<Target>> where Target: UpcastFrom<T> {}
impl<T, Target> UpcastFrom<Nullable<T>> for JsNullable<Nullable<Target>> where Target: UpcastFrom<T> {}
impl<T> UpcastFrom<Nullable<T>> for JsValue where JsValue: UpcastFrom<T> {}

impl<T: Promising> Promising for Option<T> {
    type Resolution = Option<<T as Promising>::Resolution>;
}

impl<T: Promising> Promising for Nullable<T> {
    type Resolution = Nullable<<T as Promising>::Resolution>;
}

impl<T: IntoWasmAbi> IntoWasmAbi for Clamped<T> {
    type Abi = T::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self.0.into_abi()
    }
}

impl<T: FromWasmAbi> FromWasmAbi for Clamped<T> {
    type Abi = T::Abi;

    #[inline]
    unsafe fn from_abi(js: T::Abi) -> Self {
        Clamped(T::from_abi(js))
    }

    #[inline]
    unsafe fn try_from_abi(js: T::Abi) -> Result<Self, &'static str> {
        T::try_from_abi(js).map(Clamped)
    }
}

impl IntoWasmAbi for () {
    type Abi = ();

    #[inline]
    fn into_abi(self) {
        self
    }
}

impl FromWasmAbi for () {
    type Abi = ();

    #[inline]
    unsafe fn from_abi(_js: ()) {}
}

impl Promising for () {
    type Resolution = Undefined;
}

impl UpcastFrom<()> for JsValue {}
impl UpcastFrom<()> for () {}
// `()` crosses as `undefined`, which an `Option<T>` reads as `None`.
impl<T> UpcastFrom<()> for Option<T> {}

unsafe impl ErasableGeneric for () {
    type Repr = ();
}

impl<T: WasmAbi<Prim3 = (), Prim4 = ()>> WasmAbi for Result<T, u32> {
    type Prim1 = T::Prim1;
    type Prim2 = T::Prim2;
    // The order of primitives here is such that we can pop() the possible error
    // first, deal with it and move on. Later primitives are popped off the
    // stack first.
    /// If this `Result` is an `Err`, the error value.
    type Prim3 = u32;
    /// Whether this `Result` is an `Err`.
    type Prim4 = u32;

    #[inline]
    fn split(self) -> (T::Prim1, T::Prim2, u32, u32) {
        match self {
            Ok(value) => {
                let (prim1, prim2, (), ()) = value.split();
                (prim1, prim2, 0, 0)
            }
            Err(err) => (Default::default(), Default::default(), err, 1),
        }
    }

    #[inline]
    fn join(prim1: T::Prim1, prim2: T::Prim2, err: u32, is_err: u32) -> Self {
        if is_err == 0 {
            Ok(T::join(prim1, prim2, (), ()))
        } else {
            Err(err)
        }
    }
}

impl<T, E> ReturnWasmAbi for Result<T, E>
where
    T: IntoWasmAbi,
    E: Into<JsValue>,
    T::Abi: WasmAbi<Prim3 = (), Prim4 = ()>,
{
    type Abi = Result<T::Abi, u32>;

    #[inline]
    fn return_abi(self) -> Self::Abi {
        match self {
            Ok(v) => Ok(v.into_abi()),
            Err(e) => {
                let jsval = e.into();
                Err(jsval.into_abi())
            }
        }
    }
}

unsafe impl<T: ErasableGeneric, E: ErasableGeneric> ErasableGeneric for Result<T, E> {
    type Repr = Result<<T as ErasableGeneric>::Repr, <E as ErasableGeneric>::Repr>;
}

impl<T: Promising, E> Promising for Result<T, E> {
    type Resolution = Result<<T as Promising>::Resolution, E>;
}

impl<T, E, TargetT, TargetE> UpcastFrom<Result<T, E>> for Result<TargetT, TargetE>
where
    TargetT: UpcastFrom<T>,
    TargetE: UpcastFrom<E>,
{
}
impl<T, E, TargetT, TargetE> UpcastFrom<Result<T, E>> for JsOption<Result<TargetT, TargetE>>
where
    TargetT: UpcastFrom<T>,
    TargetE: UpcastFrom<E>,
{
}
impl<T, E, TargetT, TargetE> UpcastFrom<Result<T, E>> for JsNullable<Result<TargetT, TargetE>>
where
    TargetT: UpcastFrom<T>,
    TargetE: UpcastFrom<E>,
{
}

unsafe impl ErasableGeneric for JsError {
    type Repr = JsValue;
}

impl IntoWasmAbi for JsError {
    type Abi = <JsValue as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        self.value.into_abi()
    }
}

// `JsError` is `#[repr(transparent)]` over `JsValue`
impl FromWasmAbi for JsError {
    type Abi = <JsValue as FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(js: Self::Abi) -> Self {
        JsError {
            value: JsValue::from_abi(js),
        }
    }
}

impl Promising for JsError {
    type Resolution = JsError;
}

impl UpcastFrom<JsError> for JsValue {}
impl UpcastFrom<JsError> for JsOption<JsValue> {}
impl UpcastFrom<JsError> for JsNullable<JsValue> {}
impl UpcastFrom<JsError> for JsError {}

/// The compat form of an owned vector: an in-place sequence of `JsValue`
/// handles, converted to or from `T` in Rust. The released `Vec<String>`,
/// `Vec`s of exported structs and numeric enums, and third-party impls cross
/// this way. It has no new uses: an element whose single value crosses
/// without calling into wasm is declared as an encoded sequence instead.
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::convert) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
// Note: this can't take `&[T]` because the `Into<JsValue>` impl needs
// ownership of `T`.
pub fn js_value_vector_into_abi<T: Into<JsValue>>(
    vector: Box<[T]>,
) -> <Box<[JsValue]> as IntoWasmAbi>::Abi {
    let js_vals: Box<[JsValue]> = vector.into_vec().into_iter().map(|x| x.into()).collect();

    js_vals.into_abi()
}

/// The mirror of [`js_value_vector_into_abi`], checking each element with
/// [`TryFromJsValue`].
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::convert) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
pub unsafe fn js_value_vector_from_abi<T: TryFromJsValue>(
    js: <Box<[JsValue]> as FromWasmAbi>::Abi,
) -> Box<[T]> {
    // Thrown directly, rather than inside the iterator chain or a closure, to
    // keep the function that took the `Vec` within the debug-mode error
    // stack's frame limit.
    match try_js_value_vector_from_abi(js) {
        Ok(result) => result,
        Err(msg) => throw_str(msg),
    }
}

/// The fallible form of [`js_value_vector_from_abi`].
///
/// Collecting stops at the first element of the wrong type and drops
/// everything Rust owns: the buffer, the handles not yet converted, the
/// rejected handle and the elements converted so far.
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::convert) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
pub unsafe fn try_js_value_vector_from_abi<T: TryFromJsValue>(
    js: <Box<[JsValue]> as FromWasmAbi>::Abi,
) -> Result<Box<[T]>, &'static str> {
    <Vec<JsValue> as FromWasmAbi>::from_abi(js)
        .into_iter()
        .map(|value| T::try_from_js_value(value).map_err(|_| crate::__rt::WRONG_TYPE_ERROR))
        .collect()
}
