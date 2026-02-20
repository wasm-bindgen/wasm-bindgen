#[cfg(web_sys_unstable_apis)]
use crate::generated::*;
#[cfg(web_sys_unstable_apis)]
use wasm_bindgen_test::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen_test]
fn can_use_unstable_apis() {
    let unstable_interface = GetUnstableInterface::get();
    assert_eq!(0u32, unstable_interface.enum_value());

    let dict = UnstableDictionary::new();
    dict.set_unstable_enum(UnstableEnum::B);
    assert_eq!(
        2u32,
        unstable_interface.enum_value_with_unstable_dictionary(&dict)
    );
}

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen_test]
#[allow(deprecated)]
fn dictionary_union_expansion() {
    use wasm_bindgen::JsCast;
    let obj = js_sys::Object::new();
    let a: &TypeA = obj.unchecked_ref();
    let b: &TypeB = obj.unchecked_ref();

    // Required field: deprecated unsuffixed setter and type-safe suffixed setters
    let dict = DictWithUnion::new(a);
    dict.set_view(a); // deprecated backward-compat setter
    dict.set_view_type_a(a);
    dict.set_view_type_b(b);

    // Optional field: same expansion
    dict.set_optional_view(a); // deprecated backward-compat setter
    dict.set_optional_view_type_a(a);
    dict.set_optional_view_type_b(b);
}
