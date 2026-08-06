#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `NavigationApiType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `NavigationApiType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationApiType {
    Push = "push",
    Replace = "replace",
    Reload = "reload",
    Traverse = "traverse",
}
