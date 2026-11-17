use wasm_bindgen::prelude::*;
use wasm_bindgen::SafeInteger;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/bigint.js")]
extern "C" {
    fn i64_js_identity(a: i64) -> i64;
    fn u64_js_identity(a: u64) -> u64;
    fn js_works();
}

#[wasm_bindgen]
pub fn zero() -> u64 {
    0
}

#[wasm_bindgen]
pub fn one() -> u64 {
    1
}

#[wasm_bindgen]
pub fn neg_one() -> i64 {
    -1
}

#[wasm_bindgen]
pub fn i32_min() -> i64 {
    i32::MIN as i64
}

#[wasm_bindgen]
pub fn u32_max() -> u64 {
    u32::MAX as u64
}

#[wasm_bindgen]
pub fn i64_min() -> i64 {
    i64::MIN
}

#[wasm_bindgen]
pub fn u64_max() -> u64 {
    u64::MAX
}

#[wasm_bindgen]
pub fn i64_rust_identity(a: i64) -> i64 {
    i64_js_identity(a)
}

#[wasm_bindgen]
pub fn u64_rust_identity(a: u64) -> u64 {
    u64_js_identity(a)
}

#[wasm_bindgen]
pub fn i64_jsvalue_identity(a: i64) -> JsValue {
    JsValue::from(a)
}

#[wasm_bindgen]
pub fn u64_jsvalue_identity(a: u64) -> JsValue {
    JsValue::from(a)
}

#[wasm_bindgen]
pub fn i128_min_jsvalue() -> JsValue {
    JsValue::from(i128::MIN)
}

#[wasm_bindgen]
pub fn u128_max_jsvalue() -> JsValue {
    JsValue::from(u128::MAX)
}

#[wasm_bindgen]
pub fn i64_slice(a: &[i64]) -> Vec<i64> {
    a.to_vec()
}

#[wasm_bindgen]
pub fn u64_slice(a: &[u64]) -> Vec<u64> {
    a.to_vec()
}

#[wasm_bindgen_test]
fn works() {
    js_works();
}

mod try_from_works {
    use super::*;
    use crate::JsValue;
    use core::convert::TryFrom;

    macro_rules! test_type_boundaries {
        ($($ty:ident)*) => {
            $(
                #[wasm_bindgen_test]
                fn $ty() {
                    // Not a bigint.
                    assert!($ty::try_from(JsValue::NULL).is_err());
                    assert!($ty::try_from(JsValue::from_f64(0.0)).is_err());
                    // Within range.
                    assert_eq!($ty::try_from(JsValue::from($ty::MIN)), Ok($ty::MIN));
                    // Too small.
                    assert!($ty::try_from(JsValue::from($ty::MIN) - JsValue::from(1_i64)).is_err());
                    // Within range.
                    assert_eq!($ty::try_from(JsValue::from($ty::MAX)), Ok($ty::MAX));
                    // Too large.
                    assert!($ty::try_from(JsValue::from($ty::MAX) + JsValue::from(1_i64)).is_err());
                }
            )*
        };
    }

    test_type_boundaries!(i64 u64 i128 u128);
}

#[wasm_bindgen(module = "tests/wasm/bigint.js")]
extern "C" {
    fn safe_u64_js_identity(a: SafeInteger<u64>) -> SafeInteger<u64>;
    fn safe_i64_js_identity(a: SafeInteger<i64>) -> SafeInteger<i64>;
    fn safe_opt_js_identity(a: Option<SafeInteger<u64>>) -> Option<SafeInteger<u64>>;
    fn safe_vec_js_identity(a: Vec<SafeInteger<i64>>) -> Vec<SafeInteger<i64>>;
    fn safe_is_number(a: SafeInteger<u64>) -> bool;
    #[wasm_bindgen(catch)]
    fn safe_catch(ok: bool) -> Result<SafeInteger<u64>, JsValue>;
    fn safe_call(f: &Closure<dyn Fn(SafeInteger<i64>) -> SafeInteger<i64>>) -> SafeInteger<i64>;
    fn safe_returns(n: f64) -> wasm_bindgen::SafeU64;
    fn safe_js_works();
}

const MAX_SAFE: u64 = (1 << 53) - 1;

#[wasm_bindgen]
pub fn safe_u64_rust_identity(a: SafeInteger<u64>) -> SafeInteger<u64> {
    a
}

#[wasm_bindgen]
pub fn safe_i64_rust_identity(a: SafeInteger<i64>) -> SafeInteger<i64> {
    a
}

