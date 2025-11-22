//! Regression analysis

use super::super::bivariate::Data;
use super::super::dot;
use super::super::float::Float;

/// A straight line that passes through the origin `y = m * x`
#[derive(Clone, Copy)]
pub struct Slope<A>(pub A)
where
    A: Float;

impl<A> Slope<A>
where
    A: Float,
{
    /// Fits the data to a straight line that passes through the origin using ordinary least
    /// squares
    ///
    /// - Time: `O(length)`
    pub fn fit(data: &Data<'_, A, A>) -> Slope<A> {
        let xs = data.0;
        let ys = data.1;

        let xy = dot(xs, ys);
        let x2 = dot(xs, xs);

        Slope(xy / x2)
    }
}
