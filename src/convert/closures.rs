#![allow(clippy::fn_to_numeric_cast)]

use core::mem;

use crate::convert::slices::WasmSlice;
use crate::convert::RefFromWasmAbi;
use crate::convert::{FromWasmAbi, IntoWasmAbi, ReturnWasmAbi, WasmAbi, WasmRet};
use crate::describe::{inform, WasmDescribe, FUNCTION};
use crate::throw_str;

macro_rules! stack_closures {
    ($Fn:ident $($mut:ident)? $cnt:literal $($var:ident $arg1:ident $arg2:ident $arg3:ident $arg4:ident)*) => (const _: () = {
        #[allow(coherence_leak_check)]
        impl<$($var,)* R> IntoWasmAbi for &'_ $($mut)? (dyn $Fn($($var),*) -> R + '_)
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
                let f: & $($mut)? dyn $Fn($($var),*) -> R = mem::transmute((a, b));
                $(
                    let $var = <$var as FromWasmAbi>::from_abi($var::Abi::join($arg1, $arg2, $arg3, $arg4));
                )*
                f($($var),*)
            };
            ret.return_abi().into()
        }

        #[allow(coherence_leak_check)]
        impl<$($var,)* R> WasmDescribe for dyn $Fn($($var),*) -> R + '_
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
    };);

    ($( ($cnt:literal $($var:ident $arg1:ident $arg2:ident $arg3:ident $arg4:ident)*) )*) => (
        $(
            stack_closures!(Fn $cnt $($var $arg1 $arg2 $arg3 $arg4)*);
            stack_closures!(FnMut mut $cnt $($var $arg1 $arg2 $arg3 $arg4)*);
        )*
    );
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

impl<A, R> IntoWasmAbi for &(dyn Fn(&A) -> R + '_)
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

#[allow(non_snake_case)]
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
        throw_str("closure invoked after being dropped");
    }
    // Scope all local variables before we call `return_abi` to
    // ensure they're all destroyed as `return_abi` may throw
    let ret = {
        let f: &dyn Fn(&A) -> R = mem::transmute((a, b));
        let arg = <A as RefFromWasmAbi>::ref_from_abi(A::Abi::join(arg1, arg2, arg3, arg4));
        f(&*arg)
    };
    ret.return_abi().into()
}

impl<A, R> WasmDescribe for dyn Fn(&A) -> R + '_
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

impl<A, R> IntoWasmAbi for &mut (dyn FnMut(&A) -> R + '_)
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

#[allow(non_snake_case)]
#[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
unsafe extern "C" fn invoke1_mut_ref<A: RefFromWasmAbi, R: ReturnWasmAbi>(
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
        let f: &mut dyn FnMut(&A) -> R = mem::transmute((a, b));
        let arg = <A as RefFromWasmAbi>::ref_from_abi(A::Abi::join(arg1, arg2, arg3, arg4));
        f(&*arg)
    };
    ret.return_abi().into()
}

impl<A, R> WasmDescribe for dyn FnMut(&A) -> R + '_
where
    A: RefFromWasmAbi,
    R: ReturnWasmAbi,
{
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(FUNCTION);
        inform(invoke1_mut_ref::<A, R> as usize as u32);
        inform(1);
        <&A as WasmDescribe>::describe();
        <R as WasmDescribe>::describe();
        <R as WasmDescribe>::describe();
    }
}
