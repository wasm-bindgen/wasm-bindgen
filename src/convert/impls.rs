use alloc::boxed::Box;
use alloc::vec::Vec;
use core::char;
use core::fmt::Debug;
use core::mem::{self, ManuallyDrop};
use core::ptr::NonNull;

use crate::convert::traits::{WasmAbi, WasmPrimitive};
use crate::convert::TryFromJsValue;
use crate::convert::{FromWasmAbi, IntoWasmAbi, LongRefFromWasmAbi, RefFromWasmAbi};
use crate::convert::{OptionFromWasmAbi, OptionIntoWasmAbi, ReturnWasmAbi};
use crate::{Clamped, JsError, JsValue, UnwrapThrowExt};

// Any `WasmPrimitive` or a tuple of primitives (up to 4) should be a `WasmAbi`.
macro_rules! wasm_abi_tuple {
    ($($prim:ident)* | $($unbound:ident)*) => {
        #[allow(non_snake_case, unused_parens)]
        impl<$($prim: WasmPrimitive),*> WasmAbi for ($($prim),*) {
            $(type $prim = $prim;)*
            $(type $unbound = ();)*

            #[inline]
            fn split(self) -> (Self::Prim1, Self::Prim2, Self::Prim3, Self::Prim4) {
                let ($($prim),*) = self;
                $(let $unbound = ();)*
                ($($prim,)* $($unbound,)*)
            }

            #[inline]
            fn join(
                $($prim: $prim,)*
                $($unbound: (),)*
            ) -> Self {
                $(let () = $unbound;)*
                ($($prim),*)
            }
        }
    };
}

wasm_abi_tuple!(Prim1 | Prim2 Prim3 Prim4);
wasm_abi_tuple!(Prim1 Prim2 | Prim3 Prim4);
wasm_abi_tuple!(Prim1 Prim2 Prim3 | Prim4);
wasm_abi_tuple!(Prim1 Prim2 Prim3 Prim4 |);

// Anything that implements `WasmAbi` should naturally be also `IntoWasmAbi`/`FromWasmAbi`.
impl<T: WasmPrimitive> IntoWasmAbi for T {
    type Abi = T;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self
    }
}

impl<T: WasmPrimitive> FromWasmAbi for T {
    type Abi = T;

    #[inline]
    unsafe fn from_abi(js: Self::Abi) -> Self {
        js
    }
}

// 128-bit integers are represented as two 64-bit integers in the ABI
impl IntoWasmAbi for u128 {
    type Abi = (u64, u64);

    #[inline]
    fn into_abi(self) -> Self::Abi {
        let low = self as u64;
        let high = (self >> 64) as u64;
        (low, high)
    }
}

impl FromWasmAbi for u128 {
    type Abi = (u64, u64);

    #[inline]
    unsafe fn from_abi((low, high): Self::Abi) -> Self {
        (high as u128) << 64 | low as u128
    }
}

impl IntoWasmAbi for i128 {
    type Abi = <u128 as IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        (self as u128).into_abi()
    }
}

impl FromWasmAbi for i128 {
    type Abi = <u128 as FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(js: Self::Abi) -> Self {
        u128::from_abi(js) as i128
    }
}

macro_rules! option_tagged {
    ($($t:ty),*) => ($(
        impl<Abi: WasmAbi<Prim4 = ()>> IntoWasmAbi for Option<$t>
        where
            $t: IntoWasmAbi<Abi = Abi>,
        {
            type Abi = (u32, Abi::Prim1, Abi::Prim2, Abi::Prim3);

            #[inline]
            fn into_abi(self) -> Self::Abi {
                match self {
                    None => (
                        0,
                        Default::default(),
                        Default::default(),
                        Default::default(),
                    ),
                    Some(value) => {
                        let (prim1, prim2, prim3, ()) = value.into_abi().split();
                        (1, prim1, prim2, prim3)
                    }
                }
            }
        }


        impl<Abi: WasmAbi<Prim4 = ()>> FromWasmAbi for Option<$t>
        where
            $t: FromWasmAbi<Abi = Abi>,
        {
            type Abi = (u32, Abi::Prim1, Abi::Prim2, Abi::Prim3);

            #[inline]
            unsafe fn from_abi((is_some, prim1, prim2, prim3): Self::Abi) -> Self {
                if is_some == 0 {
                    None
                } else {
                    let abi = Abi::join(prim1, prim2, prim3, ());
                    Some(<$t>::from_abi(abi))
                }
            }
        }
    )*);
}

