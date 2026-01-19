//! Bivariate analysis

pub mod regression;
mod resamples;

use super::bivariate::resamples::Resamples;
use super::float::Float;
use super::tuple::{Tuple, TupledDistributionsBuilder};
use super::univariate::Sample;

/// Bivariate `(X, Y)` data
///
/// Invariants:
///
/// - No `NaN`s in the data
/// - At least two data points in the set
pub struct Data<'a, X, Y>(&'a [X], &'a [Y]);

impl<'a, X, Y> Copy for Data<'a, X, Y> {}

#[allow(clippy::expl_impl_clone_on_copy)]
impl<'a, X, Y> Clone for Data<'a, X, Y> {
    fn clone(&self) -> Data<'a, X, Y> {
        *self
    }
}

impl<'a, X, Y> Data<'a, X, Y>
where
    X: Float,
    Y: Float,
{
    /// Creates a new data set from two existing slices
    pub fn new(xs: &'a [X], ys: &'a [Y]) -> Data<'a, X, Y> {
        assert!(
            xs.len() == ys.len()
                && xs.len() > 1
                && xs.iter().all(|x| !x.is_nan())
                && ys.iter().all(|y| !y.is_nan())
        );

        Data(xs, ys)
    }

    // TODO Remove the `T` parameter in favor of `S::Output`
    /// Returns the bootstrap distributions of the parameters estimated by the `statistic`
    ///
    /// - Multi-threaded
    /// - Time: `O(nresamples)`
    /// - Memory: `O(nresamples)`
    pub fn bootstrap<T, S>(&self, nresamples: usize, statistic: S) -> T::Distributions
    where
        S: Fn(Data<X, Y>) -> T + Sync,
        T: Tuple + Send,
        T::Distributions: Send,
        T::Builder: Send,
    {
        let mut resamples = Resamples::new(*self);
        (0..nresamples)
            .map(|_| statistic(resamples.next()))
            .fold(T::Builder::new(0), |mut sub_distributions, sample| {
                sub_distributions.push(sample);
                sub_distributions
            })
            .complete()
    }

    /// Returns a view into the `X` data
    pub fn x(&self) -> &'a Sample<X> {
        Sample::new(self.0)
    }

    /// Returns a view into the `Y` data
    pub fn y(&self) -> &'a Sample<Y> {
        Sample::new(self.1)
    }
}
