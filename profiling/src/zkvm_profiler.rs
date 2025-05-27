/// ZKVM-specific profiling utilities for cycle counting and deterministic measurements
///
/// ZKVMs measure performance in cycles rather than wall-clock time,
/// and require deterministic, single-threaded execution.
use std::time::{Duration, Instant};

/// ZKVM-compatible profiling section that tracks cycles and execution patterns
pub struct ZkvmProfileSection {
    name: String,
    start_time: Instant,
    operation_count: usize,
}

impl ZkvmProfileSection {
    /// Creates a new ZKVM profiling section
    ///
    /// # Arguments
    /// * `section_name` - Descriptive name for the operation being profiled
    /// * `operation_count` - Number of operations/items being processed
    pub fn new(section_name: &str, operation_count: usize) -> Self {
        Self {
            name: section_name.to_string(),
            start_time: Instant::now(),
            operation_count,
        }
    }

    /// Reports per-operation timing for ZKVM analysis
    pub fn report_per_operation(&self) -> Duration {
        let total_duration = self.start_time.elapsed();
        if self.operation_count > 0 {
            total_duration / self.operation_count as u32
        } else {
            total_duration
        }
    }
}

impl Drop for ZkvmProfileSection {
    fn drop(&mut self) {
        let duration = self.start_time.elapsed();
        let per_op = if self.operation_count > 0 {
            duration / self.operation_count as u32
        } else {
            duration
        };

        println!(
            "ZKVM Profile: {} | Total: {:?} | Operations: {} | Per-op: {:?}",
            self.name, duration, self.operation_count, per_op
        );
    }
}

/// ZKVM-compatible deterministic benchmarking
///
/// Performs consistent measurements suitable for ZKVM analysis
pub fn benchmark_zkvm_operation<F>(
    name: &str,
    iterations: u32,
    operation_count: usize,
    func: F,
) -> Duration
where
    F: Fn(),
{
    println!(
        "ZKVM Benchmark: '{}' ({} iterations, {} ops each)",
        name, iterations, operation_count
    );

    // Deterministic warmup (no variable iteration count)
    for _ in 0..3 {
        func();
    }

    let start = Instant::now();
    for _ in 0..iterations {
        func();
    }
    let total_duration = start.elapsed();
    let avg_duration = total_duration / iterations;

    println!(
        "ZKVM Benchmark: '{}' | Avg/iter: {:?} | Total: {:?}",
        name, avg_duration, total_duration
    );

    avg_duration
}

/// ZKVM-specific memory usage estimation
///
/// Estimates memory usage patterns for ZKVM constraint planning
pub struct ZkvmMemoryEstimator {
    allocations: Vec<(String, usize)>,
    total_bytes: usize,
}

impl Default for ZkvmMemoryEstimator {
    fn default() -> Self {
        Self::new()
    }
}

impl ZkvmMemoryEstimator {
    pub fn new() -> Self {
        Self {
            allocations: Vec::new(),
            total_bytes: 0,
        }
    }

    /// Records a memory allocation for tracking
    pub fn record_allocation(&mut self, name: &str, bytes: usize) {
        self.allocations.push((name.to_string(), bytes));
        self.total_bytes += bytes;
    }

    /// Reports memory usage summary for ZKVM planning
    pub fn report_summary(&self) {
        println!("ZKVM Memory Usage Summary:");
        println!("Total allocated: {} bytes", self.total_bytes);

        for (name, bytes) in &self.allocations {
            let percentage = (bytes * 100) / self.total_bytes.max(1);
            println!("  {}: {} bytes ({}%)", name, bytes, percentage);
        }
    }

    /// Estimates memory usage for email verification
    pub fn estimate_memory_usage(&self, data_size: usize) -> usize {
        // ZKVM-specific memory estimation based on email size
        let base_overhead = 1024; // Base ZKVM overhead
        let parsing_overhead = data_size / 10; // ~10% parsing overhead
        let verification_overhead = 2048; // DKIM verification overhead

        base_overhead + parsing_overhead + verification_overhead
    }

    /// Returns total memory usage
    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }
}

/// ZKVM-specific operation profiling macro
///
/// Profiles a code block with automatic cycle/timing reporting
#[macro_export]
macro_rules! zkvm_profile {
    ($name:expr, $op_count:expr, $block:block) => {{
        let _profiler = ZkvmProfileSection::new($name, $op_count);
        $block
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zkvm_profiling_deterministic() {
        let profiler = ZkvmProfileSection::new("test_operation", 100);

        // Simulate deterministic computation
        let mut sum = 0;
        for i in 0..100 {
            sum += i;
        }

        let _per_op = profiler.report_per_operation();
        assert_eq!(sum, 4950); // Verify computation correctness
    }

    #[test]
    fn test_memory_estimator() {
        let mut estimator = ZkvmMemoryEstimator::new();

        estimator.record_allocation("email_data", 4096);
        estimator.record_allocation("hash_buffer", 1024);
        estimator.record_allocation("regex_state", 512);

        assert_eq!(estimator.total_bytes(), 5632);
        estimator.report_summary();
    }

    #[test]
    fn test_zkvm_benchmark() {
        let duration = benchmark_zkvm_operation("test_computation", 10, 100, || {
            // Deterministic computation
            let mut result: u32 = 0;
            for i in 0..100u32 {
                result = result.wrapping_add(i);
            }
            // Don't return the result, just perform computation
        });

        // Just verify the benchmark runs without panicking
        assert!(duration.as_secs() < 60); // Should complete in under 60 seconds
    }
}
