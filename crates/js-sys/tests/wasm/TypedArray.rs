use std::mem::MaybeUninit;

use half::f16;
use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

fn has_float16_array() -> bool {
    Reflect::has_str(global().as_ref(), &"Float16Array".into()).unwrap_or(false)
}

fn has_base64_methods() -> bool {
    Reflect::has_str(Uint8Array::new_with_length(0).as_ref(), &"toBase64".into()).unwrap_or(false)
}

macro_rules! each {
    ($m:ident) => {
        $m!(Uint8Array);
        $m!(Uint8ClampedArray);
        $m!(Uint16Array);
        $m!(Uint32Array);
        $m!(Int8Array);
        $m!(Int16Array);
        $m!(Int32Array);
        $m!(Float32Array);
        $m!(Float64Array);
    };
}

macro_rules! test_inheritance {
    ($arr:ident) => {{
        let arr = $arr::new(&JsValue::undefined());
        assert!(arr.is_instance_of::<$arr>());
        let _: &Object = arr.as_ref();
        assert!(arr.is_instance_of::<Object>());
    }};
}
#[wasm_bindgen_test]
fn inheritance() {
    each!(test_inheritance);
}

macro_rules! test_undefined {
    ($arr:ident) => {{
        let arr = $arr::new(&JsValue::undefined());
        assert_eq!(arr.length(), 0);
        assert_eq!(arr.byte_length(), 0);
        assert_eq!(arr.byte_offset(), 0);
        assert!(JsValue::from(arr.buffer()).is_object());
    }};
}
#[wasm_bindgen_test]
fn new_undefined() {
    each!(test_undefined);
}

macro_rules! test_length {
    ($arr:ident) => {{
        let arr = $arr::new(&4.into());
        assert_eq!(arr.length(), 4);
        assert!(arr.byte_length() != 0);
        assert_eq!(arr.byte_offset(), 0);
        assert!(JsValue::from(arr.buffer()).is_object());
    }};
}
#[wasm_bindgen_test]
fn new_length() {
    each!(test_length);
}

macro_rules! test_subarray {
    ($arr:ident) => {{
        assert_eq!($arr::new(&4.into()).subarray(0, 1).length(), 1);
    }};
}
#[wasm_bindgen_test]
fn new_subarray() {
    each!(test_subarray);
}

macro_rules! test_fill {
    ($arr:ident) => {{
        let arr = $arr::new(&4.into());
        arr.for_each(&mut |x, _, _| {
            assert_eq!(x as f64, 0.0);
        });
        arr.fill(2 as _, 0, 2);
        arr.for_each(&mut |x, i, _| {
            if i < 2 {
                assert_eq!(x as f64, 2.0);
            } else {
                assert_eq!(x as f64, 0.0);
            }
        });
    }};
}
#[wasm_bindgen_test]
fn new_fill() {
    each!(test_fill);
}

macro_rules! test_at {
    ($arr:ident) => {{
        let arr = $arr::new(&2.into());
        arr.set_index(1, 1 as _);
        assert_eq!(arr.at(-1).unwrap() as f64, 1 as f64);
    }};
}
#[wasm_bindgen_test]
fn new_at() {
    each!(test_at);
}

macro_rules! test_copy_within {
    ($arr:ident) => {{
        let x: Vec<_> = vec![8, 5, 4, 3, 1, 2];
        let array = $arr::from(x.into_iter().map(|v| v as _).collect::<Vec<_>>().as_slice());
        array.copy_within(1, 4, 5);

        assert_eq!(array.get_index(1) as f64, 1f64);

        // if negatives were used
        array.copy_within(-1, -3, -2);
        assert_eq!(array.get_index(5) as f64, 3f64);
    }};
}
#[wasm_bindgen_test]
fn new_copy_within() {
    each!(test_copy_within);
}

macro_rules! test_get_set {
    ($arr:ident) => {{
        let arr = $arr::new(&1.into());
        assert_eq!(arr.get_index(0) as f64, 0 as f64);
        arr.set_index(0, 1 as _);
        assert_eq!(arr.get_index(0) as f64, 1 as f64);
    }};
}
#[wasm_bindgen_test]
fn new_get_set() {
    each!(test_get_set);
}

