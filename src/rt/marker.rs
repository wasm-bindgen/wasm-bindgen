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

#[cfg(all(feature = "std", target_family = "wasm", panic = "unwind"))]
use core::panic::UnwindSafe;

/// Marker trait for types that are UnwindSafe only when building with panic unwind
pub trait MaybeUnwindSafe {}

#[cfg(all(feature = "std", target_family = "wasm", panic = "unwind"))]
impl<T: UnwindSafe + ?Sized> MaybeUnwindSafe for T {}

#[cfg(not(all(feature = "std", target_family = "wasm", panic = "unwind")))]
impl<T: ?Sized> MaybeUnwindSafe for T {}

/// Private marker trait for erasable generics - types with this trait have the same
/// repr for all generic param values, and can therefore be transmuted on
/// the singular Repr type representation on ABI boundaries.
///
/// # Safety
/// This type must only be implemented on types known to be repr equivalent
/// to their Repr type.
// #[cfg_attr(
//     wbg_diagnostic,
//     diagnostic::on_unimplemented(
//         label = "generic parameter is not a valid Wasm Bindgen ErasableGeneric type",
//         note = "\nRecommendation: Add the direct `: wasm_bindgen::JsGeneric` convenience trait bound for JsValue generics, instead of `ErasableGeneric`.\n",
//     )
// )]
pub unsafe trait ErasableGeneric {
    /// The singular concrete type that all generic variants can be transmuted on
    type Repr: 'static;
}

unsafe impl<T: ErasableGeneric> ErasableGeneric for &mut T {
    type Repr = &'static mut T::Repr;
}

unsafe impl<T: ErasableGeneric> ErasableGeneric for &T {
    type Repr = &'static T::Repr;
}
