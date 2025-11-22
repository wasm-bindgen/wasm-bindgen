use super::benchmark::BenchmarkConfig;
use super::measurement::Measurement;
use super::report::{BenchmarkId, Report};
use super::{ActualSamplingMode, Bencher, Criterion};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::time::Duration;

/// PRIVATE
pub(crate) trait Routine<M: Measurement> {
    /// PRIVATE
    fn bench(&mut self, m: &M, iters: &[u64]) -> Vec<f64>;
    /// PRIVATE
    fn warm_up(&mut self, m: &M, how_long: Duration) -> (u64, u64);

    fn sample(
        &mut self,
        measurement: &M,
        id: &BenchmarkId,
        config: &BenchmarkConfig,
        criterion: &Criterion<M>,
    ) -> (ActualSamplingMode, Box<[f64]>, Box<[f64]>) {
        let wu = config.warm_up_time;
        let m_ns = config.measurement_time.as_nanos();

        criterion.report.warmup(id, wu.as_nanos() as f64);

        let (wu_elapsed, wu_iters) = self.warm_up(measurement, wu);

        // Initial guess for the mean execution time
        let met = wu_elapsed as f64 / wu_iters as f64;

        let n = config.sample_size as u64;

        let actual_sampling_mode = config
            .sampling_mode
            .choose_sampling_mode(met, n, m_ns as f64);

        let m_iters = actual_sampling_mode.iteration_counts(met, n, &config.measurement_time);

        let expected_ns = m_iters
            .iter()
            .copied()
            .map(|count| count as f64 * met)
            .sum();

        // Use saturating_add to handle overflow.
        let mut total_iters = 0u64;
        for count in m_iters.iter().copied() {
            total_iters = total_iters.saturating_add(count);
        }

        criterion
            .report
            .measurement_start(id, n, expected_ns, total_iters);

        let m_elapsed = self.bench(measurement, &m_iters);

        let m_iters_f: Vec<f64> = m_iters.iter().map(|&x| x as f64).collect();

        (
            actual_sampling_mode,
            m_iters_f.into_boxed_slice(),
            m_elapsed.into_boxed_slice(),
        )
    }
}

pub struct Function<M: Measurement, F>
where
    F: FnMut(&mut Bencher<'_, M>),
{
    f: F,
    _phamtom2: PhantomData<M>,
}
impl<M: Measurement, F> Function<M, F>
where
    F: FnMut(&mut Bencher<'_, M>),
{
    pub fn new(f: F) -> Function<M, F> {
        Function {
            f,
            _phamtom2: PhantomData,
        }
    }
}

impl<M: Measurement, F> Routine<M> for Function<M, F>
where
    F: FnMut(&mut Bencher<'_, M>),
{
    fn bench(&mut self, m: &M, iters: &[u64]) -> Vec<f64> {
        let f = &mut self.f;

        let mut b = Bencher {
            iterated: false,
            iters: 0,
            value: m.zero(),
            measurement: m,
            elapsed_time: Duration::from_millis(0),
        };

        iters
            .iter()
            .map(|iters| {
                b.iters = *iters;
                (*f)(&mut b);
                m.to_f64(&b.value)
            })
            .collect()
    }

    fn warm_up(&mut self, m: &M, how_long: Duration) -> (u64, u64) {
        let f = &mut self.f;
        let mut b = Bencher {
            iterated: false,
            iters: 1,
            value: m.zero(),
            measurement: m,
            elapsed_time: Duration::from_millis(0),
        };

        let mut total_iters = 0;
        let mut elapsed_time = Duration::from_millis(0);
        loop {
            (*f)(&mut b);

            total_iters += b.iters;
            elapsed_time += b.elapsed_time;
            if elapsed_time > how_long {
                return (elapsed_time.as_nanos() as u64, total_iters);
            }

            b.iters = b.iters.wrapping_mul(2);
        }
    }
}
