use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "number | string")]
    type CustomType;

    // Non-ASCII strings check descriptor length prefixes count chars, not bytes.
    #[wasm_bindgen(typescript_type = "\"Café\" | \"naïve\"")]
    type AccentedType;
}

#[wasm_bindgen]
pub fn single(a: CustomType) {}

#[wasm_bindgen]
pub fn slice(a: Vec<CustomType>) {}

#[wasm_bindgen]
pub fn accented(a: AccentedType) {}

#[wasm_bindgen(js_name = "Café")]
pub enum Cafe {
    Espresso = "espresso",
    Crème = "crème",
}

#[wasm_bindgen]
pub fn take_cafe(c: Cafe) -> Cafe {
    c
}

#[wasm_bindgen(js_name = "Größe")]
pub enum Groesse {
    Klein = 1,
    Gross = 2,
}

#[wasm_bindgen]
pub fn take_groesse(g: Groesse) -> Groesse {
    g
}
