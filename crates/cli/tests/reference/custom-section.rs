use wasm_bindgen::prelude::*;

// Test that custom TypeScript sections are sorted alphabetically
// Define them in non-alphabetical order

#[wasm_bindgen(typescript_custom_section)]
const TS_ZEBRA: &'static str = r#"export type ZebraType = "stripes";"#;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPLE: &'static str = r#"export interface AppleInterface {
    color: string;
}"#;

#[wasm_bindgen(typescript_custom_section)]
const TS_ENTITY_A: &'static str = r#"type EntityId = string"#;

#[wasm_bindgen(typescript_custom_section)]
const TS_ENTITY_Z: &'static str = r#"type EntityId = number"#;

#[wasm_bindgen]
pub fn test_function() -> String {
    "test".to_string()
}
