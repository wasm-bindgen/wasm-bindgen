use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "number | string")]
    type CustomType;

    // A non-ASCII `typescript_type`. The descriptor wire format writes one word
    // per `char` and prefixes it with a length, so the length must be a `char`
    // count; a UTF-8 byte count would leave the decoder reading descriptor
    // payload words as characters and silently mis-bind everything after it.
    #[wasm_bindgen(typescript_type = "\"Café\" | \"naïve\"")]
    type AccentedType;
}

#[wasm_bindgen]
pub fn single(a: CustomType) {}

#[wasm_bindgen]
pub fn slice(a: Vec<CustomType>) {}

#[wasm_bindgen]
pub fn accented(a: AccentedType) {}

// A string enum whose JS-visible name is not ASCII. Its descriptor also carries
// a length-prefixed name, so this pins the same invariant on a second path.
#[wasm_bindgen(js_name = "Café")]
pub enum Cafe {
    Espresso = "espresso",
    Crème = "crème",
}

#[wasm_bindgen]
pub fn take_cafe(c: Cafe) -> Cafe {
    c
}

// A C-style enum whose JS-visible name is not ASCII, pinning the invariant on a
// third path (`Enum`'s descriptor carries a length-prefixed qualified name).
#[wasm_bindgen(js_name = "Größe")]
pub enum Groesse {
    Klein = 1,
    Gross = 2,
}

#[wasm_bindgen]
pub fn take_groesse(g: Groesse) -> Groesse {
    g
}