macro_rules! test_slice {
    ($arr:ident) => {{
        let arr = $arr::new(&4.into());
        assert_eq!(arr.length(), 4);
        assert_eq!(arr.slice(1, 2).length(), 1);
    }};
}
#[wasm_bindgen_test]
fn new_slice() {
    each!(test_slice);
}

macro_rules! test_to_reversed_sorted {
    ($arr:ident) => {{
        let arr = $arr::new(&3.into());
        arr.set_index(0, 3 as _);
        arr.set_index(1, 1 as _);
        arr.set_index(2, 2 as _);
        let reversed = arr.to_reversed();
        assert_eq!(reversed.get_index(0) as f64, 2.0);
        assert_eq!(reversed.get_index(1) as f64, 1.0);
        assert_eq!(reversed.get_index(2) as f64, 3.0);
        let sorted = arr.to_sorted();
        assert_eq!(sorted.get_index(0) as f64, 1.0);
        assert_eq!(sorted.get_index(1) as f64, 2.0);
        assert_eq!(sorted.get_index(2) as f64, 3.0);
        assert_eq!(arr.get_index(0) as f64, 3.0);
        assert_eq!(arr.get_index(2) as f64, 2.0);
    }};
}
#[wasm_bindgen_test]
fn new_to_reversed_sorted() {
    each!(test_to_reversed_sorted);
}

macro_rules! test_with {
    ($arr:ident) => {{
        let arr = $arr::new(&2.into());
        let replaced = arr.with(1, 7 as _).unwrap();
        assert_eq!(replaced.get_index(1) as f64, 7.0);
        assert_eq!(arr.get_index(1) as f64, 0.0);
        assert_eq!(arr.with(-1, 5 as _).unwrap().get_index(1) as f64, 5.0);
        assert!(arr
            .with(2, 7 as _)
            .unwrap_err()
            .is_instance_of::<RangeError>());
    }};
}
#[wasm_bindgen_test]
fn new_with() {
    each!(test_with);
}

macro_rules! test_find_last {
    ($arr:ident) => {{
        let arr = $arr::new(&4.into());
        arr.set_index(1, 5 as _);
        arr.set_index(3, 5 as _);
        assert_eq!(
            arr.find_last(&mut |x, _, _| x as f64 == 5.0).unwrap() as f64,
            5.0
        );
        assert!(arr.find_last(&mut |x, _, _| x as f64 == 9.0).is_none());
        assert_eq!(arr.find_last_index(&mut |x, _, _| x as f64 == 5.0), 3);
        assert_eq!(arr.find_last_index(&mut |x, _, _| x as f64 == 9.0), -1);
        assert_eq!(
            arr.find_last_index(&mut |_, i, a| i < 2 && a.length() == 4),
            1
        );
    }};
}
#[wasm_bindgen_test]
fn new_find_last() {
    each!(test_find_last);
}

macro_rules! test_includes_index_of {
    ($arr:ident) => {{
        let arr = $arr::new(&4.into());
        arr.set_index(1, 5 as _);
        arr.set_index(3, 5 as _);
        assert!(arr.includes(5 as _, 0));
        assert!(arr.includes(5 as _, 2));
        assert!(!arr.includes(5 as _, 4));
        assert!(!arr.includes(9 as _, 0));
        assert_eq!(arr.index_of(5 as _, 0), 1);
        assert_eq!(arr.index_of(5 as _, 2), 3);
        assert_eq!(arr.index_of(5 as _, 4), -1);
        assert_eq!(arr.index_of(9 as _, 0), -1);
    }};
}
#[wasm_bindgen_test]
fn new_includes_index_of() {
    each!(test_includes_index_of);
}

