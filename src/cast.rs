use crate::{describe::WasmDescribe, JsValue};

/// A trait for checked and unchecked casting between JS types.
///
/// Specified [in an RFC][rfc] this trait is intended to provide support for
/// casting JS values between different types of one another. In JS there aren't
/// many static types but we've ascribed JS values with static types in Rust,
/// yet they often need to be switched to other types temporarily! This trait
/// provides both checked and unchecked casting into various kinds of values.
///
/// This trait is automatically implemented for any type imported in a
/// `#[wasm_bindgen]` `extern` block.
///
/// [rfc]: https://github.com/rustwasm/rfcs/blob/master/text/002-wasm-bindgen-inheritance-casting.md
pub trait JsCast
where
    Self: AsRef<JsValue> + Into<JsValue>,
{
    /// Test whether this JS value has a type `T`.
    ///
    /// This method will dynamically check to see if this JS object can be
    /// casted to the JS object of type `T`. Usually this uses the `instanceof`
    /// operator. This also works with primitive types like
    /// booleans/strings/numbers as well as cross-realm object like `Array`
    /// which can originate from other iframes.
    ///
    /// In general this is intended to be a more robust version of
    /// `is_instance_of`, but if you want strictly the `instanceof` operator
    /// it's recommended to use that instead.
    fn has_type<T>(&self) -> bool
    where
        T: JsCast,
    {
        T::is_type_of(self.as_ref())
    }

    /// Performs a dynamic cast (checked at runtime) of this value into the
    /// target type `T`.
    ///
    /// This method will return `Err(self)` if `self.has_type::<T>()`
    /// returns `false`, and otherwise it will return `Ok(T)` manufactured with
    /// an unchecked cast (verified correct via the `has_type` operation).
    fn dyn_into<T>(self) -> Result<T, Self>
    where
        T: JsCast,
    {
        if self.has_type::<T>() {
            Ok(self.unchecked_into())
        } else {
            Err(self)
        }
    }

    /// Performs a dynamic cast (checked at runtime) of this value into the
    /// target type `T`.
    ///
    /// This method will return `None` if `self.has_type::<T>()`
    /// returns `false`, and otherwise it will return `Some(&T)` manufactured
    /// with an unchecked cast (verified correct via the `has_type` operation).
    fn dyn_ref<T>(&self) -> Option<&T>
    where
        T: JsCast,
    {
        if self.has_type::<T>() {
            Some(self.unchecked_ref())
        } else {
            None
        }
    }

    /// Performs a zero-cost unchecked cast into the specified type.
    ///
    /// This method will convert the `self` value to the type `T`, where both
    /// `self` and `T` are simple wrappers around `JsValue`. This method **does
    /// not check whether `self` is an instance of `T`**. If used incorrectly
    /// then this method may cause runtime exceptions in both Rust and JS, this
    /// should be used with caution.
    fn unchecked_into<T>(self) -> T
    where
        T: JsCast,
    {
        T::unchecked_from_js(self.into())
    }

    /// Performs a zero-cost unchecked cast into a reference to the specified
    /// type.
    ///
    /// This method will convert the `self` value to the type `T`, where both
    /// `self` and `T` are simple wrappers around `JsValue`. This method **does
    /// not check whether `self` is an instance of `T`**. If used incorrectly
    /// then this method may cause runtime exceptions in both Rust and JS, this
    /// should be used with caution.
    ///
    /// This method, unlike `unchecked_into`, does not consume ownership of
    /// `self` and instead works over a shared reference.
    fn unchecked_ref<T>(&self) -> &T
    where
        T: JsCast,
    {
        T::unchecked_from_js_ref(self.as_ref())
    }

    /// Test whether this JS value is an instance of the type `T`.
    ///
    /// This method performs a dynamic check (at runtime) using the JS
    /// `instanceof` operator. This method returns `self instanceof T`.
    ///
    /// Note that `instanceof` does not always work with primitive values or
    /// across different realms (e.g. iframes). If you're not sure whether you
    /// specifically need only `instanceof` it's recommended to use `has_type`
    /// instead.
    fn is_instance_of<T>(&self) -> bool
    where
        T: JsCast,
    {
        T::instanceof(self.as_ref())
    }

    /// Performs a dynamic `instanceof` check to see whether the `JsValue`
    /// provided is an instance of this type.
    ///
    /// This is intended to be an internal implementation detail, you likely
    /// won't need to call this. It's generally called through the
    /// `is_instance_of` method instead.
    fn instanceof(val: &JsValue) -> bool;

    /// Performs a dynamic check to see whether the `JsValue` provided
    /// is a value of this type.
    ///
    /// Unlike `instanceof`, this can be specialised to use a custom check by
    /// adding a `#[wasm_bindgen(is_type_of = callback)]` attribute to the
    /// type import declaration.
    ///
    /// Other than that, this is intended to be an internal implementation
    /// detail of `has_type` and you likely won't need to call this.
    fn is_type_of(val: &JsValue) -> bool {
        Self::instanceof(val)
    }

    /// Performs a zero-cost unchecked conversion from a `JsValue` into an
    /// instance of `Self`
    ///
    /// This is intended to be an internal implementation detail, you likely
    /// won't need to call this.
    fn unchecked_from_js(val: JsValue) -> Self;

    /// Performs a zero-cost unchecked conversion from a `&JsValue` into an
    /// instance of `&Self`.
    ///
    /// Note the safety of this method, which basically means that `Self` must
    /// be a newtype wrapper around `JsValue`.
    ///
    /// This is intended to be an internal implementation detail, you likely
    /// won't need to call this.
    fn unchecked_from_js_ref(val: &JsValue) -> &Self;
}

