use wasm_bindgen::prelude::*;

// upcast_into must not allow extending a ScopedClosure's lifetime.
// Without the 'a: 'b bound on the UpcastFrom impl, this would compile
// and enable use-after-free: the ScopedClosure<'static> would outlive
// the borrowed closure data.
fn extend_lifetime_via_upcast(
    short: ScopedClosure<'_, dyn FnMut()>,
) -> ScopedClosure<'static, dyn FnMut()> {
    short.upcast_into()
}

fn main() {}
