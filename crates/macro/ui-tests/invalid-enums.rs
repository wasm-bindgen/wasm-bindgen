use wasm_bindgen::prelude::*;

#[wasm_bindgen]
enum A {}

#[wasm_bindgen]
pub enum B {
    D(u32),
}

#[wasm_bindgen]
pub enum C {
    X = 1 + 3,
}

#[wasm_bindgen]
pub enum D {
    X = 4294967296,
}

#[wasm_bindgen]
pub enum E {
    A = 1,
    B = "foo",
}

#[wasm_bindgen]
pub enum F {
    A = "foo",
    B = 1,
}

#[wasm_bindgen]
enum G {
    A = "foo",
    B = "bar",
    C,
}

#[wasm_bindgen]
pub enum H {
    A = 1,
    B = 1, // collision
}

#[wasm_bindgen]
pub enum I {
    A = 4294967294, // = u32::MAX - 1
    B,              // = u32::MAX
    C,              // = u32::MAX + 1
}

#[wasm_bindgen]
pub enum J {
    A,     // = 0
    B = 0, // collision
}

#[wasm_bindgen]
pub enum K {
    A = 3,
    B = 2,
    C, // = 3 -> collision
}

#[wasm_bindgen]
pub enum L {
    A = -2147483648, // i32::MIN
    B = -2147483649, // i32::MIN - 1
}

#[wasm_bindgen]
pub enum M {
    A = -1,
    B = 2147483648, // i32::MAX + 1
}

// Dynamic union variant with named fields is rejected.
#[wasm_bindgen]
pub enum N {
    A = "a",
    Bad { x: u32 },
}

// Dynamic union variant with multiple unnamed fields is rejected.
#[wasm_bindgen]
pub enum O {
    A = "a",
    Bad(u32, u32),
}

// Mixing a tuple variant with a unit variant that has no string discriminant
// is rejected (the unit variant cannot be classified).
#[wasm_bindgen]
pub enum P {
    Bare,
    Tuple(u32),
}

fn main() {}
