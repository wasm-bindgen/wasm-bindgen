#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `NavigationFocusReset` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `NavigationFocusReset`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationFocusReset {
    AfterTransition = "after-transition",
    Manual = "manual",
}
