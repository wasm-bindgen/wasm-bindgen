//! PROTOTYPE: a 2-tuple crosses as a JS array `[a, b]`. In a signature it is
//! one pointer word to a heap cell, and its cell is the `#[repr(C)]` record
//! of its parts' cells, which a sequence of tuples stores inline.

use alloc::boxed::Box;

use crate::__rt::WasmWord;
use crate::convert::{FromWasmAbi, IntoWasmAbi, WasmAbi};
use crate::describe::{inform, WasmDescribe, TUPLE};

/// A pair's cell: the `#[repr(C)]` record of its parts' cells, which rustc
/// lays out as the CLI does.
#[doc(hidden)]
#[repr(C)]
pub struct PairCell<A, B>(pub(crate) A, pub(crate) B);

impl<A: WasmAbi, B: WasmAbi> WasmAbi for (A, B) {
    type Prim1 = <WasmWord as WasmAbi>::Prim1;
    type Prim2 = ();
    type Prim3 = ();
    type Prim4 = ();
    type Cell = PairCell<A::Cell, B::Cell>;

    /// Spills to a heap cell, which `join` (or the JS glue) frees.
    fn split(self) -> (Self::Prim1, (), (), ()) {
        let cell = Box::into_raw(Box::new(self.into_cell()));
        WasmWord::from_usize(cell as usize).split()
    }

    fn join(ptr: Self::Prim1, _: (), _: (), _: ()) -> Self {
        let cell = WasmWord::join(ptr, (), (), ()).into_usize() as *mut Self::Cell;
        // SAFETY: `ptr` is a cell `split` boxed, or JS allocated at the
        // cell's layout, for Rust to own.
        Self::from_cell(*unsafe { Box::from_raw(cell) })
    }

    #[inline]
    fn into_cell(self) -> Self::Cell {
        PairCell(self.0.into_cell(), self.1.into_cell())
    }

    #[inline]
    fn from_cell(cell: Self::Cell) -> Self {
        (A::from_cell(cell.0), B::from_cell(cell.1))
    }
}

impl<A: WasmDescribe, B: WasmDescribe> WasmDescribe for (A, B) {
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(TUPLE);
        inform(2);
        A::describe();
        B::describe();
    }
}

impl<A: IntoWasmAbi, B: IntoWasmAbi> IntoWasmAbi for (A, B) {
    type Abi = (A::Abi, B::Abi);

    #[inline]
    fn into_abi(self) -> Self::Abi {
        (self.0.into_abi(), self.1.into_abi())
    }
}

impl<A: FromWasmAbi, B: FromWasmAbi> FromWasmAbi for (A, B) {
    type Abi = (A::Abi, B::Abi);

    #[inline]
    unsafe fn from_abi(js: Self::Abi) -> Self {
        (A::from_abi(js.0), B::from_abi(js.1))
    }

    /// Both parts are received whether or not the first was rejected, as
    /// Rust owns them both.
    #[inline]
    unsafe fn try_from_abi(js: Self::Abi) -> Result<Self, &'static str> {
        let a = A::try_from_abi(js.0);
        let b = B::try_from_abi(js.1);
        Ok((a?, b?))
    }
}
