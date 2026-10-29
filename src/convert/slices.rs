use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::mem::{self, MaybeUninit};
use core::ops::{Deref, DerefMut};
use core::str;

use crate::__rt::{marker::ErasableGeneric, WasmWord};
use crate::__wbindgen_copy_to_typed_array;
use crate::convert::{
    js_value_vector_from_abi, js_value_vector_into_abi, try_js_value_vector_from_abi, FromWasmAbi,
    IntoWasmAbi, LongRefFromWasmAbi, MaybeIntoWasmAbi, NullableIntoWasmAbi, OptionFromWasmAbi,
    OptionIntoWasmAbi, RefFromWasmAbi, RefMutFromWasmAbi, UpcastFrom, VectorFromWasmAbi,
    VectorIntoWasmAbi, WasmAbi,
};
use crate::describe::*;
use crate::sys::{JsNullable, JsOption};
use crate::{JsError, JsValue, Nullable};

use cfg_if::cfg_if;

/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::convert) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
// note: `WasmAbi` types do not need to be FFI-safe themselves, it's just more
// convenient to directly write `WasmSlice` in some of the manually-written FFI
// functions in `lib.rs` rather than `WasmRet<WasmSlice>`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WasmSlice {
    pub ptr: WasmWord,
    pub len: WasmWord,
}

impl WasmSlice {
    #[inline]
    pub fn from_usize(ptr: usize, len: usize) -> Self {
        Self {
            ptr: WasmWord::from_usize(ptr),
            len: WasmWord::from_usize(len),
        }
    }
}

impl WasmAbi for WasmSlice {
    /// `self.ptr`
    type Prim1 = <WasmWord as WasmAbi>::Prim1;
    /// `self.len`
    type Prim2 = <WasmWord as WasmAbi>::Prim1;
    type Prim3 = ();
    type Prim4 = ();

    #[inline]
    fn split(self) -> (Self::Prim1, Self::Prim2, (), ()) {
        (self.ptr.split().0, self.len.split().0, (), ())
    }

    #[inline]
    fn join(ptr: Self::Prim1, len: Self::Prim2, _: (), _: ()) -> Self {
        Self {
            ptr: WasmWord::join(ptr, (), (), ()),
            len: WasmWord::join(len, (), (), ()),
        }
    }
}

#[inline]
fn null_slice() -> WasmSlice {
    WasmSlice::from_usize(0, 0)
}

/// A slice with a null pointer carries a JS handle in `len`, as an interned
/// string does, and `0` is `undefined`; this is the handle of `null`.
#[inline]
fn js_null_slice() -> WasmSlice {
    WasmSlice::from_usize(0, crate::__rt::JSIDX_NULL as usize)
}

#[inline]
fn is_js_null_slice(slice: &WasmSlice) -> bool {
    slice.ptr.is_zero() && slice.len.into_usize() == crate::__rt::JSIDX_NULL as usize
}

pub struct WasmMutSlice {
    pub slice: WasmSlice,
    pub idx: u32,
}

impl WasmAbi for WasmMutSlice {
    /// `self.slice.ptr`
    type Prim1 = <WasmSlice as WasmAbi>::Prim1;
    /// `self.slice.len`
    type Prim2 = <WasmSlice as WasmAbi>::Prim2;
    /// `self.idx`
    type Prim3 = u32;
    type Prim4 = ();

    #[inline]
    fn split(self) -> (Self::Prim1, Self::Prim2, u32, ()) {
        let (ptr, len, (), ()) = self.slice.split();
        (ptr, len, self.idx, ())
    }

    #[inline]
    fn join(ptr: Self::Prim1, len: Self::Prim2, idx: u32, _: ()) -> Self {
        Self {
            slice: WasmSlice::join(ptr, len, (), ()),
            idx,
        }
    }
}

/// The representation of a mutable slice passed from JS to Rust.
pub struct MutSlice<T> {
    /// A copy of the data in the JS typed array.
    contents: Box<[T]>,
    /// A reference to the original JS typed array.
    js: JsValue,
}

impl<T> Drop for MutSlice<T> {
    fn drop(&mut self) {
        let byte_slice = unsafe {
            core::slice::from_raw_parts(
                self.contents.as_ptr() as *const u8,
                self.contents.len() * mem::size_of::<T>(),
            )
        };
        __wbindgen_copy_to_typed_array(byte_slice, &self.js);
    }
}

