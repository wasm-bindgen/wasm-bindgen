use self::inner::ColorWithCustomValues;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/enums.js")]
extern "C" {
    fn js_c_style_enum();
    fn js_c_style_enum_with_custom_values();
    fn js_handle_optional_enums(x: Option<Color>) -> Option<Color>;
    fn js_expect_enum(x: Color, y: Option<Color>);
    fn js_expect_enum_none(x: Option<Color>);
    fn js_renamed_enum(b: RenamedEnum);
    fn js_enum_with_error_variant();

    pub type FooCase;
    #[wasm_bindgen(js_name = makeFoo)]
    fn make_foo() -> FooCase;

    // Round-trip helpers: each calls the corresponding exported Rust
    // function from JS, forcing the value to cross the wasm/JS boundary
    // and exercising the dynamic-union dispatcher in `from_abi`.
    fn js_string_enum_fallback_roundtrip(e: StringEnumWithFallback) -> StringEnumWithFallback;
    fn js_nested_union_roundtrip(o: OuterUnion) -> OuterUnion;
    fn js_optional_union_roundtrip(o: Option<OuterUnion>) -> Option<OuterUnion>;
    fn js_fallback_union_roundtrip(u: FallbackUnion) -> FallbackUnion;

    // Async round-trip: imports an `async fn` returning a dynamic union,
    // exercising `Promise<Union>` resolution across the `JsFuture` seam.
    async fn js_async_union_roundtrip(o: OuterUnion) -> OuterUnion;

    // Same but with `catch`: the resolved success type is still `Union`,
    // wrapped in `Result<_, JsValue>` for the rejection path.
    #[wasm_bindgen(catch)]
    async fn js_async_union_result(o: OuterUnion) -> Result<OuterUnion, JsValue>;
}

#[wasm_bindgen]
#[derive(PartialEq, Debug)]
pub enum Color {
    Green,
    Yellow,
    Red,
}

pub mod inner {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub enum ColorWithCustomValues {
        Green = 21,
        Yellow = 34,
        Red = 2,
    }
}

#[wasm_bindgen(js_name = JsRenamedEnum)]
#[derive(Copy, Clone)]
pub enum RenamedEnum {
    A = 10,
    B = 20,
}

#[wasm_bindgen]
pub fn enum_cycle(color: Color) -> Color {
    match color {
        Color::Green => Color::Yellow,
        Color::Yellow => Color::Red,
        Color::Red => Color::Green,
    }
}

#[wasm_bindgen]
pub fn enum_with_custom_values_cycle(color: ColorWithCustomValues) -> ColorWithCustomValues {
    match color {
        ColorWithCustomValues::Green => ColorWithCustomValues::Yellow,
        ColorWithCustomValues::Yellow => ColorWithCustomValues::Red,
        ColorWithCustomValues::Red => ColorWithCustomValues::Green,
    }
}

#[wasm_bindgen_test]
fn c_style_enum() {
    js_c_style_enum();
}

#[wasm_bindgen_test]
fn c_style_enum_with_custom_values() {
    js_c_style_enum_with_custom_values();
}

