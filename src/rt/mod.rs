use crate::convert::{FromWasmAbi, IntoWasmAbi, WasmAbi, WasmRet};

use crate::JsValue;
#[cfg(all(target_family = "wasm", feature = "std", panic = "unwind"))]
use core::any::Any;
use core::borrow::{Borrow, BorrowMut};
#[cfg(target_feature = "atomics")]
use core::cell::UnsafeCell;
use core::cell::{Cell, RefCell};
use core::convert::Infallible;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::panic::{RefUnwindSafe, UnwindSafe};
#[cfg(target_feature = "atomics")]
use core::sync::atomic::{AtomicU8, Ordering};

use alloc::alloc::{alloc, dealloc, realloc, Layout};
use alloc::boxed::Box;
use alloc::rc::Rc;
use once_cell::unsync::Lazy;

pub extern crate alloc;
pub extern crate core;
#[cfg(feature = "std")]
pub extern crate std;

pub mod marker;

pub use wasm_bindgen_macro::BindgenedStruct;

// Re-export the descriptor section entry-kind discriminator bytes for the
// `#[wasm_bindgen]` macro expansion. The macro refers to these by absolute
// path (`::wasm_bindgen::__rt::DESCRIPTOR_KIND_REGULAR`) so they need to
// live somewhere reachable from user crates that depend on this one.
pub use wasm_bindgen_shared::{
    DESCRIPTOR_KIND_CAST, DESCRIPTOR_KIND_REGULAR, DESCRIPTOR_KIND_STATIC,
};

/// Wrapper implementation for JsValue errors, with atomics and std handling
pub fn js_panic(err: JsValue) {
    #[cfg(all(feature = "std", not(target_feature = "atomics")))]
    ::std::panic::panic_any(err);
    #[cfg(not(all(feature = "std", not(target_feature = "atomics"))))]
    ::core::panic!("{:?}", err);
}

// Cast between arbitrary wasm-bindgen-ABI types by going through JS.
//
// At compile time each `wbg_cast::<From, To>` instantiates a fresh
// `breaks_if_inlined::<From, To>` that does nothing useful — it just
// calls the marker import `__wbindgen_describe_cast` with five
// `i32.const` immediates:
//
//   * pointer + length of `<From as WasmDescribe>::SCHEMA_BUF[..SCHEMA_LEN]`
//   * pointer + length of `<To as WasmDescribe>::SCHEMA_BUF[..SCHEMA_LEN]`
//   * an optional invoke address (used by `wbg_cast_closure` only;
//     `null` for plain `wbg_cast`)
//
// `wasm-bindgen-cli` finds each `breaks_if_inlined` by looking for
// callers of `__wbindgen_describe_cast`, reads the five immediates
// structurally (a narrow scanner — no wasm interpretation),
// reconstructs the cast's `(From, To)` descriptor from the schema
// bytes in the data segment, synthesises a JS adapter, and replaces
// every call to `breaks_if_inlined` with a call to the import the
// JS adapter is bound to.
pub fn wbg_cast<From, To>(value: From) -> To
where
    From: IntoWasmAbi + crate::describe::WasmDescribe,
    To: FromWasmAbi + crate::describe::WasmDescribe,
{
    // Keep the helper's address-of present in the module so LLVM
    // treats it as escaping and cannot run interprocedural argument
    // elimination on it (which would strip unused prims from the
    // wasm-level signature the cli scanner inspects).
    let _keepalive: unsafe extern "C" fn(_, _, _, _) -> _ = breaks_if_inlined::<From, To>;
    core::hint::black_box(_keepalive);
    let (prim1, prim2, prim3, prim4) = value.into_abi().split();
    unsafe {
        let result = breaks_if_inlined::<From, To>(prim1, prim2, prim3, prim4);
        To::from_abi(result.join())
    }
}

/// Closure-cast variant: passes the per-`(T, UNWIND_SAFE)`
/// monomorphisation invoke shim's address into the marker call so
/// the cli scanner can resolve the function-table slot. `T` and
/// `UNWIND_SAFE` are baked into the `breaks_if_inlined_closure`
/// instantiation so the address folds to a single `i32.const` in
/// the cast body.
pub fn wbg_cast_closure<From, To, T, const UNWIND_SAFE: bool>(value: From) -> To
where
    From: IntoWasmAbi + crate::describe::WasmDescribe,
    To: FromWasmAbi + crate::describe::WasmDescribe,
    T: crate::closure::WasmClosure + ?Sized,
{
    let _keepalive: unsafe extern "C" fn(_, _, _, _) -> _ =
        breaks_if_inlined_closure::<From, To, T, UNWIND_SAFE>;
    core::hint::black_box(_keepalive);
    let (prim1, prim2, prim3, prim4) = value.into_abi().split();
    unsafe {
        let result =
            breaks_if_inlined_closure::<From, To, T, UNWIND_SAFE>(prim1, prim2, prim3, prim4);
        To::from_abi(result.join())
    }
}

// Per-import name carrier for `#[wasm_bindgen(generic)]` imported
// functions. The macro emits one zero-sized type per imported function
// implementing this trait so the JS import name folds to a stable
// rodata address inside the per-`(import, T)` courier monomorphisation.
pub trait GenericImportName {
    /// The import's shim name — the key the cli uses to recover this
    /// import's metadata (js_name, module, namespace, catch, variadic, …)
    /// from the normal AST custom section. Not re-encoded here.
    const SHIM: &'static str;
    /// Byte length of `SHIM` (const so it folds to an `i32.const`).
    const SHIM_LEN: usize;
    /// The shared signature *template* (the "descriptor"): a full function
    /// descriptor stream `[FUNCTION, 0, nargs, slots.., ret, inner_ret]`
    /// where each generic-parameter position is a `TYPE_PARAM(i)` hole.
    /// Emitted once per import (same address for every monomorphisation)
    /// and spliced with the per-`T` fills by the cli.
    const TEMPLATE: [u32; crate::describe::SCHEMA_MAX];
    /// Meaningful prefix length of `TEMPLATE`.
    const TEMPLATE_LEN: usize;
}