impl<T> Deref for MutSlice<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.contents
    }
}

impl<T> DerefMut for MutSlice<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.contents
    }
}

macro_rules! vectors {
    ($($t:ty)*) => ($(
        vectors_internal!($t);
        vectors_internal!(MaybeUninit<$t>);
    )*)
}

macro_rules! vectors_internal {
    ($t:ty) => {
        impl WasmDescribeVector for $t {
            #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
            fn describe_vector() {
                inform(VECTOR);
                <$t>::describe();
            }
        }

        impl VectorIntoWasmAbi for $t {
            type Abi = WasmSlice;

            #[inline]
            fn vector_into_abi(vector: Box<[$t]>) -> WasmSlice {
                let ptr = vector.as_ptr();
                let len = vector.len();
                mem::forget(vector);
                WasmSlice::from_usize(ptr as usize, len)
            }
        }

        impl VectorFromWasmAbi for $t {
            type Abi = WasmSlice;

            #[inline]
            unsafe fn vector_from_abi(js: WasmSlice) -> Box<[$t]> {
                let ptr = js.ptr.into_usize() as *mut $t;
                let len = js.len.into_usize();
                Vec::from_raw_parts(ptr, len, len).into_boxed_slice()
            }
        }

        impl VectorRefIntoWasmAbi for $t {
            #[inline]
            fn slice_into_abi(slice: &[$t]) -> WasmSlice {
                WasmSlice::borrow(slice)
            }
        }

        impl crate::__rt::marker::VectorRefIntoWasmAbiSealed for $t {}

        impl<'a> IntoWasmAbi for &'a mut [$t] {
            type Abi = WasmSlice;

            #[inline]
            fn into_abi(self) -> WasmSlice {
                (&*self).into_abi()
            }
        }

        impl<'a> OptionIntoWasmAbi for &'a mut [$t] {
            #[inline]
            fn none() -> WasmSlice {
                null_slice()
            }
        }

        impl_nullable_into_shared_abi!(null = js_null_slice(); ['a] &'a mut [$t]);

        impl RefFromWasmAbi for [$t] {
            type Abi = WasmSlice;
            type Anchor = Box<[$t]>;

            #[inline]
            unsafe fn ref_from_abi(js: WasmSlice) -> Box<[$t]> {
                <Box<[$t]>>::from_abi(js)
            }
        }

        impl RefMutFromWasmAbi for [$t] {
            type Abi = WasmMutSlice;
            type Anchor = MutSlice<$t>;

            #[inline]
            unsafe fn ref_mut_from_abi(js: WasmMutSlice) -> MutSlice<$t> {
                let contents = <Box<[$t]>>::from_abi(js.slice);
                let js = JsValue::from_abi(js.idx);
                MutSlice { contents, js }
            }
        }

        impl LongRefFromWasmAbi for [$t] {
            type Abi = WasmSlice;
            type Anchor = Box<[$t]>;

            #[inline]
            unsafe fn long_ref_from_abi(js: WasmSlice) -> Box<[$t]> {
                Self::ref_from_abi(js)
            }
        }
    };
}

vectors! {
    u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64
}

impl WasmDescribeVector for String {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe_vector() {
        inform(VECTOR);
        inform(NAMED_EXTERNREF);
        // Trying to use an actual loop for this breaks the Wasm interpreter.
        inform(6);
        inform('s' as u32);
        inform('t' as u32);
        inform('r' as u32);
        inform('i' as u32);
        inform('n' as u32);
        inform('g' as u32);
    }
}

impl VectorIntoWasmAbi for String {
    type Abi = <Box<[JsValue]> as IntoWasmAbi>::Abi;

    fn vector_into_abi(vector: Box<[Self]>) -> Self::Abi {
        js_value_vector_into_abi(vector)
    }
}

impl VectorFromWasmAbi for String {
    type Abi = <Box<[JsValue]> as FromWasmAbi>::Abi;

    unsafe fn vector_from_abi(js: Self::Abi) -> Box<[Self]> {
        js_value_vector_from_abi(js)
    }

    unsafe fn try_vector_from_abi(js: Self::Abi) -> Result<Box<[Self]>, &'static str> {
        try_js_value_vector_from_abi(js)
    }
}

