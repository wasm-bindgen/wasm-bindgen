use wasm_bindgen::prelude::*;

#[path = "../generated/bindings.rs"]
mod bindings;
use bindings::*;

#[wasm_bindgen(js_name = "fetch", js_namespace = "default")]
pub async fn fetch_handler(
    req: Request,
    _env: Env,
    _ctx: ExecutionContext,
) -> Result<Response, JsValue> {

}























// @cf/bytedance/stable-diffusion-xl-lightning
