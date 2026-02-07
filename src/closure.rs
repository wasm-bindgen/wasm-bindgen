//! Support for closures in `wasm-bindgen`
//!
//! This module defines the [`ScopedClosure`] type which is used to pass Rust closures
//! to JavaScript. All closures are unwind safe: panics are caught and converted to
//! JavaScript exceptions when built with `panic=unwind`.
//!
//! # Choosing a `Closure` API
//!
//! | Use Case | API | Lifetime |
//! |----------|-----|----------|
//! | Immediate/synchronous callbacks | [`ScopedClosure::borrow`] / [`ScopedClosure::borrow_mut`] | Non-`'static` allowed |
//! | Long-lived callbacks (events, timers) | [`Closure::new`] / [`ScopedClosure::own`] | `'static` required |
//! | One-shot callbacks | [`Closure::once`] / [`Closure::once_into_js`] | `'static` required |
//! | Transfer ownership to JS | Pass `Closure` by value | `'static` required |
//!
//! # Type Aliases
//!
//! - [`ScopedClosure<'a, T>`] — The unified closure type with a lifetime parameter
//! - [`StaticClosure<T>`] — Alias for `ScopedClosure<'static, T>`
//! - [`Closure<T>`] — Alias for `StaticClosure<T>` (for backwards compatibility)
//!
//! # Ownership Model
//!
//! `ScopedClosure` follows the same ownership model as other wasm-bindgen types:
//! the JavaScript reference remains valid until the Rust value is dropped. When
//! dropped, the closure is invalidated and any subsequent calls from JavaScript
//! will throw an exception.
//!
//! For borrowed closures created with `borrow`/`borrow_mut`, Rust's borrow checker
//! ensures the `ScopedClosure` cannot outlive the closure's captured data.
//!
//! See the [`ScopedClosure`] type documentation for detailed examples.

#![allow(clippy::fn_to_numeric_cast)]

use crate::cast::JsCast;
use crate::convert::*;
use crate::describe::*;
use crate::JsValue;
use crate::__rt::marker::MaybeUnwindSafe;
use alloc::boxed::Box;
use alloc::string::String;
use core::fmt;
use core::marker::PhantomData;
use core::mem;
use core::panic::AssertUnwindSafe;

#[wasm_bindgen_macro::wasm_bindgen(wasm_bindgen = crate)]
extern "C" {
    type JsClosure;

    #[wasm_bindgen(method)]
    fn _wbg_cb_unref(js: &JsClosure);
}

