use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

macro_rules! slice_args {
    ($(($slice:ident, $mut_slice:ident, $t:ty)),*) => {
        $(
            #[wasm_bindgen]
            pub fn $slice(a: &[$t]) {
                drop(a);
            }

            #[wasm_bindgen]
            pub fn $mut_slice(a: &mut [$t]) {
                drop(a);
            }
        )*
    };
}

slice_args!(
    (take_u8, take_mut_u8, u8),
    (take_i8, take_mut_i8, i8),
    (take_u16, take_mut_u16, u16),
    (take_i16, take_mut_i16, i16),
    (take_u32, take_mut_u32, u32),
    (take_i32, take_mut_i32, i32),
    (take_u64, take_mut_u64, u64),
    (take_i64, take_mut_i64, i64),
    (take_usize, take_mut_usize, usize),
    (take_isize, take_mut_isize, isize),
    (take_f32, take_mut_f32, f32),
    (take_f64, take_mut_f64, f64)
);

#[wasm_bindgen]
pub fn take_owned(a: Vec<u16>, b: Box<[i64]>) {
    drop((a, b));
}

#[wasm_bindgen]
pub fn take_optional(a: Option<Vec<f32>>, b: u32, c: Option<Box<[u64]>>) {
    drop((a, b, c));
}

#[wasm_bindgen]
pub fn take_clamped(a: Clamped<Vec<u8>>) {
    drop(a);
}

#[wasm_bindgen]
pub fn take_non_numbers(a: Vec<JsValue>, b: Vec<String>, c: Vec<Point>) {
    drop((a, b, c));
}

#[wasm_bindgen]
pub fn roundtrip(a: &[u8]) -> Vec<u8> {
    a.to_vec()
}

#[wasm_bindgen]
pub struct Point;

#[wasm_bindgen(getter_with_clone)]
pub struct Samples {
    pub values: Vec<f64>,
}

#[wasm_bindgen]
impl Samples {
    #[wasm_bindgen(constructor)]
    pub fn new(values: &[f64]) -> Samples {
        Samples {
            values: values.to_vec(),
        }
    }

    pub fn extend(&mut self, more: Vec<f64>) {
        self.values.extend(more);
    }

    pub fn copy_into(&self, out: &mut [f64]) {
        out.copy_from_slice(&self.values[..out.len()]);
    }

    pub fn sum(values: &[i32]) -> i32 {
        values.iter().sum()
    }
}
