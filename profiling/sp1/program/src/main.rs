//! SP1 zkVM Guest Program for Email Verification
//! 
//! This program runs inside the SP1 zkVM and verifies email signatures
//! while providing zero-knowledge proofs of the verification process.

#![no_main]
sp1_zkvm::entrypoint!(main);

use serde::{Deserialize, Serialize};
use zkemail_core::*;

#[derive(Serialize, Deserialize)]
pub struct EmailVerificationInput {
    pub email: Email,
}

#[derive(Serialize, Deserialize)]
pub struct EmailVerificationOutput {
    pub verification_result: EmailVerifierOutput,
    pub is_valid: bool,
}

pub fn main() {
    // Read input from the host
    let input = sp1_zkvm::io::read::<EmailVerificationInput>();
    
    // Perform email verification
    let verification_result = match verify_email(&input.email) {
        Ok(result) => {
            EmailVerificationOutput {
                verification_result: result,
                is_valid: true,
            }
        }
        Err(_) => {
            EmailVerificationOutput {
                verification_result: EmailVerifierOutput {
                    from_domain_hash: vec![],
                    public_key_hash: vec![],
                    external_inputs: vec![],
                },
                is_valid: false,
            }
        }
    };
    
    // Commit the result
    sp1_zkvm::io::commit(&verification_result);
}