/// A handle to both a closure in Rust as well as JS closure which will invoke
/// the Rust closure.
///
/// A `Closure` is the primary way that a `'static` lifetime closure is
/// transferred from Rust to JS. `Closure` currently requires that the closures
/// it's created with have the `'static` lifetime in Rust for soundness reasons.
///
/// This type is a "handle" in the sense that whenever it is dropped it will
/// invalidate the JS closure that it refers to. Any usage of the closure in JS
/// after the `Closure` has been dropped will raise an exception. It's then up
/// to you to arrange for `Closure` to be properly deallocate at an appropriate
/// location in your program.
///
/// The type parameter on `Closure` is the type of closure that this represents.
/// Currently this can only be the `Fn` and `FnMut` traits with up to 7
/// arguments (and an optional return value).
///
/// # Examples
///
/// Here are a number of examples of using `Closure`.
///
/// ## Using the `setInterval` API
///
/// Sample usage of `Closure` to invoke the `setInterval` API.
///
/// ```rust,no_run
/// use wasm_bindgen::prelude::*;
///
/// #[wasm_bindgen]
/// extern "C" {
///     fn setInterval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
///     fn clearInterval(id: i32);
///
///     #[wasm_bindgen(js_namespace = console)]
///     fn log(s: &str);
/// }
///
/// #[wasm_bindgen]
/// pub struct IntervalHandle {
///     interval_id: i32,
///     _closure: Closure<dyn FnMut()>,
/// }
///
/// impl Drop for IntervalHandle {
///     fn drop(&mut self) {
///         clearInterval(self.interval_id);
///     }
/// }
///
/// #[wasm_bindgen]
/// pub fn run() -> IntervalHandle {
///     // First up we use `Closure::new` to wrap up a Rust closure and create
///     // a JS closure.
///     let cb = Closure::new(|| {
///         log("interval elapsed!");
///     });
///
///     // Next we pass this via reference to the `setInterval` function, and
///     // `setInterval` gets a handle to the corresponding JS closure.
///     let interval_id = setInterval(&cb, 1_000);
///
///     // If we were to drop `cb` here it would cause an exception to be raised
///     // whenever the interval elapses. Instead we *return* our handle back to JS
///     // so JS can decide when to cancel the interval and deallocate the closure.
///     IntervalHandle {
///         interval_id,
///         _closure: cb,
///     }
/// }
/// ```
///
/// ## Casting a `Closure` to a `js_sys::Function`
///
/// This is the same `setInterval` example as above, except it is using
/// `web_sys` (which uses `js_sys::Function` for callbacks) instead of manually
/// writing bindings to `setInterval` and other Web APIs.
///
/// ```rust,ignore
/// use wasm_bindgen::JsCast;
///
/// #[wasm_bindgen]
/// pub struct IntervalHandle {
///     interval_id: i32,
///     _closure: Closure<dyn FnMut()>,
/// }
///
/// impl Drop for IntervalHandle {
///     fn drop(&mut self) {
///         let window = web_sys::window().unwrap();
///         window.clear_interval_with_handle(self.interval_id);
///     }
/// }
///
/// #[wasm_bindgen]
/// pub fn run() -> Result<IntervalHandle, JsValue> {
///     let cb = Closure::new(|| {
///         web_sys::console::log_1(&"interval elapsed!".into());
///     });
///
///     let window = web_sys::window().unwrap();
///     let interval_id = window.set_interval_with_callback_and_timeout_and_arguments_0(
///         // Note this method call, which uses `as_ref()` to get a `JsValue`
///         // from our `Closure` which is then converted to a `&Function`
///         // using the `JsCast::unchecked_ref` function.
///         cb.as_ref().unchecked_ref(),
///         1_000,
///     )?;
///
///     // Same as above.
///     Ok(IntervalHandle {
///         interval_id,
///         _closure: cb,
///     })
/// }
/// ```
///
/// ## Using `FnOnce` and `Closure::once` with `requestAnimationFrame`
///
/// Because `requestAnimationFrame` only calls its callback once, we can use
/// `FnOnce` and `Closure::once` with it.
///
/// ```rust,no_run
/// use wasm_bindgen::prelude::*;
///
/// #[wasm_bindgen]
/// extern "C" {
///     fn requestAnimationFrame(closure: &Closure<dyn FnMut()>) -> u32;
///     fn cancelAnimationFrame(id: u32);
///
///     #[wasm_bindgen(js_namespace = console)]
///     fn log(s: &str);
/// }
///
/// #[wasm_bindgen]
/// pub struct AnimationFrameHandle {
///     animation_id: u32,
///     _closure: Closure<dyn FnMut()>,
/// }
///
/// impl Drop for AnimationFrameHandle {
///     fn drop(&mut self) {
///         cancelAnimationFrame(self.animation_id);
///     }
/// }
///
/// // A type that will log a message when it is dropped.
/// struct LogOnDrop(&'static str);
/// impl Drop for LogOnDrop {
///     fn drop(&mut self) {
///         log(self.0);
///     }
/// }
///
/// #[wasm_bindgen]
/// pub fn run() -> AnimationFrameHandle {
///     // We are using `Closure::once` which takes a `FnOnce`, so the function
///     // can drop and/or move things that it closes over.
///     let fired = LogOnDrop("animation frame fired or canceled");
///     let cb = Closure::once(move || drop(fired));
///
///     // Schedule the animation frame!
///     let animation_id = requestAnimationFrame(&cb);
///
///     // Again, return a handle to JS, so that the closure is not dropped
///     // immediately and JS can decide whether to cancel the animation frame.
///     AnimationFrameHandle {
///         animation_id,
///         _closure: cb,
///     }
/// }
/// ```
///
/// ## Converting `FnOnce`s directly into JavaScript Functions with `Closure::once_into_js`
///
/// If we don't want to allow a `FnOnce` to be eagerly dropped (maybe because we
/// just want it to drop after it is called and don't care about cancellation)
/// then we can use the `Closure::once_into_js` function.
///
/// This is the same `requestAnimationFrame` example as above, but without
/// supporting early cancellation.
///
/// ```
/// use wasm_bindgen::prelude::*;
///
/// #[wasm_bindgen]
/// extern "C" {
///     // We modify the binding to take an untyped `JsValue` since that is what
///     // is returned by `Closure::once_into_js`.
///     //
///     // If we were using the `web_sys` binding for `requestAnimationFrame`,
///     // then the call sites would cast the `JsValue` into a `&js_sys::Function`
///     // using `f.unchecked_ref::<js_sys::Function>()`. See the `web_sys`
///     // example above for details.
///     fn requestAnimationFrame(callback: JsValue);
///
///     #[wasm_bindgen(js_namespace = console)]
///     fn log(s: &str);
/// }
///
/// // A type that will log a message when it is dropped.
/// struct LogOnDrop(&'static str);
/// impl Drop for LogOnDrop {
///     fn drop(&mut self) {
///         log(self.0);
///     }
/// }
///
/// #[wasm_bindgen]
/// pub fn run() {
///     // We are using `Closure::once_into_js` which takes a `FnOnce` and
///     // converts it into a JavaScript function, which is returned as a
///     // `JsValue`.
///     let fired = LogOnDrop("animation frame fired");
///     let cb = Closure::once_into_js(move || drop(fired));
///
///     // Schedule the animation frame!
///     requestAnimationFrame(cb);
///
///     // No need to worry about whether or not we drop a `Closure`
///     // here or return some sort of handle to JS!
/// }
/// ```
/// A closure with a lifetime parameter that represents a Rust closure passed to JavaScript.
///
/// `ScopedClosure<'a, T>` is the unified closure type. The lifetime `'a` indicates
/// how long the closure is valid:
///
/// - **`ScopedClosure<'static, T>`** (aka [`StaticClosure<T>`] or [`Closure<T>`]) - An owned
///   closure with heap-allocated data. Requires `'static` captures. Use for long-lived
///   closures like event listeners and timers. Created with [`Closure::new`] or [`ScopedClosure::own`].
///
/// - **`ScopedClosure<'a, T>`** (non-`'static`) - A borrowed closure referencing stack data.
///   Allows non-`'static` captures. Use for immediate/synchronous callbacks. Created with
///   [`ScopedClosure::borrow`] or [`ScopedClosure::borrow_mut`].
///
/// # Ownership Model
///
/// `ScopedClosure` follows the same ownership model as other wasm-bindgen types:
/// the JavaScript reference remains valid until the Rust value is dropped. When
/// dropped, the closure is invalidated and any subsequent calls from JavaScript
/// will throw: "closure invoked recursively or after being dropped".
///
/// For `'static` closures, you can also:
/// - Pass by value to transfer ownership to JS (implements [`IntoWasmAbi`])
/// - Call [`forget()`](Self::forget) to leak the closure (JS can use it indefinitely)
/// - Call [`into_js_value()`](Self::into_js_value) to transfer to JS GC management
///
/// # Lifetime Safety
///
/// For borrowed closures, Rust's borrow checker ensures that `ScopedClosure` cannot
/// be held longer than the closure's captured data:
///
/// ```ignore
/// let mut sum = 0;
/// let mut f = |x: u32| { sum += x; };  // f borrows sum
/// let closure = ScopedClosure::borrow_mut(&mut f);  // closure borrows f
/// // closure cannot outlive f, and f cannot outlive sum
/// ```
///
/// # Examples
///
/// ## Borrowed closures with `ScopedClosure::borrow_mut`
///
/// Use for immediate/synchronous callbacks where JS calls the closure right away:
///
/// ```ignore
/// use wasm_bindgen::prelude::*;
///
/// #[wasm_bindgen]
/// extern "C" {
///     fn call_immediately(cb: &ScopedClosure<dyn FnMut(u32)>);
/// }
///
/// let mut sum = 0;
/// {
///     let mut f = |x: u32| { sum += x; };
///     let closure = ScopedClosure::borrow_mut(&mut f);
///     call_immediately(&closure);
/// }  // closure dropped here, JS function invalidated
/// assert_eq!(sum, 42);
/// ```
///
/// ## Owned closures with `Closure::new`
///
/// Use for long-lived callbacks like event listeners and timers:
///
/// ```ignore
/// use wasm_bindgen::prelude::*;
///
/// #[wasm_bindgen]
/// extern "C" {
///     fn setInterval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
/// }
///
/// // Closure::new requires 'static, so use `move` to capture by value
/// let cb = Closure::new(move || {
///     // ...
/// });
/// setInterval(&cb, 1000);
/// // Must keep `cb` alive or call `cb.forget()` to transfer to JS
/// ```
///
/// ## Transferring ownership to JS
///
/// Pass a `Closure` by value to transfer ownership:
///
/// ```ignore
/// use wasm_bindgen::prelude::*;
///
/// #[wasm_bindgen]
/// extern "C" {
///     fn set_one_shot_callback(cb: Closure<dyn FnMut()>);
/// }
///
/// let cb = Closure::new(|| { /* ... */ });
/// set_one_shot_callback(cb);  // Ownership transferred, no need to store
/// ```
pub struct ScopedClosure<'a, T: ?Sized> {
    js: JsValue,
    _marker: PhantomData<T>,
    _lifetime: PhantomData<&'a ()>,
}

