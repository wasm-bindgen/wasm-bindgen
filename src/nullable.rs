use core::ops::{Deref, DerefMut};

use crate::convert::JsGeneric;
use crate::sys::JsNullable;
use crate::{JsCast, JsValue};

/// A `T` or `null` at the JS boundary.
///
/// `Nullable<T>` is the Rust-side type for a JS value of type `T | null`, as
/// `Option<T>` is for `T | undefined`, and `Option<Nullable<T>>` is the union
/// `T | null | undefined`. All three are accepted in the same positions: as an
/// argument or return value of exported and imported functions, as a public
/// field of an exported struct, and as a closure argument or return value,
/// including the `&`/`&mut` reference forms `Option<T>` supports.
///
/// | Type | JS type | JS → Rust | Rust → JS |
/// |---|---|---|---|
/// | `Option<T>` | `T \| undefined` | `undefined` and `null` → `None`; anything else goes to `T`'s conversion | `None` → `undefined` |
/// | `Nullable<T>` | `T \| null` | `null` → `Null`; anything else, `undefined` included, goes to `T`'s conversion | `Null` → `null` |
/// | `Option<Nullable<T>>` | `T \| null \| undefined` | `undefined` → `None`, `null` → `Some(Null)`, else `Some(Value(..))` | `None` → `undefined`, `Some(Null)` → `null` |
///
/// The `Option` layer of `Option<Nullable<T>>` treats only `undefined` as
/// `None`, since `null` has to reach the `Nullable` layer.
/// `opt.and_then(Nullable::value)` collapses the union to an
/// `Option<T>`. No other nesting is supported: `Nullable<Nullable<T>>`,
/// `Nullable<Option<T>>` and `Option<Option<T>>` do not cross the boundary.
///
/// Both may also name a slot of a js-sys `Function<fn(..)>` signature or a
/// `Promise<T>` resolution. Where a js-sys container's type-erased methods
/// marshal the element itself, as `Array::get` and `Map::get` do, the
/// element must be a handle: use the JS-side [`JsOption<T>`] and
/// [`JsNullable<T>`] there, which stay JS values rather than converting.
///
/// [`JsOption<T>`]: crate::sys::JsOption
/// [`JsNullable<T>`]: crate::sys::JsNullable
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Nullable<T> {
    /// JS `null`.
    Null,
    /// A value.
    Value(T),
}

impl<T> Default for Nullable<T> {
    #[inline]
    fn default() -> Self {
        Nullable::Null
    }
}

impl<T> Nullable<T> {
    /// Returns `true` if this is `Null`.
    #[inline]
    pub const fn is_null(&self) -> bool {
        matches!(self, Nullable::Null)
    }

    /// Returns `true` if this is a `Value`.
    #[inline]
    pub const fn is_value(&self) -> bool {
        matches!(self, Nullable::Value(_))
    }

    /// Converts from `&Nullable<T>` to `Nullable<&T>`.
    #[inline]
    pub const fn as_ref(&self) -> Nullable<&T> {
        match self {
            Nullable::Null => Nullable::Null,
            Nullable::Value(v) => Nullable::Value(v),
        }
    }

    /// Converts from `&mut Nullable<T>` to `Nullable<&mut T>`.
    #[inline]
    pub fn as_mut(&mut self) -> Nullable<&mut T> {
        match self {
            Nullable::Null => Nullable::Null,
            Nullable::Value(v) => Nullable::Value(v),
        }
    }