cfg_if! {
    if #[cfg(feature = "enable-interning")] {
        #[inline]
        fn unsafe_get_cached_str(x: &str) -> Option<WasmSlice> {
            // This uses 0 for the ptr as an indication that it is a JsValue and not a str.
            crate::cache::intern::unsafe_get_str(x).map(|x| WasmSlice::from_usize(0, x as usize))
        }

    } else {
        #[inline]
        fn unsafe_get_cached_str(_x: &str) -> Option<WasmSlice> {
            None
        }
    }
}

impl<T> IntoWasmAbi for Vec<T>
where
    Box<[T]>: IntoWasmAbi<Abi = WasmSlice>,
{
    type Abi = <Box<[T]> as IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self.into_boxed_slice().into_abi()
    }
}

impl<T> OptionIntoWasmAbi for Vec<T>
where
    Box<[T]>: IntoWasmAbi<Abi = WasmSlice>,
{
    #[inline]
    fn none() -> WasmSlice {
        null_slice()
    }
}

impl_nullable_into_shared_abi!(
    null = js_null_slice();
    [T] Vec<T> where Box<[T]>: IntoWasmAbi<Abi = WasmSlice>
);

/// How a borrowed `&[Self]` crosses to JS.
///
/// A sequence crosses as a [`WasmSlice`] of `len` cells. In place, the cell is
/// the element itself, which is its JS encoding (numbers, and JS handles as
/// table indices), so the slice is borrowed ([`WasmSlice::borrow`]). Encoded,
/// the cell is the element's scalar ABI, in a fresh buffer that JS decodes
/// with the element's own glue and frees ([`WasmSlice::encode`]). The CLI
/// tells them apart by the element's descriptor alone: numbers and JS handles
/// are in place, and anything else is encoded. Encoded elements are declared
/// once, with `__wbindgen_encoded_sequence!`.
pub trait VectorRefIntoWasmAbi:
    WasmDescribe + Sized + crate::__rt::marker::VectorRefIntoWasmAbiSealed
{
    fn slice_into_abi(slice: &[Self]) -> WasmSlice;
}

impl<T: VectorRefIntoWasmAbi> IntoWasmAbi for &[T] {
    type Abi = WasmSlice;

    #[inline]
    fn into_abi(self) -> WasmSlice {
        T::slice_into_abi(self)
    }
}

impl<T: VectorRefIntoWasmAbi> OptionIntoWasmAbi for &[T] {
    #[inline]
    fn none() -> WasmSlice {
        null_slice()
    }
}

impl_nullable_into_shared_abi!(null = js_null_slice(); [T: VectorRefIntoWasmAbi] &[T]);

/// An encoded sequence's cell: the `#[repr(C)]` record of a scalar ABI's
/// primitives, with unused ones `()`. That is the same split as the wasm
/// signature, which is how the CLI lays out the cell's words, so both sides
/// agree on the layout by definition.
#[repr(C)]
struct AbiCell<A: WasmAbi>(A::Prim1, A::Prim2, A::Prim3, A::Prim4);

// A `(ptr, len)` cell is two pointer words, which are `f64`s on memory64.
#[cfg(target_arch = "wasm64")]
const _: () = assert!(mem::size_of::<AbiCell<WasmSlice>>() == 16);
#[cfg(target_arch = "wasm64")]
const _: () = assert!(mem::align_of::<AbiCell<WasmSlice>>() == 8);
#[cfg(not(target_arch = "wasm64"))]
const _: () = assert!(mem::size_of::<AbiCell<WasmSlice>>() == 8);
#[cfg(not(target_arch = "wasm64"))]
const _: () = assert!(mem::align_of::<AbiCell<WasmSlice>>() == 4);

impl WasmSlice {
    /// An in-place sequence: `slice`'s own memory, borrowed.
    #[inline]
    pub fn borrow<T>(slice: &[T]) -> Self {
        Self::from_usize(slice.as_ptr() as usize, slice.len())
    }

