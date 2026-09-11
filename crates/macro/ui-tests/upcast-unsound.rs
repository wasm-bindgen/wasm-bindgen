// Regression test for #5176: `UpcastFrom` must not let safe code launder a
// short-lived borrow into a `'static` one via `upcast_into`/`upcast`.
//
// The malicious impl below is still allowed to exist, but calling the upcast
// must fail to compile because the `ErasableGeneric` repr now carries the
// borrow lifetime, so the source borrow would have to be `'static`.

use wasm_bindgen::convert::{Upcast, UpcastFrom};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type Foo;
    type Bar;
}

// Launders any `'a` source borrow into a hardcoded `'static` target.
impl<'a> UpcastFrom<&'a &'a Foo> for &'static &'static Bar {}

fn launder_into(foo: &Foo) -> &'static &'static Bar {
    let r: &&Foo = &foo;
    r.upcast_into()
}

fn main() {}
