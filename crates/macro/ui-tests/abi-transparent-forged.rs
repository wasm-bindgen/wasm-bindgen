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

fn main() {}