option_tagged!(i64, u64, f64, i128, u128);

/// The sentinel value is 2^32 + 1 for 32-bit primitive types.
///
/// 2^32 + 1 is used, because it's the smallest positive integer that cannot be
/// represented by any 32-bit primitive. While any value >= 2^32 works as a
/// sentinel value for 32-bit integers, it's a bit more tricky for `f32`. `f32`
/// can represent all powers of 2 up to 2^127 exactly. And between 2^32 and 2^33,
/// `f32` can represent all integers 2^32+512*k exactly.
const F64_ABI_OPTION_SENTINEL: f64 = 4294967297_f64;

macro_rules! option_f64 {
    ($($t:ty $(as $c:ty)?)*) => ($(
        $(impl IntoWasmAbi for $t {
            type Abi = $c;

            #[inline]
            fn into_abi(self) -> $c { self as $c }
        }

        impl FromWasmAbi for $t {
            type Abi = $c;

            #[inline]
            unsafe fn from_abi(js: $c) -> Self { js as $t }
        })?

        impl IntoWasmAbi for Option<$t> {
            type Abi = f64;

            #[inline]
            fn into_abi(self) -> Self::Abi {
                self.map(|v| v as f64).unwrap_or(F64_ABI_OPTION_SENTINEL)
            }
        }

        impl FromWasmAbi for Option<$t> {
            type Abi = f64;

            #[inline]
            unsafe fn from_abi(js: Self::Abi) -> Self {
                if js == F64_ABI_OPTION_SENTINEL {
                    None
                } else {
                    Some(js as $t)
                }
            }
        }
    )*)
}

option_f64!(
    i32
    isize as i32
    u32
    usize as u32
    f32
);

/// The sentinel value is 0xFF_FFFF for primitives with less than 32 bits.
///
/// This value is used, so all small primitive types (`bool`, `i8`, `u8`,
/// `i16`, `u16`, `char`) can use the same JS glue code. `char::MAX` is
/// 0x10_FFFF btw.
const U32_ABI_OPTION_SENTINEL: u32 = 0x00FF_FFFFu32;

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
    )*)
}

type_abi_as_u32!(i8 u8 i16 u16);

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

impl<T> IntoWasmAbi for *const T {
    type Abi = u32;

    #[inline]
    fn into_abi(self) -> u32 {
        self as u32
    }
}

impl<T> FromWasmAbi for *const T {
    type Abi = u32;

    #[inline]
    unsafe fn from_abi(js: u32) -> *const T {
        js as *const T
    }
}

impl<T> IntoWasmAbi for Option<*const T> {
    type Abi = f64;

    #[inline]
    fn into_abi(self) -> f64 {
        self.map(|ptr| ptr as u32 as f64)
            .unwrap_or(F64_ABI_OPTION_SENTINEL)
    }
}

impl<T> FromWasmAbi for Option<*const T> {
    type Abi = f64;

    #[inline]
    unsafe fn from_abi(js: f64) -> Option<*const T> {
        if js == F64_ABI_OPTION_SENTINEL {
            None
        } else {
            Some(js as u32 as *const T)
        }
    }
}

impl<T> IntoWasmAbi for *mut T {
    type Abi = u32;

    #[inline]
    fn into_abi(self) -> u32 {
        self as u32
    }
}

impl<T> FromWasmAbi for *mut T {
    type Abi = u32;

    #[inline]
    unsafe fn from_abi(js: u32) -> *mut T {
        js as *mut T
    }
}

impl<T> IntoWasmAbi for Option<*mut T> {
    type Abi = f64;

