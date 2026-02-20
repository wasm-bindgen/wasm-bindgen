use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Missing + 'a on trait object - defaults to 'static
    fn bad_immediate_closure<'a>(f: ImmediateClosure<'a, dyn FnMut()>);
}

fn main() {
    let mut called = false;
    // This should fail: closure borrows `called` which is not 'static,
    // but the function signature requires 'static due to missing + 'a
    bad_immediate_closure(ImmediateClosure::new_mut(&mut || {
        called = true;
    }));
}
