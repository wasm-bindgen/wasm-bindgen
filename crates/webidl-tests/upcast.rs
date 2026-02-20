use crate::generated::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_upcast_child_to_base() {
    // Create a ChildType instance
    let child = ChildType::new().unwrap();
    child.set_value("child value");
    child.set_child_value(42);

    // Upcast child to BaseType and pass to function expecting BaseType
    let base: BaseType = child.upcast_into();
    let result = UpcastTest::process_base(&base);
    assert_eq!(result, "child value");
}

#[wasm_bindgen_test]
fn test_upcast_grandchild_to_base() {
    // Create a GrandChildType instance
    let grandchild = GrandChildType::new().unwrap();
    grandchild.set_value("grandchild value");
    grandchild.set_child_value(99);
    grandchild.set_grand_child_value(true);

    // Upcast grandchild directly to BaseType (skipping intermediate parent)
    let base: BaseType = grandchild.upcast_into();
    let result = UpcastTest::process_base(&base);
    assert_eq!(result, "grandchild value");
}

#[wasm_bindgen_test]
fn test_upcast_grandchild_to_child() {
    // Create a GrandChildType instance
    let grandchild = GrandChildType::new().unwrap();
    grandchild.set_value("test");
    grandchild.set_child_value(123);
    grandchild.set_grand_child_value(false);

    // Upcast grandchild to ChildType
    let child: ChildType = grandchild.upcast_into();
    assert_eq!(child.child_value(), 123);

    let result = UpcastTest::process_child(&child);
    assert_eq!(result, "test");
}
