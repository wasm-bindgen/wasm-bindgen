#![allow(coherence_leak_check)]

use wasm_bindgen::describe::{AbiTransparent, WasmDescribe};

// That a type's memory is what JS reads in place inside a sequence is a layout
// promise, which safe code can neither make nor take from another type.

struct Borrowed(u64);

impl WasmDescribe for Borrowed {
    const ABI_TRANSPARENT: AbiTransparent<Self> = <u8 as WasmDescribe>::ABI_TRANSPARENT;

    fn describe() {}
}

struct Claimed(u64);

impl WasmDescribe for Claimed {
    const ABI_TRANSPARENT: AbiTransparent<Self> = AbiTransparent::yes();

    fn describe() {}
}

// A subtype is another type too.
struct Held<F>(u64, Option<F>);

impl WasmDescribe for Held<fn(&'static u8)> {
    const ABI_TRANSPARENT: AbiTransparent<Self> = unsafe { AbiTransparent::yes() };

    fn describe() {}
}

impl WasmDescribe for Held<for<'a> fn(&'a u8)> {
    const ABI_TRANSPARENT: AbiTransparent<Self> =
        <Held<fn(&'static u8)> as WasmDescribe>::ABI_TRANSPARENT;

    fn describe() {}
}

fn main() {}
