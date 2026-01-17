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

#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
use core::panic::UnwindSafe;

/// Marker trait for types that are UnwindSafe only when building with panic unwind
pub trait MaybeUnwindSafe {}

#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
impl<T: UnwindSafe + ?Sized> MaybeUnwindSafe for T {}

#[cfg(not(all(feature = "std", target_arch = "wasm32", panic = "unwind")))]
impl<T: ?Sized> MaybeUnwindSafe for T {}

/// Private marker trait for erasable generics - types with this trait have the same
/// repr for all generic param values, and can therefore be transmuted on
/// the singular Repr type representation on ABI boundaries.
///
/// # Safety
/// This type must only be implemented on types known to be repr equivalent
/// to their Repr type.
#[cfg_attr(
    wbg_diagnostic,
    diagnostic::on_unimplemented(
        message = "The generic type has been defined with a default concrete value that does not support being used as a generic",
        label = "parameter default is not a valid generic type",
        note = "Wasm Bindgen type-erasure generics only supports certain base repr types like JsValue",
    )
)]
pub unsafe trait ErasableGeneric {
    /// The singular concrete type that all generic variants can be transmuted on
    type Repr;
}

#[cfg_attr(wbg_diagnostic, diagnostic::do_not_recommend)]
unsafe impl<'a, T: ErasableGeneric> ErasableGeneric for &'a mut T {
    type Repr = &'a mut T::Repr;
}

#[cfg_attr(wbg_diagnostic, diagnostic::do_not_recommend)]
unsafe impl<'a, T: ErasableGeneric> ErasableGeneric for &'a T {
    type Repr = &'a T::Repr;
}

/// Private marker traits for types which represent Resolution or Promise<Resolution>.
/// For all types except for `Promise`, `Resolution` is equal to the type itself.
/// For `Promise` or any thenable or type extending promise, `Resolution` is the type of the promise resolution.
/// Manually implementing this trait is only required for custom thenables or types which extend Promise.
/// To manually implement this trait, use the #[wasm_bindgen(no_promising)] attribute feature.
pub trait Promising: ErasableGeneric {
    type Resolution;
}

/// Trait bound marker for types that are passed as an own generic type.
/// Encapsulating the ErasableGeneric invariant that must be maintained, that
/// the repr of the type is the type of the concrete target type repr.
/// This is useful to provide simple debuggable trait bounds for codegen.
#[cfg_attr(
    wbg_diagnostic,
    diagnostic::on_unimplemented(
        message = "Unable to call function, since the concrete generic argument or return value cannot be type-erased into the expected generic repr type for the function",
        label = "passed concrete generic type does not match the expected generic repr type",
        note = "Make sure that all erasable generic parameters satisfy the trait bound `ErasableGeneric` with the correct repr. Wasm Bindgen generic parameters and return values for functions are defined to work for specific type-erasable generic repr types only.",
    )
)]
pub trait ErasableGenericOwn<ConcreteTarget>: ErasableGeneric {}

#[cfg_attr(wbg_diagnostic, diagnostic::do_not_recommend)]
impl<T, ConcreteTarget> ErasableGenericOwn<ConcreteTarget> for T
where
    ConcreteTarget: ErasableGeneric,
    T: ErasableGeneric<Repr = <ConcreteTarget as ErasableGeneric>::Repr>,
{
}

/// Trait bound marker for types that are passed as a borrowed generic type.
/// Encapsulating the ErasableGeneric invariant that must be maintained, that
/// the repr of the type is the type of the concrete target type repr.
/// This is useful to provide simple debuggable trait bounds for codegen.
#[cfg_attr(
    wbg_diagnostic,
    diagnostic::on_unimplemented(
        message = "Unable to call this function, since the concrete generic argument or return value cannot be type-erased into the expected generic repr type for the function",
        label = "concrete generic type does not match the expected generic repr type",
        note = "Make sure that all erasable generic parameters satisfy the trait bound `ErasableGeneric` with the correct repr. Wasm Bindgen generic parameters and return values for functions are defined to work for specific type-erasable generic repr types only.",
    )
)]
pub trait ErasableGenericBorrow<Target: ?Sized> {}

#[cfg_attr(wbg_diagnostic, diagnostic::do_not_recommend)]
impl<'a, T: ?Sized + 'a, ConcreteTarget: ?Sized + 'a> ErasableGenericBorrow<ConcreteTarget> for T
where
    &'a ConcreteTarget: ErasableGeneric,
    &'a T: ErasableGeneric<Repr = <&'a ConcreteTarget as ErasableGeneric>::Repr>,
{
}

/// Trait bound marker for types that are passed as a mutable borrowed generic type.
/// Encapsulating the ErasableGeneric invariant that must be maintained, that
/// the repr of the type is the type of the concrete target type repr.
/// This is useful to provide simple debuggable trait bounds for codegen.
#[cfg_attr(
    wbg_diagnostic,
    diagnostic::on_unimplemented(
        message = "Unable to call this function, since the concrete generic argument or return value cannot be type-erased into the expected generic repr type for the function",
        label = "concrete generic type does not match the expected generic repr type",
        note = "Make sure that all erasable generic parameters satisfy the trait bound `ErasableGeneric` with the correct repr. Wasm Bindgen generic parameters and return values for functions are defined to work for specific type-erasable generic repr types only.",
    )
)]
pub trait ErasableGenericBorrowMut<Target: ?Sized> {}

#[cfg_attr(wbg_diagnostic, diagnostic::do_not_recommend)]
impl<'a, T: ?Sized + 'a, ConcreteTarget: ?Sized + 'a> ErasableGenericBorrowMut<ConcreteTarget> for T
where
    &'a mut ConcreteTarget: ErasableGeneric,
    &'a mut T: ErasableGeneric<Repr = <&'a mut ConcreteTarget as ErasableGeneric>::Repr>,
{
}
