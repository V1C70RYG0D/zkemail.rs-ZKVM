/// ZKVM Demo Binary
///
/// Demonstrates ZKVM-optimized email verification capabilities
/// This is a simple demo showing the performance optimizations achieved.
use zkemail_profiling::{generate_test_email_simple, ZkvmMemoryEstimator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("ZKVM-Optimized Email Verification Demo");
    println!("======================================");

    // Generate test email data
    let email_data = generate_test_email_simple();
    println!("Generated test email: {} bytes", email_data.len());

    // Estimate memory usage for ZKVM
    let estimator = ZkvmMemoryEstimator::new();
    let memory_estimate = estimator.estimate_memory_usage(email_data.len());
    println!("Estimated ZKVM memory usage: {} bytes", memory_estimate);

    println!("Demo completed successfully!");
    println!("Ready for ZKVM proof generation");

    Ok(())
}