/// A `'static` closure that owns its data on the heap.
///
/// This is an alias for `ScopedClosure<'static, T>`. Use this for long-lived
/// closures like event listeners and timers.
///
/// See [`ScopedClosure`] for full documentation.
pub type StaticClosure<T> = ScopedClosure<'static, T>;

/// Alias for [`StaticClosure`] for backwards compatibility.
///
/// In a future major version, `Closure` may become `ScopedClosure` with a
/// lifetime parameter.
pub type Closure<T> = StaticClosure<T>;

// ScopedClosure is Unpin because it only contains a JsValue (which is just a u32)
// and PhantomData markers. The closure data is either on the heap (owned) or
// referenced through a raw pointer (borrowed), neither of which is stored inline.
impl<T: ?Sized> Unpin for ScopedClosure<'_, T> {}

fn _assert_compiles<T>(pin: core::pin::Pin<&mut ScopedClosure<'static, T>>) {
    let _ = &mut *pin.get_mut();
}

impl<T: ?Sized> Drop for ScopedClosure<'_, T> {
    fn drop(&mut self) {
        // Invalidate the closure on the JS side.
        //
        // The JS bindings distinguish owned vs borrowed closures via the `dtor_idx`
        // encoded in `WasmDescribe`: owned closures pass a non-zero destructor
        // function pointer, borrowed closures pass `0`.
        //
        // For owned closures (`Closure::new`/`ScopedClosure::own`), this decreases
        // the refcount and frees the Rust heap data when the count reaches zero.
        //
        // For borrowed closures (`ScopedClosure::borrow`/`borrow_mut`), this sets
        // state.a = state.b = 0 to prevent any further calls to the closure.
        self.js.unchecked_ref::<JsClosure>()._wbg_cb_unref();
    }
}

