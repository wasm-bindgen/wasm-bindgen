use wasm_bindgen::prelude::*;

#[path = "../generated/bindings.rs"]
mod bindings;
use bindings::*;

#[wasm_bindgen(js_name = "fetch", js_namespace = "default")]
pub async fn fetch_handler(
    request: Request,
    env: Env,
    _ctx: ExecutionContext,
) -> Result<Response, JsValue> {
    let image_data = env
        .ai()
        .run(
            "@cf/bytedance/stable-diffusion-xl-lightning",
            &AiTextToImageInput::builder()
                .prompt("cyberpunk cat")
                .build()?,
        )
        .into_future()
        .await?;

    let headers = Headers::new()?;
    headers.set("content-type", "image/jpg");

    Response::new_with_readable_stream_and_init(
        Some(&image_data),
        &ResponseInit::builder().headers(&headers).build(),
    )
}
