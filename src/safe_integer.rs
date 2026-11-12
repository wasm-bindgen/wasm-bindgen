use alloc::boxed::Box;
use core::convert::TryFrom;
use core::fmt;
use core::mem;
use core::ops::{
    Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use crate::__rt::marker::ErasableGeneric;
use crate::convert::{
    FromWasmAbi, IntoWasmAbi, LongRefFromWasmAbi, MaybeFromWasmAbi, MaybeIntoWasmAbi,
    NullableFromWasmAbi, NullableIntoWasmAbi, RefFromWasmAbi, TryFromJsValue, UpcastFrom,
    VectorFromWasmAbi, VectorIntoWasmAbi, WasmSlice,
};
use crate::describe::{inform, WasmDescribe, WasmDescribeVector, F64, VECTOR};
use crate::sys::{JsNullable, JsOption, Promising};
use crate::{JsValue, Nullable};

/// `Number.MAX_SAFE_INTEGER`, `2^53 - 1`.
const MAX_SAFE: f64 = 9007199254740991.0;

mod sealed {
    pub trait Sealed: Copy + Default + Eq + Ord + core::hash::Hash + 'static {
        const SIGNED: bool;
        const MAX: Self;
        const MIN: Self;
        fn to_f64(self) -> f64;
        fn from_f64(n: f64) -> Self;
    }

    impl Sealed for u64 {
        const SIGNED: bool = false;
        const MAX: u64 = (1 << 53) - 1;
        const MIN: u64 = 0;
        #[inline]
        fn to_f64(self) -> f64 {
            self as f64
        }
        #[inline]
        fn from_f64(n: f64) -> u64 {
            n as u64
        }
    }

    impl Sealed for i64 {
        const SIGNED: bool = true;
        const MAX: i64 = (1 << 53) - 1;
        const MIN: i64 = -((1 << 53) - 1);
        #[inline]
        fn to_f64(self) -> f64 {
            self as f64
        }
        #[inline]
        fn from_f64(n: f64) -> i64 {
            n as i64
        }
    }
}

/// The integer types a [`SafeInteger`] can hold: `u64` and `i64`.
///
/// This trait is sealed.
pub trait SafeIntegerPrimitive: sealed::Sealed {}
impl SafeIntegerPrimitive for u64 {}
impl SafeIntegerPrimitive for i64 {}

/// A `u64` or `i64` that crosses the JS boundary as a `number` instead of a
/// `BigInt`.
///
/// Many JS APIs type integer counts as `number`. A plain `u64` in a binding
/// signature is a `BigInt` at the boundary, so `SafeInteger<u64>` is used
/// instead to keep the Rust integer type while passing a `number`. In
/// TypeScript it is `number`.
///
/// ```ignore
/// use wasm_bindgen::SafeInteger;
///
/// #[wasm_bindgen]
/// extern "C" {
///     type QueueMetrics;
///     #[wasm_bindgen(method, getter, js_name = backlogCount)]
///     fn backlog_count(this: &QueueMetrics) -> SafeInteger<u64>;
/// }
///
/// let count = metrics.backlog_count();
/// if count > 10_000 { /* ... */ }
/// let next: u64 = (count + 1).into();
/// ```
///
/// It derefs to the inner integer, so its methods are available directly,
/// and compares and does arithmetic with both itself and the inner type.
///
/// The value is expected to be within `±(2^53 - 1)` (`Number.MAX_SAFE_INTEGER`)
/// and, for `u64`, non-negative, so that it is exactly representable as a JS
/// `number`. Like the overflow checks on Rust's own integers, this is only
/// checked with debug assertions enabled: constructing a value out of range,
/// arithmetic leaving the range, or receiving a non-integer or out-of-range
/// `number` from JS then panics. Without debug assertions an out-of-range
/// value converts lossily, as an `as` cast would. [`SafeInteger::new`] and
/// the `TryFrom<JsValue>` conversion always check.
///
/// `SafeInteger` is supported in every position its conversion is reached
/// through: arguments and returns, `catch` results, `Option`, `Vec` and
/// `Box<[_]>`, borrowed slices passed into Rust, closure arguments, async
/// import results and generic positions on `experimental_generic_mono`
/// imports. A borrowed slice `&[SafeInteger<_>]` cannot be passed out to JS,
/// since the elements have to be converted; pass a `Vec` or `Box<[_]>`
/// instead.
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SafeInteger<T: SafeIntegerPrimitive>(T);

/// A `u64` that crosses the JS boundary as a `number`. See [`SafeInteger`].
pub type SafeU64 = SafeInteger<u64>;

