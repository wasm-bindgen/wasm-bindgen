#![allow(clippy::fn_to_numeric_cast)]

use alloc::boxed::Box;
use core::mem;

use crate::closure::{Closure, IntoWasmClosure, WasmClosure, WasmClosureFnOnce};
use crate::convert::slices::WasmSlice;
use crate::convert::RefFromWasmAbi;
use crate::convert::{FromWasmAbi, IntoWasmAbi, ReturnWasmAbi, WasmAbi, WasmRet};
use crate::describe::{inform, WasmDescribe, FUNCTION};
use crate::throw_str;
use crate::JsValue;
use crate::UnwrapThrowExt;

macro_rules! closures {
    ($Fn:ident $FnArgs:tt $is_mut:literal $($mut:ident)? $cnt:literal $($var:ident $arg1:ident $arg2:ident $arg3:ident $arg4:ident)*) => (const _: () = {
        #[allow(coherence_leak_check)]
        impl<$($var,)* R> IntoWasmAbi for &'_ $($mut)? (dyn $Fn $FnArgs -> R + '_)
            where $($var: FromWasmAbi,)*
                  R: ReturnWasmAbi
        {
            type Abi = WasmSlice;

            fn into_abi(self) -> WasmSlice {
                unsafe {
                    let (a, b): (usize, usize) = mem::transmute(self);
                    WasmSlice { ptr: a as u32, len: b as u32 }
                }
            }
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn invoke<$($var: FromWasmAbi,)* R: ReturnWasmAbi>(
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
            // Scope all local variables before we call `return_abi` to
            // ensure they're all destroyed as `return_abi` may throw
            let ret = {
                let f: & $($mut)? dyn $Fn $FnArgs -> R = mem::transmute((a, b));
                $(
                    let $var = <$var as FromWasmAbi>::from_abi($var::Abi::join($arg1, $arg2, $arg3, $arg4));
                )*
                f($($var),*)
            };
            ret.return_abi().into()
        }

        #[allow(coherence_leak_check)]
        impl<$($var,)* R> WasmDescribe for dyn $Fn $FnArgs -> R + '_
            where $($var: FromWasmAbi,)*
                  R: ReturnWasmAbi
        {
            #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
            fn describe() {
                inform(FUNCTION);
                inform(invoke::<$($var,)* R> as usize as u32);
                inform($cnt);
                $(<$var as WasmDescribe>::describe();)*
                <R as WasmDescribe>::describe();
                <R as WasmDescribe>::describe();
            }
        }

        #[allow(coherence_leak_check)]
        unsafe impl<$($var,)* R> WasmClosure for dyn $Fn $FnArgs -> R + 'static
            where $($var: FromWasmAbi + 'static,)*
                  R: ReturnWasmAbi + 'static,
        {
            const IS_MUT: bool = $is_mut;
        }

        impl<T, $($var,)* R> IntoWasmClosure<dyn $Fn $FnArgs -> R> for T
            where T: 'static + $Fn $FnArgs -> R,
                  $($var: FromWasmAbi + 'static,)*
                  R: ReturnWasmAbi + 'static,
        {
            fn unsize(self: Box<Self>) -> Box<dyn $Fn $FnArgs -> R> { self }
        }
    };);

    ($( ($cnt:literal $($var:ident $arg1:ident $arg2:ident $arg3:ident $arg4:ident)*) )*) => (
        $(
            closures!(Fn($($var),*) false $cnt $($var $arg1 $arg2 $arg3 $arg4)*);
            closures!(FnMut($($var),*) true mut $cnt $($var $arg1 $arg2 $arg3 $arg4)*);

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
            impl<T, $($var,)* R> WasmClosureFnOnce<($($var),*), R> for T
                where T: 'static + FnOnce($($var),*) -> R,
                    $($var: FromWasmAbi + 'static,)*
                    R: ReturnWasmAbi + 'static
            {
                type FnMut = dyn FnMut($($var),*) -> R;

                fn into_fn_mut(self) -> Box<Self::FnMut> {
                    let mut me = Some(self);
                    Box::new(move |$($var),*| {
                        let me = me.take().expect_throw("FnOnce called more than once");
                        me($($var),*)
                    })
                }

                fn into_js_function(self) -> JsValue {
                    use alloc::rc::Rc;
                    use crate::__rt::WasmRefCell;

                    let mut me = Some(self);

                    let rc1 = Rc::new(WasmRefCell::new(None));
                    let rc2 = rc1.clone();

                    let closure = Closure::wrap(Box::new(move |$($var),*| {
                        // Invoke ourself and get the result.
                        let me = me.take().expect_throw("FnOnce called more than once");
                        let result = me($($var),*);

                        // And then drop the `Rc` holding this function's `Closure`
                        // alive.
                        debug_assert_eq!(Rc::strong_count(&rc2), 1);
                        let option_closure = rc2.borrow_mut().take();
                        debug_assert!(option_closure.is_some());
                        drop(option_closure);

                        result
                    }) as Box<dyn FnMut($($var),*) -> R>);

                    let js_val = closure.as_ref().clone();

                    *rc1.borrow_mut() = Some(closure);
                    debug_assert_eq!(Rc::strong_count(&rc1), 2);
                    drop(rc1);

                    js_val
                }
            }
        )*
    );
}

closures! {
    (0)
    (1 A a1 a2 a3 a4)
    (2 A a1 a2 a3 a4 B b1 b2 b3 b4)
    (3 A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4)
    (4 A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4)
    (5 A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4 E e1 e2 e3 e4)
    (6 A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4 E e1 e2 e3 e4 F f1 f2 f3 f4)
    (7 A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4 E e1 e2 e3 e4 F f1 f2 f3 f4 G g1 g2 g3 g4)
    (8 A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4 E e1 e2 e3 e4 F f1 f2 f3 f4 G g1 g2 g3 g4 H h1 h2 h3 h4)
}

// Copy the above impls down here for where there's only one argument and it's a
// reference. We could add more impls for more kinds of references, but it
// becomes a combinatorial explosion quickly. Let's see how far we can get with
// just this one! Maybe someone else can figure out voodoo so we don't have to
// duplicate.

macro_rules! single_ref_closures {
    ($Fn:ident $is_mut:literal) => {
        const _: () = {
            #[allow(coherence_leak_check)]
            unsafe impl<A, R> WasmClosure for dyn $Fn(&A) -> R
            where
                A: RefFromWasmAbi,
                R: ReturnWasmAbi + 'static,
            {
                const IS_MUT: bool = $is_mut;
            }

            #[allow(coherence_leak_check)]
            impl<A, R> IntoWasmAbi for &mut (dyn $Fn(&A) -> R + '_)
            where
                A: RefFromWasmAbi,
                R: ReturnWasmAbi,
            {
                type Abi = WasmSlice;

                fn into_abi(self) -> WasmSlice {
                    unsafe {
                        let (a, b): (usize, usize) = mem::transmute(self);
                        WasmSlice {
                            ptr: a as u32,
                            len: b as u32,
                        }
                    }
                }
            }

            #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
            unsafe extern "C" fn invoke1_ref<A: RefFromWasmAbi, R: ReturnWasmAbi>(
                a: usize,
                b: usize,
                arg1: <A::Abi as WasmAbi>::Prim1,
                arg2: <A::Abi as WasmAbi>::Prim2,
                arg3: <A::Abi as WasmAbi>::Prim3,
                arg4: <A::Abi as WasmAbi>::Prim4,
            ) -> WasmRet<R::Abi> {
                if a == 0 {
                    throw_str("closure invoked recursively or after being dropped");
                }
                // Scope all local variables before we call `return_abi` to
                // ensure they're all destroyed as `return_abi` may throw
                let ret = {
                    let f: &mut dyn $Fn(&A) -> R = mem::transmute((a, b));
                    let arg =
                        <A as RefFromWasmAbi>::ref_from_abi(A::Abi::join(arg1, arg2, arg3, arg4));
                    f(&*arg)
                };
                ret.return_abi().into()
            }

            #[allow(coherence_leak_check)]
            impl<A, R> WasmDescribe for dyn $Fn(&A) -> R + '_
            where
                A: RefFromWasmAbi,
                R: ReturnWasmAbi,
            {
                #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
                fn describe() {
                    inform(FUNCTION);
                    inform(invoke1_ref::<A, R> as usize as u32);
                    inform(1);
                    <&A as WasmDescribe>::describe();
                    <R as WasmDescribe>::describe();
                    <R as WasmDescribe>::describe();
                }
            }
        };
    };
}

single_ref_closures!(Fn false);
single_ref_closures!(FnMut true);
