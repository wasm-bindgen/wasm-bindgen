use alloc::vec::Vec;
use core::{mem, ops};

use super::super::float::Float;
use super::super::sum;
use super::super::tuple::{Tuple, TupledDistributionsBuilder};
use super::super::univariate::Percentiles;
use super::super::univariate::Resamples;

/// A collection of data points drawn from a population
///
/// Invariants:
///
/// - The sample contains at least 2 data points
/// - The sample contains no `NaN`s
#[repr(transparent)]
pub struct Sample<A>([A]);

// TODO(rust-lang/rfcs#735) move this `impl` into a private percentiles module
impl<A> Sample<A>
where
    A: Float,
{
    /// Creates a new sample from an existing slice
    ///
    /// # Panics
    ///
    /// Panics if `slice` contains any `NaN` or if `slice` has less than two elements
    #[allow(clippy::new_ret_no_self)]
    pub fn new(slice: &[A]) -> &Sample<A> {
        assert!(slice.len() > 1 && slice.iter().all(|x| !x.is_nan()));

        unsafe { mem::transmute(slice) }
    }

    /// Returns the arithmetic average of the sample
    ///
    /// - Time: `O(length)`
    pub fn mean(&self) -> A {
        let n = self.len();

        self.sum() / A::cast(n)
    }

    /// Returns the median absolute deviation
    ///
    /// The `median` can be optionally passed along to speed up (2X) the computation
    ///
    /// - Time: `O(length)`
    /// - Memory: `O(length)`
    pub fn median_abs_dev(&self, median: Option<A>) -> A
    where
        usize: cast::From<A, Output = Result<usize, cast::Error>>,
    {
        let median = median.unwrap_or_else(|| self.percentiles().median());

        // NB Although this operation can be SIMD accelerated, the gain is negligible because the
        // bottle neck is the sorting operation which is part of the computation of the median
        let abs_devs = self.iter().map(|&x| (x - median).abs()).collect::<Vec<_>>();

        let abs_devs: &Self = Self::new(&abs_devs);

        abs_devs.percentiles().median() * A::cast(1.4826)
    }

    /// Returns a "view" into the percentiles of the sample
    ///
    /// This "view" makes consecutive computations of percentiles much faster (`O(1)`)
    ///
    /// - Time: `O(N log N) where N = length`
    /// - Memory: `O(length)`
    pub fn percentiles(&self) -> Percentiles<A>
    where
        usize: cast::From<A, Output = Result<usize, cast::Error>>,
    {
        use core::cmp::Ordering;

        // NB This function assumes that there are no `NaN`s in the sample
        fn cmp<T>(a: &T, b: &T) -> Ordering
        where
            T: PartialOrd,
        {
            match a.partial_cmp(b) {
                Some(o) => o,
                // Arbitrary way to handle NaNs that should never happen
                None => Ordering::Equal,
            }
        }

        let mut v = self.to_vec().into_boxed_slice();
        v.sort_unstable_by(cmp);

        // NB :-1: to intra-crate privacy rules
        unsafe { mem::transmute(v) }
    }

    /// Returns the standard deviation of the sample
    ///
    /// The `mean` can be optionally passed along to speed up (2X) the computation
    ///
    /// - Time: `O(length)`
    pub fn std_dev(&self, mean: Option<A>) -> A {
        self.var(mean).sqrt()
    }

    /// Returns the sum of all the elements of the sample
    ///
    /// - Time: `O(length)`
    pub fn sum(&self) -> A {
        sum(self)
    }

    /// Returns the t score between these two samples
    ///
    /// - Time: `O(length)`
    pub fn t(&self, other: &Sample<A>) -> A {
        let (x_bar, y_bar) = (self.mean(), other.mean());
        let (s2_x, s2_y) = (self.var(Some(x_bar)), other.var(Some(y_bar)));
        let n_x = A::cast(self.len());
        let n_y = A::cast(other.len());
        let num = x_bar - y_bar;
        let den = (s2_x / n_x + s2_y / n_y).sqrt();

        num / den
    }

    /// Returns the variance of the sample
    ///
    /// The `mean` can be optionally passed along to speed up (2X) the computation
    ///
    /// - Time: `O(length)`
    pub fn var(&self, mean: Option<A>) -> A {
        use core::ops::Add;

        let mean = mean.unwrap_or_else(|| self.mean());
        let slice = self;

        let sum = slice
            .iter()
            .map(|&x| (x - mean).powi(2))
            .fold(A::cast(0), Add::add);

        sum / A::cast(slice.len() - 1)
    }

    // TODO Remove the `T` parameter in favor of `S::Output`
    /// Returns the bootstrap distributions of the parameters estimated by the 1-sample statistic
    ///
    /// - Multi-threaded
    /// - Time: `O(nresamples)`
    /// - Memory: `O(nresamples)`
    pub fn bootstrap<T, S>(&self, nresamples: usize, statistic: S) -> T::Distributions
    where
        S: Fn(&Sample<A>) -> T + Sync,
        T: Tuple + Send,
        T::Distributions: Send,
        T::Builder: Send,
    {
        let mut resamples = Resamples::new(self);
        (0..nresamples)
            .map(|_| statistic(resamples.next()))
            .fold(T::Builder::new(0), |mut sub_distributions, sample| {
                sub_distributions.push(sample);
                sub_distributions
            })
            .complete()
    }
}

impl<A> ops::Deref for Sample<A> {
    type Target = [A];

    fn deref(&self) -> &[A] {
        &self.0
    }
}