/// An `i64` that crosses the JS boundary as a `number`. See [`SafeInteger`].
pub type SafeI64 = SafeInteger<i64>;

#[inline]
fn is_safe<T: SafeIntegerPrimitive>(n: f64) -> bool {
    let min = if T::SIGNED { -MAX_SAFE } else { 0.0 };
    // In range, the cast is exact exactly when `n` is an integer. `NaN`
    // fails the range check.
    n >= min && n <= MAX_SAFE && (n as i64) as f64 == n
}

impl<T: SafeIntegerPrimitive> SafeInteger<T> {
    /// The largest safe integer, `2^53 - 1` (`Number.MAX_SAFE_INTEGER`).
    pub const MAX: Self = SafeInteger(T::MAX);

    /// The smallest safe integer: `0` for `u64`, `-(2^53 - 1)`
    /// (`Number.MIN_SAFE_INTEGER`) for `i64`.
    pub const MIN: Self = SafeInteger(T::MIN);

    /// Wraps `value`, returning `None` if it is out of the safe integer range.
    #[inline]
    pub fn new(value: T) -> Option<Self> {
        if value >= T::MIN && value <= T::MAX {
            Some(SafeInteger(value))
        } else {
            None
        }
    }

    #[inline]
    fn checked(value: T) -> Self {
        debug_assert!(
            value >= T::MIN && value <= T::MAX,
            "SafeInteger out of the safe integer range"
        );
        SafeInteger(value)
    }

    #[inline]
    fn to_f64(self) -> f64 {
        self.0.to_f64()
    }

    #[inline]
    fn from_f64(n: f64) -> Self {
        debug_assert!(is_safe::<T>(n), "{n} is not a safe integer");
        SafeInteger(T::from_f64(n))
    }
}

impl<T: SafeIntegerPrimitive> Deref for SafeInteger<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: SafeIntegerPrimitive> From<T> for SafeInteger<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::checked(value)
    }
}

impl<T: SafeIntegerPrimitive> PartialEq<T> for SafeInteger<T> {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

impl<T: SafeIntegerPrimitive> PartialOrd<T> for SafeInteger<T> {
    #[inline]
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        Some(self.0.cmp(other))
    }
}

impl<T: SafeIntegerPrimitive + fmt::Debug> fmt::Debug for SafeInteger<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<T: SafeIntegerPrimitive + fmt::Display> fmt::Display for SafeInteger<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

macro_rules! binops {
    ($($imp:ident $method:ident $assign_imp:ident $assign_method:ident,)*) => ($(
        impl<T: SafeIntegerPrimitive + $imp<Output = T>> $imp for SafeInteger<T> {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: Self) -> Self {
                Self::checked($imp::$method(self.0, rhs.0))
            }
        }

        impl<T: SafeIntegerPrimitive + $imp<Output = T>> $imp<T> for SafeInteger<T> {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: T) -> Self {
                Self::checked($imp::$method(self.0, rhs))
            }
        }

        impl<T: SafeIntegerPrimitive + $imp<Output = T>> $assign_imp for SafeInteger<T> {
            #[inline]
            fn $assign_method(&mut self, rhs: Self) {
                *self = $imp::$method(*self, rhs);
            }
        }

        impl<T: SafeIntegerPrimitive + $imp<Output = T>> $assign_imp<T> for SafeInteger<T> {
            #[inline]
            fn $assign_method(&mut self, rhs: T) {
                *self = $imp::$method(*self, rhs);
            }
        }
    )*)
}

binops! {
    Add add AddAssign add_assign,
    Sub sub SubAssign sub_assign,
    Mul mul MulAssign mul_assign,
    Div div DivAssign div_assign,
    Rem rem RemAssign rem_assign,
}

impl Neg for SafeInteger<i64> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        SafeInteger(-self.0)
    }
}

macro_rules! concrete {
    ($($t:ident)*) => ($(
        impl From<SafeInteger<$t>> for $t {
            #[inline]
            fn from(value: SafeInteger<$t>) -> $t {
                value.0
            }
        }

        impl PartialEq<SafeInteger<$t>> for $t {
            #[inline]
            fn eq(&self, other: &SafeInteger<$t>) -> bool {
                *self == other.0
            }
        }

        impl PartialOrd<SafeInteger<$t>> for $t {
            #[inline]
            fn partial_cmp(&self, other: &SafeInteger<$t>) -> Option<core::cmp::Ordering> {
                Some(self.cmp(&other.0))
            }
        }
    )*)
}

concrete!(u64 i64);

