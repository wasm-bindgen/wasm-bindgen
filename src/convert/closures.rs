use alloc::boxed::Box;
use core::mem;

#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
use crate::__rt::maybe_catch_unwind;
use crate::closure::{
    Closure, IntoWasmClosure, WasmClosure, WasmClosureFnOnce, WasmClosureFnOnceAbort,
};
use crate::convert::slices::WasmSlice;
use crate::convert::RefFromWasmAbi;
use crate::convert::{FromWasmAbi, IntoWasmAbi, ReturnWasmAbi, WasmAbi, WasmRet};
use crate::describe::{inform, WasmDescribe, FUNCTION};
use crate::throw_str;
use crate::JsValue;
use crate::UnwrapThrowExt;
#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
use core::panic::AssertUnwindSafe;

macro_rules! closures {
    // Unwind safe passing
    ([$($maybe_unwind_safe:tt)*] $($rest:tt)*) => {
        closures!(@process [$($maybe_unwind_safe)*] $($rest)*);
    };

    // One-arity recurse
    (@process [$($unwind_safe:tt)*] ($($var:ident $arg1:ident $arg2:ident $arg3:ident $arg4:ident)*) $($rest:tt)*) => {
        closures!(@impl_for_args ($($var),*) FromWasmAbi [$($unwind_safe)*] $($var::from_abi($var) => $var $arg1 $arg2 $arg3 $arg4)*);
        closures!(@process [$($unwind_safe)*] $($rest)*);
    };

    // Base case
    (@process [$($unwind_safe:tt)*]) => {};

    // A counter helper to count number of arguments.
    (@count_one $ty:ty) => (1);

    (@describe ( $($ty:ty),* )) => {
        // Needs to be a constant so that interpreter doesn't crash on
        // unsupported operations in debug mode.
        const ARG_COUNT: u32 = 0 $(+ closures!(@count_one $ty))*;
        inform(ARG_COUNT);
        $(<$ty>::describe();)*
    };

    // This silly helper is because by default Rust infers `|var_with_ref_type| ...` closure
    // as `impl Fn(&'outer_lifetime A)` instead of `impl for<'temp_lifetime> Fn(&'temp_lifetime A)`
    // while `|var_with_ref_type: &A|` makes it use the higher-order generic as expected.
    (@closure ($($ty:ty),*) $($var:ident)* $body:block) => (move |$($var: $ty),*| $body);

    (@impl_for_fn $is_mut:literal [$($mut:ident)?] $Fn:ident $FnArgs:tt $FromWasmAbi:ident $($var_expr:expr => $var:ident $arg1:ident $arg2:ident $arg3:ident $arg4:ident)*) => (const _: () = {
        impl<$($var,)* R> IntoWasmAbi for &'_ $($mut)? (dyn $Fn $FnArgs -> R + '_)
        where
            Self: WasmDescribe,
        {
            type Abi = WasmSlice;

            fn into_abi(self) -> WasmSlice {
                unsafe {
                    let (a, mut b): (usize, usize) = mem::transmute(self);
                    b |= 0x80000000;
                    WasmSlice { ptr: a as u32, len: b as u32 }
                }
            }
        }

        // Generate invoke function that checks unwind_safe flag when unwinding is available
        // unwind_safe flag is the MSV of the vtable pointer for a closure, distinguishing
        // closures which are and are not unwind safe
        #[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
        #[allow(non_snake_case)]
        unsafe extern "C-unwind" fn invoke<$($var: $FromWasmAbi,)* R: ReturnWasmAbi>(
            a: usize,
            b: usize,
            $(
            $arg1: <$var::Abi as WasmAbi>::Prim1,
            $arg2: <$var::Abi as WasmAbi>::Prim2,
            $arg3: <$var::Abi as WasmAbi>::Prim3,
            $arg4: <$var::Abi as WasmAbi>::Prim4,
            )*
        ) -> WasmRet<R::Abi> {
            if a == 0 {
                throw_str("closure invoked recursively or after being dropped");
            }
            let unwind_safe = (b & 0x80000000) != 0;
            let b = b & 0x7FFFFFFF;
            let ret = {
                let f: & $($mut)? dyn $Fn $FnArgs -> R = mem::transmute((a, b));
                $(
                    let $var = $var::Abi::join($arg1, $arg2, $arg3, $arg4);
                )*
                if unwind_safe {
                    maybe_catch_unwind(AssertUnwindSafe(|| f($($var_expr),*)))
                } else {
                    f($($var_expr),*)
                }
            };
            ret.return_abi().into()
        }

        // When unwinding is not available, generate a simple invoke function
        #[cfg(not(all(feature = "std", target_arch = "wasm32", panic = "unwind")))]
        #[allow(non_snake_case)]
        unsafe extern "C-unwind" fn invoke<$($var: $FromWasmAbi,)* R: ReturnWasmAbi>(
            a: usize,
            b: usize,
            $(
            $arg1: <$var::Abi as WasmAbi>::Prim1,
            $arg2: <$var::Abi as WasmAbi>::Prim2,
            $arg3: <$var::Abi as WasmAbi>::Prim3,
            $arg4: <$var::Abi as WasmAbi>::Prim4,
            )*
        ) -> WasmRet<R::Abi> {
            if a == 0 {
                throw_str("closure invoked recursively or after being dropped");
            }
            let ret = {
                let f: & $($mut)? dyn $Fn $FnArgs -> R = mem::transmute((a, b));
                $(
                    let $var = $var::Abi::join($arg1, $arg2, $arg3, $arg4);
                )*
                f($($var_expr),*)
            };
            ret.return_abi().into()
        }

        #[allow(clippy::fn_to_numeric_cast)]
        impl<$($var,)* R> WasmDescribe for dyn $Fn $FnArgs -> R + '_
        where
            $($var: $FromWasmAbi,)*
            R: ReturnWasmAbi,
        {
            #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
            fn describe() {
                inform(FUNCTION);
                inform(invoke::<$($var,)* R> as *const () as usize as u32);
                closures!(@describe $FnArgs);
                R::describe();
                R::describe();
            }
        }

        unsafe impl<$($var,)* R> WasmClosure for dyn $Fn $FnArgs -> R + '_
        where
            Self: WasmDescribe,
        {
            const IS_MUT: bool = $is_mut;
        }

        impl<T, $($var,)* R> IntoWasmClosure<dyn $Fn $FnArgs -> R> for T
        where
            T: 'static + $Fn $FnArgs -> R,
        {
            fn unsize(self: Box<Self>) -> Box<dyn $Fn $FnArgs -> R> { self }
        }
    };);

    (@impl_for_args $FnArgs:tt $FromWasmAbi:ident [$($maybe_unwind_safe:tt)*] $($var_expr:expr => $var:ident $arg1:ident $arg2:ident $arg3:ident $arg4:ident)*) => {
        closures!(@impl_for_fn false [] Fn $FnArgs $FromWasmAbi $($var_expr => $var $arg1 $arg2 $arg3 $arg4)*);
        closures!(@impl_for_fn true [mut] FnMut $FnArgs $FromWasmAbi $($var_expr => $var $arg1 $arg2 $arg3 $arg4)*);

        // The memory safety here in these implementations below is a bit tricky. We
        // want to be able to drop the `Closure` object from within the invocation of a
        // `Closure` for cases like promises. That means that while it's running we
        // might drop the `Closure`, but that shouldn't invalidate the environment yet.
        //
        // Instead what we do is to wrap closures in `Rc` variables. The main `Closure`
        // has a strong reference count which keeps the trait object alive. Each
        // invocation of a closure then *also* clones this and gets a new reference
        // count. When the closure returns it will release the reference count.
        //
        // This means that if the main `Closure` is dropped while it's being invoked
        // then destruction is deferred until execution returns. Otherwise it'll
        // deallocate data immediately.

        #[allow(non_snake_case, unused_parens)]
        impl<T, $($var,)* R> WasmClosureFnOnce<dyn FnMut $FnArgs -> R, $FnArgs, R> for T
        where
            T: 'static + (FnOnce $FnArgs -> R),
            $($var: $FromWasmAbi + 'static,)*
            R: ReturnWasmAbi + 'static,
            $($maybe_unwind_safe)*
        {
            fn into_fn_mut(self) -> Box<dyn FnMut $FnArgs -> R> {
                let mut me = Some(self);
                Box::new(move |$($var),*| {
                    let me = me.take().expect_throw("FnOnce called more than once");
                    me($($var),*)
                })
            }

            fn into_js_function(self) -> JsValue {
                use alloc::rc::Rc;
                use crate::__rt::WasmRefCell;

                let rc1 = Rc::new(WasmRefCell::new(None));
                let rc2 = rc1.clone();

                let closure = Closure::once(closures!(@closure $FnArgs $($var)* {
                    let result = self($($var),*);

                    // And then drop the `Rc` holding this function's `Closure`
                    // alive.
                    debug_assert_eq!(Rc::strong_count(&rc2), 1);
                    let option_closure = rc2.borrow_mut().take();
                    debug_assert!(option_closure.is_some());
                    drop(option_closure);

                    result
                }));

                let js_val = closure.as_ref().clone();

                *rc1.borrow_mut() = Some(closure);
                debug_assert_eq!(Rc::strong_count(&rc1), 2);
                drop(rc1);

                js_val
            }
        }

        #[allow(non_snake_case, unused_parens)]
        impl<T, $($var,)* R> WasmClosureFnOnceAbort<dyn FnMut $FnArgs -> R, $FnArgs, R> for T
        where
            T: 'static + (FnOnce $FnArgs -> R),
            $($var: $FromWasmAbi + 'static,)*
            R: ReturnWasmAbi + 'static,
        {
            fn into_fn_mut(self) -> Box<dyn FnMut $FnArgs -> R> {
                let mut me = Some(self);
                Box::new(move |$($var),*| {
                    let me = me.take().expect_throw("FnOnce called more than once");
                    me($($var),*)
                })
            }

            fn into_js_function(self) -> JsValue {
                use alloc::rc::Rc;
                use crate::__rt::WasmRefCell;

                let rc1 = Rc::new(WasmRefCell::new(None));
                let rc2 = rc1.clone();

                let closure = Closure::once_aborting(closures!(@closure $FnArgs $($var)* {
                    let result = self($($var),*);

                    // And then drop the `Rc` holding this function's `Closure`
                    // alive.
                    debug_assert_eq!(Rc::strong_count(&rc2), 1);
                    let option_closure = rc2.borrow_mut().take();
                    debug_assert!(option_closure.is_some());
                    drop(option_closure);

                    result
                }));

                let js_val = closure.as_ref().clone();

                *rc1.borrow_mut() = Some(closure);
                debug_assert_eq!(Rc::strong_count(&rc1), 2);
                drop(rc1);

                js_val
            }
        }
    };

    ([$($unwind_safe:tt)*] $( ($($var:ident $arg1:ident $arg2:ident $arg3:ident $arg4:ident)*) )*) => ($(
        closures!(@impl_for_args ($($var),*) FromWasmAbi [$($maybe_unwind_safe)*] $($var::from_abi($var) => $var $arg1 $arg2 $arg3 $arg4)*);
    )*);
}

#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
closures! {
    [T: core::panic::UnwindSafe,]
    ()
    (A a1 a2 a3 a4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4 E e1 e2 e3 e4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4 E e1 e2 e3 e4 F f1 f2 f3 f4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4 E e1 e2 e3 e4 F f1 f2 f3 f4 G g1 g2 g3 g4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4 E e1 e2 e3 e4 F f1 f2 f3 f4 G g1 g2 g3 g4 H h1 h2 h3 h4)
}

#[cfg(not(all(feature = "std", target_arch = "wasm32", panic = "unwind")))]
closures! {
    []
    ()
    (A a1 a2 a3 a4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4 E e1 e2 e3 e4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4 E e1 e2 e3 e4 F f1 f2 f3 f4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4 E e1 e2 e3 e4 F f1 f2 f3 f4 G g1 g2 g3 g4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4 E e1 e2 e3 e4 F f1 f2 f3 f4 G g1 g2 g3 g4 H h1 h2 h3 h4)
}

// Copy the above impls down here for where there's only one argument and it's a
// reference. We could add more impls for more kinds of references, but it
// becomes a combinatorial explosion quickly. Let's see how far we can get with
// just this one! Maybe someone else can figure out voodoo so we don't have to
// duplicate.

// We need to allow coherence leak check just for these traits because we're providing separate implementation for `Fn(&A)` variants when `Fn(A)` one already exists.
#[allow(coherence_leak_check)]
const _: () = {
    #[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
    closures!(@impl_for_args (&A) RefFromWasmAbi [T: core::panic::UnwindSafe,] &*A::ref_from_abi(A) => A a1 a2 a3 a4);

    #[cfg(not(all(feature = "std", target_arch = "wasm32", panic = "unwind")))]
    closures!(@impl_for_args (&A) RefFromWasmAbi [] &*A::ref_from_abi(A) => A a1 a2 a3 a4);
};
