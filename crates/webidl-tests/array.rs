use crate::generated::*;
use wasm_bindgen::Clamped;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn take_and_return_a_bunch_of_slices() {
    let f = TestArrays::new().unwrap();
    assert_eq!(f.strings("y"), "x");
    assert_eq!(f.byte_strings("yz"), "xx");
    assert_eq!(f.usv_strings("abc"), "efg");
    assert_eq!(f.f32_with_f32_slice(&mut [1.0, 2.0]), [3.0, 4.0, 5.0]);
    assert_eq!(f.f64_with_f64_slice(&mut [1.0, 2.0]), [3.0, 4.0, 5.0]);
    assert_eq!(f.i8_with_i8_slice(&mut [1, 2]), [3, 4, 5]);
    assert_eq!(f.i16_with_i16_slice(&mut [1, 2]), [3, 4, 5]);
    assert_eq!(f.i32_with_i32_slice(&mut [1, 2]), [3, 4, 5]);
    assert_eq!(f.u8_with_u8_slice(&mut [1, 2]), [3, 4, 5]);
    assert_eq!(f.u16_with_u16_slice(&mut [1, 2]), [3, 4, 5]);
    assert_eq!(f.u32_with_u32_slice(&mut [1, 2]), Ok(vec![3, 4, 5]));
    assert_eq!(
        f.u8_clamped_with_u8_clamped_slice(Clamped(&mut [1, 2])).0,
        [3, 4, 5]
    );
    // Arrays now have generic types, but u8 doesn't implement ErasableGeneric
    // so we need to work with them as JsValue arrays
    use wasm_bindgen::JsCast;
    assert_eq!(
        f.octet_array()
            .unchecked_ref::<js_sys::Array>()
            .iter()
            .map(|v| v.as_f64().unwrap())
            .collect::<Vec<_>>(),
        [3., 4., 5.]
    );
    assert_eq!(
        f.octet_sequence()
            .unchecked_ref::<js_sys::Array>()
            .iter()
            .map(|v| v.as_f64().unwrap())
            .collect::<Vec<_>>(),
        [3., 4., 5.]
    );
}
