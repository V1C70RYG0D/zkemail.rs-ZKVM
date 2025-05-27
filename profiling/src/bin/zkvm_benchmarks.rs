#[cfg(any(feature = "risc0", feature = "sp1"))]
use zkemail_profiling::{create_test_email, ZkvmBenchmarkSuite};

#[cfg(any(feature = "risc0", feature = "sp1"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running ZKVM benchmarks for zkemail.rs-zkvm");

    let mut benchmark_suite = ZkvmBenchmarkSuite::new();

    // Test with different email sizes
    let email_sizes = vec![1, 10, 100]; // KB

    for size in email_sizes {
        println!("Benchmarking {}KB email...", size);

        let email = create_test_email(size);

        #[cfg(feature = "risc0")]
        {
            println!("  Running RISC0 benchmark...");
            if let Err(e) = benchmark_suite.benchmark_risc0_email_verification(&email) {
                eprintln!("RISC0 benchmark failed: {}", e);
            }
        }

        #[cfg(feature = "sp1")]
        {
            println!("  Running SP1 benchmark...");
            if let Err(e) = benchmark_suite.benchmark_sp1_email_verification(&email) {
                eprintln!("SP1 benchmark failed: {}", e);
            }
        }
    }

    // Generate and print report
    let report = benchmark_suite.generate_report();
    println!("\n{}", report);

    // Save results to file
    std::fs::write("zkvm_benchmark_results.md", report)?;
    println!("Results saved to zkvm_benchmark_results.md");

    Ok(())
}

#[cfg(not(any(feature = "risc0", feature = "sp1")))]
fn main() {
    println!("ZKVM benchmarks require either 'risc0' or 'sp1' feature to be enabled.");
    println!("Run with: cargo run --features risc0 or cargo run --features sp1");
}