impl<T: SafeIntegerPrimitive> WasmDescribe for SafeInteger<T> {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(F64)
    }
}

impl<T: SafeIntegerPrimitive> IntoWasmAbi for SafeInteger<T> {
    type Abi = f64;

    #[inline]
    fn into_abi(self) -> f64 {
        self.to_f64()
    }
}

impl<T: SafeIntegerPrimitive> FromWasmAbi for SafeInteger<T> {
    type Abi = f64;

    #[inline]
    unsafe fn from_abi(js: f64) -> Self {
        Self::from_f64(js)
    }
}

// `Option` and `Nullable` take `f64`'s presence-tagged ABI rather than a
// sentinel, since every safe integer is a valid value.
impl<T: SafeIntegerPrimitive> MaybeIntoWasmAbi for SafeInteger<T> {
    type MaybeAbi = <f64 as MaybeIntoWasmAbi>::MaybeAbi;

    #[inline]
    fn into_maybe_abi(self) -> Self::MaybeAbi {
        self.to_f64().into_maybe_abi()
    }

    #[inline]
    fn none() -> Self::MaybeAbi {
        <f64 as MaybeIntoWasmAbi>::none()
    }
}

impl<T: SafeIntegerPrimitive> NullableIntoWasmAbi for SafeInteger<T> {
    #[inline]
    fn null() -> Self::MaybeAbi {
        <f64 as NullableIntoWasmAbi>::null()
    }
}

impl<T: SafeIntegerPrimitive> MaybeFromWasmAbi for SafeInteger<T> {
    type MaybeAbi = <f64 as MaybeFromWasmAbi>::MaybeAbi;

    #[inline]
    unsafe fn from_maybe_abi(js: Self::MaybeAbi) -> Self {
        Self::from_f64(f64::from_maybe_abi(js))
    }

    #[inline]
    fn is_none(js: &Self::MaybeAbi) -> bool {
        <f64 as MaybeFromWasmAbi>::is_none(js)
    }
}

impl<T: SafeIntegerPrimitive> NullableFromWasmAbi for SafeInteger<T> {
    #[inline]
    fn is_null(js: &Self::MaybeAbi) -> bool {
        <f64 as NullableFromWasmAbi>::is_null(js)
    }
}

// A borrowed `SafeInteger` crosses by copy, like the other scalars.
impl<T: SafeIntegerPrimitive> IntoWasmAbi for &SafeInteger<T> {
    type Abi = f64;

    #[inline]
    fn into_abi(self) -> f64 {
        (*self).into_abi()
    }
}

impl<T: SafeIntegerPrimitive> MaybeIntoWasmAbi for &SafeInteger<T> {
    type MaybeAbi = <f64 as MaybeIntoWasmAbi>::MaybeAbi;

    #[inline]
    fn into_maybe_abi(self) -> Self::MaybeAbi {
        (*self).into_maybe_abi()
    }

    #[inline]
    fn none() -> Self::MaybeAbi {
        <f64 as MaybeIntoWasmAbi>::none()
    }
}

impl<T: SafeIntegerPrimitive> NullableIntoWasmAbi for &SafeInteger<T> {
    #[inline]
    fn null() -> Self::MaybeAbi {
        <f64 as NullableIntoWasmAbi>::null()
    }
}

// Vectors cross as `Float64Array`s. `SafeInteger<T>` has the size and
// alignment of `f64`, so each element is converted in place and the
// allocation is handed over as-is.
const _: () = assert!(
    mem::size_of::<SafeInteger<u64>>() == mem::size_of::<f64>()
        && mem::align_of::<SafeInteger<u64>>() == mem::align_of::<f64>()
        && mem::size_of::<SafeInteger<i64>>() == mem::size_of::<f64>()
        && mem::align_of::<SafeInteger<i64>>() == mem::align_of::<f64>()
);

impl<T: SafeIntegerPrimitive> WasmDescribeVector for SafeInteger<T> {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe_vector() {
        inform(VECTOR);
        inform(F64);
    }
}

impl<T: SafeIntegerPrimitive> VectorIntoWasmAbi for SafeInteger<T> {
    type Abi = WasmSlice;

    fn vector_into_abi(vector: Box<[Self]>) -> WasmSlice {
        let len = vector.len();
        let ptr = Box::into_raw(vector) as *mut Self;
        for i in 0..len {
            unsafe {
                let elem = ptr.add(i);
                let n = (*elem).to_f64();
                elem.cast::<f64>().write(n);
            }
        }
        let floats =
            unsafe { Box::from_raw(core::ptr::slice_from_raw_parts_mut(ptr.cast::<f64>(), len)) };
        f64::vector_into_abi(floats)
    }
}

