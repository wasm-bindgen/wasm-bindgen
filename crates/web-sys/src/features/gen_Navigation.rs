#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "Navigation",
        typescript_type = "Navigation"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Navigation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`*"]
    pub type Navigation;
    #[cfg(feature = "NavigationHistoryEntry")]
    #[wasm_bindgen(method, getter, js_class = "Navigation", js_name = "currentEntry")]
    #[doc = "Getter for the `currentEntry` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/currentEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`, `NavigationHistoryEntry`*"]
    pub fn current_entry(this: &Navigation) -> Option<NavigationHistoryEntry>;
    #[cfg(feature = "NavigationTransition")]
    #[wasm_bindgen(method, getter, js_class = "Navigation", js_name = "transition")]
    #[doc = "Getter for the `transition` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/transition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`, `NavigationTransition`*"]
    pub fn transition(this: &Navigation) -> Option<NavigationTransition>;
    #[cfg(feature = "NavigationActivation")]
    #[wasm_bindgen(method, getter, js_class = "Navigation", js_name = "activation")]
    #[doc = "Getter for the `activation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/activation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`, `NavigationActivation`*"]
    pub fn activation(this: &Navigation) -> Option<NavigationActivation>;
    #[wasm_bindgen(method, getter, js_class = "Navigation", js_name = "canGoBack")]
    #[doc = "Getter for the `canGoBack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/canGoBack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`*"]
    pub fn can_go_back(this: &Navigation) -> bool;
    #[wasm_bindgen(method, getter, js_class = "Navigation", js_name = "canGoForward")]
    #[doc = "Getter for the `canGoForward` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/canGoForward)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`*"]
    pub fn can_go_forward(this: &Navigation) -> bool;
    #[wasm_bindgen(method, getter, js_class = "Navigation", js_name = "onnavigate")]
    #[doc = "Getter for the `onnavigate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/onnavigate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`*"]
    pub fn onnavigate(this: &Navigation) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Navigation", js_name = "onnavigate")]
    #[doc = "Setter for the `onnavigate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/onnavigate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`*"]
    pub fn set_onnavigate(this: &Navigation, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Navigation", js_name = "onnavigatesuccess")]
    #[doc = "Getter for the `onnavigatesuccess` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/onnavigatesuccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`*"]
    pub fn onnavigatesuccess(this: &Navigation) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Navigation", js_name = "onnavigatesuccess")]
    #[doc = "Setter for the `onnavigatesuccess` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/onnavigatesuccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`*"]
    pub fn set_onnavigatesuccess(this: &Navigation, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Navigation", js_name = "onnavigateerror")]
    #[doc = "Getter for the `onnavigateerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/onnavigateerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`*"]
    pub fn onnavigateerror(this: &Navigation) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Navigation", js_name = "onnavigateerror")]
    #[doc = "Setter for the `onnavigateerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/onnavigateerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`*"]
    pub fn set_onnavigateerror(this: &Navigation, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Navigation",
        js_name = "oncurrententrychange"
    )]
    #[doc = "Getter for the `oncurrententrychange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/oncurrententrychange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`*"]
    pub fn oncurrententrychange(this: &Navigation) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "Navigation",
        js_name = "oncurrententrychange"
    )]
    #[doc = "Setter for the `oncurrententrychange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/oncurrententrychange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`*"]
    pub fn set_oncurrententrychange(this: &Navigation, value: Option<&::js_sys::Function>);
    #[cfg(feature = "NavigationResult")]
    #[wasm_bindgen(method, js_class = "Navigation")]
    #[doc = "The `back()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/back)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`, `NavigationResult`*"]
    pub fn back(this: &Navigation) -> NavigationResult;
    #[cfg(all(feature = "NavigationOptions", feature = "NavigationResult",))]
    #[wasm_bindgen(method, js_class = "Navigation", js_name = "back")]
    #[doc = "The `back()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/back)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`, `NavigationOptions`, `NavigationResult`*"]
    pub fn back_with_options(this: &Navigation, options: &NavigationOptions) -> NavigationResult;
    #[wasm_bindgen(method, js_class = "Navigation")]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`*"]
    pub fn entries(this: &Navigation) -> ::js_sys::Array;
    #[cfg(feature = "NavigationResult")]
    #[wasm_bindgen(method, js_class = "Navigation")]
    #[doc = "The `forward()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/forward)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`, `NavigationResult`*"]
    pub fn forward(this: &Navigation) -> NavigationResult;
    #[cfg(all(feature = "NavigationOptions", feature = "NavigationResult",))]
    #[wasm_bindgen(method, js_class = "Navigation", js_name = "forward")]
    #[doc = "The `forward()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/forward)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`, `NavigationOptions`, `NavigationResult`*"]
    pub fn forward_with_options(this: &Navigation, options: &NavigationOptions)
        -> NavigationResult;
    #[cfg(feature = "NavigationResult")]
    #[wasm_bindgen(method, js_class = "Navigation")]
    #[doc = "The `navigate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/navigate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`, `NavigationResult`*"]
    pub fn navigate(this: &Navigation, url: &str) -> NavigationResult;
    #[cfg(all(feature = "NavigationNavigateOptions", feature = "NavigationResult",))]
    #[wasm_bindgen(method, js_class = "Navigation", js_name = "navigate")]
    #[doc = "The `navigate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/navigate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`, `NavigationNavigateOptions`, `NavigationResult`*"]
    pub fn navigate_with_options(
        this: &Navigation,
        url: &str,
        options: &NavigationNavigateOptions,
    ) -> NavigationResult;
    #[cfg(feature = "NavigationResult")]
    #[wasm_bindgen(method, js_class = "Navigation")]
    #[doc = "The `reload()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/reload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`, `NavigationResult`*"]
    pub fn reload(this: &Navigation) -> NavigationResult;
    #[cfg(all(feature = "NavigationReloadOptions", feature = "NavigationResult",))]
    #[wasm_bindgen(method, js_class = "Navigation", js_name = "reload")]
    #[doc = "The `reload()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/reload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`, `NavigationReloadOptions`, `NavigationResult`*"]
    pub fn reload_with_options(
        this: &Navigation,
        options: &NavigationReloadOptions,
    ) -> NavigationResult;
    #[cfg(feature = "NavigationResult")]
    #[wasm_bindgen(method, js_class = "Navigation", js_name = "traverseTo")]
    #[doc = "The `traverseTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/traverseTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`, `NavigationResult`*"]
    pub fn traverse_to(this: &Navigation, key: &str) -> NavigationResult;
    #[cfg(all(feature = "NavigationOptions", feature = "NavigationResult",))]
    #[wasm_bindgen(method, js_class = "Navigation", js_name = "traverseTo")]
    #[doc = "The `traverseTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/traverseTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`, `NavigationOptions`, `NavigationResult`*"]
    pub fn traverse_to_with_options(
        this: &Navigation,
        key: &str,
        options: &NavigationOptions,
    ) -> NavigationResult;
    #[cfg(feature = "NavigationUpdateCurrentEntryOptions")]
    #[wasm_bindgen(method, js_class = "Navigation", js_name = "updateCurrentEntry")]
    #[doc = "The `updateCurrentEntry()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/updateCurrentEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigation`, `NavigationUpdateCurrentEntryOptions`*"]
    pub fn update_current_entry(this: &Navigation, options: &NavigationUpdateCurrentEntryOptions);
}
