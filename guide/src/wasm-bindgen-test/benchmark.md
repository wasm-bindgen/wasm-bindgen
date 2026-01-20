# Benchmark

You can now code wasm benchmarks in the same way you code write them for other platforms.
We modified the [criterion](https://github.com/bheisler/criterion.rs) to support wasm while retaining
only some basic bench capabilities.
Thanks to the criterion, we were able to obtain stable, accurate, and reliable benching results!

I'll assume that we have a crate, `mycrate`, whose `lib.rs` contains the following code:

```rust
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
```

### Step 1 - Add Dependency to Cargo.toml ###

To enable benchmarks, add the following to your `Cargo.toml` file:

```toml
[dev-dependencies]
wasm-bindgen-test = "0.3.56"
```

### Step 2 - Add Benchmark ###

As an example, we'll benchmark our implementation of the Fibonacci function. Create a benchmark
file at `$PROJECT/benches/my_benchmark.rs` with the following contents:

```rust
use wasm_bindgen_test::{wasm_bindgen_bench, Criterion};
use mycrate::fibonacci;

#[wasm_bindgen_bench]
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(20)));
}
```

### Step 3 - Run Benchmark ###

To run this benchmark, use the following commands:

```sh
export CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner
cargo bench --target wasm32-unknown-unknown
```

You should see output similar to this:

```
     Running benches/my_benchmark.rs (target/wasm32-unknown-unknown/release/deps/my_benchmark-18dc40eb54e84f66.wasm)
Warming up for 3.0000 s
Collecting 100 samples in estimated 5.0216 s (545k iterations)
fib 20                  time:   [9.1197 µs 9.1268 µs 9.1339 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```

### Step 4 - Optimize ###

This fibonacci function is quite inefficient. We can do better:

```rust
pub fn fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}
```

Running the benchmark now produces output like this:

```
     Running benches/my_benchmark.rs (target/wasm32-unknown-unknown/release/deps/my_benchmark-18dc40eb54e84f66.wasm)
Warming up for 3.0000 s
Collecting 100 samples in estimated 5.0000 s (3.2B iterations)
fib 20                  time:   [1.5432 ns 1.5434 ns 1.5438 ns]
                        change: [−99.983% −99.983% −99.983%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  7 (7.00%) high severe
```

As you can see, Criterion is statistically confident that our optimization has made an improvement.
If we introduce a performance regression, Criterion will instead print a message indicating this.

### Step 5 - Asynchronous function ###

It also supports benchmarking asynchronous functions:

```rust
use wasm_bindgen_test::{wasm_bindgen_bench, Criterion};

#[wasm_bindgen_bench]
async fn bench(c: &mut Criterion) {
    c.bench_async_function(
        "bench desc",
         Box::pin(
             b.iter_future(|| async {
                 // Code to benchmark goes here
             })
         )
    ).await;
}
```

### Step 6 - Run benchmark in browser ###

Similar to test, you can use `wasm_bindgen_test_configure!` to configure the execution environment.

### Step 7 - Configuration ###

* `WASM_BINDGEN_BENCH_RESULT`: Path for the custom benchmark result file.
