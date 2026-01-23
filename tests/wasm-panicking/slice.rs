#![cfg(test)]

use paste::paste;
use std::mem::MaybeUninit;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/slice.js")]
extern "C" {
    fn js_export();

    fn js_import();

    fn js_pass_array();

    fn js_export_mut();

    fn js_return_vec();

    fn js_clamped(val: Clamped<&[u8]>, offset: u8);
    #[wasm_bindgen(js_name = js_clamped)]
    fn js_clamped2(val: Clamped<Vec<u8>>, offset: u8);
    #[wasm_bindgen(js_name = js_clamped)]
    fn js_clamped3(val: Clamped<&mut [u8]>, offset: u8);

    #[wasm_bindgen(js_name = js_clamped)]
    fn js_clamped_uninit(val: Clamped<&[MaybeUninit<u8>]>, offset: u8);
    #[wasm_bindgen(js_name = js_clamped)]
    fn js_clamped2_uninit(val: Clamped<Vec<MaybeUninit<u8>>>, offset: u8);
    #[wasm_bindgen(js_name = js_clamped)]
    fn js_clamped3_uninit(val: Clamped<&mut [MaybeUninit<u8>]>, offset: u8);
}

macro_rules! export_macro {
    ($($i:ident),*) => ( paste! { $(
        #[wasm_bindgen]
        pub fn [<export_ $i>](a: &[$i]) -> Vec<$i> {
            assert_eq!(a.len(), 2);
            assert_eq!(a[0], 1 as $i);
            assert_eq!(a[1], 2 as $i);
            a.to_vec()
        }

        #[wasm_bindgen]
        pub fn [<export_optional_ $i>](a: Option<Vec<$i>>) -> Option<Vec<$i>> {
            a.map(|a| {
                assert_eq!(a.len(), 2);
                assert_eq!(a[0], 1 as $i);
                assert_eq!(a[1], 2 as $i);
                a.to_vec()
            })

        }

        #[wasm_bindgen]
        pub fn [<export_uninit_ $i>](a: &[MaybeUninit<$i>]) -> Vec<MaybeUninit<$i>> {
            assert_eq!(a.len(), 2);
            let slice = slice_ref(a);
            assert_eq!(slice[0], 1 as $i);
            assert_eq!(slice[1], 2 as $i);
            a.to_vec()
        }

        #[wasm_bindgen]
        pub fn [<export_optional_uninit_ $i>](a: Option<Vec<MaybeUninit<$i>>>) -> Option<Vec<MaybeUninit<$i>>> {
            a.map(|a| {
                assert_eq!(a.len(), 2);
                let slice = slice_ref(&a);
                assert_eq!(slice[0], 1 as $i);
                assert_eq!(slice[1], 2 as $i);
                a.to_vec()
            })

        }
    )* } );
}

export_macro!(i8, u8, i16, u16, i32, u32, isize, usize, f32, f64);

#[wasm_bindgen_test]
fn export() {
    js_export();
}

macro_rules! import_macro {
    ($($i:ident),*) => ( paste! { $(
        #[wasm_bindgen(module = "tests/wasm/slice.js")]
        extern "C" {
            fn [<import_js_ $i>](a: &[$i], b: Option<&[$i]>, c: Option<&[$i]>) -> Vec<$i>;
        }

        #[wasm_bindgen]
        pub fn [<import_rust_ $i>](a: &[$i]) -> Vec<$i> {
            assert_eq!(a.len(), 2);
            assert_eq!(a[0], 1 as $i);
            assert_eq!(a[1], 2 as $i);
            [<import_js_ $i>](a, Some(a), None)
        }

        #[wasm_bindgen(module = "tests/wasm/slice.js")]
        extern "C" {
            fn [<import_js_uninit_ $i>](a: &[MaybeUninit<$i>], b: Option<&[MaybeUninit<$i>]>, c: Option<&[MaybeUninit<$i>]>) -> Vec<MaybeUninit<$i>>;
        }

        #[wasm_bindgen]
        pub fn [<import_rust_uninit_ $i>](a: &[MaybeUninit<$i>]) -> Vec<MaybeUninit<$i>> {
            assert_eq!(a.len(), 2);
            let slice = slice_ref(a);
            assert_eq!(slice[0], 1 as $i);
            assert_eq!(slice[1], 2 as $i);
            [<import_js_uninit_ $i>](a, Some(a), None)
        }
    )* } )
}

import_macro!(i8, u8, i16, u16, i32, u32, isize, usize, f32, f64);

#[wasm_bindgen_test]
fn import() {
    js_import();
}

macro_rules! pass_array_marco {
    ($($i:ident),*) => ( paste! { $(
        #[wasm_bindgen]
        pub fn [<pass_array_rust_ $i>](a: &[$i]) {
            assert_eq!(a.len(), 2);
            assert_eq!(a[0], 1 as $i);
            assert_eq!(a[1], 2 as $i);
        }

        #[wasm_bindgen]
        pub fn [<pass_array_rust_uninit_ $i>](a: &[MaybeUninit<$i>]) {
            assert_eq!(a.len(), 2);
            let a = slice_ref(a);
            assert_eq!(a[0], 1 as $i);
            assert_eq!(a[1], 2 as $i);
        }
    )* } )
}

pass_array_marco!(i8, u8, i16, u16, i32, u32, isize, usize, f32, f64);

#[wasm_bindgen_test]
fn pass_array() {
    js_pass_array();
}

