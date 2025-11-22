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
        format!("{:.4}", n)
    } else if n < 100.0 {
        format!("{:.3}", n)
    } else if n < 1000.0 {
        format!("{:.2}", n)
    } else if n < 10000.0 {
        format!("{:.1}", n)
    } else {
        format!("{:.0}", n)
    }
}

fn signed_short(n: f64) -> String {
    let n_abs = fabs(n);

    let sign = if n >= 0.0 { '+' } else { '\u{2212}' };
    if n_abs < 10.0 {
        format!("{}{:.4}", sign, n_abs)
    } else if n_abs < 100.0 {
        format!("{}{:.3}", sign, n_abs)
    } else if n_abs < 1000.0 {
        format!("{}{:.2}", sign, n_abs)
    } else if n_abs < 10000.0 {
        format!("{}{:.1}", sign, n_abs)
    } else {
        format!("{}{:.0}", sign, n_abs)
    }
}

pub fn iter_count(iterations: u64) -> String {
    if iterations < 10_000 {
        format!("{} iterations", iterations)
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