impl<'a, T> ScopedClosure<'a, T>
where
    T: ?Sized + WasmClosure,
{
    /// Creates a scoped closure by borrowing an immutable `Fn` closure.
    ///
    /// This is the recommended way to pass closures to JavaScript for immediate/
    /// synchronous use. Unlike [`Closure::new`], this does not require the closure
    /// to be `'static`, allowing you to capture references to local variables.
    ///
    /// The returned `ScopedClosure<'a, _>` has lifetime `'a` from the closure
    /// reference, which means it cannot outlive the closure or any data the
    /// closure captures.
    ///
    /// For closures that need mutable state (`FnMut`), use [`borrow_mut`](Self::borrow_mut).
    ///
    /// # When to use scoped closures
    ///
    /// Use `ScopedClosure::borrow` or `ScopedClosure::borrow_mut` when:
    /// - JavaScript will call the closure immediately and not retain it
    /// - You need to capture non-`'static` references
    /// - You want automatic cleanup when the `ScopedClosure` is dropped
    ///
    /// # Closure lifetime
    ///
    /// The JavaScript function is only valid while the `ScopedClosure` exists.
    /// Once dropped, the JavaScript function is invalidated. If JavaScript retains
    /// a reference and calls it later, it will throw: "closure invoked recursively
    /// or after being dropped".
    ///
    /// Rust's borrow checker ensures `ScopedClosure` cannot outlive the closure's
    /// captured data, preventing use-after-free bugs.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use wasm_bindgen::prelude::*;
    ///
    /// #[wasm_bindgen]
    /// extern "C" {
    ///     fn call_with_value(cb: &ScopedClosure<dyn Fn(u32)>, value: u32);
    /// }
    ///
    /// let data = vec![1, 2, 3];
    /// let f = || {
    ///     // Can access `data` without moving it
    ///     println!("data len: {}", data.len());
    /// };
    /// let closure = ScopedClosure::borrow(&f);
    /// call_with_value(&closure, 42);
    /// ```
    pub fn borrow<F>(t: &'a F) -> ScopedClosure<'a, F::Static>
    where
        F: UnsizeClosureRef<T> + ?Sized,
    {
        let t: &T = t.unsize_closure_ref();
        let (ptr, len): (u32, u32) = unsafe { mem::transmute_copy(&t) };
        ScopedClosure {
            js: crate::__rt::wbg_cast(BorrowedClosure::<T> {
                data: WasmSlice { ptr, len },
                unwind_safe: true,
                _marker: PhantomData,
            }),
            _marker: PhantomData,
            _lifetime: PhantomData,
        }
    }

    /// Creates a scoped closure by mutably borrowing a `FnMut` closure.
    ///
    /// This is the most common variant for scoped closures since most closures
    /// that mutate captured state need `FnMut`.
    ///
    /// See [`borrow`](Self::borrow) for full documentation on scoped closures.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use wasm_bindgen::prelude::*;
    ///
    /// #[wasm_bindgen]
    /// extern "C" {
    ///     fn call_three_times(cb: &ScopedClosure<dyn FnMut(u32)>);
    /// }
    ///
    /// let mut sum = 0;
    /// let closure = ScopedClosure::borrow_mut(&mut |x: u32| {
    ///     sum += x;
    /// });
    /// call_three_times(&closure);
    /// // closure dropped, `sum` is accessible again
    /// assert_eq!(sum, 6); // 1 + 2 + 3
    /// ```
    pub fn borrow_mut<F>(t: &'a mut F) -> ScopedClosure<'a, F::Static>
    where
        F: UnsizeClosureRefMut<T> + ?Sized,
    {
        let t: &mut T = t.unsize_closure_ref();
        let (ptr, len): (u32, u32) = unsafe { mem::transmute_copy(&t) };
        ScopedClosure {
            js: crate::__rt::wbg_cast(BorrowedClosure::<T> {
                data: WasmSlice { ptr, len },
                unwind_safe: true,
                _marker: PhantomData,
            }),
            _marker: PhantomData,
            _lifetime: PhantomData,
        }
    }

    /// Like [`borrow`](Self::borrow), but does not catch panics.
    ///
    /// If the closure panics, the process will abort. This variant does not
    /// require `UnwindSafe`.
    pub fn borrow_aborting<F>(t: &'a F) -> ScopedClosure<'a, F::Static>
    where
        F: UnsizeClosureRef<T> + ?Sized,
    {
        let t: &T = t.unsize_closure_ref();
        let (ptr, len): (u32, u32) = unsafe { mem::transmute_copy(&t) };
        ScopedClosure {
            js: crate::__rt::wbg_cast(BorrowedClosure::<T> {
                data: WasmSlice { ptr, len },
                unwind_safe: false,
                _marker: PhantomData,
            }),
            _marker: PhantomData,
            _lifetime: PhantomData,
        }
    }

    /// Like [`borrow_mut`](Self::borrow_mut), but does not catch panics.
    ///
    /// If the closure panics, the process will abort. This variant does not
    /// require `UnwindSafe`.
    pub fn borrow_mut_aborting<F>(t: &'a mut F) -> ScopedClosure<'a, F::Static>
    where
        F: UnsizeClosureRefMut<T> + ?Sized,
    {
        let t: &mut T = t.unsize_closure_ref();
        let (ptr, len): (u32, u32) = unsafe { mem::transmute_copy(&t) };
        ScopedClosure {
            js: crate::__rt::wbg_cast(BorrowedClosure::<T> {
                data: WasmSlice { ptr, len },
                unwind_safe: false,
                _marker: PhantomData,
            }),
            _marker: PhantomData,
            _lifetime: PhantomData,
        }
    }
}

/// Methods for creating and managing `'static` closures.
///
/// These methods are only available on `StaticClosure<T>` (aka `Closure<T>`),
/// not on borrowed `ScopedClosure<'a, T>` where `'a` is not `'static`.
impl<T> ScopedClosure<'static, T>
where
    T: ?Sized + WasmClosure,
{
    /// Creates a new owned `Closure` from the provided Rust function.
    ///
    /// Note that the closure provided here, `F`, has a few requirements
    /// associated with it:
    ///
    /// * It must implement `Fn` or `FnMut` (for `FnOnce` functions see
    ///   `Closure::once` and `Closure::once_into_js`).
    ///
    /// * It must be `'static`, aka no stack references (use the `move`
    ///   keyword).
    ///
    /// * It can have at most 7 arguments.
    ///
    /// * Its arguments and return values are all types that can be shared with
    ///   JS (i.e. have `#[wasm_bindgen]` annotations or are simple numbers,
    ///   etc.)
    pub fn new<F>(t: F) -> Self
    where
        F: IntoWasmClosure<T> + 'static,
    {
        Self::_wrap(Box::new(t).unsize(), true)
    }

    /// Alias for [`new`](Self::new) — creates an owned `'static` closure.
    ///
    /// This name is symmetric with [`borrow`](ScopedClosure::borrow) and
    /// [`borrow_mut`](ScopedClosure::borrow_mut) for borrowed closures.
    ///
    /// See [`new`](Self::new) for full documentation.
    pub fn own<F>(t: F) -> Self
    where
        F: IntoWasmClosure<T> + 'static,
    {
        Self::new(t)
    }

    /// Creates a new instance of `Closure` from the provided Rust function.
    ///
    /// Unlike `new`, this version does NOT catch panics and does NOT require `UnwindSafe`.
    /// If the closure panics, the process will abort.
    ///
    /// Use this when:
    /// - Your closure captures types that aren't `UnwindSafe` (like `Rc<Cell<T>>`)
    /// - You don't need panic catching across the JS boundary
    /// - You prefer abort-on-panic behavior
    ///
    /// Note that the closure provided here, `F`, has a few requirements
    /// associated with it:
    ///
    /// * It must implement `Fn` or `FnMut` (for `FnOnce` functions see
    ///   `Closure::once` and `Closure::once_into_js`).
    ///
    /// * It must be `'static`, aka no stack references (use the `move`
    ///   keyword).
    ///
    /// * It can have at most 7 arguments.
    ///
    /// * Its arguments and return values are all types that can be shared with
    ///   JS (i.e. have `#[wasm_bindgen]` annotations or are simple numbers,
    ///   etc.)
    pub fn new_aborting<F>(t: F) -> Self
    where
        F: IntoWasmClosure<T> + 'static,
    {
        Self::_wrap(Box::new(t).unsize(), false)
    }

    /// Alias for [`new_aborting`](Self::new_aborting) — creates an owned `'static` closure
    /// that aborts on panic.
    ///
    /// This name is symmetric with [`borrow_aborting`](ScopedClosure::borrow_aborting) and
    /// [`borrow_mut_aborting`](ScopedClosure::borrow_mut_aborting) for borrowed closures.
    ///
    /// See [`new_aborting`](Self::new_aborting) for full documentation.
    pub fn own_aborting<F>(t: F) -> Self
    where
        F: IntoWasmClosure<T> + 'static,
    {
        Self::new_aborting(t)
    }

    /// A more direct version of `Closure::new` which creates a `Closure` from
    /// a `Box<dyn Fn>`/`Box<dyn FnMut>`, which is how it's kept internally.
    ///
    /// This version catches panics when unwinding is available.
    pub fn wrap<F>(data: Box<F>) -> Self
    where
        F: MaybeUnwindSafe + IntoWasmClosure<T> + ?Sized,
    {
        Self::_wrap(data.unsize(), true)
    }

    /// A more direct version of `Closure::new` which creates a `Closure` from
    /// a `Box<dyn Fn>`/`Box<dyn FnMut>`, which is how it's kept internally.
    ///
    /// Unlike `wrap`, this version does NOT catch panics and does NOT require `UnwindSafe`.
    /// If the closure panics, the process will abort.
    ///
    /// Use this when:
    /// - Your closure captures types that aren't `UnwindSafe` (like `Rc<Cell<T>>`)
    /// - You don't need panic catching across the JS boundary
    /// - You prefer abort-on-panic behavior
    pub fn wrap_aborting<F>(data: Box<F>) -> Self
    where
        F: IntoWasmClosure<T> + ?Sized,
    {
        Self::_wrap(data.unsize(), false)
    }

    #[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
    fn _wrap(data: Box<T>, unwind_safe: bool) -> Self {
        Self {
            js: crate::__rt::wbg_cast(OwnedClosureUnwind { data, unwind_safe }),
            _marker: PhantomData,
            _lifetime: PhantomData,
        }
    }

    #[cfg(not(all(feature = "std", target_arch = "wasm32", panic = "unwind")))]
    fn _wrap(data: Box<T>, _unwind_safe: bool) -> Self {
        Self {
            js: crate::__rt::wbg_cast(OwnedClosure(data)),
            _marker: PhantomData,
            _lifetime: PhantomData,
        }
    }

    /// Release memory management of this closure from Rust to the JS GC.
    ///
    /// When a `Closure` is dropped it will release the Rust memory and
    /// invalidate the associated JS closure, but this isn't always desired.
    /// Some callbacks are alive for the entire duration of the program or for a
    /// lifetime dynamically managed by the JS GC. This function can be used
    /// to drop this `Closure` while keeping the associated JS function still
    /// valid.
    ///
    /// If the platform supports weak references, the Rust memory will be
    /// reclaimed when the JS closure is GC'd. If weak references is not
    /// supported, this can be dangerous if this function is called many times
    /// in an application because the memory leak will overwhelm the page
    /// quickly and crash the wasm.
    ///
    /// # Safety Note
    ///
    /// This method is only available on `'static` closures (`Closure<T>` /
    /// `StaticClosure<T>`). Calling it on a borrowed `ScopedClosure` would be
    /// unsound because the closure data would become invalid when the borrow ends.
    pub fn into_js_value(self) -> JsValue {
        let idx = self.js.idx;
        mem::forget(self);
        JsValue::_new(idx)
    }

    /// Same as `mem::forget(self)`.
    ///
    /// This can be used to fully relinquish closure ownership to the JS.
    ///
    /// # Safety Note
    ///
    /// This method is only available on `'static` closures (`Closure<T>` /
    /// `StaticClosure<T>`). Calling it on a borrowed `ScopedClosure` would be
    /// unsound because the closure data would become invalid when the borrow ends.
    pub fn forget(self) {
        mem::forget(self);
    }

    /// Create a `Closure` from a function that can only be called once.
    ///
    /// Since we have no way of enforcing that JS cannot attempt to call this
    /// `FnOne(A...) -> R` more than once, this produces a `Closure<dyn FnMut(A...)
    /// -> R>` that will dynamically throw a JavaScript error if called more
    /// than once.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use wasm_bindgen::prelude::*;
    ///
    /// // Create an non-`Copy`, owned `String`.
    /// let mut s = String::from("Hello");
    ///
    /// // Close over `s`. Since `f` returns `s`, it is `FnOnce` and can only be
    /// // called once. If it was called a second time, it wouldn't have any `s`
    /// // to work with anymore!
    /// let f = move || {
    ///     s += ", World!";
    ///     s
    /// };
    ///
    /// // Create a `Closure` from `f`. Note that the `Closure`'s type parameter
    /// // is `FnMut`, even though `f` is `FnOnce`.
    /// let closure: Closure<dyn FnMut() -> String> = Closure::once(f);
    /// ```
    ///
    /// Note: the `A` and `R` type parameters are here just for backward compat
    /// and will be removed in the future.
    pub fn once<F, A, R>(fn_once: F) -> Self
    where
        F: WasmClosureFnOnce<T, A, R>,
        F: MaybeUnwindSafe,
    {
        Closure::_wrap(fn_once.into_fn_mut(), true)
    }

    /// Create a `Closure` from a function that can only be called once.
    ///
    /// Unlike `once`, this version does NOT catch panics and does NOT require `UnwindSafe`.
    /// If the closure panics, the process will abort.
    ///
    /// Use this when:
    /// - Your closure captures types that aren't `UnwindSafe` (like `Rc<Cell<T>>`)
    /// - You don't need panic catching across the JS boundary
    /// - You prefer abort-on-panic behavior
    ///
    /// Since we have no way of enforcing that JS cannot attempt to call this
    /// `FnOnce(A...) -> R` more than once, this produces a `Closure<dyn FnMut(A...)
    /// -> R>` that will dynamically throw a JavaScript error if called more
    /// than once.
    ///
    /// Note: the `A` and `R` type parameters are here just for backward compat
    /// and will be removed in the future.
    pub fn once_aborting<F, A, R>(fn_once: F) -> Self
    where
        F: WasmClosureFnOnceAbort<T, A, R>,
    {
        Closure::_wrap(fn_once.into_fn_mut(), false)
    }

    /// Convert a `FnOnce(A...) -> R` into a JavaScript `Function` object.
    ///
    /// If the JavaScript function is invoked more than once, it will throw an
    /// exception.
    ///
    /// Unlike `Closure::once`, this does *not* return a `Closure` that can be
    /// dropped before the function is invoked to deallocate the closure. The
    /// only way the `FnOnce` is deallocated is by calling the JavaScript
    /// function. If the JavaScript function is never called then the `FnOnce`
    /// and everything it closes over will leak.
    ///
    /// ```rust,ignore
    /// use wasm_bindgen::{prelude::*, JsCast};
    ///
    /// let f = Closure::once_into_js(move || {
    ///     // ...
    /// });
    ///
    /// assert!(f.is_instance_of::<js_sys::Function>());
    /// ```
    ///
    /// Note: the `A` and `R` type parameters are here just for backward compat
    /// and will be removed in the future.
    pub fn once_into_js<F, A, R>(fn_once: F) -> JsValue
    where
        F: WasmClosureFnOnce<T, A, R>,
    {
        fn_once.into_js_function()
    }

    /// Convert a `FnOnce(A...) -> R` into a JavaScript `Function` object.
    ///
    /// Unlike `once_into_js`, this version does NOT catch panics and does NOT require `UnwindSafe`.
    /// If the closure panics, the process will abort.
    ///
    /// If the JavaScript function is invoked more than once, it will throw an
    /// exception.
    ///
    /// Unlike `Closure::once_aborting`, this does *not* return a `Closure` that can be
    /// dropped before the function is invoked to deallocate the closure. The
    /// only way the `FnOnce` is deallocated is by calling the JavaScript
    /// function. If the JavaScript function is never called then the `FnOnce`
    /// and everything it closes over will leak.
    ///
    /// Note: the `A` and `R` type parameters are here just for backward compat
    /// and will be removed in the future.
    pub fn once_into_js_aborting<F, A, R>(fn_once: F) -> JsValue
    where
        F: WasmClosureFnOnceAbort<T, A, R>,
    {
        fn_once.into_js_function()
    }
}

/// A trait for converting an `FnOnce(A...) -> R` into a `FnMut(A...) -> R` that
/// will throw if ever called more than once.
#[doc(hidden)]
pub trait WasmClosureFnOnce<FnMut: ?Sized, A, R>: 'static {
    fn into_fn_mut(self) -> Box<FnMut>;

    fn into_js_function(self) -> JsValue;
}

/// A trait for converting an `FnOnce(A...) -> R` into a `FnMut(A...) -> R` that
/// will throw if ever called more than once. This variant does not require UnwindSafe.
#[doc(hidden)]
pub trait WasmClosureFnOnceAbort<FnMut: ?Sized, A, R>: 'static {
    fn into_fn_mut(self) -> Box<FnMut>;

    fn into_js_function(self) -> JsValue;
}

impl<T: ?Sized> AsRef<JsValue> for ScopedClosure<'_, T> {
    fn as_ref(&self) -> &JsValue {
        &self.js
    }
}

/// Internal representation of an owned closure that we send to JS.
/// This is used when panic=abort or when panic=unwind but without the unwind_safe flag.
#[repr(transparent)]
struct OwnedClosure<T: ?Sized>(Box<T>);

/// Internal representation of an owned closure with unwind safety flag. Used
/// when panic=unwind to pass both the closure and the unwind_safe flag to JS.
#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
struct OwnedClosureUnwind<T: ?Sized> {
    data: Box<T>,
    unwind_safe: bool,
}

struct BorrowedClosure<T: ?Sized> {
    data: WasmSlice,
    unwind_safe: bool,
    _marker: PhantomData<T>,
}

unsafe extern "C" fn destroy<T: ?Sized>(a: usize, mut b: usize) {
    if a == 0 {
        return;
    }
    // Mask out unwind_safe flag
    b &= !0x80000000;
    drop(mem::transmute_copy::<_, Box<T>>(&(a, b)));
}

impl<T> WasmDescribe for OwnedClosure<T>
where
    T: WasmClosure + ?Sized,
{
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(CLOSURE);
        inform(destroy::<T> as *const () as usize as u32);
        inform(T::IS_MUT as u32);
        T::describe();
    }
}

