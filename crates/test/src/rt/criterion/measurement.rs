//! This module defines a set of traits that can be used to plug different measurements (eg.
//! Unix's Processor Time, CPU or GPU performance counters, etc.) into Criterion.rs. It also
//! includes the [`WallTime`] struct which defines the default wall-clock time measurement.

use super::format::short;
use super::Throughput;
use crate::__rt::web_time::Instant;
use alloc::format;
use alloc::string::String;
use core::time::Duration;
use libm::pow;

/// Trait providing functions to format measured values to string so that they can be displayed on
/// the command line or in the reports. The functions of this trait take measured values in f64
/// form; implementors can assume that the values are of the same scale as those produced by the
/// associated [`Measurement`] (eg. if your measurement produces values in nanoseconds, the
/// values passed to the formatter will be in nanoseconds).
///
/// Implementors are encouraged to format the values in a way that is intuitive for humans and
/// uses the SI prefix system. For example, the format used by [`WallTime`] can display the value
/// in units ranging from picoseconds to seconds depending on the magnitude of the elapsed time
/// in nanoseconds.
pub trait ValueFormatter {
    /// Format the value (with appropriate unit) and return it as a string.
    fn format_value(&self, value: f64) -> String {
        let mut values = [value];
        let unit = self.scale_values(value, &mut values);
        format!("{:>6} {unit}", short(values[0]))
    }

    /// Format the value as a throughput measurement. The value represents the measurement value;
    /// the implementor will have to calculate bytes per second, iterations per cycle, etc.
    fn format_throughput(&self, throughput: &Throughput, value: f64) -> String {
        let mut values = [value];
        let unit = self.scale_throughputs(value, throughput, &mut values);
        format!("{:>6} {unit}", short(values[0]))
    }

    /// Scale the given values to some appropriate unit and return the unit string.
    ///
    /// The given typical value should be used to choose the unit. This function may be called
    /// multiple times with different datasets; the typical value will remain the same to ensure
    /// that the units remain consistent within a graph. The typical value will not be NaN.
    /// Values will not contain NaN as input, and the transformed values must not contain NaN.
    fn scale_values(&self, typical_value: f64, values: &mut [f64]) -> &'static str;

    /// Convert the given measured values into throughput numbers based on the given throughput
    /// value, scale them to some appropriate unit, and return the unit string.
    ///
    /// The given typical value should be used to choose the unit. This function may be called
    /// multiple times with different datasets; the typical value will remain the same to ensure
    /// that the units remain consistent within a graph. The typical value will not be NaN.
    /// Values will not contain NaN as input, and the transformed values must not contain NaN.
    fn scale_throughputs(
        &self,
        typical_value: f64,
        throughput: &Throughput,
        values: &mut [f64],
    ) -> &'static str;

    /// Scale the values and return a unit string designed for machines.
    ///
    /// For example, this is used for the CSV file output. Implementations should modify the given
    /// values slice to apply the desired scaling (if any) and return a string representing the unit
    /// the modified values are in.
    fn scale_for_machines(&self, values: &mut [f64]) -> &'static str;
}

/// Trait for all types which define something Criterion.rs can measure. The only measurement
/// currently provided is [`WallTime`], but third party crates or benchmarks may define more.
///
/// This trait defines two core methods, `start` and `end`. `start` is called at the beginning of
/// a measurement to produce some intermediate value (for example, the wall-clock time at the start
/// of that set of iterations) and `end` is called at the end of the measurement with the value
/// returned by `start`.
///
pub trait Measurement {
    /// This type represents an intermediate value for the measurements. It will be produced by the
    /// start function and passed to the end function. An example might be the wall-clock time as
    /// of the `start` call.
    type Intermediate;

    /// Criterion.rs will call this before iterating the benchmark.
    fn start(&self) -> Self::Intermediate;

    /// Criterion.rs will call this after iterating the benchmark to get the measured value.
    fn end(&self, i: Self::Intermediate) -> Duration;

    /// Combine two values. Criterion.rs sometimes needs to perform measurements in multiple batches
    /// of iterations, so the value from one batch must be added to the sum of the previous batches.
    fn add(&self, v1: &Duration, v2: &Duration) -> Duration;

    /// Return a "zero" value for the Value type which can be added to another value.
    fn zero(&self) -> Duration;

    /// Converts the measured value to f64 so that it can be used in statistical analysis.
    fn to_f64(&self, value: &Duration) -> f64;

    /// Return a trait-object reference to the value formatter for this measurement.
    fn formatter(&self) -> &dyn ValueFormatter;
}

/// Default Formatter
pub(crate) struct DurationFormatter;