#[wasm_bindgen_test]
fn bigint64array_es2023_methods() {
    let arr = BigInt64Array::new_from_slice(&[3, -1, 2]);
    assert_eq!(arr.to_sorted().to_vec(), vec![-1, 2, 3]);
    assert_eq!(arr.to_reversed().to_vec(), vec![2, -1, 3]);
    assert_eq!(arr.to_vec(), vec![3, -1, 2]);
    assert_eq!(arr.with(0, i64::MIN).unwrap().get_index(0), i64::MIN);
    assert!(arr.with(3, 0).is_err());
    assert_eq!(arr.find_last(&mut |x, _, _| x < 0), Some(-1));
    assert_eq!(arr.find_last(&mut |x, _, _| x > 10), None);
    assert_eq!(arr.find_last_index(&mut |x, _, _| x > 0), 2);
    assert!(arr.includes(-1, 0));
    assert!(!arr.includes(3, 1));
    assert_eq!(arr.index_of(2, 0), 2);
    assert_eq!(arr.index_of(3, 1), -1);

    let arr = BigUint64Array::new_from_slice(&[1, u64::MAX]);
    assert!(arr.includes(u64::MAX, 0));
    assert_eq!(arr.index_of(u64::MAX, 0), 1);
    assert_eq!(arr.find_last(&mut |x, _, _| x > 1), Some(u64::MAX));
}

#[wasm_bindgen_test]
fn float16array_inheritance() {
    if !has_float16_array() {
        return;
    }

    let arr = Float16Array::new(&JsValue::undefined());
    assert!(arr.is_instance_of::<Float16Array>());
    let _: &Object = arr.as_ref();
    assert!(arr.is_instance_of::<Object>());
}

#[wasm_bindgen_test]
fn float16array_basic_methods() {
    if !has_float16_array() {
        return;
    }

    let arr = Float16Array::new(&4.into());
    assert_eq!(arr.length(), 4);
    assert_eq!(arr.byte_length(), 8);
    assert_eq!(arr.byte_offset(), 0);
    assert!(JsValue::from(arr.buffer()).is_object());

    arr.fill_with_f32(1.0, 0, 2);
    assert_eq!(arr.get_index_as_f32(0), 1.0);
    assert_eq!(arr.get_index_as_f32(1), 1.0);
    assert_eq!(arr.get_index_as_f32(2), 0.0);

    arr.set_index_from_f32(2, -2.0);
    assert_eq!(arr.at_as_f32(-2).unwrap(), -2.0);

    arr.copy_within(3, 0, 1);
    assert_eq!(arr.get_index_as_f32(3), 1.0);

    assert_eq!(arr.subarray(1, 3).length(), 2);
    assert_eq!(arr.slice(1, 3).length(), 2);

    let mut seen = Vec::new();
    arr.for_each_as_f32(&mut |value, _, _| seen.push(value));
    assert_eq!(seen, vec![1.0, 1.0, -2.0, 1.0]);
}

#[wasm_bindgen_test]
fn float16array_u16_helpers() {
    if !has_float16_array() {
        return;
    }

    let initial = [0x3c00, 0xc000, 0x3555];
    let arr = Float16Array::new_from_u16_slice(&initial);
    assert_eq!(arr.get_index_as_f32(0), 1.0);
    assert_eq!(arr.get_index_as_f32(1), -2.0);

    let mut copied = [0; 3];
    arr.copy_to_u16_slice(&mut copied);
    assert_eq!(copied, initial);
    assert_eq!(arr.to_u16_vec(), initial);

    let replacement = [0x0001, 0x7bff, 0xfc00];
    arr.copy_from_u16_slice(&replacement);
    assert_eq!(arr.to_u16_vec(), replacement);
}

#[wasm_bindgen_test]
fn float16array_half_helpers() {
    if !has_float16_array() {
        return;
    }

    let initial_half = [f16::from_f32(1.0), f16::from_f32(-2.0), f16::from_f32(0.5)];
    let initial_bits = initial_half.map(f16::to_bits);
    let arr = Float16Array::new_from_u16_slice(&initial_bits);

    let mut copied_bits = [0; 3];
    arr.copy_to_u16_slice(&mut copied_bits);
    assert_eq!(copied_bits.map(f16::from_bits), initial_half);

    let copied_half: Vec<f16> = arr.to_u16_vec().into_iter().map(f16::from_bits).collect();
    assert_eq!(copied_half.as_slice(), &initial_half);

    let replacement_half = [f16::from_f32(0.25), f16::from_f32(6.5), f16::from_f32(-4.0)];
    let replacement_bits = replacement_half.map(f16::to_bits);
    arr.copy_from_u16_slice(&replacement_bits);

    let replacement_round_trip: Vec<f16> =
        arr.to_u16_vec().into_iter().map(f16::from_bits).collect();
    assert_eq!(replacement_round_trip.as_slice(), &replacement_half);
}

