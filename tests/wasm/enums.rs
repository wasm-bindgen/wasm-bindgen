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

#[wasm_bindgen]
pub enum StringEnumWithFallback {
    Red = "red",
    Green = "green",
    Blue = "blue",
    Another(FooCase),
    AnotherOther(Bar),
    AnotherColor(Color),
    Wow(Option<u32>),
    Wow2(Option<u64>),
    Other(String),
}

#[wasm_bindgen]
pub fn string_enum_fallback_roundtrip(e: StringEnumWithFallback) -> StringEnumWithFallback {
    e
}

#[wasm_bindgen_test]
fn test_string_enum_with_fallback() {
    assert!(matches!(
        string_enum_fallback_roundtrip(StringEnumWithFallback::Red),
        StringEnumWithFallback::Red
    ));

    assert!(matches!(
        string_enum_fallback_roundtrip(StringEnumWithFallback::Green),
        StringEnumWithFallback::Green
    ));

    assert!(matches!(
        string_enum_fallback_roundtrip(StringEnumWithFallback::Blue),
        StringEnumWithFallback::Blue
    ));

    let foo = make_foo();
    assert!(matches!(
        string_enum_fallback_roundtrip(StringEnumWithFallback::Another(foo)),
        StringEnumWithFallback::Another(_)
    ));

    let bar = Bar::new(42);
    let result = string_enum_fallback_roundtrip(StringEnumWithFallback::AnotherOther(bar));

    assert!(matches!(result, StringEnumWithFallback::AnotherOther(ref b) if b.value() == 42));

    let result =
        string_enum_fallback_roundtrip(StringEnumWithFallback::AnotherColor(Color::Yellow));
    assert!(
        matches!(result, StringEnumWithFallback::AnotherColor(color) if color == Color::Yellow)
    );

    assert!(
        matches!(string_enum_fallback_roundtrip(StringEnumWithFallback::Other("custom".to_string())), StringEnumWithFallback::Other(s) if s == "custom")
    );

    assert!(
        matches!(string_enum_fallback_roundtrip(StringEnumWithFallback::Other("yellow".to_string())),
        StringEnumWithFallback::Other(s) if s == "yellow")
    );

    assert!(
        matches!(string_enum_fallback_roundtrip(StringEnumWithFallback::Wow(Some(42))),
        StringEnumWithFallback::Wow(Some(val)) if val == 42)
    );

    assert!(matches!(
        string_enum_fallback_roundtrip(StringEnumWithFallback::Wow(None)),
        StringEnumWithFallback::Wow(None)
    ));

    assert!(
        matches!(string_enum_fallback_roundtrip(StringEnumWithFallback::Wow2(Some(99))),
        StringEnumWithFallback::Wow2(Some(val)) if val == 99)
    );

    assert!(matches!(
        string_enum_fallback_roundtrip(StringEnumWithFallback::Wow2(None)),
        StringEnumWithFallback::Wow2(None)
    ));
}
