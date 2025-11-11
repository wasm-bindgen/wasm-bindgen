use core::mem;

use super::super::float::Float;
use super::super::rand_util::{new_rng, Rng};
use super::super::univariate::Sample;

use alloc::vec::Vec;

pub struct Resamples<'a, A>
where
    A: Float,
{
    rng: Rng,
    sample: &'a [A],
    stage: Option<Vec<A>>,
}

#[allow(clippy::should_implement_trait)]
impl<'a, A> Resamples<'a, A>
where
    A: 'a + Float,
{
    pub fn new(sample: &'a Sample<A>) -> Resamples<'a, A> {
        let slice = sample;

        Resamples {
            rng: new_rng(),
            sample: slice,
            stage: None,
        }
    }

    pub fn next(&mut self) -> &Sample<A> {
        let n = self.sample.len();
        let rng = &mut self.rng;

        match self.stage {
            None => {
                let mut stage = Vec::with_capacity(n);

                for _ in 0..n {
                    let idx = rng.rand_range(0u64..(self.sample.len() as u64));
                    stage.push(self.sample[idx as usize]);
                }

                self.stage = Some(stage);
            }
            Some(ref mut stage) => {
                for elem in stage.iter_mut() {
                    let idx = rng.rand_range(0u64..(self.sample.len() as u64));
                    *elem = self.sample[idx as usize];
                }
            }
        }

        if let Some(ref v) = self.stage {
            unsafe { mem::transmute::<&[A], &Sample<A>>(v) }
        } else {
            unreachable!();
        }
    }
}
