//! Hooks for responding to hard-abort and reinit events on the Wasm instance.
//!
//! # Hard abort
//!
//! A hard abort occurs when the WebAssembly instance encounters a
//! non-recoverable error — an `unreachable` instruction, out-of-memory, or
//! stack overflow — that cannot be caught by Rust's `catch_unwind`.  The
//! instance is poisoned and no further exports can be called.
//!
//! Use [`set_on_abort`] to register a callback that runs at the moment of
//! termination.  Returns the previously registered handler (`None` if unset),
//! mirroring the `std::panic::set_hook` convention.
//!
//! **Experimental — only available when built with `panic=unwind`.**
//! [`set_on_abort`] returns `None` and the callback will never fire on
//! `panic=abort` builds.
//!
//! # Reinit
//!
//! [`reinit()`] writes a sentinel value (`0xFFFFFFFF`) into the termination
//! flag.  The next call to any export detects this, creates a fresh
//! `WebAssembly.Instance` from the same module, and then calls
//! [`set_on_reinit`]'s callback (if any) on the new instance.
//!
//! Use [`set_on_reinit`] to register a reinit callback; it likewise returns
//! the previously registered handler.
//!
//! **Experimental — only available when built with `panic=unwind` and when
//! wasm-bindgen is invoked with `--experimental-reset-state-function`.**
//! Without that flag the JS-side guard that detects the sentinel is not
//! emitted, so calling [`reinit()`] writes the sentinel but it is never acted
//! upon.  [`reinit()`] and [`set_on_reinit`] are no-ops on `panic=abort`
//! builds.
#[doc(hidden)]
pub use crate::__rt::reinit;
#[doc(hidden)]
pub use crate::__rt::set_on_abort;
#[doc(hidden)]
pub use crate::__rt::set_on_reinit;
