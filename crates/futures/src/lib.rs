//! Bridging the gap between Rust Futures and JavaScript Promises.
//!
//! This crate is now a thin shim re-exporting from [`js_sys::futures`].
//! The implementation has been moved into `js-sys` so that
//! [`js_sys::Promise`] can implement [`core::future::IntoFuture`] directly,
//! enabling `promise.await` without any wrapper type.
//!
//! All public items (`JsFuture`, `spawn_local`, `future_to_promise`,
//! `future_to_promise_typed`) are re-exported unchanged for backwards
//! compatibility.

#![cfg_attr(not(feature = "std"), no_std)]

pub use js_sys::futures::{future_to_promise, future_to_promise_typed, spawn_local, JsFuture};

#[cfg(feature = "futures-core-03-stream")]
pub use js_sys::futures::stream;

#[cfg(not(target_feature = "atomics"))]
#[allow(deprecated)]
pub use js_sys::futures::jspi_block_on_promise;

#[cfg(all(target_os = "emscripten", wasm_bindgen_unstable_tokio))]
pub mod tokio;

/// Stand-in for unsupported targets: `#[wasm_bindgen(experimental_tokio)]`
/// expansions still resolve, and the unsatisfiable bound reports why at the
/// attribute instead of as an unresolved path.
#[cfg(not(all(target_os = "emscripten", wasm_bindgen_unstable_tokio)))]
#[doc(hidden)]
pub mod tokio {
    #[diagnostic::on_unimplemented(
        message = "`#[wasm_bindgen(experimental_tokio)]` is only supported on the \
                   `wasm32-unknown-emscripten` target with `--cfg wasm_bindgen_unstable_tokio`",
        label = "this async export is driven on a tokio event loop"
    )]
    pub trait ExperimentalTokioSupported {}

    pub fn schedule<F: ExperimentalTokioSupported, C>(_future: F, _on_complete: C) {
        unreachable!()
    }

    pub fn schedule_isolated<F: ExperimentalTokioSupported, C>(_future: F, _on_complete: C) {
        unreachable!()
    }

    pub fn block_on<F: ExperimentalTokioSupported + core::future::Future>(_future: F) -> F::Output {
        unreachable!()
    }

    pub fn block_on_isolated<F: ExperimentalTokioSupported + core::future::Future>(
        _future: F,
    ) -> F::Output {
        unreachable!()
    }
}

pub use js_sys;
pub use wasm_bindgen;
