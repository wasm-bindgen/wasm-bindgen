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
    js_value_vector_from_abi, js_value_vector_into_abi, FromWasmAbi, IntoWasmAbi,
    LongRefFromWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi, RefFromWasmAbi, RefMutFromWasmAbi,
    UpcastFrom, VectorFromWasmAbi, VectorIntoWasmAbi, WasmAbi,
};
use crate::describe::*;
use crate::{JsError, JsValue};

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
                borrowed_slice_into_abi(slice)
            }
        }

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

/// How a borrowed `&[Self]` crosses to JS: each element's own ABI, `len`
/// times.
///
/// Where an element's memory already is its ABI (numbers, and JS handles,
/// which are table indices) that is the slice's own memory, borrowed with no
/// copy ([`borrowed_slice_into_abi`]). Any other element is encoded into a
/// fresh buffer of its ABIs, which JS decodes and frees
/// ([`encoded_slice_into_abi`]).
pub trait VectorRefIntoWasmAbi: WasmDescribe + Sized {
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

/// [`VectorRefIntoWasmAbi`] for an element whose memory is its ABI.
#[inline]
pub fn borrowed_slice_into_abi<T>(slice: &[T]) -> WasmSlice {
    WasmSlice::from_usize(slice.as_ptr() as usize, slice.len())
}

/// [`VectorRefIntoWasmAbi`] for any other element, encoding each with
/// `encode`.
pub fn encoded_slice_into_abi<'a, T, A: WasmAbi>(
    slice: &'a [T],
    encode: impl FnMut(&'a T) -> A,
) -> WasmSlice {
    let abis: Box<[A]> = slice.iter().map(encode).collect();
    let len = abis.len();
    WasmSlice::from_usize(Box::into_raw(abis) as *mut A as usize, len)
}

/// [`VectorFromWasmAbi`] for an element whose memory is not its ABI: decodes
/// each element from the buffer of their ABIs that JS encoded.
///
/// # Safety
///
/// `js` must be a buffer of `T::Abi`s allocated by JS for Rust to own.
pub unsafe fn encoded_vector_from_abi<T: FromWasmAbi>(js: WasmSlice) -> Box<[T]> {
    let ptr = js.ptr.into_usize() as *mut T::Abi;
    let len = js.len.into_usize();
    Vec::from_raw_parts(ptr, len, len)
        .into_iter()
        .map(|abi| T::from_abi(abi))
        .collect()
}

// An owned vector of elements that own nothing encodes as its borrowed slice.
macro_rules! encoded_vectors {
    ($($t:ty)*) => ($(
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
                <$t>::slice_into_abi(&vector)
            }
        }
    )*)
}

encoded_vectors! { bool char &str }

impl VectorFromWasmAbi for bool {
    type Abi = WasmSlice;

    #[inline]
    unsafe fn vector_from_abi(js: WasmSlice) -> Box<[bool]> {
        encoded_vector_from_abi(js)
    }
}

impl VectorFromWasmAbi for char {
    type Abi = WasmSlice;

    #[inline]
    unsafe fn vector_from_abi(js: WasmSlice) -> Box<[char]> {
        encoded_vector_from_abi(js)
    }
}

impl VectorRefIntoWasmAbi for String {
    fn slice_into_abi(slice: &[Self]) -> WasmSlice {
        encoded_slice_into_abi(slice, |s| s.into_abi())
    }
}

impl VectorRefIntoWasmAbi for &str {
    fn slice_into_abi(slice: &[Self]) -> WasmSlice {
        encoded_slice_into_abi(slice, |s| (*s).into_abi())
    }
}

impl VectorRefIntoWasmAbi for bool {
    fn slice_into_abi(slice: &[Self]) -> WasmSlice {
        encoded_slice_into_abi(slice, |b| (*b).into_abi())
    }
}

impl VectorRefIntoWasmAbi for char {
    fn slice_into_abi(slice: &[Self]) -> WasmSlice {
        encoded_slice_into_abi(slice, |c| (*c).into_abi())
    }
}

impl VectorRefIntoWasmAbi for JsValue {
    #[inline]
    fn slice_into_abi(slice: &[Self]) -> WasmSlice {
        borrowed_slice_into_abi(slice)
    }
}

impl VectorRefIntoWasmAbi for JsError {
    #[inline]
    fn slice_into_abi(slice: &[Self]) -> WasmSlice {
        borrowed_slice_into_abi(slice)
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

// Borrowed shapes that deref to `str` cross exactly like `&str`. `&&str` is
// omitted: the blanket `&T` describe gives `REF REF STRING`, which the CLI
// can't bind.
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

unsafe impl ErasableGeneric for &str {
    type Repr = &'static str;
}

unsafe impl<T: ErasableGeneric> ErasableGeneric for Box<[T]> {
    type Repr = Box<[T::Repr]>;
}

impl UpcastFrom<&str> for &str {}

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

impl<T: VectorFromWasmAbi> FromWasmAbi for Box<[T]> {
    type Abi = <T as VectorFromWasmAbi>::Abi;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        T::vector_from_abi(js)
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

unsafe impl<T: ErasableGeneric> ErasableGeneric for &[T] {
    type Repr = &'static [T::Repr];
}

impl<'a, T, Target> UpcastFrom<&'a [T]> for &'a [Target] where Target: UpcastFrom<T> {}