    /// An encoded sequence: a fresh buffer of each element's cell, its scalar
    /// ABI by `f`, which JS decodes and frees.
    pub fn encode<'a, T: IntoWasmAbi + 'a>(
        slice: &'a [T],
        mut f: impl FnMut(&'a T) -> T::Abi,
    ) -> Self {
        let cells: Box<[AbiCell<T::Abi>]> = slice
            .iter()
            .map(|elem| {
                let (p1, p2, p3, p4) = f(elem).split();
                AbiCell(p1, p2, p3, p4)
            })
            .collect();
        let len = cells.len();
        Self::from_usize(Box::into_raw(cells) as *mut AbiCell<T::Abi> as usize, len)
    }

    /// Takes an encoded sequence from JS, converting each element from its
    /// cell's ABI with `FromWasmAbi::try_from_abi`. A rejected element stops
    /// the decode, dropping the buffer and every element converted so far.
    ///
    /// # Safety
    ///
    /// `self` must be `len` cells of `T::Abi` that JS allocated at the cell's
    /// alignment, for Rust to own.
    pub unsafe fn try_decode<T: FromWasmAbi>(self) -> Result<Box<[T]>, &'static str> {
        let ptr = self.ptr.into_usize() as *mut AbiCell<T::Abi>;
        let len = self.len.into_usize();
        Vec::from_raw_parts(ptr, len, len)
            .into_iter()
            .map(|AbiCell(p1, p2, p3, p4)| T::try_from_abi(T::Abi::join(p1, p2, p3, p4)))
            .collect()
    }
}

/// Declares a sequence of `$t` with encoded cells, emitting every impl it
/// needs together with its descriptor, for each side listed: `ref_into`
/// (`&[T]` to JS), `into` (`Vec<T>` / `Box<[T]>` to JS) and `from` (`Vec<T>`
/// / `Box<[T]>` from JS). `$encode` gives a borrowed element's scalar ABI. An
/// owned vector to JS encodes as its borrowed slice, so `into` is only for
/// elements that own nothing, which it checks at compile time. `from` takes
/// its element with `FromWasmAbi` (see [`WasmSlice::try_decode`]).
#[macro_export]
#[doc(hidden)]
macro_rules! __wbindgen_encoded_sequence {
    ($t:ty, $encode:expr, [$($side:ident),+ $(,)?]) => {
        $($crate::__wbindgen_encoded_sequence!(@$side $t, $encode);)+
    };
    (@ref_into $t:ty, $encode:expr) => {
        $crate::__wbindgen_coverage! {
            #[automatically_derived]
            impl $crate::convert::VectorRefIntoWasmAbi for $t {
                #[inline]
                fn slice_into_abi(slice: &[Self]) -> $crate::convert::WasmSlice {
                    $crate::convert::WasmSlice::encode(slice, $encode)
                }
            }
        }

        #[automatically_derived]
        impl $crate::__rt::marker::VectorRefIntoWasmAbiSealed for $t {}
    };
    (@into $t:ty, $encode:expr) => {
        const _: () = assert!(!$crate::__rt::core::mem::needs_drop::<$t>());

        $crate::__wbindgen_coverage! {
            #[automatically_derived]
            impl $crate::describe::WasmDescribeVector for $t {
                fn describe_vector() {
                    $crate::describe::inform($crate::describe::VECTOR);
                    <Self as $crate::describe::WasmDescribe>::describe();
                }
            }
        }

        #[automatically_derived]
        impl $crate::convert::VectorIntoWasmAbi for $t {
            type Abi = $crate::convert::WasmSlice;

            #[inline]
            fn vector_into_abi(
                vector: $crate::__rt::alloc::boxed::Box<[Self]>,
            ) -> $crate::convert::WasmSlice {
                <Self as $crate::convert::VectorRefIntoWasmAbi>::slice_into_abi(&vector)
            }
        }
    };
    (@from $t:ty, $encode:expr) => {
        #[automatically_derived]
        impl $crate::convert::VectorFromWasmAbi for $t {
            type Abi = $crate::convert::WasmSlice;

            #[inline]
            unsafe fn vector_from_abi(
                js: $crate::convert::WasmSlice,
            ) -> $crate::__rt::alloc::boxed::Box<[Self]> {
                // Thrown directly, as a closure's frames would push the
                // caller past the debug-mode error stack's frame limit.
                match js.try_decode() {
                    $crate::__rt::core::result::Result::Ok(decoded) => decoded,
                    $crate::__rt::core::result::Result::Err(msg) => $crate::throw_str(msg),
                }
            }

            #[inline]
            unsafe fn try_vector_from_abi(
                js: $crate::convert::WasmSlice,
            ) -> $crate::__rt::core::result::Result<
                $crate::__rt::alloc::boxed::Box<[Self]>,
                &'static str,
            > {
                js.try_decode()
            }
        }
    };
}