// Bare-minimal call-site courier for a single owned-argument, unit-return
// `#[wasm_bindgen(generic)]` imported function. Mirrors `wbg_cast`: the
// public wrapper splits the argument's ABI into prims and calls the
// `#[inline(never)]` courier, whose body deposits the import name and the
// concrete argument schema as `i32.const` immediates for the cli scanner.
//
// `N` keys the JS import name (fixed per imported function); `T` keys the
// concrete argument type (varies per monomorphisation). The courier's wasm
// signature equals the named JS import's signature, so the cli can rewrite
// the call site directly to the synthesised import.
pub fn wbg_generic_import_1<N, T>(value: T)
where
    N: GenericImportName,
    T: IntoWasmAbi + crate::describe::WasmDescribe,
{
    let _keepalive: unsafe extern "C" fn(_, _, _, _) = breaks_if_inlined_generic_import::<N, T>;
    core::hint::black_box(_keepalive);
    let (prim1, prim2, prim3, prim4) = value.into_abi().split();
    unsafe {
        breaks_if_inlined_generic_import::<N, T>(prim1, prim2, prim3, prim4);
    }
}

#[inline(never)]
#[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
unsafe extern "C" fn breaks_if_inlined_generic_import<N, T>(
    prim1: <T::Abi as WasmAbi>::Prim1,
    prim2: <T::Abi as WasmAbi>::Prim2,
    prim3: <T::Abi as WasmAbi>::Prim3,
    prim4: <T::Abi as WasmAbi>::Prim4,
) where
    N: GenericImportName,
    T: IntoWasmAbi + crate::describe::WasmDescribe,
{
    super::__wbindgen_describe_generic_import(
        N::SHIM.as_ptr(),
        N::SHIM_LEN,
        N::TEMPLATE.as_ptr(),
        N::TEMPLATE_LEN,
        FromBuf::<T>::BUF.as_ptr(),
        <T as crate::describe::WasmDescribe>::SCHEMA_LEN,
    );
    keep_prims_alive(prim1, prim2, prim3, prim4);
}

// Schema-buffer forwarders: re-expose a generic type's `SCHEMA_BUF`
// associated const through a non-generic static so we can hand a
// stable address to the cli. Wasm-ld resolves `&FromBuf::<F>::BUF` to
// an `i32.const` pointing into the data segment.
struct FromBuf<F: crate::describe::WasmDescribe + ?Sized>(core::marker::PhantomData<F>);
impl<F: crate::describe::WasmDescribe + ?Sized> FromBuf<F> {
    const BUF: [u32; crate::describe::SCHEMA_MAX] = F::SCHEMA_BUF;
}
struct ToBuf<T: crate::describe::WasmDescribe + ?Sized>(core::marker::PhantomData<T>);
impl<T: crate::describe::WasmDescribe + ?Sized> ToBuf<T> {
    const BUF: [u32; crate::describe::SCHEMA_MAX] = T::SCHEMA_BUF;
}

#[inline(never)]
#[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
unsafe extern "C" fn breaks_if_inlined<From, To>(
    prim1: <From::Abi as WasmAbi>::Prim1,
    prim2: <From::Abi as WasmAbi>::Prim2,
    prim3: <From::Abi as WasmAbi>::Prim3,
    prim4: <From::Abi as WasmAbi>::Prim4,
) -> WasmRet<To::Abi>
where
    From: IntoWasmAbi + crate::describe::WasmDescribe,
    To: FromWasmAbi + crate::describe::WasmDescribe,
{
    super::__wbindgen_describe_cast(
        FromBuf::<From>::BUF.as_ptr(),
        <From as crate::describe::WasmDescribe>::SCHEMA_LEN,
        ToBuf::<To>::BUF.as_ptr(),
        <To as crate::describe::WasmDescribe>::SCHEMA_LEN,
        core::ptr::null(),
    );
    // The cli rewrites this whole function to a JS adapter import
    // before it ever runs. Force each prim and the return slot to
    // be observably used via volatile reads/writes so the optimiser
    // can't dead-eliminate them from the wasm-level signature, even
    // when individual prims or `WasmRet<To::Abi>` are zero-sized
    // (e.g. `To = ()`).
    keep_prims_alive(prim1, prim2, prim3, prim4);
    make_ret::<To>()
}

#[inline(never)]
#[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
unsafe extern "C" fn breaks_if_inlined_closure<
    From,
    To,
    T: crate::closure::WasmClosure + ?Sized,
    const UNWIND_SAFE: bool,
>(
    prim1: <From::Abi as WasmAbi>::Prim1,
    prim2: <From::Abi as WasmAbi>::Prim2,
    prim3: <From::Abi as WasmAbi>::Prim3,
    prim4: <From::Abi as WasmAbi>::Prim4,
) -> WasmRet<To::Abi>
where
    From: IntoWasmAbi + crate::describe::WasmDescribe,
    To: FromWasmAbi + crate::describe::WasmDescribe,
{
    super::__wbindgen_describe_cast(
        FromBuf::<From>::BUF.as_ptr(),
        <From as crate::describe::WasmDescribe>::SCHEMA_LEN,
        ToBuf::<To>::BUF.as_ptr(),
        <To as crate::describe::WasmDescribe>::SCHEMA_LEN,
        T::invoke_shim_addr::<UNWIND_SAFE>(),
    );
    keep_prims_alive(prim1, prim2, prim3, prim4);
    make_ret::<To>()
}