#[wasm_bindgen_test]
fn view() {
    let x = [1, 2, 3];
    let array = unsafe { Int32Array::view(&x) };
    assert_eq!(array.length(), 3);
    array.for_each(&mut |x, i, _| {
        assert_eq!(x, (i + 1) as i32);
    });
}

#[wasm_bindgen_test]
fn from() {
    let x: Vec<i32> = vec![1, 2, 3];
    let array = Int32Array::from(x.as_slice());
    assert_eq!(array.length(), 3);
    array.for_each(&mut |x, i, _| {
        assert_eq!(x, (i + 1) as i32);
    });
}

#[wasm_bindgen_test]
fn copy_to() {
    let mut x = [0; 10];
    let array = Int32Array::new(&10.into());
    array.fill(5, 0, 10);
    array.copy_to(&mut x);
    for i in x.iter() {
        assert_eq!(*i, 5);
    }
}

#[wasm_bindgen_test]
fn copy_to_uninit() {
    let mut x = [MaybeUninit::uninit(); 10];
    let array = Int32Array::new(&10.into());
    array.fill(5, 0, 10);
    let x = array.copy_to_uninit(&mut x);
    for i in x.iter() {
        assert_eq!(*i, 5);
    }
}

#[wasm_bindgen_test]
fn copy_from() {
    let x = [1, 2, 3];
    let array = Int32Array::new(&3.into());
    array.copy_from(&x);
    array.for_each(&mut |x, i, _| {
        assert_eq!(x, (i + 1) as i32);
    });
}