crate::__wbindgen_encoded_sequence!(bool, |b| (*b).into_abi(), [ref_into, into, from]);
crate::__wbindgen_encoded_sequence!(char, |c| (*c).into_abi(), [ref_into, into, from]);
crate::__wbindgen_encoded_sequence!(&str, |s| (*s).into_abi(), [ref_into, into]);
crate::__wbindgen_encoded_sequence!(&String, |s| (*s).into_abi(), [ref_into]);
crate::__wbindgen_encoded_sequence!(&Box<str>, |s| (*s).into_abi(), [ref_into]);
crate::__wbindgen_encoded_sequence!(&Cow<'_, str>, |s| (*s).into_abi(), [ref_into]);
// An owned `Vec<String>` keeps its released `JsValue` form.
crate::__wbindgen_encoded_sequence!(String, |s| s.into_abi(), [ref_into]);

impl crate::__rt::marker::VectorRefIntoWasmAbiSealed for JsValue {}

impl VectorRefIntoWasmAbi for JsValue {
    #[inline]
    fn slice_into_abi(slice: &[Self]) -> WasmSlice {
        WasmSlice::borrow(slice)
    }
}

impl crate::__rt::marker::VectorRefIntoWasmAbiSealed for JsError {}

impl VectorRefIntoWasmAbi for JsError {
    #[inline]
    fn slice_into_abi(slice: &[Self]) -> WasmSlice {
        WasmSlice::borrow(slice)
    }
}

impl<T> FromWasmAbi for Vec<T>
where
    Box<[T]>: FromWasmAbi<Abi = WasmSlice>,
{
    type Abi = <Box<[T]> as FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(js: Self::Abi) -> Self {
        <Box<[T]>>::from_abi(js).into()
    }

    #[inline]
    unsafe fn try_from_abi(js: Self::Abi) -> Result<Self, &'static str> {
        <Box<[T]>>::try_from_abi(js).map(Into::into)
    }
}

impl<T> OptionFromWasmAbi for Vec<T>
where
    Box<[T]>: FromWasmAbi<Abi = WasmSlice>,
{
    #[inline]
    fn is_none(abi: &WasmSlice) -> bool {
        abi.ptr.is_zero()
    }
}

impl_nullable_from_shared_abi!(
    is_null = |abi| is_js_null_slice(abi);
    [T] Vec<T> where Box<[T]>: FromWasmAbi<Abi = WasmSlice>
);

impl IntoWasmAbi for String {
    type Abi = <Vec<u8> as IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        // This is safe because the JsValue is immediately looked up in the heap and
        // then returned, so use-after-free cannot occur.
        unsafe_get_cached_str(&self).unwrap_or_else(|| self.into_bytes().into_abi())
    }
}

impl OptionIntoWasmAbi for String {
    #[inline]
    fn none() -> Self::Abi {
        null_slice()
    }
}

impl_nullable_into_shared_abi!(null = js_null_slice(); [] String);

impl FromWasmAbi for String {
    type Abi = <Vec<u8> as FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(js: Self::Abi) -> Self {
        String::from_utf8_unchecked(<Vec<u8>>::from_abi(js))
    }
}

impl OptionFromWasmAbi for String {
    #[inline]
    fn is_none(slice: &WasmSlice) -> bool {
        slice.ptr.is_zero()
    }
}

impl_nullable_from_shared_abi!(is_null = |slice| is_js_null_slice(slice); [] String);

impl<'a> IntoWasmAbi for &'a str {
    type Abi = <&'a [u8] as IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        // This is safe because the JsValue is immediately looked up in the heap and
        // then returned, so use-after-free cannot occur.
        unsafe_get_cached_str(self).unwrap_or_else(|| self.as_bytes().into_abi())
    }
}

impl OptionIntoWasmAbi for &str {
    #[inline]
    fn none() -> Self::Abi {
        null_slice()
    }
}

