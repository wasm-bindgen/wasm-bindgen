//! # ⚠️ Unstable
//!
//! This is an internal module, no stability guarantees are provided. Use at
//! your own risk.

#![allow(clippy::missing_safety_doc)]

mod closures;
mod impls;
mod slices;
mod traits;

pub use self::impls::*;
pub use self::slices::{VectorRefIntoWasmAbi, WasmSlice};
pub use self::traits::*;
