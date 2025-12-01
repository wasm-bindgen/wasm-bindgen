use crate::{closure::WasmClosure, describe::WasmDescribe};

/// Marker trait for types that support `#[wasm_bindgen(constructor)]`.
#[cfg_attr(
    wbg_diagnostic,
    diagnostic::on_unimplemented(
        message = "JavaScript constructors are not supported for `{Self}`",
        label = "this function cannot be the constructor of `{Self}`",
        note = "`#[wasm_bindgen(constructor)]` is only supported for `struct`s and cannot be used for `enum`s.",
        note = "Consider removing the `constructor` option and using a regular static method instead."
    )
)]
pub trait SupportsConstructor {}
pub struct CheckSupportsConstructor<T: SupportsConstructor>(T);

/// Marker trait for types that support `#[wasm_bindgen(getter)]` or
/// `#[wasm_bindgen(Setter)]` on instance methods.
#[cfg_attr(
    wbg_diagnostic,
    diagnostic::on_unimplemented(
        message = "JavaScript instance getters and setters are not supported for `{Self}`",
        label = "this method cannot be a getter or setter for `{Self}`",
        note = "`#[wasm_bindgen(getter)]` and `#[wasm_bindgen(setter)]` are only supported for `struct`s and cannot be used for `enum`s.",
    )
)]
pub trait SupportsInstanceProperty {}
pub struct CheckSupportsInstanceProperty<T: SupportsInstanceProperty>(T);

/// Marker trait for types that support `#[wasm_bindgen(getter)]` or
/// `#[wasm_bindgen(Setter)]` on static methods.
#[cfg_attr(
    wbg_diagnostic,
    diagnostic::on_unimplemented(
        message = "JavaScript static getters and setters are not supported for `{Self}`",
        label = "this static function cannot be a static getter or setter on `{Self}`",
        note = "`#[wasm_bindgen(getter)]` and `#[wasm_bindgen(setter)]` are only supported for `struct`s and cannot be used for `enum`s.",
    )
)]
pub trait SupportsStaticProperty {}
pub struct CheckSupportsStaticProperty<T: SupportsStaticProperty>(T);

/// Marker type representing an untyped JavaScript value.
///
/// This is the default type parameter for `JsValue<T>`, which means that `JsValue`
/// without a type parameter is equivalent to `JsValue<AnyType>`. This marker type
/// provides compatibility with existing code while allowing the generic `JsValue<T>`
/// system to work alongside these untyped cases.
///
/// This type is not designed to be inhabited itself.
///
/// ```
#[doc(hidden)]
pub struct AnyType;

// Minimal trait implementations for AnyType to support Closure generics.
impl WasmDescribe for AnyType {
    fn describe() {}
}

// Ensures that JsValue supports clone
impl Clone for AnyType {
    fn clone(&self) -> Self {
        Self
    }
}

// This allows for closures to unwrap JsRef<T> as T
unsafe impl WasmClosure for AnyType {
    const IS_MUT: bool = false;
}
