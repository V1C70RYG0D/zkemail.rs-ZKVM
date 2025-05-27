/// ZKVM Integration Benchmarks
///
/// This file contains examples of how to measure performance in ZKVM environments.
/// Unlike traditional Criterion benchmarks, these focus on ZKVM-specific metrics
/// like cycle counts and memory usage within actual ZKVM frameworks.

/// Example benchmark for RISC0 integration
#[cfg(feature = "risc0")]
mod risc0_benchmarks {
    use zkemail_core::Email;
    use zkemail_profiling::test_data::create_test_email;
    use zkemail_profiling::zkvm_integration::risc0_integration::*;

    pub fn benchmark_risc0_email_verification() {
        let test_email = create_test_email(10); // 10KB test email

        println!("RISC0 Email Verification Benchmark");
        println!("Email size: {}KB", test_email.raw_email.len() / 1024);

        let metrics = zkemail_profiling::zkvm_integration::performance::measure_email_verification_performance(&test_email);

        println!("Estimated cycles: {}", metrics.cycles_estimate);
        println!("Estimated memory: {} bytes", metrics.memory_usage_bytes);
        println!("Host execution time: {}ms", metrics.execution_time_ms);

        // In a real scenario, you would generate an actual RISC0 proof here
        if let Err(e) = generate_risc0_proof(test_email) {
            eprintln!("RISC0 proof generation failed: {}", e);
        }
    }
}

/// Example benchmark for SP1 integration
#[cfg(feature = "sp1")]
mod sp1_benchmarks {
    use zkemail_core::Email;
    use zkemail_profiling::test_data::create_test_email;
    use zkemail_profiling::zkvm_integration::sp1_integration::*;

    pub fn benchmark_sp1_email_verification() {
        let test_email = create_test_email(10); // 10KB test email

        println!("SP1 Email Verification Benchmark");
        println!("Email size: {}KB", test_email.raw_email.len() / 1024);

        let metrics = zkemail_profiling::zkvm_integration::performance::measure_email_verification_performance(&test_email);

        println!("Estimated cycles: {}", metrics.cycles_estimate);
        println!("Estimated memory: {} bytes", metrics.memory_usage_bytes);
        println!("Host execution time: {}ms", metrics.execution_time_ms);

        // In a real scenario, you would generate an actual SP1 proof here
        if let Err(e) = generate_sp1_proof(test_email) {
            eprintln!("SP1 proof generation failed: {}", e);
        }
    }
}

fn main() {
    println!("ZKVM Performance Examples for zkemail.rs-zkvm");
    println!("==============================================");

    #[cfg(feature = "risc0")]
    {
        println!("\nRunning RISC0 benchmarks...");
        risc0_benchmarks::benchmark_risc0_email_verification();
    }

    #[cfg(feature = "sp1")]
    {
        println!("\nRunning SP1 benchmarks...");
        sp1_benchmarks::benchmark_sp1_email_verification();
    }

    #[cfg(not(any(feature = "risc0", feature = "sp1")))]
    {
        println!("No ZKVM features enabled. Run with:");
        println!("cargo run --features risc0");
        println!("cargo run --features sp1");
    }

    println!("\nNote: These are estimation examples. Actual ZKVM proof generation");
    println!("requires compiled guest programs and proper ZKVM setup.");
}
