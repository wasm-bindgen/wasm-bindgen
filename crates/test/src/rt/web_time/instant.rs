//! Re-implementation of [`std::time::Instant`].
//!
//! See <https://github.com/rust-lang/rust/blob/1.83.0/library/std/src/time.rs#L271-L468>.

use core::ops::Sub;
use core::time::Duration;

use super::js::PERFORMANCE;
#[cfg(target_feature = "atomics")]
use super::js::TIME_ORIGIN;

/// See [`std::time::Instant`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Instant(Duration);

impl Instant {
    /// See [`std::time::Instant::now()`].
    ///
    /// # Panics
    ///
    /// This call will panic if the [`Performance` object] was not found, e.g.
    /// calling from a [worklet].
    ///
    /// [`Performance` object]: https://developer.mozilla.org/en-US/docs/Web/API/performance_property
    /// [worklet]: https://developer.mozilla.org/en-US/docs/Web/API/Worklet
    #[must_use]
    pub fn now() -> Self {
        let now = PERFORMANCE.with(|performance| {
            let performance = performance
                .as_ref()
                .expect("`Performance` object not found");

            #[cfg(not(target_feature = "atomics"))]
            return performance.now();
            #[cfg(target_feature = "atomics")]
            TIME_ORIGIN.with(|origin| performance.now() + origin)
        });

        assert!(
            now.is_sign_positive(),
            "negative `DOMHighResTimeStamp`s are not supported"
        );
        Self(time_stamp_to_duration(now))
    }

    /// See [`std::time::Instant::duration_since()`].
    #[must_use]
    pub fn duration_since(&self, earlier: Self) -> Duration {
        self.checked_duration_since(earlier).unwrap_or_default()
    }

    /// See [`std::time::Instant::checked_duration_since()`].
    #[must_use]
    pub fn checked_duration_since(&self, earlier: Self) -> Option<Duration> {
        self.0.checked_sub(earlier.0)
    }

    /// See [`std::time::Instant::elapsed()`].
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        Self::now() - *self
    }
}

impl Sub<Self> for Instant {
    type Output = Duration;

    /// Returns the amount of time elapsed from another instant to this one,
    /// or zero duration if that instant is later than this one.
    fn sub(self, rhs: Self) -> Duration {
        self.duration_since(rhs)
    }
}

/// Converts a `DOMHighResTimeStamp` to a [`Duration`].
///
/// # Note
///
/// Keep in mind that like [`Duration::from_secs_f64()`] this doesn't do perfect
/// rounding.
fn time_stamp_to_duration(time_stamp: f64) -> Duration {
    let time_stamp = F64(time_stamp);

    Duration::from_millis(time_stamp.trunc() as u64)
        + Duration::from_nanos(F64(time_stamp.fract() * 1.0e6).internal_round_ties_even() as u64)
}

/// [`f64`] `no_std` compatibility wrapper.
#[derive(Clone, Copy)]
struct F64(f64);

impl F64 {
    /// See [`f64::trunc()`].
    fn trunc(self) -> f64 {
        libm::trunc(self.0)
    }

    /// See [`f64::fract()`].
    fn fract(self) -> f64 {
        self.0 - self.trunc()
    }

    /// A specialized version of [`f64::round_ties_even()`]. [`f64`] must be
    /// positive and have an exponent smaller than `52`.
    ///
    /// - We expect `DOMHighResTimeStamp` to always be positive. We check that
    ///   in [`Instant::now()`].
    /// - We only round the fractional part after multiplying it by `1e6`. A
    ///   fraction always has a negative exponent. `1e6` has an exponent of
    ///   `19`. Therefor the resulting exponent can at most be `19`.
    ///
    /// [`f64::round_ties_even()`]: https://doc.rust-lang.org/1.83.0/std/primitive.f64.html#method.round_ties_even
    fn internal_round_ties_even(self) -> f64 {
        /// Put `debug_assert!` in a function to clap `coverage(off)` on it.
        ///
        /// See <https://github.com/rust-lang/rust/issues/80549>.
        fn check(this: f64) {
            debug_assert!(this.is_sign_positive(), "found negative input");
            debug_assert!(
                {
                    let exponent: u64 = this.to_bits() >> 52 & 0x7ff;
                    exponent < 0x3ff + 52
                },
                "found number with exponent bigger than 51"
            );
        }

        check(self.0);

        // See <https://github.com/rust-lang/libm/blob/libm-v0.2.11/src/math/rint.rs>.

        let one_over_e = 1.0 / f64::EPSILON;
        // REMOVED: We don't support numbers with exponents bigger than 51.
        // REMOVED: We don't support negative numbers.
        // REMOVED: We don't support numbers with exponents bigger than 51.
        let xplusoneovere = self.0 + one_over_e;
        xplusoneovere - one_over_e
        // REMOVED: We don't support negative numbers.
    }
}
