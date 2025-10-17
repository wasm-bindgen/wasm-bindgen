use alloc::format;
use alloc::string::String;
use libm::{fabs, pow};

pub fn change(pct: f64, signed: bool) -> String {
    if signed {
        format!("{:>+6}%", signed_short(pct * 1e2))
    } else {
        format!("{:>6}%", short(pct * 1e2))
    }
}

pub fn time(ns: f64) -> String {
    if ns < 1.0 {
        format!("{:>6} ps", short(ns * 1e3))
    } else if ns < pow(10f64, 3f64) {
        format!("{:>6} ns", short(ns))
    } else if ns < pow(10f64, 6f64) {
        format!("{:>6} µs", short(ns / 1e3))
    } else if ns < pow(10f64, 9f64) {
        format!("{:>6} ms", short(ns / 1e6))
    } else {
        format!("{:>6} s", short(ns / 1e9))
    }
}

pub fn short(n: f64) -> String {
    if n < 10.0 {
        format!("{n:.4}")
    } else if n < 100.0 {
        format!("{n:.3}")
    } else if n < 1000.0 {
        format!("{n:.2}")
    } else if n < 10000.0 {
        format!("{n:.1}")
    } else {
        format!("{n:.0}")
    }
}

fn signed_short(n: f64) -> String {
    let n_abs = fabs(n);

    let sign = if n >= 0.0 { '+' } else { '\u{2212}' };
    if n_abs < 10.0 {
        format!("{sign}{n_abs:.4}")
    } else if n_abs < 100.0 {
        format!("{sign}{n_abs:.3}")
    } else if n_abs < 1000.0 {
        format!("{sign}{n_abs:.2}")
    } else if n_abs < 10000.0 {
        format!("{sign}{n_abs:.1}")
    } else {
        format!("{sign}{n_abs:.0}")
    }
}

pub fn iter_count(iterations: u64) -> String {
    if iterations < 10_000 {
        format!("{iterations} iterations")
    } else if iterations < 1_000_000 {
        format!("{:.0}k iterations", (iterations as f64) / 1000.0)
    } else if iterations < 10_000_000 {
        format!("{:.1}M iterations", (iterations as f64) / (1000.0 * 1000.0))
    } else if iterations < 1_000_000_000 {
        format!("{:.0}M iterations", (iterations as f64) / (1000.0 * 1000.0))
    } else if iterations < 10_000_000_000 {
        format!(
            "{:.1}B iterations",
            (iterations as f64) / (1000.0 * 1000.0 * 1000.0)
        )
    } else {
        format!(
            "{:.0}B iterations",
            (iterations as f64) / (1000.0 * 1000.0 * 1000.0)
        )
    }
}