impl<T> WasmDescribe for BorrowedClosure<T>
where
    T: WasmClosure + ?Sized,
{
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(CLOSURE);
        inform(0);
        inform(T::IS_MUT as u32);
        T::describe();
    }
}

impl<T> IntoWasmAbi for OwnedClosure<T>
where
    T: WasmClosure + ?Sized,
{
    type Abi = WasmSlice;

    fn into_abi(self) -> WasmSlice {
        use core::mem::ManuallyDrop;
        let (a, b): (usize, usize) = unsafe { mem::transmute_copy(&ManuallyDrop::new(self)) };
        WasmSlice {
            ptr: a as u32,
            len: b as u32,
        }
    }
}

impl<T> IntoWasmAbi for BorrowedClosure<T>
where
    T: WasmClosure + ?Sized,
{
    type Abi = WasmSlice;
    fn into_abi(self) -> WasmSlice {
        let WasmSlice { ptr, mut len } = self.data;
        if self.unwind_safe {
            len |= 0x80000000;
        }
        WasmSlice { ptr, len }
    }
}

#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
impl<T> WasmDescribe for OwnedClosureUnwind<T>
where
    T: WasmClosure + ?Sized,
{
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        // Delegate to the inner closure's descriptor - type info is the same
        OwnedClosure::<T>::describe();
    }
}