/// Force each `prim` argument to be observably consumed via volatile
/// writes so the optimiser preserves them in the wasm-level signature
/// of the caller. Without this, when `WasmRet<To::Abi>` is zero-sized
/// (e.g. `To = ()`), the parameters look unused after dead-store
/// elimination and the function's wasm signature collapses to
/// `() -> ()`, breaking the cli-support assumption that the helper's
/// signature carries the cast ABI.
///
/// The function body is never executed at runtime: the cli rewrites
/// the containing `breaks_if_inlined*` into a JS adapter import.
#[inline(always)]
#[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
unsafe fn keep_prims_alive<P1, P2, P3, P4>(p1: P1, p2: P2, p3: P3, p4: P4) {
    let mut scratch = core::mem::MaybeUninit::<(P1, P2, P3, P4)>::uninit();
    core::ptr::write_volatile(scratch.as_mut_ptr(), (p1, p2, p3, p4));
}

/// Manufacture a return value of type `WasmRet<To::Abi>` whose bytes
/// are sourced through a volatile read so the optimiser cannot
/// statically prove the value is uninit, dead, or zero-sized-and-
/// thus-empty. This preserves the wasm-level return signature of the
/// containing `breaks_if_inlined*` function across all `To`s. The
/// containing function is never executed at runtime — the cli rewrites
/// it into a JS adapter import.
#[inline(always)]
#[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
unsafe fn make_ret<To>() -> WasmRet<To::Abi>
where
    To: FromWasmAbi + crate::describe::WasmDescribe,
{
    let scratch = core::mem::MaybeUninit::<WasmRet<To::Abi>>::uninit();
    core::ptr::read_volatile(scratch.as_ptr())
}

pub(crate) const JSIDX_OFFSET: u32 = 1024; // keep in sync with js/mod.rs
pub(crate) const JSIDX_UNDEFINED: u32 = JSIDX_OFFSET;
pub(crate) const JSIDX_NULL: u32 = JSIDX_OFFSET + 1;
pub(crate) const JSIDX_TRUE: u32 = JSIDX_OFFSET + 2;
pub(crate) const JSIDX_FALSE: u32 = JSIDX_OFFSET + 3;
pub(crate) const JSIDX_RESERVED: u32 = JSIDX_OFFSET + 4;

pub(crate) struct ThreadLocalWrapper<T>(pub(crate) T);

#[cfg(not(target_feature = "atomics"))]
unsafe impl<T> Sync for ThreadLocalWrapper<T> {}

#[cfg(not(target_feature = "atomics"))]
unsafe impl<T> Send for ThreadLocalWrapper<T> {}

/// Wrapper around [`Lazy`] adding `Send + Sync` when `atomics` is not enabled.
pub struct LazyCell<T, F = fn() -> T>(ThreadLocalWrapper<Lazy<T, F>>);

impl<T, F> LazyCell<T, F> {
    pub const fn new(init: F) -> LazyCell<T, F> {
        Self(ThreadLocalWrapper(Lazy::new(init)))
    }
}

impl<T, F: FnOnce() -> T> LazyCell<T, F> {
    pub fn force(this: &Self) -> &T {
        &this.0 .0
    }
}

impl<T> Deref for LazyCell<T> {
    type Target = T;

    fn deref(&self) -> &T {
        ::once_cell::unsync::Lazy::force(&self.0 .0)
    }
}

#[cfg(not(target_feature = "atomics"))]
pub use LazyCell as LazyLock;

#[cfg(target_feature = "atomics")]
pub struct LazyLock<T, F = fn() -> T> {
    state: AtomicU8,
    data: UnsafeCell<Data<T, F>>,
}

#[cfg(target_feature = "atomics")]
enum Data<T, F> {
    Value(T),
    Init(F),
}

#[cfg(target_feature = "atomics")]
impl<T, F> LazyLock<T, F> {
    const STATE_UNINIT: u8 = 0;
    const STATE_INITIALIZING: u8 = 1;
    const STATE_INIT: u8 = 2;

    pub const fn new(init: F) -> LazyLock<T, F> {
        Self {
            state: AtomicU8::new(Self::STATE_UNINIT),
            data: UnsafeCell::new(Data::Init(init)),
        }
    }
}

#[cfg(target_feature = "atomics")]
impl<T> Deref for LazyLock<T> {
    type Target = T;

