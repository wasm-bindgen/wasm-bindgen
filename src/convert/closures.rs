#![allow(clippy::fn_to_numeric_cast)]

use core::mem;

use crate::convert::slices::WasmSlice;
use crate::convert::RefFromWasmAbi;
use crate::convert::{FromWasmAbi, IntoWasmAbi, ReturnWasmAbi, WasmAbi, WasmRet};
use crate::describe::{SerializedDescriptor, WasmDescribe, FUNCTION};
use crate::throw_str;

#[repr(C)]
#[allow(non_snake_case)]
pub struct FuncDescriptor<ArgDescriptors, R: ReturnWasmAbi> {
    tag: u32,
    invoke_fn: *const (),
    count: u32,
    arg_descriptors: ArgDescriptors,
    ret_and_inner: [R::Descriptor; 2],
}

unsafe impl<ArgDescriptors, R: ReturnWasmAbi> SerializedDescriptor
    for FuncDescriptor<ArgDescriptors, R>
{
}

impl<ArgDescriptors, R: ReturnWasmAbi> FuncDescriptor<ArgDescriptors, R> {
    pub const fn new(invoke_fn: *const (), count: u32, arg_descriptors: ArgDescriptors) -> Self {
        Self {
            tag: FUNCTION,
            invoke_fn,
            count,
            arg_descriptors,
            ret_and_inner: [R::DESCRIPTOR, R::DESCRIPTOR],
        }
    }
}

macro_rules! stack_closures {
    (@count_one $var:ty) => (1);

    (@func_descriptor $invoke:ident $($var:ty)*) => {
        type Descriptor = FuncDescriptor<($(<$var as WasmDescribe>::Descriptor,)*), R>;

        const DESCRIPTOR: Self::Descriptor = FuncDescriptor::new(
            $invoke::<$($var,)* R> as *const (),
            (0 $(+ stack_closures!(@count_one $var))*),
            ($(<$var as WasmDescribe>::DESCRIPTOR,)*)
        );
    };

    ($( ($($var:ident $arg1:ident $arg2:ident $arg3:ident $arg4:ident)*) )*) => ($(const _: () = {
        #[allow(coherence_leak_check)]
        impl<$($var,)* R> IntoWasmAbi for &'_ (dyn Fn($($var),*) -> R + '_)
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
                throw_str("closure invoked after being dropped");
            }
            // Scope all local variables before we call `return_abi` to
            // ensure they're all destroyed as `return_abi` may throw
            let ret = {
                let f: &dyn Fn($($var),*) -> R = mem::transmute((a, b));
                $(
                    let $var = <$var as FromWasmAbi>::from_abi($var::Abi::join($arg1, $arg2, $arg3, $arg4));
                )*
                f($($var),*)
            };
            ret.return_abi().into()
        }

        #[allow(coherence_leak_check)]
        impl<$($var,)* R> WasmDescribe for dyn Fn($($var),*) -> R + '_
            where $($var: FromWasmAbi,)*
                  R: ReturnWasmAbi
        {
            stack_closures!(@func_descriptor invoke $($var)*);
        }

        #[allow(coherence_leak_check)]
        impl<$($var,)* R> IntoWasmAbi for &'_ mut (dyn FnMut($($var),*) -> R + '_)
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
        unsafe extern "C" fn invoke_mut<$($var: FromWasmAbi,)* R: ReturnWasmAbi>(
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
                let f: &mut dyn FnMut($($var),*) -> R = mem::transmute((a, b));
                $(
                    let $var = <$var as FromWasmAbi>::from_abi($var::Abi::join($arg1, $arg2, $arg3, $arg4));
                )*
                f($($var),*)
            };
            ret.return_abi().into()
        }

        #[allow(coherence_leak_check)]
        impl<$($var,)* R> WasmDescribe for dyn FnMut($($var),*) -> R + '_
            where $($var: FromWasmAbi,)*
                  R: ReturnWasmAbi
        {
            stack_closures!(@func_descriptor invoke_mut $($var)*);
        }
    };)*)
}

stack_closures! {
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

impl<'a, A, R> WasmDescribe for dyn Fn(&A) -> R + 'a
where
    A: RefFromWasmAbi,
    R: ReturnWasmAbi,
{
    type Descriptor = FuncDescriptor<(<&'a A as WasmDescribe>::Descriptor,), R>;

    const DESCRIPTOR: Self::Descriptor = FuncDescriptor::new(
        invoke1_ref::<A, R> as *const (),
        1,
        (<&A as WasmDescribe>::DESCRIPTOR,),
    );
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

impl<'a, A, R> WasmDescribe for dyn FnMut(&A) -> R + 'a
where
    A: RefFromWasmAbi,
    R: ReturnWasmAbi,
{
    type Descriptor = FuncDescriptor<(<&'a A as WasmDescribe>::Descriptor,), R>;

    const DESCRIPTOR: Self::Descriptor = FuncDescriptor::new(
        invoke1_mut_ref::<A, R> as *const (),
        1,
        (<&A as WasmDescribe>::DESCRIPTOR,),
    );
}