#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
impl<T> IntoWasmAbi for OwnedClosureUnwind<T>
where
    T: WasmClosure + ?Sized,
{
    type Abi = WasmSlice;

    fn into_abi(self) -> WasmSlice {
        use core::mem::ManuallyDrop;
        let (a, b): (usize, usize) = unsafe { mem::transmute_copy(&ManuallyDrop::new(self.data)) };
        // Pack unwind_safe into most significant bit (bit 31) of vtable
        let b_with_flag = if self.unwind_safe {
            (b as u32) | 0x80000000
        } else {
            b as u32
        };
        WasmSlice {
            ptr: a as u32,
            len: b_with_flag,
        }
    }
}

impl<T> WasmDescribe for ScopedClosure<'_, T>
where
    T: WasmClosure + ?Sized,
{
    #[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
    fn describe() {
        inform(EXTERNREF);
    }
}

// `ScopedClosure` can be passed by reference to imports (for any lifetime).
impl<T> IntoWasmAbi for &ScopedClosure<'_, T>
where
    T: WasmClosure + ?Sized,
{
    type Abi = u32;

    fn into_abi(self) -> u32 {
        (&self.js).into_abi()
    }
}

impl<T> OptionIntoWasmAbi for &ScopedClosure<'_, T>
where
    T: WasmClosure + ?Sized,
{
    fn none() -> Self::Abi {
        0
    }
}

