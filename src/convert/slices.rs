use alloc::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::mem::{self, MaybeUninit};
use core::ops::{Deref, DerefMut};
use core::str;

use crate::__rt::{marker::ErasableGeneric, WasmWord};
use crate::__wbindgen_copy_to_typed_array;
use crate::convert::{
    FromWasmAbi, IntoWasmAbi, LongRefFromWasmAbi, MaybeIntoWasmAbi, NullableIntoWasmAbi,
    OptionFromWasmAbi, OptionIntoWasmAbi, RefFromWasmAbi, RefIntoWasmAbi, RefMutFromWasmAbi,
    UpcastFrom, VectorFromWasmAbi, VectorIntoWasmAbi, WasmAbi,
};
use crate::describe::*;
use crate::sys::{JsNullable, JsOption};
use crate::{JsValue, Nullable};

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
        uninit_vectors!($t);
    )*)
}

// A `MaybeUninit` element has no ABI of its own, so its sequences are
// declared here, crossing in place as its number's do.
macro_rules! uninit_vectors {
    ($t:ty) => {
        impl IntoWasmAbi for Box<[MaybeUninit<$t>]> {
            type Abi = WasmSlice;

            #[inline]
            fn into_abi(self) -> WasmSlice {
                let len = self.len();
                WasmSlice::from_usize(Box::into_raw(self) as *mut MaybeUninit<$t> as usize, len)
            }
        }

        impl FromWasmAbi for Box<[MaybeUninit<$t>]> {
            type Abi = WasmSlice;

            #[inline]
            unsafe fn from_abi(js: WasmSlice) -> Self {
                let ptr = js.ptr.into_usize() as *mut MaybeUninit<$t>;
                let len = js.len.into_usize();
                Vec::from_raw_parts(ptr, len, len).into_boxed_slice()
            }
        }

        impl IntoWasmAbi for &[MaybeUninit<$t>] {
            type Abi = WasmSlice;

            #[inline]
            fn into_abi(self) -> WasmSlice {
                WasmSlice::borrow(self)
            }
        }
    };
}

