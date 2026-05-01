//! Performance benchmarks for PeTTa
//!
//! This module provides benchmarking utilities to measure performance
//! of various operations in the PeTTa runtime.

use std::time::{Duration, Instant};

/// Benchmark result with statistics
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: &'static str,
    pub iterations: u64,
    pub total_time: Duration,
    pub avg_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
}

impl BenchmarkResult {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            iterations: 0,
            total_time: Duration::ZERO,
            avg_time: Duration::ZERO,
            min_time: Duration::MAX,
            max_time: Duration::ZERO,
        }
    }

    pub fn record(&mut self, duration: Duration) {
        self.iterations += 1;
        self.total_time += duration;
        self.avg_time = self.total_time / self.iterations as u32;
        if duration < self.min_time {
            self.min_time = duration;
        }
        if duration > self.max_time {
            self.max_time = duration;
        }
    }

    pub fn ops_per_sec(&self) -> f64 {
        if self.total_time.as_secs_f64() > 0.0 {
            self.iterations as f64 / self.total_time.as_secs_f64()
        } else {
            0.0
        }
    }
}

impl std::fmt::Display for BenchmarkResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} iterations, avg: {:?}, min: {:?}, max: {:?}, ops/sec: {:.2}",
            self.name,
            self.iterations,
            self.avg_time,
            self.min_time,
            self.max_time,
            self.ops_per_sec()
        )
    }
}

/// Run a benchmark with warmup
pub fn benchmark<F>(name: &'static str, warmup: u64, iterations: u64, mut f: F) -> BenchmarkResult
where
    F: FnMut(),
{
    let mut result = BenchmarkResult::new(name);

    // Warmup phase
    for _ in 0..warmup {
        f();
    }

    // Benchmark phase
    for _ in 0..iterations {
        let start = Instant::now();
        f();
        result.record(start.elapsed());
    }

    result
}

/// Benchmark with setup and cleanup
pub fn benchmark_with_setup<F, G, H>(
    name: &'static str,
    warmup: u64,
    iterations: u64,
    setup: F,
    mut f: G,
    cleanup: H,
) -> BenchmarkResult
where
    F: Fn() -> (),
    G: FnMut(),
    H: Fn(),
{
    let mut result = BenchmarkResult::new(name);

    // Warmup
    for _ in 0..warmup {
        setup();
        f();
        cleanup();
    }

    // Benchmark
    for _ in 0..iterations {
        setup();
        let start = Instant::now();
        f();
        let duration = start.elapsed();
        cleanup();
        result.record(duration);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_basic() {
        let result = benchmark("test", 10, 100, || {
            let _ = 2 + 2;
        });
        assert_eq!(result.name, "test");
        assert_eq!(result.iterations, 100);
    }

    #[test]
    fn test_benchmark_with_setup() {
        let mut counter = 0;
        let result = benchmark_with_setup(
            "test_with_setup",
            10,
            100,
            || {
                counter = 0;
            },
            || {
                counter += 1;
            },
            || {},
        );
        assert_eq!(result.name, "test_with_setup");
        assert_eq!(result.iterations, 100);
    }
}