/// `'static` closures can be passed by value to JS, transferring ownership.
///
/// This is useful for one-shot callbacks where you want JS to own the closure.
/// The closure will be cleaned up by JS GC (if weak references are supported)
/// or will leak (if weak references are not supported).
///
/// # Example
///
/// ```ignore
/// #[wasm_bindgen]
/// extern "C" {
///     fn set_one_shot_callback(cb: Closure<dyn FnMut()>);
/// }
///
/// let cb = Closure::new(|| { /* ... */ });
/// set_one_shot_callback(cb);  // Ownership transferred to JS
/// // No need to store or forget the closure
/// ```
impl<T> IntoWasmAbi for ScopedClosure<'static, T>
where
    T: WasmClosure + ?Sized,
{
    type Abi = u32;

    fn into_abi(self) -> u32 {
        let idx = self.js.idx;
        mem::forget(self);
        idx
    }
}

impl<T> OptionIntoWasmAbi for ScopedClosure<'static, T>
where
    T: WasmClosure + ?Sized,
{
    fn none() -> Self::Abi {
        0
    }
}

fn _check() {
    fn _assert<T: IntoWasmAbi>() {}
    // By reference (any lifetime)
    _assert::<&ScopedClosure<dyn Fn()>>();
    _assert::<&ScopedClosure<dyn Fn(String)>>();
    _assert::<&ScopedClosure<dyn Fn() -> String>>();
    _assert::<&ScopedClosure<dyn FnMut()>>();
    _assert::<&ScopedClosure<dyn FnMut(String)>>();
    _assert::<&ScopedClosure<dyn FnMut() -> String>>();
    // By value (only 'static)
    _assert::<ScopedClosure<'static, dyn Fn()>>();
    _assert::<ScopedClosure<'static, dyn FnMut()>>();
    _assert::<Closure<dyn Fn()>>();
    _assert::<Closure<dyn FnMut()>>();
}