    fn deref(&self) -> &T {
        let mut state = self.state.load(Ordering::Acquire);

        loop {
            match state {
                Self::STATE_INIT => {
                    let Data::Value(value) = (unsafe { &*self.data.get() }) else {
                        unreachable!()
                    };
                    return value;
                }
                Self::STATE_UNINIT => {
                    if let Err(new_state) = self.state.compare_exchange_weak(
                        Self::STATE_UNINIT,
                        Self::STATE_INITIALIZING,
                        Ordering::Acquire,
                        Ordering::Relaxed,
                    ) {
                        state = new_state;
                        continue;
                    }

                    let data = unsafe { &mut *self.data.get() };
                    let Data::Init(init) = data else {
                        unreachable!()
                    };
                    *data = Data::Value(init());
                    self.state.store(Self::STATE_INIT, Ordering::Release);
                    state = Self::STATE_INIT;
                }
                Self::STATE_INITIALIZING => {
                    // TODO: Block here if possible. This would require
                    // detecting if we can in the first place.
                    state = self.state.load(Ordering::Acquire);
                }
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(target_feature = "atomics")]
unsafe impl<T, F: Sync> Sync for LazyLock<T, F> {}

#[cfg(target_feature = "atomics")]
unsafe impl<T, F: Send> Send for LazyLock<T, F> {}

#[macro_export]
#[doc(hidden)]
#[cfg(not(target_feature = "atomics"))]
macro_rules! __wbindgen_thread_local {
    ($wasm_bindgen:tt, $actual_ty:ty) => {{
        static _VAL: $wasm_bindgen::__rt::LazyCell<$actual_ty> =
            $wasm_bindgen::__rt::LazyCell::new(init);
        $wasm_bindgen::JsThreadLocal { __inner: &_VAL }
    }};
}

#[macro_export]
#[doc(hidden)]
#[cfg(target_feature = "atomics")]
#[allow_internal_unstable(thread_local)]
macro_rules! __wbindgen_thread_local {
    ($wasm_bindgen:tt, $actual_ty:ty) => {{
        #[thread_local]
        static _VAL: $wasm_bindgen::__rt::LazyCell<$actual_ty> =
            $wasm_bindgen::__rt::LazyCell::new(init);
        $wasm_bindgen::JsThreadLocal {
            __inner: || unsafe { $wasm_bindgen::__rt::LazyCell::force(&_VAL) as *const $actual_ty },
        }
    }};
}

#[macro_export]
#[doc(hidden)]
#[cfg(not(wasm_bindgen_unstable_test_coverage))]
macro_rules! __wbindgen_coverage {
    ($item:item) => {
        $item
    };
}

#[macro_export]
#[doc(hidden)]
#[cfg(wasm_bindgen_unstable_test_coverage)]
#[allow_internal_unstable(coverage_attribute)]
macro_rules! __wbindgen_coverage {
    ($item:item) => {
        #[coverage(off)]
        $item
    };
}

#[inline]
pub fn assert_not_null<T>(s: *mut T) {
    if s.is_null() {
        throw_null();
    }
}

#[cfg(target_arch = "wasm64")]
pub type WasmWordRepr = f64;
#[cfg(not(target_arch = "wasm64"))]
pub type WasmWordRepr = u32;

/// Signed counterpart of [`WasmWordRepr`]. Used for `isize` ABI lowering so
/// that negative values sign-extend correctly when widening to `f64`.
#[cfg(target_arch = "wasm64")]
pub type WasmSignedWordRepr = f64;
#[cfg(not(target_arch = "wasm64"))]
pub type WasmSignedWordRepr = i32;

/// Coerce a raw `*const T` into the `WasmWordRepr` ABI value expected by
/// the named ptr-bearing intrinsics (`__wbindgen_string_new`, the typed
/// array constructors, etc).
#[inline]
pub fn ptr_to_word<T>(p: *const T) -> WasmWordRepr {
    #[cfg(target_arch = "wasm64")]
    {
        p as usize as f64
    }
    #[cfg(not(target_arch = "wasm64"))]
    {
        p as usize as u32
    }
}

/// Coerce a `usize` length into `WasmWordRepr`.
#[inline]
pub fn len_to_word(len: usize) -> WasmWordRepr {
    #[cfg(target_arch = "wasm64")]
    {
        len as f64
    }
    #[cfg(not(target_arch = "wasm64"))]
    {
        len as u32
    }
}

/// A single pointer-sized machine word using the JS-number ABI on wasm64.
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct WasmWord(WasmWordRepr);

impl WasmWord {
    #[inline]
    pub fn from_usize(value: usize) -> Self {
        #[cfg(target_arch = "wasm64")]
        {
            Self(value as f64)
        }
        #[cfg(not(target_arch = "wasm64"))]
        {
            Self(value as u32)
        }
    }

    #[inline]
    pub fn into_usize(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn from_isize(value: isize) -> Self {
        #[cfg(target_arch = "wasm64")]
        {
            Self(value as f64)
        }
        #[cfg(not(target_arch = "wasm64"))]
        {
            Self(value as u32)
        }
    }

    #[inline]
    pub fn into_isize(self) -> isize {
        self.0 as isize
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        #[cfg(target_arch = "wasm64")]
        {
            self.0 == 0.0
        }
        #[cfg(not(target_arch = "wasm64"))]
        {
            self.0 == 0
        }
    }
}

impl WasmAbi for WasmWord {
    type Prim1 = WasmWordRepr;
    type Prim2 = ();
    type Prim3 = ();
    type Prim4 = ();

    #[inline]
    fn split(self) -> (Self::Prim1, (), (), ()) {
        (self.0, (), (), ())
    }

    #[inline]
    fn join(prim1: Self::Prim1, _: (), _: (), _: ()) -> Self {
        Self(prim1)
    }
}

/// A typed raw pointer using the JS-number ABI on wasm64.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct WasmPtr<T> {
    word: WasmWord,
    _marker: PhantomData<*mut T>,
}

impl<T> Default for WasmPtr<T> {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl<T> WasmPtr<T> {
    #[inline]
    pub fn from_ptr(ptr: *mut T) -> Self {
        Self::from_usize(ptr as usize)
    }

    #[inline]
    pub fn into_ptr(self) -> *mut T {
        self.into_usize() as *mut T
    }

    #[inline]
    pub fn from_usize(value: usize) -> Self {
        Self {
            word: WasmWord::from_usize(value),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn into_usize(self) -> usize {
        self.word.into_usize()
    }

    #[inline]
    pub fn null() -> Self {
        Self::from_usize(0)
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        self.word.is_zero()
    }
}

impl<T> WasmAbi for WasmPtr<T> {
    type Prim1 = <WasmWord as WasmAbi>::Prim1;
    type Prim2 = ();
    type Prim3 = ();
    type Prim4 = ();

    #[inline]
    fn split(self) -> (Self::Prim1, (), (), ()) {
        self.word.split()
    }

    #[inline]
    fn join(prim1: Self::Prim1, _: (), _: (), _: ()) -> Self {
        Self {
            word: WasmWord::join(prim1, (), (), ()),
            _marker: PhantomData,
        }
    }
}

// The wasm64 representation of `WasmWord` is `f64`, which has enough
// mantissa precision to roundtrip any in-range pointer value. On wasm32
// the representation is `u32`, so this test is only meaningful on wasm64.
#[cfg(all(test, target_arch = "wasm64"))]
mod tests {
    use super::WasmWord;

    #[test]
    fn wasm_word_roundtrips_large_pointer_values() {
        let value = 1usize << 60;
        assert_eq!(WasmWord::from_usize(value).into_usize(), value);

        let signed = -(1isize << 40);
        assert_eq!(WasmWord::from_isize(signed).into_isize(), signed);
    }
}

#[cold]
#[inline(never)]
fn throw_null() -> ! {
    super::throw_str("null pointer passed to rust");
}

/// A wrapper around the `RefCell` from the standard library.
///
/// Now why, you may ask, would we do that? Surely `RefCell` in libstd is
/// quite good. And you're right, it is indeed quite good! Functionally
/// nothing more is needed from `RefCell` in the standard library but for
/// now this crate is also sort of optimizing for compiled code size.
///
/// One major factor to larger binaries in Rust is when a panic happens.
/// Panicking in the standard library involves a fair bit of machinery
/// (formatting, panic hooks, synchronization, etc). It's all worthwhile if
/// you need it but for something like `WasmRefCell` here we don't actually
/// need all that!
///
/// This is just a wrapper around all Rust objects passed to JS intended to
/// guard accidental reentrancy, so this vendored version is intended solely
/// to not panic in libstd. Instead when it "panics" it calls our `throw`
/// function in this crate which raises an error in JS.
pub struct WasmRefCell<T: ?Sized> {
    inner: RefCell<T>,
}

impl<T: ?Sized> UnwindSafe for WasmRefCell<T> {}
impl<T: ?Sized> RefUnwindSafe for WasmRefCell<T> {}

impl<T: ?Sized> WasmRefCell<T> {
    pub fn new(value: T) -> WasmRefCell<T>
    where
        T: Sized,
    {
        WasmRefCell {
            inner: RefCell::new(value),
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        match self.inner.try_borrow() {
            Ok(inner) => Ref { inner },
            Err(_) => borrow_fail(),
        }
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        match self.inner.try_borrow_mut() {
            Ok(inner) => RefMut { inner },
            Err(_) => borrow_fail(),
        }
    }

    pub fn into_inner(self) -> T
    where
        T: Sized,
    {
        self.inner.into_inner()
    }
}

pub struct Ref<'b, T: ?Sized + 'b> {
    inner: core::cell::Ref<'b, T>,
}

impl<T: ?Sized> Deref for Ref<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T: ?Sized> Borrow<T> for Ref<'_, T> {
    #[inline]
    fn borrow(&self) -> &T {
        self
    }
}

pub struct RefMut<'b, T: ?Sized + 'b> {
    inner: core::cell::RefMut<'b, T>,
}

impl<T: ?Sized> Deref for RefMut<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T: ?Sized> DerefMut for RefMut<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: ?Sized> Borrow<T> for RefMut<'_, T> {
    #[inline]
    fn borrow(&self) -> &T {
        self
    }
}

impl<T: ?Sized> BorrowMut<T> for RefMut<'_, T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}

#[cfg(panic = "unwind")]
fn borrow_fail() -> ! {
    panic!(
        "recursive use of an object detected which would lead to \
		 unsafe aliasing in rust",
    )
}

#[cfg(not(panic = "unwind"))]
fn borrow_fail() -> ! {
    super::throw_str(
        "recursive use of an object detected which would lead to \
		 unsafe aliasing in rust",
    );
}

/// A type that encapsulates an `Rc<WasmRefCell<T>>` as well as a `Ref`
/// to the contents of that `WasmRefCell`.
///
/// The `'static` requirement is an unfortunate consequence of how this
/// is implemented.
pub struct RcRef<T: ?Sized + 'static> {
    // The 'static is a lie.
    //
    // We could get away without storing this, since we're in the same module as
    // `WasmRefCell` and can directly manipulate its `borrow`, but I'm considering
    // turning it into a wrapper around `std`'s `RefCell` to reduce `unsafe` in
    // which case that would stop working. This also requires less `unsafe` as is.
    //
    // It's important that this goes before `Rc` so that it gets dropped first.
    ref_: Ref<'static, T>,
    _rc: Rc<WasmRefCell<T>>,
}

impl<T: ?Sized> UnwindSafe for RcRef<T> {}

impl<T: ?Sized> RcRef<T> {
    pub fn new(rc: Rc<WasmRefCell<T>>) -> Self {
        let ref_ = unsafe { (*Rc::as_ptr(&rc)).borrow() };
        Self { _rc: rc, ref_ }
    }
}

impl<T: ?Sized> Deref for RcRef<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.ref_
    }
}

impl<T: ?Sized> Borrow<T> for RcRef<T> {
    #[inline]
    fn borrow(&self) -> &T {
        &self.ref_
    }
}

/// A type that encapsulates an `Rc<WasmRefCell<T>>` as well as a
/// `RefMut` to the contents of that `WasmRefCell`.
///
/// The `'static` requirement is an unfortunate consequence of how this
/// is implemented.
pub struct RcRefMut<T: ?Sized + 'static> {
    ref_: RefMut<'static, T>,
    _rc: Rc<WasmRefCell<T>>,
}

impl<T: ?Sized> RcRefMut<T> {
    pub fn new(rc: Rc<WasmRefCell<T>>) -> Self {
        let ref_ = unsafe { (*Rc::as_ptr(&rc)).borrow_mut() };
        Self { _rc: rc, ref_ }
    }
}

impl<T: ?Sized> Deref for RcRefMut<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.ref_
    }
}

