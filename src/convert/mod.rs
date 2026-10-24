//! # ⚠️ Unstable
//!
//! This is an internal module, no stability guarantees are provided. Use at
//! your own risk.

#![allow(clippy::missing_safety_doc)]

/// `NullableIntoWasmAbi` for a type whose own ABI has a spare value for
/// `Null` beside its `OptionIntoWasmAbi::none`, so `Nullable<Self>` shares
/// that ABI as `Option<Self>` does.
macro_rules! impl_nullable_into_shared_abi {
    (null = $null:expr; [$($g:tt)*] $t:ty $(where $($w:tt)*)?) => {
        impl<$($g)*> $crate::convert::NullableIntoWasmAbi for $t $(where $($w)*)? {
            #[inline]
            fn null() -> Self::MaybeAbi {
                $null
            }
        }
    };
}

/// The receiving side of [`impl_nullable_into_shared_abi!`].
macro_rules! impl_nullable_from_shared_abi {
    (is_null = |$abi:ident| $is_null:expr; [$($g:tt)*] $t:ty $(where $($w:tt)*)?) => {
        impl<$($g)*> $crate::convert::NullableFromWasmAbi for $t $(where $($w)*)? {
            #[inline]
            fn is_null($abi: &Self::MaybeAbi) -> bool {
                $is_null
            }
        }
    };
}

pub(crate) use impl_nullable_into_shared_abi;

mod closures;
mod impls;
mod slices;
mod traits;

pub use self::impls::*;
pub use self::slices::{VectorRefIntoWasmAbi, WasmSlice};
pub use self::traits::*;