    /// Maps a `Nullable<T>` to `Nullable<U>` by applying a function to a
    /// contained value, leaving `Null` as it is.
    #[inline]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Nullable<U> {
        match self {
            Nullable::Null => Nullable::Null,
            Nullable::Value(v) => Nullable::Value(f(v)),
        }
    }

    /// Returns the contained value as `Some`, or `None` for `Null`.
    #[inline]
    pub fn value(self) -> Option<T> {
        match self {
            Nullable::Value(v) => Some(v),
            Nullable::Null => None,
        }
    }

    /// Returns the contained value as `Some`, or `None` for `Null`.
    #[inline]
    pub fn into_option(self) -> Option<T> {
        self.value()
    }

    /// Returns the contained value.
    ///
    /// # Panics
    ///
    /// Panics if this is `Null`.
    #[inline]
    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            Nullable::Value(v) => v,
            Nullable::Null => panic!("called `Nullable::unwrap()` on a `Null` value"),
        }
    }

    /// Returns the contained value.
    ///
    /// # Panics
    ///
    /// Panics with `msg` if this is `Null`.
    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) -> T {
        match self {
            Nullable::Value(v) => v,
            Nullable::Null => panic!("{}", msg),
        }
    }

    /// Returns the contained value or `default`.
    #[inline]
    pub fn unwrap_or(self, default: T) -> T {
        self.value().unwrap_or(default)
    }

    /// Returns the contained value or computes it from `f`.
    #[inline]
    pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
        self.value().unwrap_or_else(f)
    }

    /// Returns the contained value or `T::default()`.
    #[inline]
    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        self.value().unwrap_or_default()
    }

    /// Returns `true` if this is a `Value` and `f` holds for it.
    #[inline]
    pub fn is_value_and(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            Nullable::Value(v) => f(v),
            Nullable::Null => false,
        }
    }

    /// Returns `true` if this is `Null`, or a `Value` for which `f` holds.
    #[inline]
    pub fn is_null_or(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            Nullable::Value(v) => f(v),
            Nullable::Null => true,
        }
    }

    /// Returns the contained value as `Ok`, or `Err(err)` for `Null`.
    #[inline]
    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Nullable::Value(v) => Ok(v),
            Nullable::Null => Err(err),
        }
    }

    /// Returns the contained value as `Ok`, or `Err(err())` for `Null`.
    #[inline]
    pub fn ok_or_else<E, F: FnOnce() -> E>(self, err: F) -> Result<T, E> {
        match self {
            Nullable::Value(v) => Ok(v),
            Nullable::Null => Err(err()),
        }
    }

    /// Returns `Null` for `Null`, otherwise calls `f` on the contained value.
    #[inline]
    pub fn and_then<U, F: FnOnce(T) -> Nullable<U>>(self, f: F) -> Nullable<U> {
        match self {
            Nullable::Value(v) => f(v),
            Nullable::Null => Nullable::Null,
        }
    }

    /// Returns the contained value if `predicate` holds for it, otherwise
    /// `Null`.
    #[inline]
    pub fn filter<P: FnOnce(&T) -> bool>(self, predicate: P) -> Nullable<T> {
        match self {
            Nullable::Value(v) if predicate(&v) => Nullable::Value(v),
            _ => Nullable::Null,
        }
    }

    /// Takes the value out, leaving `Null` in its place.
    #[inline]
    pub fn take(&mut self) -> Nullable<T> {
        core::mem::replace(self, Nullable::Null)
    }

    /// Replaces the contents with `Value(value)`, returning the old contents.
    #[inline]
    pub fn replace(&mut self, value: T) -> Nullable<T> {
        core::mem::replace(self, Nullable::Value(value))
    }

    /// Returns an iterator over the contained value, if any.
    #[inline]
    pub fn iter(&self) -> core::option::IntoIter<&T> {
        self.as_ref().value().into_iter()
    }

    /// Returns a mutable iterator over the contained value, if any.
    #[inline]
    pub fn iter_mut(&mut self) -> core::option::IntoIter<&mut T> {
        self.as_mut().value().into_iter()
    }
}

impl<T: Deref> Nullable<T> {
    /// Converts from `Nullable<T>` (or `&Nullable<T>`) to
    /// `Nullable<&T::Target>`.
    #[inline]
    pub fn as_deref(&self) -> Nullable<&T::Target> {
        self.as_ref().map(|v| v.deref())
    }
}

impl<T: DerefMut> Nullable<T> {
    /// Converts from `Nullable<T>` (or `&mut Nullable<T>`) to
    /// `Nullable<&mut T::Target>`.
    #[inline]
    pub fn as_deref_mut(&mut self) -> Nullable<&mut T::Target> {
        self.as_mut().map(|v| v.deref_mut())
    }
}

impl<T: Copy> Nullable<&T> {
    /// Maps a `Nullable<&T>` to a `Nullable<T>` by copying the referent.
    #[inline]
    pub fn copied(self) -> Nullable<T> {
        self.map(|&v| v)
    }
}

impl<T: Clone> Nullable<&T> {
    /// Maps a `Nullable<&T>` to a `Nullable<T>` by cloning the referent.
    #[inline]
    pub fn cloned(self) -> Nullable<T> {
        self.map(|v| v.clone())
    }
}

impl<T: Copy> Nullable<&mut T> {
    /// Maps a `Nullable<&mut T>` to a `Nullable<T>` by copying the referent.
    #[inline]
    pub fn copied(self) -> Nullable<T> {
        self.map(|&mut v| v)
    }
}

