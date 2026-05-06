//! Microbenchmark comparing string-enum and dynamic-union round-trip cost.
//!
//! Both variants below describe the same JS-side type (`"a" | "b" | "c"`)
//! but differ in their wasm ABI:
//!
//! - `StringEnum` uses a u32 index ABI. No JS string allocation per crossing.
//! - `DynamicUnion` uses an externref ABI. Each crossing allocates an
//!   externref slot and a JS string.
//!
//! Each iteration roundtrips a value through a JS shim (`echoVariant`)
//! which forces both `into_abi` (Rust → JS) and `from_abi` (JS → Rust).

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::{wasm_bindgen_bench, Criterion};

#[wasm_bindgen(inline_js = r#"
    export function echoVariant(v) { return v; }
"#)]
extern "C" {
    fn echoVariant(v: JsValue) -> JsValue;
}

#[wasm_bindgen]
pub enum StringEnum {
    A = "a",
    B = "b",
    C = "c",
}

#[wasm_bindgen]
pub enum DynamicUnion {
    A = "a",
    B = "b",
    C = "c",
    Other(String),
}

// We can't directly invoke `echoVariant` with our typed arguments because
// the function takes `JsValue`. Instead we wrap each side: convert the
// typed value to `JsValue`, send it through, and convert back. This is
// what wasm-bindgen does at the boundary for any function call that takes
// or returns one of these types.
#[inline(never)]
fn roundtrip_string_enum(v: StringEnum) -> StringEnum {
    let js: JsValue = v.into();
    let out = echoVariant(js);
    StringEnum::from_js_value(&out).unwrap()
}

#[inline(never)]
fn roundtrip_dynamic_union(v: DynamicUnion) -> DynamicUnion {
    let js: JsValue = v.into();
    let out = echoVariant(js);
    use wasm_bindgen::convert::TryFromJsValue;
    DynamicUnion::try_from_js_value(out).unwrap()
}

#[wasm_bindgen_bench]
fn bench_string_enum_roundtrip(c: &mut Criterion) {
    c.bench_function("string_enum_roundtrip", |b| {
        b.iter(|| {
            let _ = roundtrip_string_enum(StringEnum::B);
        });
    });
}

#[wasm_bindgen_bench]
fn bench_dynamic_union_roundtrip(c: &mut Criterion) {
    c.bench_function("dynamic_union_roundtrip", |b| {
        b.iter(|| {
            let _ = roundtrip_dynamic_union(DynamicUnion::B);
        });
    });
}
