//! # ⚠️ Unstable
//!
//! This is an internal module, no stability guarantees are provided. Use at
//! your own risk.

#![allow(clippy::missing_safety_doc)]

/// `MaybeIntoWasmAbi` for a type whose own ABI has spare values for `None`
/// and `Null`, so `Option<Self>` and `Nullable<Self>` share that ABI. The
/// legacy `OptionIntoWasmAbi` is derived from it.
macro_rules! impl_maybe_into_shared_abi {
    (none = $none:expr; null = $null:expr; [$($g:tt)*] $t:ty $(where $($w:tt)*)?) => {
        impl<$($g)*> $crate::convert::MaybeIntoWasmAbi for $t $(where $($w)*)? {
            type MaybeAbi = <Self as $crate::convert::IntoWasmAbi>::Abi;

            #[inline]
            fn into_maybe_abi(self) -> Self::MaybeAbi {
                $crate::convert::IntoWasmAbi::into_abi(self)
            }

            #[inline]
            fn none() -> Self::MaybeAbi {
                $none
            }

            #[inline]
            fn null() -> Self::MaybeAbi {
                $null
            }
        }

        impl<$($g)*> $crate::convert::OptionIntoWasmAbi for $t $(where $($w)*)? {
            #[inline]
            fn none() -> Self::Abi {
                <Self as $crate::convert::MaybeIntoWasmAbi>::none()
            }
        }
    };
}

/// The receiving side of [`impl_maybe_into_shared_abi!`].
macro_rules! impl_maybe_from_shared_abi {
    (
        is_none = |$none_abi:ident| $is_none:expr;
        is_null = |$null_abi:ident| $is_null:expr;
        [$($g:tt)*] $t:ty $(where $($w:tt)*)?
    ) => {
        impl<$($g)*> $crate::convert::MaybeFromWasmAbi for $t $(where $($w)*)? {
            type MaybeAbi = <Self as $crate::convert::FromWasmAbi>::Abi;

            #[inline]
            unsafe fn from_maybe_abi(abi: Self::MaybeAbi) -> Self {
                <Self as $crate::convert::FromWasmAbi>::from_abi(abi)
            }

            #[inline]
            unsafe fn try_from_maybe_abi(abi: Self::MaybeAbi) -> Result<Self, &'static str> {
                <Self as $crate::convert::FromWasmAbi>::try_from_abi(abi)
            }

            #[inline]
            fn is_none($none_abi: &Self::MaybeAbi) -> bool {
                $is_none
            }

            #[inline]
            fn is_null($null_abi: &Self::MaybeAbi) -> bool {
                $is_null
            }
        }

        impl<$($g)*> $crate::convert::OptionFromWasmAbi for $t $(where $($w)*)? {
            #[inline]
            fn is_none(abi: &Self::Abi) -> bool {
                <Self as $crate::convert::MaybeFromWasmAbi>::is_none(abi)
            }
        }
    };
}

pub(crate) use impl_maybe_into_shared_abi;

mod closures;
mod impls;
mod slices;
mod traits;

pub use self::impls::*;
pub use self::slices::{VectorRefIntoWasmAbi, WasmSlice};
pub use self::traits::*;