    #[inline]
    fn into_abi(self) -> f64 {
        self.map(|ptr| ptr as u32 as f64)
            .unwrap_or(F64_ABI_OPTION_SENTINEL)
    }
}

impl<T> FromWasmAbi for Option<*mut T> {
    type Abi = f64;

    #[inline]
    unsafe fn from_abi(js: f64) -> Option<*mut T> {
        if js == F64_ABI_OPTION_SENTINEL {
            None
        } else {
            Some(js as u32 as *mut T)
        }
    }
}

impl<T> IntoWasmAbi for NonNull<T> {
    type Abi = u32;

    #[inline]
    fn into_abi(self) -> u32 {
        self.as_ptr() as u32
    }
}

impl<T> OptionIntoWasmAbi for NonNull<T> {
    #[inline]
    fn none() -> u32 {
        0
    }
}

impl<T> FromWasmAbi for NonNull<T> {
    type Abi = u32;

    #[inline]
    unsafe fn from_abi(js: Self::Abi) -> Self {
        // SAFETY: Checked in bindings.
        NonNull::new_unchecked(js as *mut T)
    }
}

impl<T> OptionFromWasmAbi for NonNull<T> {
    #[inline]
    fn is_none(js: &u32) -> bool {
        *js == 0
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

impl<T: OptionIntoWasmAbi> IntoWasmAbi for Option<T> {
    type Abi = T::Abi;

    #[inline]
    fn into_abi(self) -> T::Abi {
        match self {
            None => T::none(),
            Some(me) => me.into_abi(),
        }
    }
}

impl<T: OptionFromWasmAbi> FromWasmAbi for Option<T> {
    type Abi = T::Abi;

    #[inline]
    unsafe fn from_abi(js: T::Abi) -> Self {
        if T::is_none(&js) {
            None
        } else {
            Some(T::from_abi(js))
        }
    }
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
}

impl<T, E> ReturnWasmAbi for Result<T, E>
where
    T: IntoWasmAbi,
    E: Into<JsValue>,
    T::Abi: WasmAbi<Prim3 = (), Prim4 = ()>,
{
    /// The order of primitives here is such that we can pop() the possible error
    /// first, deal with it and move on. Later primitives are popped off the
    /// stack first.
    type Abi = (
        <T::Abi as WasmAbi>::Prim1,
        <T::Abi as WasmAbi>::Prim2,
        // If this `Result` is an `Err`, the error value.
        u32,
        // Whether this `Result` is an `Err`.
        u32,
    );

    #[inline]
    fn return_abi(self) -> Self::Abi {
        match self {
            Ok(v) => {
                let (prim1, prim2, (), ()) = v.into_abi().split();
                (prim1, prim2, 0, 0)
            }
            Err(e) => {
                let jsval = e.into().into_abi();
                (Default::default(), Default::default(), jsval, 1)
            }
        }
    }
}

impl IntoWasmAbi for JsError {
    type Abi = <JsValue as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        self.value.into_abi()
    }
}

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

/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::convert) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
pub unsafe fn js_value_vector_from_abi<T: TryFromJsValue>(
    js: <Box<[JsValue]> as FromWasmAbi>::Abi,
) -> Box<[T]>
where
    T::Error: Debug,
{
    let js_vals = <Vec<JsValue> as FromWasmAbi>::from_abi(js);

    let mut result = Vec::with_capacity(js_vals.len());
    for value in js_vals {
        // We push elements one-by-one instead of using `collect` in order to improve
        // error messages. When using `collect`, this `expect_throw` is buried in a
        // giant chain of internal iterator functions, which results in the actual
        // function that takes this `Vec` falling off the end of the call stack.
        // So instead, make sure to call it directly within this function.
        //
        // This is only a problem in debug mode. Since this is the browser's error stack
        // we're talking about, it can only see functions that actually make it to the
        // final Wasm binary (i.e., not inlined functions). All of those internal
        // iterator functions get inlined in release mode, and so they don't show up.
        result.push(
            T::try_from_js_value(value).expect_throw("array contains a value of the wrong type"),
        );
    }
    result.into_boxed_slice()
}
