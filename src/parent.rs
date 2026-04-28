//! `Parent<T>` — storage wrapper for the parent field of an exported Rust
//! type that uses `#[wasm_bindgen(extends = Parent)]`.
//!
//! When a child exported struct inherits from a parent exported struct, the
//! JS side must be able to dispatch parent methods on a child instance. Each
//! parent method's wasm shim expects a `*const WasmRefCell<Parent>` (or
//! equivalently an `Rc<WasmRefCell<Parent>>` raw pointer), but the child's
//! own `__wbg_ptr` points at a `WasmRefCell<Child>`. Passing the child's
//! pointer to a parent method would be type confusion.
//!
//! The fix: every JS instance carries a separate `__wbg_ptr_<Class>` for
//! each class in its inheritance chain. For the child to materialize a
//! parent pointer, the parent data must live in its own `Rc<WasmRefCell<T>>`
//! allocation that the wasm runtime can clone on demand. `Parent<T>` is that
//! storage — a newtype around `Rc<WasmRefCell<T>>`.
//!
//! Users declare their parent field as `parent: Parent<Animal>` and
//! initialize it with `Animal::new(...).into()` or `Parent::new(value)`.

use crate::__rt::alloc::rc::Rc;
use crate::__rt::{Ref, RefMut, WasmRefCell};

/// Storage wrapper required for `#[wasm_bindgen(parent)]` fields on a struct
/// that uses `#[wasm_bindgen(extends = Parent)]`.
///
/// Under the hood this is an `Rc<WasmRefCell<T>>` so that wasm-bindgen can
/// produce a separately-refcounted parent pointer for JS-side prototype
/// dispatch. Use [`Parent::borrow`] / [`Parent::borrow_mut`] to access the
/// inner value.
pub struct Parent<T> {
    inner: Rc<WasmRefCell<T>>,
}

impl<T> Parent<T> {
    /// Wraps a value in a new `Parent<T>`.
    pub fn new(value: T) -> Self {
        Self {
            inner: Rc::new(WasmRefCell::new(value)),
        }
    }

    /// Immutably borrows the wrapped value.
    ///
    /// Panics (or throws on the wasm target) if the value is currently
    /// mutably borrowed.
    pub fn borrow(&self) -> Ref<'_, T> {
        self.inner.borrow()
    }

    /// Mutably borrows the wrapped value.
    ///
    /// Panics (or throws on the wasm target) if the value is currently
    /// borrowed.
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }

    /// Internal accessor used by the `#[wasm_bindgen(extends = ...)]`
    /// codegen to clone the inner `Rc` when producing an ancestor ABI
    /// pointer for JS. Not part of the public API.
    #[doc(hidden)]
    pub fn __wbg_clone_rc(&self) -> Rc<WasmRefCell<T>> {
        Rc::clone(&self.inner)
    }
}

impl<T> From<T> for Parent<T> {
    fn from(value: T) -> Self {
        Parent::new(value)
    }
}
