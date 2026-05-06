use wasm_bindgen::prelude::*;

// Declare the TS interface that backs the imported type below so tsc can
// resolve `ImportedShape` when it appears in generated `.d.ts` outputs.
#[wasm_bindgen(typescript_custom_section)]
const IMPORTED_SHAPE: &'static str = r#"
interface ImportedShape {
    label: string;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ImportedShape")]
    pub type ImportedShape;
}

#[wasm_bindgen]
pub struct ExportedShape {
    pub size: u32,
}

#[wasm_bindgen]
impl ExportedShape {
    #[wasm_bindgen(constructor)]
    pub fn new(size: u32) -> ExportedShape {
        ExportedShape { size }
    }
}

/// A union with literal variants, an exported struct, a string catch-all,
/// and an interface-only import as the fallback variant. The `fallback`
/// attribute makes the last tuple variant accept whatever didn't match
/// earlier (required because `instanceof ImportedShape` is meaningless).
#[wasm_bindgen(fallback)]
pub enum ApiResponse {
    Loading = "loading",
    Empty = "empty",
    Shape(ExportedShape),
    Message(String),
    Imported(ImportedShape),
}

#[wasm_bindgen]
pub fn echo_api_response(r: ApiResponse) -> ApiResponse {
    r
}

/// A union with no fallback string variant - all string literals are closed.
#[wasm_bindgen]
pub enum Status {
    Success = "success",
    Failure = "failure",
    Detail(String),
}

#[wasm_bindgen]
pub fn echo_status(s: Status) -> Status {
    s
}

/// A nested union: one variant payload is itself a dynamic union.
#[wasm_bindgen]
pub enum Wrapped {
    Plain = "plain",
    Status(Status),
    Shape(ExportedShape),
}

#[wasm_bindgen]
pub fn echo_wrapped(w: Wrapped) -> Wrapped {
    w
}

#[wasm_bindgen]
pub fn echo_optional_wrapped(w: Option<Wrapped>) -> Option<Wrapped> {
    w
}
