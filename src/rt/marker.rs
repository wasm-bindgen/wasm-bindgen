use wasm_bindgen_shared::tys::EXTERNREF;

use crate::{
    cast::JsCast,
    closure::WasmClosure,
    convert::{FromWasmAbi, IntoWasmAbi, LongRefFromWasmAbi, RefFromWasmAbi},
    describe::{inform, WasmDescribe},
    JsValue,
};
use core::mem::ManuallyDrop;

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
/// This type is only inhabited when unwrapping JsValue<AnyType> -> AnyType as a
/// a recursion-breaking detail.
///
/// ```
#[doc(hidden)]
pub struct AnyType(u32);

impl Clone for AnyType {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl AsRef<JsValue> for AnyType {
    #[inline]
    fn as_ref(&self) -> &JsValue {
        // SAFETY: AnyType has the same representation as JsValue
        unsafe { core::mem::transmute(self) }
    }
}

impl From<AnyType> for JsValue {
    #[inline]
    fn from(val: AnyType) -> JsValue {
        // SAFETY: AnyType has the same representation as JsValue (both are u32)
        unsafe { core::mem::transmute(val) }
    }
}

/// Marker trait for types which implement generics, indicating that they can be
/// transmuted on their generics across ABI boundaries.
pub trait GenericType {}

// ABI implementations for AnyType - identical to JsValue/JsRef<T>
impl IntoWasmAbi for AnyType {
    type Abi = u32;

    #[inline]
    fn into_abi(self) -> u32 {
        self.0
    }
}

impl FromWasmAbi for AnyType {
    type Abi = u32;

    #[inline]
    unsafe fn from_abi(js: u32) -> Self {
        AnyType(js)
    }
}

impl IntoWasmAbi for &AnyType {
    type Abi = u32;

    #[inline]
    fn into_abi(self) -> u32 {
        self.0
    }
}

impl RefFromWasmAbi for AnyType {
    type Abi = u32;
    type Anchor = ManuallyDrop<AnyType>;

    #[inline]
    unsafe fn ref_from_abi(js: u32) -> Self::Anchor {
        ManuallyDrop::new(AnyType(js))
    }
}

impl LongRefFromWasmAbi for AnyType {
    type Abi = u32;
    type Anchor = AnyType;

    #[inline]
    unsafe fn long_ref_from_abi(js: u32) -> Self::Anchor {
        Self::from_abi(js)
    }
}

impl WasmDescribe for AnyType {
    fn describe() {
        inform(EXTERNREF);
    }
}

unsafe impl WasmClosure for AnyType {
    const IS_MUT: bool = false;
}

impl JsCast for AnyType {
    #[inline]
    fn instanceof(_val: &JsValue) -> bool {
        true
    }

    #[inline]
    fn unchecked_from_js(val: JsValue) -> Self {
        // SAFETY: AnyType has the same representation as JsValue (both are u32)
        unsafe { core::mem::transmute(val) }
    }

    #[inline]
    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        // SAFETY: AnyType has the same representation as JsValue (both are u32)
        unsafe { core::mem::transmute(val) }
    }
}
