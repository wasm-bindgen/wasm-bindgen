use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tests")]
extern "C" {
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

#[wasm_bindgen]
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
