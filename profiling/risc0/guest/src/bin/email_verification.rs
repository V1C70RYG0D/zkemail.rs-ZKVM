use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSerialize};
use zkemail_core::{Email, verify_email};

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

fn main() {
    // Read input from host
    let input: EmailVerificationInput = env::read();
    
    let start_time = risc0_zkvm::guest::env::get_cycle_count();
    
    // Perform email verification using ZKVM-optimized implementation
    let verification_result = verify_email(&input.email);
    
    let end_time = risc0_zkvm::guest::env::get_cycle_count();
    let verification_time_ms = ((end_time - start_time) as f64 / 1000.0) as u64; // Approximate conversion
    
    let result = match verification_result {
        Ok(output) => EmailVerificationResult {
            is_valid: true,
            from_domain_hash: output.from_domain_hash,
            public_key_hash: output.public_key_hash,
            external_inputs: output.external_inputs,
            verification_time_ms,
        },
        Err(_) => EmailVerificationResult {
            is_valid: false,
            from_domain_hash: vec![],
            public_key_hash: vec![],
            external_inputs: vec![],
            verification_time_ms,
        }
    };
    
    // Write result to host
    env::commit(&result);
}