impl DurationFormatter {
    fn bytes_per_second(&self, bytes: f64, typical: f64, values: &mut [f64]) -> &'static str {
        let bytes_per_second = bytes * (1e9 / typical);
        let (denominator, unit) = if bytes_per_second < 1024.0 {
            (1.0, "  B/s")
        } else if bytes_per_second < 1024.0 * 1024.0 {
            (1024.0, "KiB/s")
        } else if bytes_per_second < 1024.0 * 1024.0 * 1024.0 {
            (1024.0 * 1024.0, "MiB/s")
        } else {
            (1024.0 * 1024.0 * 1024.0, "GiB/s")
        };

        for val in values {
            let bytes_per_second = bytes * (1e9 / *val);
            *val = bytes_per_second / denominator;
        }

        unit
    }

    fn bytes_per_second_decimal(
        &self,
        bytes: f64,
        typical: f64,
        values: &mut [f64],
    ) -> &'static str {
        let bytes_per_second = bytes * (1e9 / typical);
        let (denominator, unit) = if bytes_per_second < 1000.0 {
            (1.0, "  B/s")
        } else if bytes_per_second < 1000.0 * 1000.0 {
            (1000.0, "KB/s")
        } else if bytes_per_second < 1000.0 * 1000.0 * 1000.0 {
            (1000.0 * 1000.0, "MB/s")
        } else {
            (1000.0 * 1000.0 * 1000.0, "GB/s")
        };

        for val in values {
            let bytes_per_second = bytes * (1e9 / *val);
            *val = bytes_per_second / denominator;
        }

        unit
    }

    fn elements_per_second(&self, elems: f64, typical: f64, values: &mut [f64]) -> &'static str {
        let elems_per_second = elems * (1e9 / typical);
        let (denominator, unit) = if elems_per_second < 1000.0 {
            (1.0, " elem/s")
        } else if elems_per_second < 1000.0 * 1000.0 {
            (1000.0, "Kelem/s")
        } else if elems_per_second < 1000.0 * 1000.0 * 1000.0 {
            (1000.0 * 1000.0, "Melem/s")
        } else {
            (1000.0 * 1000.0 * 1000.0, "Gelem/s")
        };

        for val in values {
            let elems_per_second = elems * (1e9 / *val);
            *val = elems_per_second / denominator;
        }

        unit
    }

    fn bits_per_second(&self, bits: f64, typical: f64, values: &mut [f64]) -> &'static str {
        let bits_per_second = bits * (1e9 / typical);
        let (denominator, unit) = if bits_per_second < 1000.0 {
            (1.0, "  b/s")
        } else if bits_per_second < 1000.0 * 1000.0 {
            (1000.0, "Kb/s")
        } else if bits_per_second < 1000.0 * 1000.0 * 1000.0 {
            (1000.0 * 1000.0, "Mb/s")
        } else {
            (1000.0 * 1000.0 * 1000.0, "Gb/s")
        };

        for val in values {
            let bits_per_second = bits * (1e9 / *val);
            *val = bits_per_second / denominator;
        }

        unit
    }
}
impl ValueFormatter for DurationFormatter {
    fn scale_throughputs(
        &self,
        typical: f64,
        throughput: &Throughput,
        values: &mut [f64],
    ) -> &'static str {
        match *throughput {
            Throughput::Bytes(bytes) => self.bytes_per_second(bytes as f64, typical, values),
            Throughput::BytesDecimal(bytes) => {
                self.bytes_per_second_decimal(bytes as f64, typical, values)
            }
            Throughput::Elements(elems) => self.elements_per_second(elems as f64, typical, values),
            Throughput::Bits(bits) => self.bits_per_second(bits as f64, typical, values),
        }
    }

    fn scale_values(&self, ns: f64, values: &mut [f64]) -> &'static str {
        let (factor, unit) = if ns < pow(10f64, 0f64) {
            (pow(10f64, 3f64), "ps")
        } else if ns < pow(10f64, 3f64) {
            (pow(10f64, 0f64), "ns")
        } else if ns < pow(10f64, 6f64) {
            (pow(10f64, -3f64), "µs")
        } else if ns < pow(10f64, 9f64) {
            (pow(10f64, -6f64), "ms")
        } else {
            (pow(10f64, -9f64), "s")
        };

        for val in values {
            *val *= factor;
        }

        unit
    }

    fn scale_for_machines(&self, _values: &mut [f64]) -> &'static str {
        // no scaling is needed
        "ns"
    }
}

/// `WallTime` is the default measurement in Criterion.rs. It measures the elapsed time from the
/// beginning of a series of iterations to the end.
pub struct WallTime;

impl Measurement for WallTime {
    type Intermediate = Instant;

    fn start(&self) -> Self::Intermediate {
        Instant::now()
    }

    fn end(&self, i: Self::Intermediate) -> Duration {
        i.elapsed()
    }

    fn add(&self, v1: &Duration, v2: &Duration) -> Duration {
        *v1 + *v2
    }

    fn zero(&self) -> Duration {
        Duration::from_secs(0)
    }

    fn to_f64(&self, val: &Duration) -> f64 {
        val.as_nanos() as f64
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        &DurationFormatter
    }
}