impl_nullable_into_shared_abi!(null = js_null_slice(); [] &str);

// Borrowed shapes that deref to `str` cross exactly like `&str`.
macro_rules! str_ref_into_wasm_abi {
    ($($t:ty),* $(,)?) => ($(
        impl IntoWasmAbi for &$t {
            type Abi = <&'static str as IntoWasmAbi>::Abi;

            #[inline]
            fn into_abi(self) -> Self::Abi {
                <&str as IntoWasmAbi>::into_abi(self)
            }
        }

        impl OptionIntoWasmAbi for &$t {
            #[inline]
            fn none() -> Self::Abi {
                <&str as OptionIntoWasmAbi>::none()
            }
        }

        impl_nullable_into_shared_abi!(null = <&str as NullableIntoWasmAbi>::null(); [] &$t);
    )*)
}

str_ref_into_wasm_abi!(String, Box<str>, Cow<'_, str>);

impl RefFromWasmAbi for str {
    type Abi = <[u8] as RefFromWasmAbi>::Abi;
    type Anchor = Box<str>;

    #[inline]
    unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
        mem::transmute::<Box<[u8]>, Box<str>>(<Box<[u8]>>::from_abi(js))
    }
}

impl LongRefFromWasmAbi for str {
    type Abi = <[u8] as RefFromWasmAbi>::Abi;
    type Anchor = Box<str>;

    #[inline]
    unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
        Self::ref_from_abi(js)
    }
}

unsafe impl<'a> ErasableGeneric for &'a str {
    type Repr = &'a str;
}

unsafe impl<T: ErasableGeneric> ErasableGeneric for Box<[T]> {
    type Repr = Box<[T::Repr]>;
}

impl<'a> UpcastFrom<&'a str> for &'a str {}
impl UpcastFrom<String> for String {}
impl<'a, Target: UpcastFrom<&'a str>> UpcastFrom<&'a str> for Option<Target> {}
impl<'a, Target: UpcastFrom<&'a str>> UpcastFrom<&'a str> for Nullable<Target> {}
impl<Target: UpcastFrom<String>> UpcastFrom<String> for Option<Target> {}
impl<Target: UpcastFrom<String>> UpcastFrom<String> for Nullable<Target> {}
// `String` and `&str` label the same JS string, a `&str` element's lifetime
// being phantom, so they upcast both ways, as each does with `JsString`. An
// inferred `Array<&str>` settles to `Array<String>`, and a `Function<fn(&str)>`
// slot relates to a `Function<fn(String)>` one.
impl UpcastFrom<&str> for String {}
impl UpcastFrom<String> for &str {}

// A Rust string stands for a JS string in a generic position, as `bool` does
// for a JS boolean, so `Promise<String>` relabels straight to `Promise<JsValue>`.
impl UpcastFrom<String> for JsValue {}
impl UpcastFrom<String> for JsOption<JsValue> {}
impl UpcastFrom<String> for JsNullable<JsValue> {}
impl UpcastFrom<&str> for JsValue {}
impl UpcastFrom<&str> for JsOption<JsValue> {}
impl UpcastFrom<&str> for JsNullable<JsValue> {}

impl<T, Target> UpcastFrom<Box<[T]>> for Box<[Target]> where Target: UpcastFrom<T> {}

unsafe impl<T: ErasableGeneric> ErasableGeneric for Vec<T> {
    type Repr = Vec<T::Repr>;
}

impl<T, Target> UpcastFrom<Vec<T>> for Vec<Target> where Target: UpcastFrom<T> {}

impl<T: VectorIntoWasmAbi> IntoWasmAbi for Box<[T]> {
    type Abi = <T as VectorIntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        T::vector_into_abi(self)
    }
}

impl<T> OptionIntoWasmAbi for Box<[T]>
where
    Self: IntoWasmAbi<Abi = WasmSlice>,
{
    fn none() -> WasmSlice {
        null_slice()
    }
}

impl_nullable_into_shared_abi!(
    null = js_null_slice();
    [T] Box<[T]> where Self: IntoWasmAbi<Abi = WasmSlice>
);

impl<T: VectorFromWasmAbi> FromWasmAbi for Box<[T]> {
    type Abi = <T as VectorFromWasmAbi>::Abi;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        T::vector_from_abi(js)
    }

    #[inline]
    unsafe fn try_from_abi(js: Self::Abi) -> Result<Self, &'static str> {
        T::try_vector_from_abi(js)
    }
}

