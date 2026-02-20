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
