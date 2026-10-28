// Owned vectors whose elements own memory or JS handles. JS encodes every
// element before it allocates the buffer, and reads every element's words out
// before it decodes any, so an element's own glue may call into Wasm. Each
// wrapper crosses as the type it wraps, with its vectors encoded.

use wasm_bindgen::convert::{
    FromWasmAbi, IntoWasmAbi, VectorFromWasmAbi, VectorIntoWasmAbi, WasmSlice,
};
use wasm_bindgen::describe::{inform, WasmDescribe, WasmDescribeVector, VECTOR};
use wasm_bindgen::prelude::*;

macro_rules! encoded {
    ($($name:ident($t:ty);)*) => ($(
        pub struct $name(pub $t);

        impl WasmDescribe for $name {
            fn describe() {
                <$t>::describe()
            }
        }

        impl IntoWasmAbi for $name {
            type Abi = <$t as IntoWasmAbi>::Abi;

            fn into_abi(self) -> Self::Abi {
                self.0.into_abi()
            }
        }

        impl FromWasmAbi for $name {
            type Abi = <$t as FromWasmAbi>::Abi;

            unsafe fn from_abi(js: Self::Abi) -> Self {
                $name(<$t>::from_abi(js))
            }
        }

        impl WasmDescribeVector for $name {
            fn describe_vector() {
                inform(VECTOR);
                <$t>::describe();
            }
        }

        impl VectorIntoWasmAbi for $name {
            type Abi = WasmSlice;

            fn vector_into_abi(vector: Box<[Self]>) -> WasmSlice {
                WasmSlice::encode_owned(vector, IntoWasmAbi::into_abi)
            }
        }

        impl VectorFromWasmAbi for $name {
            type Abi = WasmSlice;

            unsafe fn vector_from_abi(js: WasmSlice) -> Box<[Self]> {
                js.try_decode().unwrap_or_else(|msg| wasm_bindgen::throw_str(msg))
            }
        }
    )*)
}

encoded! {
    Name(String);
    MaybeName(Option<String>);
    MaybeFoo(Option<Foo>);
    NullableFoo(Nullable<Foo>);
    MaybeByte(Option<u8>);
}

#[wasm_bindgen]
extern "C" {
    pub type Foo;

    fn js_names(v: Vec<Name>) -> Vec<Name>;
    fn js_maybe_foos(v: Vec<MaybeFoo>) -> Vec<MaybeFoo>;
}

#[wasm_bindgen]
pub fn names(v: Vec<Name>) -> Vec<Name> {
    v
}

#[wasm_bindgen]
pub fn maybe_names(v: Option<Vec<MaybeName>>) -> Option<Vec<MaybeName>> {
    v
}

#[wasm_bindgen]
pub fn maybe_foos(v: Vec<MaybeFoo>) -> Vec<MaybeFoo> {
    v
}

#[wasm_bindgen]
pub fn nullable_foos(v: Vec<NullableFoo>) -> Vec<NullableFoo> {
    v
}

#[wasm_bindgen]
pub fn maybe_bytes(v: Vec<MaybeByte>) -> Vec<MaybeByte> {
    v
}

#[wasm_bindgen]
pub fn driver() {
    js_names(js_names(Vec::new()));
    js_maybe_foos(js_maybe_foos(Vec::new()));
}