impl<T: ?Sized> DerefMut for RcRefMut<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.ref_
    }
}

impl<T: ?Sized> Borrow<T> for RcRefMut<T> {
    #[inline]
    fn borrow(&self) -> &T {
        &self.ref_
    }
}

impl<T: ?Sized> BorrowMut<T> for RcRefMut<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.ref_
    }
}

#[no_mangle]
pub extern "C" fn __wbindgen_malloc(size: WasmWord, align: WasmWord) -> WasmPtr<u8> {
    let size = size.into_usize();
    let align = align.into_usize();
    if let Ok(layout) = Layout::from_size_align(size, align) {
        unsafe {
            if layout.size() > 0 {
                let ptr = alloc(layout);
                if !ptr.is_null() {
                    return WasmPtr::from_ptr(ptr);
                }
            } else {
                return WasmPtr::from_usize(align);
            }
        }
    }

    malloc_failure();
}

#[no_mangle]
pub unsafe extern "C" fn __wbindgen_realloc(
    ptr: WasmPtr<u8>,
    old_size: WasmWord,
    new_size: WasmWord,
    align: WasmWord,
) -> WasmPtr<u8> {
    let ptr = ptr.into_ptr();
    let old_size = old_size.into_usize();
    let new_size = new_size.into_usize();
    let align = align.into_usize();
    debug_assert!(old_size > 0);
    debug_assert!(new_size > 0);
    if let Ok(layout) = Layout::from_size_align(old_size, align) {
        let ptr = realloc(ptr, layout, new_size);
        if !ptr.is_null() {
            return WasmPtr::from_ptr(ptr);
        }
    }
    malloc_failure();
}

