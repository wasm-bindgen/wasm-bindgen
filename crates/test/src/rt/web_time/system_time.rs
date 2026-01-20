//! Re-implementation of [`std::time::SystemTime`].
//!
//! See <https://github.com/rust-lang/rust/blob/1.83.0/library/std/src/time.rs#L470-L707>.

use core::time::Duration;

use super::js::Date;

/// See [`std::time::SystemTime`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SystemTime(pub(crate) Duration);

impl SystemTime {
    /// See [`std::time::SystemTime::UNIX_EPOCH`].
    pub const UNIX_EPOCH: Self = Self(Duration::ZERO);

    /// See [`std::time::SystemTime::now()`].
    #[must_use]
    pub fn now() -> Self {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        let ms = Date::now() as i64;
        let ms = ms.try_into().expect("found negative timestamp");

        Self(Duration::from_millis(ms))
    }

    /// See [`std::time::SystemTime::duration_since()`].
    pub fn duration_since(&self, earlier: Self) -> Result<Duration, SystemTimeError> {
        // See <https://github.com/rust-lang/rust/blob/1.83.0/library/std/src/sys/pal/unsupported/time.rs#L34-L36>.
        self.0.checked_sub(earlier.0).ok_or(SystemTimeError)
    }
}

/// See [`std::time::SystemTimeError`].
#[derive(Clone, Debug)]
pub struct SystemTimeError;