impl<T> fmt::Debug for ScopedClosure<'_, T>
where
    T: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Closure {{ ... }}")
    }
}

/// An internal trait for the `Closure` type.
///
/// This trait is not stable and it's not recommended to use this in bounds or
/// implement yourself.
#[doc(hidden)]
pub unsafe trait WasmClosure: WasmDescribe {
    const IS_MUT: bool;
}

unsafe impl<T: WasmClosure> WasmClosure for AssertUnwindSafe<T> {
    const IS_MUT: bool = T::IS_MUT;
}

/// An internal trait for the `Closure` type.
///
/// This trait is not stable and it's not recommended to use this in bounds or
/// implement yourself.
#[doc(hidden)]
pub trait IntoWasmClosure<T: ?Sized> {
    fn unsize(self: Box<Self>) -> Box<T>;
}

impl<T: ?Sized + WasmClosure> IntoWasmClosure<T> for T {
    fn unsize(self: Box<Self>) -> Box<T> {
        self
    }
}

/// Trait for converting a reference to a closure into a trait object reference.
///
/// This trait is not stable and it's not recommended to use this in bounds or
/// implement yourself.
#[doc(hidden)]
pub trait UnsizeClosureRef<T: ?Sized> {
    /// The `'static` version of `T`. For example, if `T` is `dyn Fn() + 'a`,
    /// then `Static` is `dyn Fn()` (implicitly `'static`).
    type Static: ?Sized + WasmClosure;

    fn unsize_closure_ref(&self) -> &T;
}

/// Trait for converting a mutable reference to a closure into a trait object reference.
///
/// This trait is not stable and it's not recommended to use this in bounds or
/// implement yourself.
#[doc(hidden)]
pub trait UnsizeClosureRefMut<T: ?Sized> {
    /// The `'static` version of `T`. For example, if `T` is `dyn FnMut() + 'a`,
    /// then `Static` is `dyn FnMut()` (implicitly `'static`).
    type Static: ?Sized + WasmClosure;

    fn unsize_closure_ref(&mut self) -> &mut T;
}