#[cold]
fn malloc_failure() -> ! {
    cfg_if::cfg_if! {
        if #[cfg(debug_assertions)] {
            super::throw_str("invalid malloc request")
        } else if #[cfg(feature = "std")] {
            std::process::abort();
        } else if #[cfg(target_arch = "wasm32")] {
            // stable
            core::arch::wasm32::unreachable();
        } else if #[cfg(target_arch = "wasm64")] {
            // unstable, need simd_wasm64 feature
            core::arch::wasm64::unreachable();
        } else {
            unreachable!()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn __wbindgen_free(ptr: WasmPtr<u8>, size: WasmWord, align: WasmWord) {
    let size = size.into_usize();
    // This happens for zero-length slices, and in that case `ptr` is
    // likely bogus so don't actually send this to the system allocator
    if size == 0 {
        return;
    }
    let ptr = ptr.into_ptr();
    let align = align.into_usize();
    let layout = Layout::from_size_align_unchecked(size, align);
    dealloc(ptr, layout);
}

/// This is a curious function necessary to get wasm-bindgen working today,
/// and it's a bit of an unfortunate hack.
///
/// The general problem is that somehow we need the above two symbols to
/// exist in the final output binary (__wbindgen_malloc and
/// __wbindgen_free). These symbols may be called by JS for various
/// bindings, so we for sure need to make sure they're exported.
///
/// The problem arises, though, when what if no Rust code uses the symbols?
/// For all intents and purposes it looks to LLVM and the linker like the
/// above two symbols are dead code, so they're completely discarded!
///
/// Specifically what happens is this:
///
/// * The above two symbols are generated into some object file inside of
///   libwasm_bindgen.rlib
/// * The linker, LLD, will not load this object file unless *some* symbol
///   is loaded from the object. In this case, if the Rust code never calls
///   __wbindgen_malloc or __wbindgen_free then the symbols never get linked
///   in.
/// * Later when `wasm-bindgen` attempts to use the symbols they don't
///   exist, causing an error.
///
/// This function is a weird hack for this problem. We inject a call to this
/// function in all generated code. Usage of this function should then
/// ensure that the above two intrinsics are translated.
///
/// Due to how rustc creates object files this function (and anything inside
/// it) will be placed into the same object file as the two intrinsics
/// above. That means if this function is called and referenced we'll pull
/// in the object file and link the intrinsics.
///
/// Ideas for how to improve this are most welcome!
#[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
pub fn link_mem_intrinsics() {
    crate::link::link_intrinsics();
}

#[cfg_attr(target_feature = "atomics", thread_local)]
static GLOBAL_EXNDATA: ThreadLocalWrapper<Cell<[u32; 2]>> = ThreadLocalWrapper(Cell::new([0; 2]));

#[cfg(panic = "unwind")]
#[no_mangle]
pub static mut __instance_terminated: u32 = 0;

/// Stores the Wasm indirect-function-table index of the registered hard-abort
/// callback.  Zero means no callback is registered.
#[cfg(panic = "unwind")]
#[no_mangle]
pub static mut __abort_handler: u32 = 0;

/// Register a callback invoked when a hard abort (instance termination) occurs.
///
/// Returns the previously registered handler, or `None` if none was set.
/// This mirrors the `std::panic::set_hook` convention and lets callers chain
/// or restore handlers.
///
/// The callback fires after the terminated flag is set, so any re-entrant
/// export call from within the handler is immediately blocked.  A throwing
/// or panicking handler cannot suppress the original error.
///
/// **Experimental — only available when built with `panic=unwind`.**
/// On `panic=abort` builds the no-op stub always returns `None` and the
/// callback will never fire.
#[cfg(panic = "unwind")]
pub fn set_on_abort(f: fn()) -> Option<fn()> {
    // On wasm32, function pointers are indices into the Wasm
    // __indirect_function_table. Casting fn() -> usize -> u32 extracts
    // that index without touching linear memory.
    unsafe {
        let prev = __abort_handler;
        __abort_handler = f as usize as u32;
        if prev != 0 {
            Some(core::mem::transmute::<usize, fn()>(prev as usize))
        } else {
            None
        }
    }
}

/// No-op stub for `panic=abort` builds — handler will never fire.
#[cfg(not(panic = "unwind"))]
pub fn set_on_abort(_f: fn()) -> Option<fn()> {
    None
}

/// Schedule the instance for reinitialization before the next export call.
///
/// The reinit machinery is automatically emitted when this function is used.
/// Works with both `panic=unwind` and `panic=abort` builds.
pub fn schedule_reinit() {
    crate::__wbindgen_reinit();
}

#[no_mangle]
pub unsafe extern "C" fn __wbindgen_exn_store(idx: u32) {
    debug_assert_eq!(GLOBAL_EXNDATA.0.get()[0], 0);
    GLOBAL_EXNDATA.0.set([1, idx]);
}

pub fn take_last_exception() -> Result<(), super::JsValue> {
    let ret = if GLOBAL_EXNDATA.0.get()[0] == 1 {
        Err(super::JsValue::_new(GLOBAL_EXNDATA.0.get()[1]))
    } else {
        Ok(())
    };
    GLOBAL_EXNDATA.0.set([0, 0]);
    ret
}

/// An internal helper trait for usage in `#[wasm_bindgen]` on `async`
/// functions to convert the return value of the function to
/// `Result<JsValue, JsValue>` which is what we'll return to JS (where an
/// error is a failed future).
pub trait IntoJsResult {
    fn into_js_result(self) -> Result<JsValue, JsValue>;
}

impl IntoJsResult for () {
    fn into_js_result(self) -> Result<JsValue, JsValue> {
        Ok(JsValue::undefined())
    }
}

impl<T: Into<JsValue>> IntoJsResult for T {
    fn into_js_result(self) -> Result<JsValue, JsValue> {
        Ok(self.into())
    }
}

impl<T: Into<JsValue>, E: Into<JsValue>> IntoJsResult for Result<T, E> {
    fn into_js_result(self) -> Result<JsValue, JsValue> {
        match self {
            Ok(e) => Ok(e.into()),
            Err(e) => Err(e.into()),
        }
    }
}

impl<E: Into<JsValue>> IntoJsResult for Result<(), E> {
    fn into_js_result(self) -> Result<JsValue, JsValue> {
        match self {
            Ok(()) => Ok(JsValue::undefined()),
            Err(e) => Err(e.into()),
        }
    }
}

/// An internal helper trait for usage in `#[wasm_bindgen(start)]`
/// functions to throw the error (if it is `Err`).
pub trait Start {
    fn start(self);
}

impl Start for () {
    #[inline]
    fn start(self) {}
}

impl<E: Into<JsValue>> Start for Result<(), E> {
    #[inline]
    fn start(self) {
        if let Err(e) = self {
            crate::throw_val(e.into());
        }
    }
}

/// An internal helper struct for usage in `#[wasm_bindgen(main)]`
/// functions to throw the error (if it is `Err`).
pub struct MainWrapper<T>(pub Option<T>);

pub trait Main {
    fn __wasm_bindgen_main(&mut self);
}

impl Main for &mut &mut MainWrapper<()> {
    #[inline]
    fn __wasm_bindgen_main(&mut self) {}
}

impl Main for &mut &mut MainWrapper<Infallible> {
    #[inline]
    fn __wasm_bindgen_main(&mut self) {}
}

impl<E: Into<JsValue>> Main for &mut &mut MainWrapper<Result<(), E>> {
    #[inline]
    fn __wasm_bindgen_main(&mut self) {
        if let Err(e) = self.0.take().unwrap() {
            crate::throw_val(e.into());
        }
    }
}

impl<E: core::fmt::Debug> Main for &mut MainWrapper<Result<(), E>> {
    #[inline]
    fn __wasm_bindgen_main(&mut self) {
        if let Err(e) = self.0.take().unwrap() {
            crate::throw_str(&alloc::format!("{e:?}"));
        }
    }
}

pub const fn flat_len<T, const SIZE: usize>(slices: [&[T]; SIZE]) -> usize {
    let mut len = 0;
    let mut i = 0;
    while i < slices.len() {
        len += slices[i].len();
        i += 1;
    }
    len
}

pub const fn flat_byte_slices<const RESULT_LEN: usize, const SIZE: usize>(
    slices: [&[u8]; SIZE],
) -> [u8; RESULT_LEN] {
    let mut result = [0; RESULT_LEN];

    let mut slice_index = 0;
    let mut result_offset = 0;

    while slice_index < slices.len() {
        let mut i = 0;
        let slice = slices[slice_index];
        while i < slice.len() {
            result[result_offset] = slice[i];
            i += 1;
            result_offset += 1;
        }
        slice_index += 1;
    }

    result
}

// NOTE: This method is used to encode u32 into a variable-length-integer during the compile-time .
// Generally speaking, the length of the encoded variable-length-integer depends on the size of the integer
// but the maximum capacity can be used here to simplify the amount of code during the compile-time .
pub const fn encode_u32_to_fixed_len_bytes(value: u32) -> [u8; 5] {
    let mut result: [u8; 5] = [0; 5];
    let mut i = 0;
    while i < 4 {
        result[i] = ((value >> (7 * i)) | 0x80) as u8;
        i += 1;
    }
    result[4] = (value >> (7 * 4)) as u8;
    result
}

#[cfg(all(target_family = "wasm", feature = "std", panic = "unwind"))]
#[wasm_bindgen_macro::wasm_bindgen(wasm_bindgen = crate, raw_module = "__wbindgen_placeholder__")]
extern "C" {
    fn __wbindgen_panic_error(msg: &JsValue) -> JsValue;
}

#[cfg(all(target_family = "wasm", feature = "std", panic = "unwind"))]
pub fn panic_to_panic_error(val: std::boxed::Box<dyn Any + Send>) -> JsValue {
    #[cfg(not(target_feature = "atomics"))]
    {
        if let Some(s) = val.downcast_ref::<JsValue>() {
            return __wbindgen_panic_error(&s);
        }
    }
    let maybe_panic_msg: Option<&str> = if let Some(s) = val.downcast_ref::<&str>() {
        Some(s)
    } else if let Some(s) = val.downcast_ref::<std::string::String>() {
        Some(s)
    } else {
        None
    };
    let err: JsValue = __wbindgen_panic_error(&JsValue::from_str(
        maybe_panic_msg.unwrap_or("No panic message available"),
    ));
    err
}

#[cfg(all(target_family = "wasm", feature = "std", panic = "unwind"))]
pub fn maybe_catch_unwind<F: FnOnce() -> R + std::panic::UnwindSafe, R>(f: F) -> R {
    let result = std::panic::catch_unwind(f);
    match result {
        Ok(val) => val,
        Err(e) => {
            crate::throw_val(panic_to_panic_error(e));
        }
    }
}

#[cfg(not(all(target_family = "wasm", feature = "std", panic = "unwind")))]
pub fn maybe_catch_unwind<F: FnOnce() -> R, R>(f: F) -> R {
    f()
}

/// Compile-time requirement that `T: RefUnwindSafe` under `panic = "unwind"`.
///
/// Emitted by `#[wasm_bindgen]` for the receiver type of `&self` / `&mut self`
/// methods and the pointee of `&T` / `&mut T` arguments on exported functions.
///
/// Stdlib's `&mut T: !UnwindSafe` blanket means we cannot use the closure's
/// own `UnwindSafe` bound to validate user types — every `&mut self` method
/// would fail unconditionally. Instead, this requires the *logical* unwind
/// safety property (`T: RefUnwindSafe`): the user's type must not contain
/// interior mutability whose invariants could be silently broken when a
/// panic is caught by [`maybe_catch_unwind`]. This is the same property
/// `RefCell`, `Cell`, `Mutex` advertise when refusing to implement
/// `RefUnwindSafe`.
///
/// Users whose type is genuinely safe to observe after a caught panic can
/// opt in with `impl RefUnwindSafe for MyType {}` or by wrapping interior-
/// mutable fields in `std::panic::AssertUnwindSafe`.
///
/// No-op outside `panic = "unwind"` builds (where panics abort instead).
#[cfg(all(target_family = "wasm", feature = "std", panic = "unwind"))]
#[inline(always)]
pub fn ensure_ref_unwind_safe<T: ?Sized + std::panic::RefUnwindSafe>() {}

#[cfg(not(all(target_family = "wasm", feature = "std", panic = "unwind")))]
#[inline(always)]
pub fn ensure_ref_unwind_safe<T: ?Sized>() {}

/// Compile-time requirement that `T: UnwindSafe` under `panic = "unwind"`.
///
/// Used for owned receiver / argument types where the value is consumed
/// inside the catch boundary; mirrors [`ensure_ref_unwind_safe`] but for
/// owned-value contexts where `UnwindSafe` (rather than `RefUnwindSafe`)
/// is the relevant property.
///
/// No-op outside `panic = "unwind"` builds.
#[cfg(all(target_family = "wasm", feature = "std", panic = "unwind"))]
#[inline(always)]
pub fn ensure_unwind_safe<T: ?Sized + std::panic::UnwindSafe>() {}

#[cfg(not(all(target_family = "wasm", feature = "std", panic = "unwind")))]
#[inline(always)]
pub fn ensure_unwind_safe<T: ?Sized>() {}

/// Trait for element types to implement `Into<JsValue>` for vectors of
/// themselves, which isn't possible directly thanks to the orphan rule.
///
/// This trait is restored from the pre-wbg_cast world: each element
/// type provides its own per-monomorphisation conversion to a JS
/// array, calling the appropriate descriptor-section intrinsic
/// (`__wbindgen_array_new` + `__wbindgen_array_push` for the
/// JsValue-element case, typed-array constructors for primitives).
///
/// Implementations are provided by the `#[wasm_bindgen]` macro for
/// user structs / enums, and by this crate directly for `JsValue`,
/// `String`, and the primitive numeric types.
pub trait VectorIntoJsValue: Sized {
    fn vector_into_jsvalue(vector: Box<[Self]>) -> JsValue;
}

impl<T: VectorIntoJsValue> From<Box<[T]>> for JsValue {
    fn from(vector: Box<[T]>) -> Self {
        T::vector_into_jsvalue(vector)
    }
}

/// Default implementation strategy for `Vec<T>` where `T: Into<JsValue>`:
/// build an empty JS Array and push each value. Used by the
/// `VectorIntoJsValue` impls for `JsValue`, `String`, and macro-emitted
/// impls for user struct vectors.
pub fn js_value_vector_into_jsvalue<T: Into<JsValue>>(vector: Box<[T]>) -> JsValue {
    let result = unsafe { JsValue::_new(super::__wbindgen_array_new()) };
    for value in vector.into_vec() {
        let js: JsValue = value.into();
        unsafe { super::__wbindgen_array_push(result.idx, js.idx) }
        // `__wbindgen_array_push` takes ownership of `js` already.
        core::mem::forget(js);
    }
    result
}