impl<T: SafeIntegerPrimitive> VectorFromWasmAbi for SafeInteger<T> {
    type Abi = WasmSlice;

    unsafe fn vector_from_abi(js: WasmSlice) -> Box<[Self]> {
        let floats = f64::vector_from_abi(js);
        let len = floats.len();
        let ptr = Box::into_raw(floats) as *mut f64;
        for i in 0..len {
            let elem = ptr.add(i);
            let n = Self::from_f64(*elem);
            elem.cast::<Self>().write(n);
        }
        Box::from_raw(core::ptr::slice_from_raw_parts_mut(ptr.cast::<Self>(), len))
    }
}

impl<T: SafeIntegerPrimitive> RefFromWasmAbi for [SafeInteger<T>] {
    type Abi = WasmSlice;
    type Anchor = Box<[SafeInteger<T>]>;

    #[inline]
    unsafe fn ref_from_abi(js: WasmSlice) -> Self::Anchor {
        <Box<[SafeInteger<T>]>>::from_abi(js)
    }
}

impl<T: SafeIntegerPrimitive> LongRefFromWasmAbi for [SafeInteger<T>] {
    type Abi = WasmSlice;
    type Anchor = Box<[SafeInteger<T>]>;

    #[inline]
    unsafe fn long_ref_from_abi(js: WasmSlice) -> Self::Anchor {
        <Box<[SafeInteger<T>]>>::from_abi(js)
    }
}

unsafe impl<T: SafeIntegerPrimitive> ErasableGeneric for SafeInteger<T> {
    type Repr = Self;
}

impl<T: SafeIntegerPrimitive> Promising for SafeInteger<T> {
    type Resolution = Self;
}

impl<T: SafeIntegerPrimitive> UpcastFrom<SafeInteger<T>> for SafeInteger<T> {}
impl<T: SafeIntegerPrimitive> UpcastFrom<SafeInteger<T>> for f64 {}
impl<T: SafeIntegerPrimitive> UpcastFrom<SafeInteger<T>> for JsOption<f64> {}
impl<T: SafeIntegerPrimitive> UpcastFrom<SafeInteger<T>> for JsNullable<f64> {}
impl<T: SafeIntegerPrimitive> UpcastFrom<SafeInteger<T>> for JsValue {}
impl<T: SafeIntegerPrimitive> UpcastFrom<SafeInteger<T>> for JsOption<JsValue> {}
impl<T: SafeIntegerPrimitive> UpcastFrom<SafeInteger<T>> for JsNullable<JsValue> {}
impl<T: SafeIntegerPrimitive, Target: UpcastFrom<SafeInteger<T>>> UpcastFrom<SafeInteger<T>>
    for Option<Target>
{
}
impl<T: SafeIntegerPrimitive, Target: UpcastFrom<SafeInteger<T>>> UpcastFrom<SafeInteger<T>>
    for Nullable<Target>
{
}

impl<T: SafeIntegerPrimitive> From<SafeInteger<T>> for JsValue {
    #[inline]
    fn from(n: SafeInteger<T>) -> JsValue {
        JsValue::from_f64(n.to_f64())
    }
}

impl<T: SafeIntegerPrimitive> PartialEq<SafeInteger<T>> for JsValue {
    #[inline]
    fn eq(&self, other: &SafeInteger<T>) -> bool {
        self.as_f64() == Some(other.to_f64())
    }
}

// Unlike the ABI conversion, this always checks: a value that is not a safe
// integer is an `Err`.
impl<T: SafeIntegerPrimitive> TryFromJsValue for SafeInteger<T> {
    #[inline]
    fn try_from_js_value_ref(value: &JsValue) -> Option<Self> {
        let n = value.as_f64()?;
        if is_safe::<T>(n) {
            Some(SafeInteger(T::from_f64(n)))
        } else {
            None
        }
    }
}

impl<T: SafeIntegerPrimitive> TryFrom<JsValue> for SafeInteger<T> {
    type Error = JsValue;

    #[inline]
    fn try_from(value: JsValue) -> Result<Self, JsValue> {
        Self::try_from_js_value(value)
    }
}

impl<T: SafeIntegerPrimitive> TryFrom<&JsValue> for SafeInteger<T> {
    type Error = JsValue;

    #[inline]
    fn try_from(value: &JsValue) -> Result<Self, JsValue> {
        Self::try_from_js_value_ref(value).ok_or_else(|| value.clone())
    }
}
