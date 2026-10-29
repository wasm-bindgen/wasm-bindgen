// An owned `ScopedClosure` moves to JS, so a generic import must not erase its
// borrow lifetime to `'static` the way it does for a call-scoped `&T` argument.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn take_owned<'a>(cb: ScopedClosure<'a, dyn FnMut() + 'a>);
}

fn launder() {
    let mut n = 0u32;
    let mut f = || n += 1;
    take_owned(ScopedClosure::borrow_mut(&mut f));
}

fn main() {}
