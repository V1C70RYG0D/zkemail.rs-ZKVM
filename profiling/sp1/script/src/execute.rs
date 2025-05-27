//! SP1 Execution Script (without proof generation)
//! 
//! This script executes the SP1 program for performance measurement
//! without the overhead of proof generation.

use sp1_sdk::{ProverClient, SP1Stdin};
use zkemail_profiling::create_test_email;

/// Include the program ELF
const ELF: &[u8] = include_bytes!("../../elf/zkmail-sp1-program");

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmailVerificationInput {
    pub email: zkemail_core::Email,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmailVerificationOutput {
    pub verification_result: zkemail_core::EmailVerifierOutput,
    pub is_valid: bool,
    pub cycle_count: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("SP1 zkEmail - Execution Mode");
    
    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();
    
    // Test with different email sizes
    let test_sizes = vec![1, 5, 10, 20, 50];
    
    for size in test_sizes {
        println!("\n--- Testing {}KB email ---", size);
        
        let test_email = create_test_email(size);
        let input = EmailVerificationInput { email: test_email };
        
        stdin.write(&input);
        
        let start_time = std::time::Instant::now();
        let (output, report) = client.execute(ELF, stdin.clone()).run()?;
        let execution_time = start_time.elapsed();
        
        let result = output.read::<EmailVerificationOutput>();
        
        println!("Email verification: {}", if result.is_valid { "VALID" } else { "INVALID" });
        println!("Cycles in zkVM: {}", result.cycle_count);
        println!("Total instructions: {}", report.total_instruction_count());
        println!("Total cycles: {}", report.total_cycles());
        println!("Host execution time: {:?}", execution_time);
        
        // Clear stdin for next iteration
        stdin = SP1Stdin::new();
    }
    
    Ok(())
}