macro_rules! import_mut_macro {
    ($($i:ident),*) => ( paste! {
        $(
            #[wasm_bindgen(module = "tests/wasm/slice.js")]
            extern "C" {
                fn [<import_mut_js_ $i>](a: &mut [$i], b: Option<&mut [$i]>, c: Option<&mut [$i]>);
                fn [<import_mut_js_uninit_ $i>](a: &mut [MaybeUninit<$i>], b: Option<&mut [MaybeUninit<$i>]>, c: Option<&mut [MaybeUninit<$i>]>);
            }

            fn [<import_mut_rust_ $i>]() {
                let mut buf1 = [
                    1 as $i,
                    2 as $i,
                    3 as $i,
                ];
                let mut buf2 = [
                    4 as $i,
                    5 as $i,
                    6 as $i,
                ];
                [<import_mut_js_ $i>](&mut buf1, Some(&mut buf2), None);
                assert_eq!(buf1[0], 4 as $i);
                assert_eq!(buf1[1], 5 as $i);
                assert_eq!(buf1[2], 3 as $i);
                assert_eq!(buf2[0], 8 as $i);
                assert_eq!(buf2[1], 7 as $i);
                assert_eq!(buf2[2], 6 as $i);
            }

            fn [<import_mut_rust_uninit_ $i>]() {
                let mut buf1 = [
                    1 as $i,
                    2 as $i,
                    3 as $i,
                ];
                let mut buf2 = [
                    4 as $i,
                    5 as $i,
                    6 as $i,
                ];
                [<import_mut_js_uninit_ $i>](slice_uninit_mut(&mut buf1), Some(slice_uninit_mut(&mut buf2)), None);
                assert_eq!(buf1[0], 4 as $i);
                assert_eq!(buf1[1], 5 as $i);
                assert_eq!(buf1[2], 3 as $i);
                assert_eq!(buf2[0], 8 as $i);
                assert_eq!(buf2[1], 7 as $i);
                assert_eq!(buf2[2], 6 as $i);
            }
        )*

        #[wasm_bindgen_test]
        fn import_mut() {
            $([<import_mut_rust_ $i>]();)*
            $([<import_mut_rust_uninit_ $i>]();)*
        }
    } )
}

import_mut_macro!(i8, u8, i16, u16, i32, u32, f32, f64);

macro_rules! export_mut_macro {
    ($($i:ident),*) => ( paste! { $(
        #[wasm_bindgen]
        pub fn [<export_mut_ $i>](a: &mut [$i])  {
            assert_eq!(a.len(), 3);
            assert_eq!(a[0], 1 as $i);
            assert_eq!(a[1], 2 as $i);
            assert_eq!(a[2], 3 as $i);
            a[0] = 4 as $i;
            a[1] = 5 as $i;
        }

        #[wasm_bindgen]
        pub fn [<export_mut_uninit_ $i>](a: &mut [MaybeUninit<$i>])  {
            assert_eq!(a.len(), 3);
            let a = slice_mut(a);
            assert_eq!(a[0], 1 as $i);
            assert_eq!(a[1], 2 as $i);
            assert_eq!(a[2], 3 as $i);
            a[0] = 4 as $i;
            a[1] = 5 as $i;
        }
    )* } )
}

export_mut_macro!(i8, u8, i16, u16, i32, u32, isize, usize, f32, f64);

#[wasm_bindgen_test]
fn export_mut() {
    js_export_mut();
}

#[wasm_bindgen]
pub fn return_vec_broken_vec() -> Vec<u32> {
    vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
}

#[wasm_bindgen]
pub fn return_vec_web_main() -> ReturnVecApplication {
    ReturnVecApplication::new()
}

#[wasm_bindgen]
pub struct ReturnVecApplication {
    thing: Vec<u32>,
}

#[wasm_bindgen]
impl ReturnVecApplication {
    #[allow(clippy::vec_init_then_push)]
    pub fn new() -> ReturnVecApplication {
        let mut thing = vec![];
        thing.push(0);
        thing.push(0);
        thing.push(0);
        thing.push(0);
        thing.push(0);

        ReturnVecApplication { thing }
    }

    #[allow(clippy::assigning_clones)] // false positive, should be fixed by https://github.com/rust-lang/rust-clippy/pull/12756
    pub fn tick(&mut self) {
        self.thing = self.thing.clone();
    }
}

#[wasm_bindgen_test]
fn return_vec() {
    js_return_vec();
}

#[wasm_bindgen_test]
fn take_clamped() {
    js_clamped(Clamped(&[1, 2, 3]), 1);
    js_clamped2(Clamped(vec![4, 5, 6]), 4);
    js_clamped3(Clamped(&mut [7, 8, 9]), 7);

    js_clamped_uninit(Clamped(slice_uninit_ref(&[1, 2, 3])), 1);
    js_clamped2_uninit(Clamped(slice_uninit_ref(&[4, 5, 6]).to_vec()), 4);
    js_clamped3_uninit(Clamped(slice_uninit_mut(&mut [7, 8, 9])), 7);
}

fn slice_ref<T>(slice: &[MaybeUninit<T>]) -> &[T] {
    unsafe { &*(std::ptr::from_ref(slice) as *const [T]) }
}

fn slice_mut<T>(slice: &mut [MaybeUninit<T>]) -> &mut [T] {
    unsafe { &mut *(std::ptr::from_mut(slice) as *mut [T]) }
}

fn slice_uninit_ref<T>(slice: &[T]) -> &[MaybeUninit<T>] {
    unsafe { &*(std::ptr::from_ref(slice) as *const [MaybeUninit<T>]) }
}

fn slice_uninit_mut<T>(slice: &mut [T]) -> &mut [MaybeUninit<T>] {
    unsafe { &mut *(std::ptr::from_mut(slice) as *mut [MaybeUninit<T>]) }
}
