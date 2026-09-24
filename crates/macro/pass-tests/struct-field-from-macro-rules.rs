// Field getters must not trip `unsafe_op_in_unsafe_fn` when the field name
// comes from a local `macro_rules!` body (or from `paste!` inside one), where
// the field span belongs to the user's crate. Edition 2024 warns on this lint
// by default, so deny it here to catch a regression.
#![deny(unsafe_op_in_unsafe_fn)]

use wasm_bindgen::prelude::*;

macro_rules! make_struct {
    () => {
        #[wasm_bindgen]
        pub struct Foo {
            pub field: u32,
            #[wasm_bindgen(readonly)]
            pub readonly: u32,
            #[wasm_bindgen(getter_with_clone)]
            pub cloned: String,
        }
    };
}

make_struct!();

fn main() {}
