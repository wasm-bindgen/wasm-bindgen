// DEPENDENCY: web-sys = { path = '{root}/crates/web-sys', features = ['console', 'Url', 'MediaSourceEnum', 'MediaSourceReadyState'] }

use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{MediaSourceEnum, MediaSourceReadyState, Url};

#[wasm_bindgen]
pub fn get_url() -> Url {
    assert_eq!(MediaSourceReadyState::Closed, MediaSourceReadyState::Closed);
    Url::new("https://example.com").unwrap()
}

#[wasm_bindgen]
pub fn get_media_source() -> MediaSourceEnum {
    MediaSourceEnum::Camera
}