impl<T: Clone> Nullable<&mut T> {
    /// Maps a `Nullable<&mut T>` to a `Nullable<T>` by cloning the referent.
    #[inline]
    pub fn cloned(self) -> Nullable<T> {
        self.map(|v| v.clone())
    }
}

impl<T> IntoIterator for Nullable<T> {
    type Item = T;
    type IntoIter = core::option::IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.value().into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Nullable<T> {
    type Item = &'a T;
    type IntoIter = core::option::IntoIter<&'a T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Nullable<T> {
    type Item = &'a mut T;
    type IntoIter = core::option::IntoIter<&'a mut T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// `?` on a `Nullable<T>` under `--cfg wasm_bindgen_unstable_try_trait`,
/// which needs nightly for `try_trait_v2`. `Null` short-circuits a function
/// returning `Nullable<U>` to `Null` and one returning `Option<U>` to `None`,
/// and `None` from an `Option<T>` short-circuits a `Nullable<U>` function to
/// `Null`.
#[cfg(wasm_bindgen_unstable_try_trait)]
mod try_trait {
    use core::convert::Infallible;
    use core::ops::{ControlFlow, FromResidual, Residual, Try};

    use super::Nullable;

    impl<T> Residual<T> for Nullable<Infallible> {
        type TryType = Nullable<T>;
    }

    impl<T> Try for Nullable<T> {
        type Output = T;
        type Residual = Nullable<Infallible>;

        #[inline]
        fn from_output(output: T) -> Self {
            Nullable::Value(output)
        }

        #[inline]
        fn branch(self) -> ControlFlow<Self::Residual, T> {
            match self {
                Nullable::Value(v) => ControlFlow::Continue(v),
                Nullable::Null => ControlFlow::Break(Nullable::Null),
            }
        }
    }

    impl<T> FromResidual<Nullable<Infallible>> for Nullable<T> {
        #[inline]
        fn from_residual(_: Nullable<Infallible>) -> Self {
            Nullable::Null
        }
    }

    impl<T> FromResidual<Option<Infallible>> for Nullable<T> {
        #[inline]
        fn from_residual(_: Option<Infallible>) -> Self {
            Nullable::Null
        }
    }

    impl<T> FromResidual<Nullable<Infallible>> for Option<T> {
        #[inline]
        fn from_residual(_: Nullable<Infallible>) -> Self {
            None
        }
    }
}

impl<T> From<T> for Nullable<T> {
    #[inline]
    fn from(value: T) -> Self {
        Nullable::Value(value)
    }
}

impl<T> From<Option<T>> for Nullable<T> {
    #[inline]
    fn from(option: Option<T>) -> Self {
        match option {
            Some(v) => Nullable::Value(v),
            None => Nullable::Null,
        }
    }
}

impl<T> From<Nullable<T>> for Option<T> {
    #[inline]
    fn from(nullable: Nullable<T>) -> Self {
        nullable.value()
    }
}

impl<T: Into<JsValue>> From<Nullable<T>> for JsValue {
    #[inline]
    fn from(nullable: Nullable<T>) -> Self {
        match nullable {
            Nullable::Null => JsValue::NULL,
            Nullable::Value(v) => v.into(),
        }
    }
}

impl<T: JsGeneric> From<Nullable<T>> for JsNullable<T> {
    #[inline]
    fn from(nullable: Nullable<T>) -> Self {
        match nullable {
            Nullable::Null => JsNullable::new(),
            Nullable::Value(v) => JsNullable::wrap(v),
        }
    }
}

/// `None` is `undefined`, which `JsNullable` admits as its other empty state.
impl<T: JsGeneric> From<Option<Nullable<T>>> for JsNullable<T> {
    #[inline]
    fn from(option: Option<Nullable<T>>) -> Self {
        match option {
            Some(nullable) => nullable.into(),
            None => JsValue::UNDEFINED.unchecked_into(),
        }
    }
}

impl<T: JsGeneric> From<JsNullable<T>> for Option<Nullable<T>> {
    #[inline]
    fn from(nullable: JsNullable<T>) -> Self {
        if JsValue::is_undefined(&nullable) {
            None
        } else if JsValue::is_null(&nullable) {
            Some(Nullable::Null)
        } else {
            Some(Nullable::Value(nullable.unchecked_into()))
        }
    }
}
