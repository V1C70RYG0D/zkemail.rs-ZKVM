/// ZKVM-specific profiling and optimization tools for zkemail.rs
///
/// This crate provides ZKVM-compatible profiling utilities, test data generators,
/// and memory profiling tools specifically designed for zero-knowledge virtual machines.
///
/// ## Features
///
/// - **ZKVM Profiling**: Cycle counting and deterministic measurements
/// - **Memory Estimation**: ZKVM constraint planning and usage tracking
/// - **Test Data Generation**: Realistic email data for ZKVM testing
/// - **ZKVM Integration**: Examples for RISC0 and SP1 integration
pub mod memory_profiler;
pub mod test_data;
pub mod zkvm_benchmarks;
pub mod zkvm_integration;
pub mod zkvm_profiler;

// Re-export main APIs for convenience
pub use memory_profiler::{
    profile_memory_usage, setup_memory_profiler, start_memory_profiling, MemoryProfileSection,
};
pub use test_data::*;
pub use zkvm_benchmarks::{ZkvmBenchmarkResult, ZkvmBenchmarkSuite};
pub use zkvm_integration::performance::ZkvmPerformanceMetrics;
pub use zkvm_profiler::{benchmark_zkvm_operation, ZkvmMemoryEstimator, ZkvmProfileSection};

// Note: zkvm_profile macro is automatically exported at crate root