/// A simplified trait for converting `JsValue` to any target type.
///
/// Effectively a custom `From<JsValue>` with `JsCast` semantics where possible,
/// and used specifically in the generic typing system.
///
/// This trait provides a unified interface for all runtime JavaScript value
/// conversions, supporting primitives, JS types, and Option types through a
/// single method, and without needing to implement ref cases required by `JsCast`.
///
/// This trait is automatically implemented for all types that implement `JsCast`,
/// and has manual implementations for primitive types that use WebAssembly-compliant
/// conversion logic.
pub trait JsValueCast: Sized {
    /// Converts a `JsValue` directly to `Self`.
    ///
    /// # Safety
    ///
    /// This method does not perform runtime type checking. For primitive types,
    /// it will attempt WebAssembly-compliant conversion which may fail with a panic.
    /// For JS types, it performs an unchecked cast which may cause runtime errors
    /// if the JS value is not actually of the expected type.
    fn unchecked_from_js_value(val: JsValue) -> Self;
}

/// Blanket implementation of `JsValueCast` for all types that implement `JsCast`.
///
/// This automatically provides `JsValueCast` support for all JS wrapper types
/// (like `JsString`, `Object`, `Array`, etc.) by delegating to their existing
/// `JsCast::unchecked_from_js` implementation.
impl<T> JsValueCast for T
where
    T: JsCast,
{
    #[inline]
    fn unchecked_from_js_value(val: JsValue) -> Self {
        JsCast::unchecked_from_js(val)
    }
}

/// Trait implemented for wrappers around `JsValue`s generated by `#[wasm_bindgen]`.
#[doc(hidden)]
pub trait JsObject: JsCast + JsUpcastRef + JsValueCast + WasmDescribe {}

/// A trait for upcasting generic reference types to `&JsValue`.
///
/// For example, an imported function that accepts a generic `&T` argument
/// may use `JsUpcastRef` to convert `&T` into `&JsValue`.
///
/// Unlike `JsCast`, which requires types to already be able to be represented
/// as a JsValue, `JsUpcastRef` is able to apply to arbitrary primitive types
/// that otherwise would require an explicit conversion. This is done by providing
/// constant reference slots for JS values available only at the generic bindgen
/// call interface, by referencing constant slots in __rt.
///
/// By setting the `USES_VALUE_SLOTS` constant, the bindgen is then able to inform
/// the bindgen process that it is using these externref slots and that they should
/// be cleared after the function call, guaranteeing the JsValue indices are
/// live for as long as the function call only.
///
/// This trait is automatically implemented for any type supporting `AsRef<JsValue>`.
pub trait JsUpcastRef<T = JsValue> {
    /// Whether upcast_ref and upcast_ref_mut require the use of local
    /// externref value slots which must be cleared after the call.
    const USES_VALUE_SLOTS: bool = false;

    /// Performs an upcast of the reference into a more general reference type.
    #[inline]
    fn upcast_ref(&self) -> &T {
        panic!("generic reference upcast not supported for this type")
    }
}

/// A supertrait that captures the requirements for generic bindgen types.
///
/// This captures the usage requirements for generics in arbitrary function
/// positions allowing the type to be cast into externref via JsValue.
///
/// For an imported function, using generic type <T: GenericCast>:
/// - Returning a T value requires JsValueCast to obtain T from JsValue
///   with zero cost, while ensuring we
/// - Passing an argument of type &T requires JsUpcastRef to obtain &JsValue
/// - Passing an argument of type T requires Into<JsValue> to obtain JsValue
///
pub trait GenericCast: JsUpcastRef + Into<JsValue> + JsValueCast + 'static {}

impl<T> GenericCast for T where T: JsUpcastRef + Into<JsValue> + JsValueCast + 'static {}