impl<T> OptionFromWasmAbi for Box<[T]>
where
    Self: FromWasmAbi<Abi = WasmSlice>,
{
    fn is_none(slice: &WasmSlice) -> bool {
        slice.ptr.is_zero()
    }
}

impl_nullable_from_shared_abi!(
    is_null = |slice| is_js_null_slice(slice);
    [T] Box<[T]> where Self: FromWasmAbi<Abi = WasmSlice>
);

impl<T: ErasableGeneric<Repr = JsValue> + WasmDescribe> VectorFromWasmAbi for T {
    type Abi = WasmSlice;

    #[inline]
    unsafe fn vector_from_abi(js: WasmSlice) -> Box<[Self]> {
        let ptr = js.ptr.into_usize() as *mut T;
        let len = js.len.into_usize();
        Vec::from_raw_parts(ptr, len, len).into_boxed_slice()
    }
}

impl<T: ErasableGeneric<Repr = JsValue> + WasmDescribe> VectorIntoWasmAbi for T {
    type Abi = WasmSlice;

    #[inline]
    fn vector_into_abi(vector: Box<[T]>) -> WasmSlice {
        let ptr = vector.as_ptr();
        let len = vector.len();
        mem::forget(vector);
        WasmSlice::from_usize(ptr as usize, len)
    }
}

unsafe impl<'a, T: ErasableGeneric> ErasableGeneric for &'a [T]
where
    <T as ErasableGeneric>::Repr: 'a,
{
    type Repr = &'a [T::Repr];
}

impl<'a, T, Target> UpcastFrom<&'a [T]> for &'a [Target] where Target: UpcastFrom<T> {}

// A borrowed number, `bool`, `char` or borrowed string crosses by copy, so
// `&T`, `Option<&T>` and `Nullable<&T>` have `T`'s ABIs, as generic code
// borrowing an element (`contains(&T)`) needs for every `T`.
macro_rules! ref_by_copy {
    ($($t:ty => $base:ty,)*) => ($(
        impl IntoWasmAbi for &$t {
            type Abi = <$base as IntoWasmAbi>::Abi;

            #[inline]
            fn into_abi(self) -> Self::Abi {
                (*self).into_abi()
            }
        }

        impl MaybeIntoWasmAbi for &$t {
            type MaybeAbi = <$base as MaybeIntoWasmAbi>::MaybeAbi;

            #[inline]
            fn into_maybe_abi(self) -> Self::MaybeAbi {
                (*self).into_maybe_abi()
            }

            #[inline]
            fn none() -> Self::MaybeAbi {
                <$base as MaybeIntoWasmAbi>::none()
            }
        }

        impl NullableIntoWasmAbi for &$t {
            #[inline]
            fn null() -> Self::MaybeAbi {
                <$base as NullableIntoWasmAbi>::null()
            }
        }
    )*)
}

ref_by_copy! {
    u8 => u8, i8 => i8, u16 => u16, i16 => i16, u32 => u32, i32 => i32,
    u64 => u64, i64 => i64, u128 => u128, i128 => i128, usize => usize,
    isize => isize, f32 => f32, f64 => f64, bool => bool, char => char,
    &str => &'static str, &String => &'static String,
    &Box<str> => &'static Box<str>, &Cow<'_, str> => &'static Cow<'static, str>,
}

impl<'a, T: WasmDescribe> IntoWasmAbi for &'a Option<T>
where
    Option<&'a T>: IntoWasmAbi,
{
    type Abi = <Option<&'a T> as IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self.as_ref().into_abi()
    }
}

impl<'a, T: WasmDescribe> IntoWasmAbi for &'a Nullable<T>
where
    Nullable<&'a T>: IntoWasmAbi,
{
    type Abi = <Nullable<&'a T> as IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self.as_ref().into_abi()
    }
}

impl<'a, T: WasmDescribe> OptionIntoWasmAbi for &'a Nullable<T>
where
    Nullable<&'a T>: OptionIntoWasmAbi,
{
    #[inline]
    fn none() -> Self::Abi {
        <Nullable<&'a T> as OptionIntoWasmAbi>::none()
    }
}
