use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSerialize};
use zkemail_core::{Email, PublicKey, ExternalInput};
use zkemail_profiling::test_data::create_test_email;
use methods::{EMAIL_VERIFICATION_ELF, EMAIL_VERIFICATION_ID};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
struct EmailVerificationInput {
    email: Email,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
struct EmailVerificationResult {
    is_valid: bool,
    from_domain_hash: Vec<u8>,
    public_key_hash: Vec<u8>,
    external_inputs: Vec<String>,
    verification_time_ms: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a test email for verification
    let test_email = create_test_email(10); // 10KB test email
    
    let input = EmailVerificationInput {
        email: test_email,
    };
    
    println!("Starting RISC0 email verification...");
    
    // Create executor environment and add input
    let env = ExecutorEnv::builder()
        .write(&input)?
        .build()?;
    
    // Create a prover
    let prover = default_prover();
    
    // Generate proof
    let start = std::time::Instant::now();
    let receipt = prover.prove(env, EMAIL_VERIFICATION_ELF)?;
    let proof_generation_time = start.elapsed();
    
    println!("Proof generated in: {:?}", proof_generation_time);
    
    // Verify the receipt
    receipt.verify(EMAIL_VERIFICATION_ID)?;
    println!("Receipt verified successfully!");
    
    // Extract the result
    let result: EmailVerificationResult = receipt.journal.decode()?;
    
    println!("Email verification result:");
    println!("  Valid: {}", result.is_valid);
    println!("  Domain hash: {:?}", result.from_domain_hash);
    println!("  Public key hash: {:?}", result.public_key_hash);
    println!("  External inputs: {:?}", result.external_inputs);
    println!("  Verification time (cycles): {} ms", result.verification_time_ms);
    println!("  Total proof time: {:?}", proof_generation_time);
    
    Ok(())
}
