#![allow(clippy::fn_to_numeric_cast)]

use alloc::boxed::Box;
use core::mem;

use crate::closure::{Closure, IntoWasmClosure, WasmClosure, WasmClosureFnOnce};
use crate::convert::slices::WasmSlice;
use crate::convert::ArgFromWasmAbi;
use crate::convert::{IntoWasmAbi, ReturnWasmAbi, WasmAbi, WasmRet};
use crate::describe::{inform, WasmDescribe, FUNCTION};
use crate::throw_str;
use crate::FromWasmAbi;
use crate::JsValue;
use crate::UnwrapThrowExt;

type Ref<'f, F> = &'f F;
type RefMut<'f, F> = &'f mut F;
type RefOnce<'f, F> = Box<F>;

macro_rules! stack_closures {
    (@ $ref:ident $Fn:ident $cnt:tt $($var:ident $arg1:ident $arg2:ident $arg3:ident $arg4:ident)*) => {const _: () = {
        #[allow(coherence_leak_check)]
        impl<$($var,)* R> WasmDescribe for dyn for<'a> $Fn($($var),*) -> R + '_
        where
            $($var: ArgFromWasmAbi<false>,)*
            R: ReturnWasmAbi,
        {
            #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
            fn describe() {
                inform(FUNCTION);
                inform(invoke::<$($var,)* R> as usize as u32);
                inform($cnt);
                $($var::describe();)*
                R::describe();
                R::describe();
            }
        }

        impl<$($var,)* R> WasmClosure for dyn for<'a> $Fn($($var),*) -> R + '_
        where
            $($var: ArgFromWasmAbi<false>,)*
            R: ReturnWasmAbi,
        {
            const IS_MUT: bool = false;
        }

        #[allow(coherence_leak_check)]
        impl<'f, $($var,)* R> IntoWasmAbi for $ref<'f, dyn for<'a> $Fn($($var),*) -> R + '_>
        where
            $($var: 'f + ArgFromWasmAbi<false>,)*
            R: ReturnWasmAbi,
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
        unsafe extern "C" fn invoke<$($var: ArgFromWasmAbi<false>,)* R: ReturnWasmAbi>(
            a: usize,
            b: usize,
            $(
            $arg1: <<$var::Anchor as FromWasmAbi>::Abi as WasmAbi>::Prim1,
            $arg2: <<$var::Anchor as FromWasmAbi>::Abi as WasmAbi>::Prim2,
            $arg3: <<$var::Anchor as FromWasmAbi>::Abi as WasmAbi>::Prim3,
            $arg4: <<$var::Anchor as FromWasmAbi>::Abi as WasmAbi>::Prim4,
            )*
        ) -> WasmRet<R::Abi> {
            if a == 0 {
                throw_str("closure invoked after being dropped");
            }
            let f: $ref<'_, dyn $Fn($($var::SameButOver<'_>),*) -> R + '_> = mem::transmute((a, b));
            $(
                let mut $var = <$var::Anchor as FromWasmAbi>::from_abi_prims($arg1, $arg2, $arg3, $arg4);
            )*
            let ret = f($(
                $var::arg_from_anchor(&mut $var),
            )*);
            ret.return_abi().into()
        }

        impl<T, $($var,)* R> IntoWasmClosure<dyn for<'a> $Fn($($var),*) -> R> for T
        where
            $($var: ArgFromWasmAbi<false>,)*
            T: for<'a> $Fn($($var),*) -> R + 'static,
        {
            fn unsize(self: Box<Self>) -> Box<dyn for<'a> $Fn($($var),*) -> R> { self }
        }
    };};

    ($(($cnt:tt $($var:ident $arg1:ident $arg2:ident $arg3:ident $arg4:ident)*))*) => ($(
        stack_closures!(@ Ref Fn $cnt $($var $arg1 $arg2 $arg3 $arg4)*);
        stack_closures!(@ RefMut FnMut $cnt $($var $arg1 $arg2 $arg3 $arg4)*);
        stack_closures!(@ RefOnce FnOnce $cnt $($var $arg1 $arg2 $arg3 $arg4)*);
    )*);
}

stack_closures! {
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

// // The memory safety in the implementation below is a bit tricky. We
// // want to be able to drop the `Closure` object from within the invocation of a
// // `Closure` for cases like promises. That means that while it's running we
// // might drop the `Closure`, but that shouldn't invalidate the environment yet.
// //
// // Instead what we do is to wrap closures in `Rc` variables. The main `Closure`
// // has a strong reference count which keeps the trait object alive. Each
// // invocation of a closure then *also* clones this and gets a new reference
// // count. When the closure returns it will release the reference count.
// //
// // This means that if the main `Closure` is dropped while it's being invoked
// // then destruction is deferred until execution returns. Otherwise it'll
// // deallocate data immediately.
// #[allow(non_snake_case, unused_parens)]
// impl<T, $($var,)* R> WasmClosureFnOnce<dyn for<'a> FnOnce($($var),*) -> R + 'static> for T
// where
//     T: for<'a> FnOnce($($var),*) -> R + 'static,
//     $($var: 'static + ArgFromWasmAbi<false>,)*
//     R: ReturnWasmAbi,
//     dyn for<'a> FnMut($($var),*) -> R + 'static: WasmClosure,
// {
//     fn into_fn_mut(self) -> Box<dyn for<'a> FnMut($($var),*) -> R + 'static> {
//         let mut me = Some(self);
//         Box::new(move |$($var),*| {
//             let me = me.take().expect_throw("FnOnce called more than once");
//             me($($var),*)
//         })
//     }

//     fn into_js_function(self) -> JsValue {
//         use alloc::rc::Rc;
//         use crate::__rt::WasmRefCell;

//         let mut me = Some(self);

//         let rc1 = Rc::new(WasmRefCell::new(None));
//         let rc2 = rc1.clone();

//         fn constrain<
//             T: for<'a> FnMut($($var),*) -> R + 'static,
//             $($var: 'static + ArgFromWasmAbi<false>,)*
//             R: ReturnWasmAbi,
//         >(f: T) -> T {
//             f
//         }

//         let closure = Closure::new(constrain::<_, $($var,)* R>(move |$($var),*| {
//             // Invoke ourself and get the result.
//             let me = me.take().expect_throw("FnOnce called more than once");
//             let result = me($($var),*);

//             // And then drop the `Rc` holding this function's `Closure`
//             // alive.
//             debug_assert_eq!(Rc::strong_count(&rc2), 1);
//             let option_closure = rc2.borrow_mut().take();
//             debug_assert!(option_closure.is_some());
//             drop(option_closure);

//             result
//         }));

//         let js_val = closure.as_ref().clone();

//         *rc1.borrow_mut() = Some(closure);
//         debug_assert_eq!(Rc::strong_count(&rc1), 2);
//         drop(rc1);

//         js_val
//     }
// }