#[wasm_bindgen]
pub fn safe_opt_rust_identity(a: Option<SafeInteger<i64>>) -> Option<SafeInteger<i64>> {
    a
}

#[wasm_bindgen]
pub fn safe_slice_doubled(a: &[SafeInteger<u64>]) -> Vec<SafeInteger<u64>> {
    a.iter().map(|n| *n * 2).collect()
}

#[wasm_bindgen]
pub fn safe_opt_vec_identity(a: Option<Vec<SafeInteger<i64>>>) -> Option<Vec<SafeInteger<i64>>> {
    a
}

#[wasm_bindgen]
pub fn safe_catch_export(ok: bool) -> Result<SafeInteger<u64>, JsError> {
    if ok {
        Ok(SafeInteger::MAX)
    } else {
        Err(JsError::new("not ok"))
    }
}

#[wasm_bindgen_test]
fn safe_integer_round_trips() {
    for v in [0, 1, MAX_SAFE] {
        assert_eq!(safe_u64_js_identity(v.into()), v);
    }
    for v in [0, -1, MAX_SAFE as i64, -(MAX_SAFE as i64)] {
        assert_eq!(safe_i64_js_identity(v.into()), v);
    }
    assert!(safe_is_number(SafeInteger::MAX));

    // `Some(2^53 - 1)` is `Number.MAX_SAFE_INTEGER`, which a sentinel `None`
    // encoding would collide with.
    let max = SafeInteger::<u64>::MAX;
    assert_eq!(safe_opt_js_identity(Some(max)), Some(max));
    assert_eq!(safe_opt_js_identity(Some(0.into())), Some(0.into()));
    assert_eq!(safe_opt_js_identity(None), None);

    let values: Vec<wasm_bindgen::SafeI64> = vec![0.into(), (-5).into(), SafeInteger::MIN];
    assert_eq!(safe_vec_js_identity(values.clone()), values);
    assert!(safe_vec_js_identity(Vec::new()).is_empty());

    assert_eq!(safe_catch(true).unwrap(), MAX_SAFE);
    assert!(safe_catch(false).is_err());

    let double = Closure::new(|n: SafeInteger<i64>| n * 2);
    assert_eq!(safe_call(&double), -42);

    safe_js_works();
}

#[wasm_bindgen_test]
fn safe_integer_ergonomics() {
    let n = SafeInteger::from(10_000_u64);
    assert!(n > 9_999);
    assert!(10_001 > n);
    assert_eq!(n + 1, 10_001);
    assert_eq!(n.count_ones(), 10_000_u64.count_ones());
    let mut m = n;
    m -= 1;
    m *= SafeInteger::from(2);
    assert_eq!(u64::from(m), 19_998);
    assert_eq!(-SafeInteger::from(3_i64), -3);
    assert_eq!(format!("{n} {n:?}"), "10000 10000");
    assert_eq!(SafeInteger::new(MAX_SAFE), Some(SafeInteger::MAX));
    assert_eq!(SafeInteger::new(MAX_SAFE + 1), None);
    assert_eq!(JsValue::from(n), JsValue::from_f64(10_000.0));
}

#[wasm_bindgen_test]
fn safe_integer_try_from_js_value() {
    use core::convert::TryFrom;

    assert_eq!(
        SafeInteger::<u64>::try_from(JsValue::from_f64(MAX_SAFE as f64)),
        Ok(SafeInteger::MAX)
    );
    assert_eq!(
        SafeInteger::<i64>::try_from(JsValue::from_f64(-(MAX_SAFE as f64))),
        Ok(SafeInteger::MIN)
    );
    for bad in [
        JsValue::from_f64(1.5),
        JsValue::from_f64((MAX_SAFE + 1) as f64),
        JsValue::from_f64(-1.0),
        JsValue::from_f64(f64::NAN),
        JsValue::from(1_u64),
        JsValue::NULL,
    ] {
        assert!(SafeInteger::<u64>::try_from(&bad).is_err(), "{bad:?}");
    }
    assert!(SafeInteger::<i64>::try_from(JsValue::from_f64(-((MAX_SAFE + 1) as f64))).is_err());
}

#[cfg(debug_assertions)]
#[wasm_bindgen_test]
#[should_panic(expected = "9007199254740992 is not a safe integer")]
fn safe_integer_rejects_unsafe_integer() {
    safe_returns((MAX_SAFE + 1) as f64);
}

#[cfg(debug_assertions)]
#[wasm_bindgen_test]
#[should_panic(expected = "1.5 is not a safe integer")]
fn safe_integer_rejects_fraction() {
    safe_returns(1.5);
}