macro_rules! vectors_internal {
    ($t:ty) => {
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

/// A sequence crosses as a [`WasmSlice`] of `len` cells. In place, the cell is
/// the element itself, which is its JS encoding (numbers, and JS handles as
/// table indices), so a `&[T]` is borrowed and a `Vec<T>` hands its buffer
/// over. Encoded, the cell is the element's ABI, in a fresh buffer that JS
/// decodes with the element's own glue and frees: `&T`'s ABI for a `&[T]`,
/// which lends its elements, and `T`'s for a `Vec<T>`, which gives them. An
/// element is in place when it is
/// [`ABI_TRANSPARENT`](WasmDescribe::ABI_TRANSPARENT), and the sequence's
/// descriptor marks an encoded one with `ABI_CELL`.
impl<T: RefIntoWasmAbi> IntoWasmAbi for &[T] {
    type Abi = WasmSlice;

    #[inline]
    fn into_abi(self) -> WasmSlice {
        if T::ABI_TRANSPARENT.get() {
            WasmSlice::borrow(self)
        } else {
            WasmSlice::encode(self, T::ref_into_abi)
        }
    }
}

/// A borrowed vector lends its storage as the slice of it does, so a
/// `&[Vec<T>]` lends each of its vectors. It is described as the vector,
/// which JS reads by copy: a view is the unsized `&[T]` alone, as a generic
/// `&T` can pass a `&Vec<T>` to JS that keeps it.
impl<T: RefIntoWasmAbi> IntoWasmAbi for &Vec<T> {
    type Abi = WasmSlice;

    #[inline]
    fn into_abi(self) -> WasmSlice {
        self.as_slice().into_abi()
    }
}

impl<T: RefIntoWasmAbi> IntoWasmAbi for &Box<[T]> {
    type Abi = WasmSlice;

    #[inline]
    fn into_abi(self) -> WasmSlice {
        (**self).into_abi()
    }
}

impl<'a, T> OptionIntoWasmAbi for &'a [T]
where
    &'a [T]: IntoWasmAbi<Abi = WasmSlice>,
{
    #[inline]
    fn none() -> WasmSlice {
        null_slice()
    }
}

impl_nullable_into_shared_abi!(
    null = js_null_slice();
    ['a, T] &'a [T] where Self: IntoWasmAbi<Abi = WasmSlice>
);

// A `(ptr, len)` cell is two pointer words, which are `f64`s on memory64.
#[cfg(target_arch = "wasm64")]
const _: () = assert!(WasmSlice::CELL.size() == 16 && WasmSlice::CELL.align() == 8);
#[cfg(not(target_arch = "wasm64"))]
const _: () = assert!(WasmSlice::CELL.size() == 8 && WasmSlice::CELL.align() == 4);

/// A buffer of `len` cells of `A`, freed on drop.
struct Cells<A: WasmAbi> {
    ptr: *mut u8,
    len: usize,
    abi: PhantomData<A>,
}

impl<A: WasmAbi> Cells<A> {
    const HOLDS_A_WORD: () = assert!(
        A::CELL.size() != 0,
        "a sequence's element must cross as at least one word"
    );

    fn layout(len: usize) -> Layout {
        let () = Self::HOLDS_A_WORD;
        len.checked_mul(A::CELL.size())
            .and_then(|size| Layout::from_size_align(size, A::CELL.align()).ok())
            .expect("capacity overflow")
    }

    /// An empty buffer is dangling, as `__wbindgen_malloc` gives JS.
    fn new(len: usize) -> Self {
        let layout = Self::layout(len);
        let ptr = if layout.size() == 0 {
            layout.align() as *mut u8
        } else {
            let ptr = unsafe { alloc(layout) };
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            ptr
        };
        Self {
            ptr,
            len,
            abi: PhantomData,
        }
    }

    fn encode(elems: impl ExactSizeIterator<Item = A>) -> WasmSlice {
        let cells = Self::new(elems.len());
        for (i, elem) in elems.enumerate() {
            unsafe { elem.write_cell(cells.ptr.add(i * A::CELL.size())) }
        }
        let slice = WasmSlice::from_usize(cells.ptr as usize, cells.len);
        mem::forget(cells);
        slice
    }
}

impl<A: WasmAbi> Drop for Cells<A> {
    fn drop(&mut self) {
        let layout = Self::layout(self.len);
        if layout.size() != 0 {
            unsafe { dealloc(self.ptr, layout) }
        }
    }
}

impl WasmSlice {
    /// An in-place sequence: `slice`'s own memory, borrowed.
    #[inline]
    pub fn borrow<T>(slice: &[T]) -> Self {
        Self::from_usize(slice.as_ptr() as usize, slice.len())
    }

    /// An encoded sequence: a fresh buffer of each element's cell, its ABI
    /// by `f`, which JS decodes and frees.
    pub fn encode<'a, T: 'a, A: WasmAbi>(slice: &'a [T], f: impl FnMut(&'a T) -> A) -> Self {
        Cells::encode(slice.iter().map(f))
    }

    /// An encoded sequence that gives its elements: a fresh buffer of each
    /// element's cell, its ABI by `f`, which JS decodes, takes and frees.
    pub fn encode_owned<T, A: WasmAbi>(vector: Box<[T]>, f: impl FnMut(T) -> A) -> Self {
        Cells::encode(Vec::from(vector).into_iter().map(f))
    }

    /// Takes an encoded sequence from JS, converting each element from its
    /// cell's ABI with `FromWasmAbi::try_from_abi`. Every cell is received
    /// whether or not one before it was rejected or its conversion unwound,
    /// as Rust owns them all. A rejected element drops the buffer and every
    /// other element, and the first conversion to unwind goes on unwinding
    /// once the rest are received.
    ///
    /// # Safety
    ///
    /// `self` must be `len` cells of `T::Abi` that JS allocated at the cell's
    /// alignment, for Rust to own.
    pub unsafe fn try_decode<T: FromWasmAbi>(self) -> Result<Box<[T]>, &'static str> {
        let cells = Cells::<T::Abi> {
            ptr: self.ptr.into_usize() as *mut u8,
            len: self.len.into_usize(),
            abi: PhantomData,
        };
        let mut elems = Vec::with_capacity(cells.len);
        let mut rejected = None;
        let mut unwound = None;
        for i in 0..cells.len {
            let cell = cells.ptr.add(i * T::Abi::CELL.size());
            match catch_conversion(|| unsafe { T::try_from_abi(T::Abi::read_cell(cell)) }) {
                Ok(Ok(elem)) if rejected.is_none() && unwound.is_none() => elems.push(elem),
                Ok(Ok(_)) => {}
                Ok(Err(msg)) => rejected = rejected.or(Some(msg)),
                Err(payload) => unwound = unwound.or(Some(payload)),
            }
        }
        if let Some(payload) = unwound {
            drop((elems, cells));
            resume_conversion(payload);
        }
        match rejected {
            Some(msg) => Err(msg),
            None => Ok(elems.into_boxed_slice()),
        }
    }
}

cfg_if! {
    if #[cfg(all(feature = "std", panic = "unwind"))] {
        type Unwound = Box<dyn core::any::Any + Send>;

        // A guard that received the rest while unwinding would abort on a
        // second element that panics, so each conversion is caught instead.
        #[inline]
        fn catch_conversion<R>(f: impl FnOnce() -> R) -> Result<R, Unwound> {
            std::panic::catch_unwind(core::panic::AssertUnwindSafe(f))
        }

        fn resume_conversion(payload: Unwound) -> ! {
            std::panic::resume_unwind(payload)
        }
    } else {
        type Unwound = core::convert::Infallible;

        #[inline]
        fn catch_conversion<R>(f: impl FnOnce() -> R) -> Result<R, Unwound> {
            Ok(f())
        }

        fn resume_conversion(payload: Unwound) -> ! {
            match payload {}
        }
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

impl<T: IntoWasmAbi> IntoWasmAbi for Box<[T]> {
    type Abi = WasmSlice;

    #[inline]
    fn into_abi(self) -> WasmSlice {
        if T::ABI_TRANSPARENT.get() {
            let len = self.len();
            WasmSlice::from_usize(Box::into_raw(self) as *mut T as usize, len)
        } else {
            WasmSlice::encode_owned(self, T::into_abi)
        }
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

impl<T: FromWasmAbi> FromWasmAbi for Box<[T]> {
    type Abi = WasmSlice;

    #[inline]
    unsafe fn from_abi(js: WasmSlice) -> Self {
        // Thrown directly, as a closure's frames would push the caller past
        // the debug-mode error stack's frame limit.
        match Self::try_from_abi(js) {
            Ok(vector) => vector,
            Err(msg) => crate::throw_str(msg),
        }
    }

    #[inline]
    unsafe fn try_from_abi(js: WasmSlice) -> Result<Self, &'static str> {
        if T::ABI_TRANSPARENT.get() {
            let ptr = js.ptr.into_usize() as *mut T;
            let len = js.len.into_usize();
            Ok(Vec::from_raw_parts(ptr, len, len).into_boxed_slice())
        } else {
            js.try_decode()
        }
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

// The soft-deprecated vector traits are unused, and kept for the types that
// erase to `JsValue` as bindings that predate sequences crossing by their
// element's own ABI call these.
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

impl<'a, T: WasmDescribe> IntoWasmAbi for Nullable<&'a Option<T>>
where
    Nullable<Option<&'a T>>: IntoWasmAbi,
{
    type Abi = <Nullable<Option<&'a T>> as IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self.map(Option::as_ref).into_abi()
    }
}
