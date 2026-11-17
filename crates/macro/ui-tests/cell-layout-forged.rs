#![allow(coherence_leak_check)]

use wasm_bindgen::convert::{CellLayout, WasmAbi};

// How much of a sequence's buffer a type's cell takes is a layout promise,
// which safe code can neither make nor take from another type.

struct Borrowed(u64);

impl WasmAbi for Borrowed {
    type Prim1 = u64;
    type Prim2 = ();
    type Prim3 = ();
    type Prim4 = ();

    const CELL: CellLayout<Self> = <u32 as WasmAbi>::CELL;

    fn split(self) -> (u64, (), (), ()) {
        (self.0, (), (), ())
    }

    fn join(prim: u64, _: (), _: (), _: ()) -> Self {
        Borrowed(prim)
    }
}

struct Claimed(u64);

impl WasmAbi for Claimed {
    type Prim1 = u64;
    type Prim2 = ();
    type Prim3 = ();
    type Prim4 = ();

    const CELL: CellLayout<Self> = CellLayout::new(1, 1);

    fn split(self) -> (u64, (), (), ()) {
        (self.0, (), (), ())
    }

    fn join(prim: u64, _: (), _: (), _: ()) -> Self {
        Claimed(prim)
    }
}

// A subtype is another type too.
struct Held<F>(u64, Option<F>);

impl WasmAbi for Held<fn(&'static u8)> {
    type Prim1 = u64;
    type Prim2 = ();
    type Prim3 = ();
    type Prim4 = ();

    const CELL: CellLayout<Self> = unsafe { CellLayout::new(1, 1) };

    fn split(self) -> (u64, (), (), ()) {
        (self.0, (), (), ())
    }

    fn join(prim: u64, _: (), _: (), _: ()) -> Self {
        Held(prim, None)
    }
}

impl WasmAbi for Held<for<'a> fn(&'a u8)> {
    type Prim1 = u64;
    type Prim2 = ();
    type Prim3 = ();
    type Prim4 = ();

    const CELL: CellLayout<Self> = <Held<fn(&'static u8)> as WasmAbi>::CELL;

    fn split(self) -> (u64, (), (), ()) {
        (self.0, (), (), ())
    }

    fn join(prim: u64, _: (), _: (), _: ()) -> Self {
        Held(prim, None)
    }
}

fn main() {}
