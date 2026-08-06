#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `NavigationScrollBehavior` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `NavigationScrollBehavior`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationScrollBehavior {
    AfterTransition = "after-transition",
    Manual = "manual",
}