#[wasm_bindgen_test]
fn to_vec() {
    let array = Int32Array::new(&10.into());
    array.fill(5, 0, 10);
    assert_eq!(array.to_vec(), vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
}

#[wasm_bindgen_test]
fn from_slice_heap_growth() {
    let slice = std::slice::from_ref(&1);

    _ = (0..10_000)
        .map(|_i| Int32Array::from(slice))
        .collect::<Vec<_>>();
}

#[wasm_bindgen_test]
fn copy_to_heap_growth() {
    let mut v = vec![];
    for _ in 0..10_000 {
        let x = Uint8Array::new_with_length(10);
        let mut y = [0; 10];
        // When the externref table capacity is insufficient,
        // it will be allocated and the array buffer will be detached.
        x.copy_to(&mut y);
        // Simulate the operation of allocating multiple JS objects in a function
        v.push(x);
    }
}

#[wasm_bindgen_test]
fn copy_from_heap_growth() {
    let mut v = Vec::with_capacity(10_000);
    for _ in 0..10_000 {
        let x = Uint8Array::new_with_length(10);
        x.copy_from(&[1; 10]);
        v.push(x);
    }
}

#[wasm_bindgen_test]
fn raw_copy_to_ptr_heap_growth() {
    let mut v = Vec::with_capacity(10_000);
    for _ in 0..10_000 {
        let x = Uint8Array::new_with_length(10);
        let mut y = [0; 10];
        // When the externref table capacity is insufficient,
        // it will be allocated and the array buffer will be detached.
        unsafe {
            x.raw_copy_to_ptr(y.as_mut_ptr());
        }
        // Simulate the operation of allocating multiple JS objects in a function
        v.push(x);
    }
}

#[wasm_bindgen_test]
fn to_vec_heap_growth() {
    let mut v = Vec::with_capacity(10_000);
    for _ in 0..10_000 {
        let x = Uint8Array::new_with_length(10);
        // When the externref table capacity is insufficient,
        // it will be allocated and the array buffer will be detached.
        x.to_vec();
        // Simulate the operation of allocating multiple JS objects in a function
        v.push(x);
    }
}

macro_rules! gen_integer_tests {
    ($(($name:ident, $js:ident, $rust:ident),)*) => ($(
        #[wasm_bindgen_test]
        fn $name() {
            let buf1 = vec![1, 2, 3, 4, $rust::MIN, $rust::MAX];
            let array = $js::new_from_slice(&buf1);
            let buf2 = array.to_vec();
            assert_eq!(buf1, buf2);
            let mut buf3 = vec![0; 2];
            array.subarray(0, 2).copy_to(&mut buf3);
            assert_eq!(buf3, vec![1, 2]);
            let buf4 = $js::new_with_length(3);
            buf4.subarray(1, 3).copy_from(&buf3);
            assert!(buf4.get_index(0) == 0);
            assert!(buf4.get_index(1) == 1);
            assert!(buf4.get_index(2) == 2);
        }
    )*);
}

macro_rules! gen_float_tests {
    ($(($name:ident, $js:ident, $rust:ident),)*) => ($(
        #[wasm_bindgen_test]
        fn $name() {
            let buf1 = vec![1.0, 2.0, 3.0, 4.0, $rust::MIN, $rust::MAX];
            let array = $js::new_from_slice(&buf1);
            let buf2 = array.to_vec();
            assert_eq!(buf1, buf2);
            let mut buf3 = vec![0.0; 2];
            array.subarray(0, 2).copy_to(&mut buf3);
            assert_eq!(buf3, vec![1.0, 2.0]);
            let buf4 = $js::new_with_length(3);
            buf4.subarray(1, 3).copy_from(&buf3);
            assert!(buf4.get_index(0) == 0.0);
            assert!(buf4.get_index(1) == 1.0);
            assert!(buf4.get_index(2) == 2.0);
        }
    )*);
}

gen_integer_tests! {
    (test_i8_copy, Int8Array, i8),
    (test_i16_copy, Int16Array, i16),
    (test_i32_copy, Int32Array, i32),
    (test_u8_copy, Uint8Array, u8),
    (test_u8c_copy, Uint8ClampedArray, u8),
    (test_u16_copy, Uint16Array, u16),
    (test_u32_copy, Uint32Array, u32),
    (test_i64_copy, BigInt64Array, i64),
    (test_u64_copy, BigUint64Array, u64),
}

gen_float_tests! {
    (test_f32_copy, Float32Array, f32),
    (test_f64_copy, Float64Array, f64),
}

#[wasm_bindgen_test]
fn uint8array_base64_hex_round_trip() {
    if !has_base64_methods() {
        return;
    }

    let bytes = vec![0xfb, 0xff, 0xbf, 0x01];
    let arr = Uint8Array::new_from_slice(&bytes);
    assert_eq!(arr.to_base64(), "+/+/AQ==");
    assert_eq!(arr.to_hex(), "fbffbf01");
    assert_eq!(Uint8Array::new_with_length(0).to_base64(), "");
    assert_eq!(Uint8Array::new_with_length(0).to_hex(), "");

    assert_eq!(Uint8Array::from_base64("+/+/AQ==").unwrap().to_vec(), bytes);
    assert_eq!(Uint8Array::from_hex("fbffbf01").unwrap().to_vec(), bytes);
    assert_eq!(Uint8Array::from_hex("FBFFBF01").unwrap().to_vec(), bytes);
    assert_eq!(Uint8Array::from_base64("").unwrap().length(), 0);

    let all: Vec<u8> = (0..=255).collect();
    let all_arr = Uint8Array::new_from_slice(&all);
    assert_eq!(
        Uint8Array::from_base64(&String::from(all_arr.to_base64()))
            .unwrap()
            .to_vec(),
        all
    );
    assert_eq!(
        Uint8Array::from_hex(&String::from(all_arr.to_hex()))
            .unwrap()
            .to_vec(),
        all
    );
}

#[wasm_bindgen_test]
fn uint8array_base64_options() {
    if !has_base64_methods() {
        return;
    }

    let options = Base64Options::new();
    assert!(options.get_alphabet().is_none());
    assert!(options.get_last_chunk_handling().is_none());
    assert!(options.get_omit_padding().is_none());

    options.set_alphabet(Base64Alphabet::Base64Url);
    options.set_last_chunk_handling(Base64LastChunkHandling::Strict);
    options.set_omit_padding(true);
    assert_eq!(options.get_alphabet(), Some(Base64Alphabet::Base64Url));
    assert_eq!(
        options.get_last_chunk_handling(),
        Some(Base64LastChunkHandling::Strict)
    );
    assert_eq!(options.get_omit_padding(), Some(true));

    let bytes = vec![0xfb, 0xff, 0xbf, 0x01];
    let arr = Uint8Array::new_from_slice(&bytes);
    assert_eq!(arr.to_base64_with_options(&options), "-_-_AQ");
    options.set_omit_padding(false);
    assert_eq!(arr.to_base64_with_options(&options), "-_-_AQ==");

    assert!(Uint8Array::from_base64_with_options("-_-_AQ", &options)
        .unwrap_err()
        .is_instance_of::<SyntaxError>());
    assert_eq!(
        Uint8Array::from_base64_with_options("-_-_AQ==", &options)
            .unwrap()
            .to_vec(),
        bytes
    );
    options.set_last_chunk_handling(Base64LastChunkHandling::Loose);
    assert_eq!(
        Uint8Array::from_base64_with_options("-_-_AQ", &options)
            .unwrap()
            .to_vec(),
        bytes
    );
    assert!(Uint8Array::from_base64_with_options("+/+/AQ==", &options).is_err());
}

#[wasm_bindgen_test]
fn uint8array_base64_hex_invalid() {
    if !has_base64_methods() {
        return;
    }

    assert!(Uint8Array::from_base64("!!!!")
        .unwrap_err()
        .is_instance_of::<SyntaxError>());
    assert!(Uint8Array::from_base64("A")
        .unwrap_err()
        .is_instance_of::<SyntaxError>());
    assert!(Uint8Array::from_hex("abc")
        .unwrap_err()
        .is_instance_of::<SyntaxError>());
    assert!(Uint8Array::from_hex("zz")
        .unwrap_err()
        .is_instance_of::<SyntaxError>());
    assert!(Uint8Array::new_with_length(4)
        .set_from_base64("!!!!")
        .unwrap_err()
        .is_instance_of::<SyntaxError>());
    assert!(Uint8Array::new_with_length(4)
        .set_from_hex("zz")
        .unwrap_err()
        .is_instance_of::<SyntaxError>());
}

#[wasm_bindgen_test]
fn uint8array_set_from_base64_hex() {
    if !has_base64_methods() {
        return;
    }

    let target = Uint8Array::new_with_length(4);
    let result = target.set_from_base64("+/+/AQ==").unwrap();
    assert_eq!(result.read(), 8);
    assert_eq!(result.written(), 4);
    assert_eq!(target.to_vec(), vec![0xfb, 0xff, 0xbf, 0x01]);

    let short = Uint8Array::new_with_length(5);
    let result = short.set_from_base64("AAECAwQFBgcI").unwrap();
    assert_eq!(result.read(), 4);
    assert_eq!(result.written(), 3);
    assert_eq!(short.to_vec(), vec![0, 1, 2, 0, 0]);

    let options = Base64Options::new();
    options.set_alphabet(Base64Alphabet::Base64Url);
    let target = Uint8Array::new_with_length(4);
    let result = target
        .set_from_base64_with_options("-_-_AQ", &options)
        .unwrap();
    assert_eq!(result.read(), 6);
    assert_eq!(result.written(), 4);
    assert_eq!(target.to_vec(), vec![0xfb, 0xff, 0xbf, 0x01]);

    let short = Uint8Array::new_with_length(2);
    let result = short.set_from_hex("0001020304").unwrap();
    assert_eq!(result.read(), 4);
    assert_eq!(result.written(), 2);
    assert_eq!(short.to_vec(), vec![0, 1]);
}
