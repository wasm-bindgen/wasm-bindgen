use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type SomeJsType;
}

#[wasm_bindgen]
pub fn imported_type_by_value(x: SomeJsType) {
    /* ... */
}

#[wasm_bindgen]
pub fn imported_type_by_shared_ref(x: &SomeJsType) {
    /* ... */
}

#[wasm_bindgen]
pub fn return_imported_type() -> SomeJsType {
    unimplemented!()
}

#[wasm_bindgen]
pub fn take_option_imported_type(x: Option<SomeJsType>) {
    /* ... */
}

#[wasm_bindgen]
pub fn return_option_imported_type() -> Option<SomeJsType> {
    unimplemented!()
}
