use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tests")]
extern "C" {
    #[wasm_bindgen(typescript_type = "ImportedType")]
    pub type ImportedType;
}

#[wasm_bindgen]
pub struct ExportedStruct {
    value: u32,
}

#[wasm_bindgen]
impl ExportedStruct {
    #[wasm_bindgen(constructor)]
    pub fn new(value: u32) -> ExportedStruct {
        ExportedStruct { value }
    }
}

// `private` suppresses the `export` keyword on the generated type alias.
#[wasm_bindgen(private)]
pub enum Status {
    Success = "success",
    Error = "error",
    Data(String),
}

#[wasm_bindgen]
pub fn echo_status(status: Status) -> Status {
    status
}

#[wasm_bindgen]
pub enum ApiResponse {
    Loading = "loading",
    Empty = "empty",
    Message(String),
    Struct(ExportedStruct),
    Imported(ImportedType),
}

#[wasm_bindgen]
pub fn echo_response(response: ApiResponse) -> ApiResponse {
    response
}

#[wasm_bindgen]
pub enum Wrapper {
    Plain = "plain",
    Inner(Status),
    Direct(ExportedStruct),
}

#[wasm_bindgen]
pub fn echo_wrapper(w: Wrapper) -> Wrapper {
    w
}

#[wasm_bindgen]
pub fn echo_optional_wrapper(w: Option<Wrapper>) -> Option<Wrapper> {
    w
}

// `#[wasm_bindgen(fallback)]` makes the last tuple variant act as a
// catch-all, accepting whatever didn't match an earlier variant. Required
// when the variant's type lacks a meaningful runtime check.
#[wasm_bindgen(fallback)]
pub enum FallbackUnion {
    Loading = "loading",
    Anything(ImportedType),
}

#[wasm_bindgen]
pub fn echo_fallback(u: FallbackUnion) -> FallbackUnion {
    u
}
