//! # ⚠️ Unstable
//!
//! This is an internal module, no stability guarantees are provided. Use at
//! your own risk.

#![allow(clippy::missing_safety_doc)]

/// `NullableIntoWasmAbi` for a type whose own ABI has a spare value for
/// `Null`, so `Nullable<Self>` shares that ABI as `Option<Self>` does.
macro_rules! impl_nullable_into_shared_abi {
    (null = $null:expr; [$($g:tt)*] $t:ty $(where $($w:tt)*)?) => {
        impl<$($g)*> $crate::convert::NullableIntoWasmAbi for $t $(where $($w)*)? {
            type NullableAbi = <Self as $crate::convert::IntoWasmAbi>::Abi;

            #[inline]
            fn into_nullable_abi(self) -> Self::NullableAbi {
                $crate::convert::IntoWasmAbi::into_abi(self)
            }

            #[inline]
            fn null() -> Self::NullableAbi {
                $null
            }

            #[inline]
            fn nullable_none() -> Self::NullableAbi {
                <Self as $crate::convert::OptionIntoWasmAbi>::none()
            }
        }
    };
}

/// The receiving side of [`impl_nullable_into_shared_abi!`].
macro_rules! impl_nullable_from_shared_abi {
    (is_null = |$abi:ident| $is_null:expr; [$($g:tt)*] $t:ty $(where $($w:tt)*)?) => {
        impl<$($g)*> $crate::convert::NullableFromWasmAbi for $t $(where $($w)*)? {
            type NullableAbi = <Self as $crate::convert::FromWasmAbi>::Abi;

            #[inline]
            unsafe fn from_nullable_abi(abi: Self::NullableAbi) -> Self {
                <Self as $crate::convert::FromWasmAbi>::from_abi(abi)
            }

            #[inline]
            unsafe fn try_from_nullable_abi(
                abi: Self::NullableAbi,
            ) -> Result<Self, &'static str> {
                <Self as $crate::convert::FromWasmAbi>::try_from_abi(abi)
            }

            #[inline]
            fn is_null($abi: &Self::NullableAbi) -> bool {
                $is_null
            }

            #[inline]
            fn nullable_is_none(abi: &Self::NullableAbi) -> bool {
                <Self as $crate::convert::OptionFromWasmAbi>::is_none(abi)
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