#[wasm_bindgen]
pub fn handle_optional_enums(x: Option<Color>) -> Option<Color> {
    x
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum EnumWithErrorVariant {
    OK,
    Warning,
    Error,
}

#[wasm_bindgen_test]
fn test_optional_enums() {
    use self::Color::*;

    assert_eq!(js_handle_optional_enums(None), None);
    assert_eq!(js_handle_optional_enums(Some(Green)), Some(Green));
    assert_eq!(js_handle_optional_enums(Some(Yellow)), Some(Yellow));
    assert_eq!(js_handle_optional_enums(Some(Red)), Some(Red));
}

#[wasm_bindgen_test]
fn test_optional_enum_values() {
    use self::Color::*;

    js_expect_enum(Green, Some(Green));
    js_expect_enum(Yellow, Some(Yellow));
    js_expect_enum(Red, Some(Red));
    js_expect_enum_none(None);
}

#[wasm_bindgen_test]
fn test_renamed_enum() {
    js_renamed_enum(RenamedEnum::B);
}

#[wasm_bindgen_test]
fn test_enum_with_error_variant() {
    js_enum_with_error_variant();
}

// Exported struct for testing enum variants
#[wasm_bindgen]
#[derive(PartialEq, Debug)]
pub struct Bar {
    value: u32,
}

#[wasm_bindgen]
impl Bar {
    #[wasm_bindgen(constructor)]
    pub fn new(value: u32) -> Bar {
        Bar { value }
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> u32 {
        self.value
    }
}

// A dynamic union mixing string literals, an exported struct, a c-style
// enum, an `Option<u32>`, and a `String` catch-all. Variant order matters
// since dispatch is in source order: narrower types come first, the
// `String` catch-all is last.
#[wasm_bindgen]
pub enum StringEnumWithFallback {
    Red = "red",
    Green = "green",
    Blue = "blue",
    AnotherOther(Bar),
    AnotherColor(Color),
    Wow(Option<u32>),
    Other(String),
}

#[wasm_bindgen]
pub fn string_enum_fallback_roundtrip(e: StringEnumWithFallback) -> StringEnumWithFallback {
    e
}

#[wasm_bindgen_test]
fn test_string_enum_with_fallback() {
    assert!(matches!(
        js_string_enum_fallback_roundtrip(StringEnumWithFallback::Red),
        StringEnumWithFallback::Red
    ));

    assert!(matches!(
        js_string_enum_fallback_roundtrip(StringEnumWithFallback::Green),
        StringEnumWithFallback::Green
    ));

    assert!(matches!(
        js_string_enum_fallback_roundtrip(StringEnumWithFallback::Blue),
        StringEnumWithFallback::Blue
    ));

    let bar = Bar::new(42);
    let result = js_string_enum_fallback_roundtrip(StringEnumWithFallback::AnotherOther(bar));

    assert!(matches!(result, StringEnumWithFallback::AnotherOther(ref b) if b.value() == 42));

    let result =
        js_string_enum_fallback_roundtrip(StringEnumWithFallback::AnotherColor(Color::Yellow));
    assert!(
        matches!(result, StringEnumWithFallback::AnotherColor(color) if color == Color::Yellow)
    );

    assert!(
        matches!(js_string_enum_fallback_roundtrip(StringEnumWithFallback::Other("custom".to_string())), StringEnumWithFallback::Other(s) if s == "custom")
    );

    assert!(
        matches!(js_string_enum_fallback_roundtrip(StringEnumWithFallback::Other("yellow".to_string())),
        StringEnumWithFallback::Other(s) if s == "yellow")
    );

    assert!(
        matches!(js_string_enum_fallback_roundtrip(StringEnumWithFallback::Wow(Some(42))),
        StringEnumWithFallback::Wow(Some(val)) if val == 42)
    );

    assert!(matches!(
        js_string_enum_fallback_roundtrip(StringEnumWithFallback::Wow(None)),
        StringEnumWithFallback::Wow(None)
    ));
}

#[wasm_bindgen]
pub enum InnerUnion {
    Foo = "foo",
    Bar = "bar",
    Number(u32),
}

#[wasm_bindgen]
pub enum OuterUnion {
    Loading = "loading",
    Wrapped(InnerUnion),
    Bare(Bar),
}

#[wasm_bindgen]
pub fn nested_union_roundtrip(o: OuterUnion) -> OuterUnion {
    o
}

#[wasm_bindgen]
pub fn optional_union_roundtrip(o: Option<OuterUnion>) -> Option<OuterUnion> {
    o
}

#[wasm_bindgen_test]
fn test_nested_union_roundtrip() {
    // Outer string-literal variant.
    assert!(matches!(
        js_nested_union_roundtrip(OuterUnion::Loading),
        OuterUnion::Loading
    ));

    // Nested inner string-literal variant: must dispatch through the outer
    // union into the inner union and recover the right inner variant.
    assert!(matches!(
        js_nested_union_roundtrip(OuterUnion::Wrapped(InnerUnion::Foo)),
        OuterUnion::Wrapped(InnerUnion::Foo)
    ));

    // Nested inner type variant.
    assert!(matches!(
        js_nested_union_roundtrip(OuterUnion::Wrapped(InnerUnion::Number(7))),
        OuterUnion::Wrapped(InnerUnion::Number(7))
    ));

    // Bare exported-struct variant (not nested).
    let bar = Bar::new(99);
    let result = js_nested_union_roundtrip(OuterUnion::Bare(bar));
    assert!(matches!(result, OuterUnion::Bare(b) if b.value() == 99));
}

#[wasm_bindgen_test]
fn test_optional_union_roundtrip() {
    assert!(js_optional_union_roundtrip(None).is_none());

    assert!(matches!(
        js_optional_union_roundtrip(Some(OuterUnion::Loading)),
        Some(OuterUnion::Loading)
    ));

    assert!(matches!(
        js_optional_union_roundtrip(Some(OuterUnion::Wrapped(InnerUnion::Bar))),
        Some(OuterUnion::Wrapped(InnerUnion::Bar))
    ));

    assert!(matches!(
        js_optional_union_roundtrip(Some(OuterUnion::Wrapped(InnerUnion::Number(42)))),
        Some(OuterUnion::Wrapped(InnerUnion::Number(42)))
    ));
}

// `#[wasm_bindgen(fallback)]` makes the last tuple variant act as an
// unconditional catch-all. This is the supported pattern when the variant's
// payload type has no meaningful runtime check (e.g., interface-only
// imports).
#[wasm_bindgen(fallback)]
pub enum FallbackUnion {
    One = "one",
    Two = "two",
    Anything(FooCase),
}

#[wasm_bindgen]
pub fn fallback_union_roundtrip(u: FallbackUnion) -> FallbackUnion {
    u
}

#[wasm_bindgen_test]
fn test_fallback_union_roundtrip() {
    // Literal variants still take precedence over the fallback.
    assert!(matches!(
        js_fallback_union_roundtrip(FallbackUnion::One),
        FallbackUnion::One
    ));
    assert!(matches!(
        js_fallback_union_roundtrip(FallbackUnion::Two),
        FallbackUnion::Two
    ));

    // An interface-only imported value falls into `Anything` because
    // `instanceof FooCase` is meaningless and the variant is the fallback.
    let foo = make_foo();
    assert!(matches!(
        js_fallback_union_roundtrip(FallbackUnion::Anything(foo)),
        FallbackUnion::Anything(_)
    ));
}

// Verifies that an imported `async fn` returning a dynamic union resolves
// correctly: the JS-side `Promise<Union>` flows through `JsFuture` and the
// closure-shim's `from_abi` runs the union dispatcher. This case was
// missed when unions were originally added — the original
// `From<Promise<T>> for JsFuture<T>` impl required `T: JsGeneric`, which
// dynamic unions cannot satisfy because they are tagged Rust enums (not
// `#[repr(transparent)]` wrappers around `JsValue`). The bound has been
// loosened to `T: FromWasmAbi + 'static`, which is the actual minimum the
// closure shim needs.
#[wasm_bindgen]
pub async fn async_union_roundtrip(o: OuterUnion) -> OuterUnion {
    o
}

#[wasm_bindgen_test]
async fn test_async_union_roundtrip() {
    let bar = Bar::new(99);
    let result = js_async_union_roundtrip(OuterUnion::Bare(bar)).await;
    assert!(matches!(result, OuterUnion::Bare(b) if b.value() == 99));
}

// `catch` projects to `Result<Union, JsValue>` on the Rust side; the
// success type still flows through the same `JsFuture` seam.
#[wasm_bindgen_test]
async fn test_async_union_result_catch() {
    let res = js_async_union_result(OuterUnion::Wrapped(InnerUnion::Number(123))).await;
    assert!(matches!(
        res,
        Ok(OuterUnion::Wrapped(InnerUnion::Number(123)))
    ));
}
