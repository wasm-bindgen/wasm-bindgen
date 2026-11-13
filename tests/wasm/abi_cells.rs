//! A wide ABI's cell is a record of the cells it is made of, which a
//! sequence stores one of per element, while the value itself crosses as a
//! pointer to that record.

use std::alloc::{alloc, dealloc, Layout};

use wasm_bindgen::__rt::WasmWord;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi, WasmAbi, WasmSlice};
use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen_test::*;

#[derive(Clone, Debug, PartialEq)]
struct Named {
    name: String,
    n: u32,
}

struct NamedAbi(WasmSlice, u32);

impl NamedAbi {
    /// `{ ptr, len, n }`
    const N: usize = WasmSlice::CELL_SIZE;
    const LAYOUT: Layout = match Layout::from_size_align(Self::CELL_SIZE, Self::CELL_ALIGN) {
        Ok(layout) => layout,
        Err(_) => panic!(),
    };
}

impl WasmAbi for NamedAbi {
    type Prim1 = <WasmWord as WasmAbi>::Prim1;
    type Prim2 = ();
    type Prim3 = ();
    type Prim4 = ();

    const CELL_SIZE: usize = Self::N + u32::CELL_SIZE;
    const CELL_ALIGN: usize = WasmSlice::CELL_ALIGN;

    fn split(self) -> (Self::Prim1, (), (), ()) {
        unsafe {
            let cell = alloc(Self::LAYOUT);
            self.write_cell(cell);
            WasmWord::from_usize(cell as usize).split()
        }
    }

    fn join(ptr: Self::Prim1, _: (), _: (), _: ()) -> Self {
        let cell = WasmWord::join(ptr, (), (), ()).into_usize() as *mut u8;
        unsafe {
            let abi = Self::read_cell(cell);
            dealloc(cell, Self::LAYOUT);
            abi
        }
    }

    unsafe fn write_cell(self, cell: *mut u8) {
        self.0.write_cell(cell);
        self.1.write_cell(cell.add(Self::N));
    }

    unsafe fn read_cell(cell: *const u8) -> Self {
        NamedAbi(
            WasmSlice::read_cell(cell),
            u32::read_cell(cell.add(Self::N)),
        )
    }
}

/// An ABI written against `WasmAbi` before it had a cell, whose cell is the
/// record of its primitives.
struct Flat(u32, f64);

impl WasmAbi for Flat {
    type Prim1 = u32;
    type Prim2 = f64;
    type Prim3 = ();
    type Prim4 = ();

    fn split(self) -> (u32, f64, (), ()) {
        (self.0, self.1, (), ())
    }

    fn join(a: u32, b: f64, _: (), _: ()) -> Self {
        Flat(a, b)
    }
}

struct Sample(u32, f64);

impl WasmDescribe for Sample {
    fn describe() {}
}

impl IntoWasmAbi for Sample {
    type Abi = Flat;

    fn into_abi(self) -> Flat {
        Flat(self.0, self.1)
    }
}

impl FromWasmAbi for Sample {
    type Abi = Flat;

    unsafe fn from_abi(js: Flat) -> Self {
        Sample(js.0, js.1)
    }
}

impl WasmDescribe for Named {
    fn describe() {}
}

impl IntoWasmAbi for Named {
    type Abi = NamedAbi;

    fn into_abi(self) -> NamedAbi {
        NamedAbi(self.name.into_abi(), self.n)
    }
}

impl FromWasmAbi for Named {
    type Abi = NamedAbi;

    unsafe fn from_abi(js: NamedAbi) -> Self {
        Named {
            name: String::from_abi(js.0),
            n: js.1,
        }
    }
}

fn items() -> Vec<Named> {
    vec![
        Named {
            name: "a".into(),
            n: 1,
        },
        Named {
            name: String::new(),
            n: 0,
        },
        Named {
            name: "ü🌍".into(),
            n: u32::MAX,
        },
    ]
}

#[wasm_bindgen_test]
fn wide_cell_layout() {
    assert_eq!((NamedAbi::CELL_SIZE, NamedAbi::CELL_ALIGN), (12, 4));
    assert_eq!(NamedAbi::N, 8);
}

#[wasm_bindgen_test]
fn default_cells_round_trip_as_a_sequence() {
    assert_eq!((Flat::CELL_SIZE, Flat::CELL_ALIGN), (16, 8));
    let slice = WasmSlice::encode_owned(
        vec![Sample(1, 0.5), Sample(2, 1.5)].into_boxed_slice(),
        Sample::into_abi,
    );
    let decoded = unsafe { slice.try_decode::<Sample>() }.unwrap();
    let decoded: Vec<_> = decoded.iter().map(|s| (s.0, s.1)).collect();
    assert_eq!(decoded, [(1, 0.5), (2, 1.5)]);
}

#[wasm_bindgen_test]
fn wide_cells_round_trip_as_a_sequence() {
    let items = items();
    let slice = WasmSlice::encode(&items, |item| item.clone().into_abi());
    assert_eq!(slice.len.into_usize(), 3);

    // The buffer is the cells in order, 12 bytes each, with `n` third.
    let cells = slice.ptr.into_usize() as *const u32;
    assert_eq!(unsafe { *cells.add(2) }, 1);
    assert_eq!(unsafe { *cells.add(3 + 1) }, 0);
    assert_eq!(unsafe { *cells.add(6 + 2) }, u32::MAX);

    let decoded = unsafe { slice.try_decode::<Named>() }.unwrap();
    assert_eq!(&*decoded, &*items);
}

#[wasm_bindgen_test]
fn wide_value_spills_to_its_cell() {
    for item in items() {
        let (ptr, (), (), ()) = item.clone().into_abi().split();
        let back = unsafe { Named::from_abi(NamedAbi::join(ptr, (), (), ())) };
        assert_eq!(back, item);
    }
}
