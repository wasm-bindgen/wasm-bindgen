use crate::convert::JsGeneric;
use crate::sys::JsNullable;
use crate::{JsCast, JsValue};

/// A `T`, `null` or `undefined` at the JS boundary.
///
/// `Nullable<T>` is the Rust-side type for a JS value of type
/// `T | null | undefined`. It is accepted everywhere `Option<T>` is: as an
/// argument or return value of exported and imported functions, as a public
/// field of an exported struct, and as a closure argument or return value,
/// including the `&`/`&mut` reference forms `Option<T>` supports. Unlike
/// `Option<T>`, it keeps `null` and `undefined` apart in both directions:
///
/// | Type | JS type | JS → Rust | Rust → JS |
/// |---|---|---|---|
/// | `Option<T>` | `T \| undefined` | `undefined` → `None`; anything else, `null` included under `wasm_bindgen_unstable_strict_option`, goes to `T`'s conversion | `None` → `undefined` |
/// | `Nullable<T>` | `T \| null \| undefined` | `null` → `Null`, `undefined` → `Undefined`, else `T`'s conversion | `Null` → `null`, `Undefined` → `undefined`, `Value(v)` → `v` |
///
/// Without the cfg, `Option<T>` still receives `null` as `None` for
/// primitives, strings, slices, enums and exported structs. Code relying on
/// that should move to `Nullable<T>`.
///
/// Use `Option<T>` or `Nullable<T>` for a value crossing the boundary
/// directly. Inside a JS type, such as `Array<T>` or `Promise<T>`, and in
/// generic positions, use the JS-side handles [`JsOption<T>`] and
/// [`JsNullable<T>`] instead, which stay JS values rather than converting.
///
/// [`JsOption<T>`]: crate::sys::JsOption
/// [`JsNullable<T>`]: crate::sys::JsNullable
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Nullable<T> {
    /// JS `null`.
    Null,
    /// JS `undefined`.
    Undefined,
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

    /// Returns `true` if this is `Undefined`.
    #[inline]
    pub const fn is_undefined(&self) -> bool {
        matches!(self, Nullable::Undefined)
    }

    /// Returns `true` if this is `Null` or `Undefined`.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        !self.is_value()
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
            Nullable::Undefined => Nullable::Undefined,
            Nullable::Value(v) => Nullable::Value(v),
        }
    }

    /// Converts from `&mut Nullable<T>` to `Nullable<&mut T>`.
    #[inline]
    pub fn as_mut(&mut self) -> Nullable<&mut T> {
        match self {
            Nullable::Null => Nullable::Null,
            Nullable::Undefined => Nullable::Undefined,
            Nullable::Value(v) => Nullable::Value(v),
        }
    }

    /// Maps a `Nullable<T>` to `Nullable<U>` by applying a function to a
    /// contained value, leaving `Null` and `Undefined` as they are.
    #[inline]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Nullable<U> {
        match self {
            Nullable::Null => Nullable::Null,
            Nullable::Undefined => Nullable::Undefined,
            Nullable::Value(v) => Nullable::Value(f(v)),
        }
    }

    /// Returns the contained value as `Some`, or `None` for `Null` and
    /// `Undefined`.
    #[inline]
    pub fn value(self) -> Option<T> {
        match self {
            Nullable::Value(v) => Some(v),
            Nullable::Null | Nullable::Undefined => None,
        }
    }

    /// Returns the contained value as `Some`, or `None` for `Null` and
    /// `Undefined`.
    #[inline]
    pub fn into_option(self) -> Option<T> {
        self.value()
    }

    /// Returns the contained value.
    ///
    /// # Panics
    ///
    /// Panics if this is `Null` or `Undefined`.
    #[inline]
    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            Nullable::Value(v) => v,
            Nullable::Null => panic!("called `Nullable::unwrap()` on a `Null` value"),
            Nullable::Undefined => panic!("called `Nullable::unwrap()` on an `Undefined` value"),
        }
    }

    /// Returns the contained value.
    ///
    /// # Panics
    ///
    /// Panics with `msg` if this is `Null` or `Undefined`.
    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) -> T {
        match self {
            Nullable::Value(v) => v,
            Nullable::Null | Nullable::Undefined => panic!("{}", msg),
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
}

impl<T> From<T> for Nullable<T> {
    #[inline]
    fn from(value: T) -> Self {
        Nullable::Value(value)
    }
}

/// `None` becomes `Null`, the canonical empty value of a nullable type,
/// matching [`JsNullable::new`].
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

impl<T: JsGeneric> From<Nullable<T>> for JsNullable<T> {
    #[inline]
    fn from(nullable: Nullable<T>) -> Self {
        match nullable {
            Nullable::Null => JsNullable::new(),
            Nullable::Undefined => JsValue::UNDEFINED.unchecked_into(),
            Nullable::Value(v) => JsNullable::wrap(v),
        }
    }
}

impl<T: JsGeneric> From<JsNullable<T>> for Nullable<T> {
    #[inline]
    fn from(nullable: JsNullable<T>) -> Self {
        if JsValue::is_null(&nullable) {
            Nullable::Null
        } else if JsValue::is_undefined(&nullable) {
            Nullable::Undefined
        } else {
            Nullable::Value(nullable.unchecked_into())
        }
    }
}
