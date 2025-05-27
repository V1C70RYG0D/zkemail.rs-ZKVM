/// ZKVM Integration Examples for zkemail.rs
///
/// This module provides examples of how to integrate zkemail.rs-zkvm
/// with actual ZKVM frameworks (RISC0 and SP1) for proof generation.

#[cfg(feature = "risc0")]
pub mod risc0_integration {
    use borsh::{BorshDeserialize, BorshSerialize};
    use zkemail_core::*;

    #[derive(BorshSerialize, BorshDeserialize)]
    pub struct EmailVerificationInput {
        pub email: Email,
    }

    #[derive(BorshSerialize, BorshDeserialize)]
    pub struct EmailVerificationOutput {
        pub verification_result: EmailVerifierOutput,
        pub is_valid: bool,
    }

    /// RISC0 guest program for email verification
    /// This would be compiled to RISC0 ELF and executed in the zkVM
    pub fn risc0_guest_main() {
        use risc0_zkvm::guest::env;

        let input: EmailVerificationInput = env::read();

        match verify_email(&input.email) {
            Ok(result) => {
                let output = EmailVerificationOutput {
                    verification_result: result,
                    is_valid: true,
                };
                env::commit(&output);
            }
            Err(_) => {
                let output = EmailVerificationOutput {
                    verification_result: EmailVerifierOutput {
                        from_domain_hash: vec![],
                        public_key_hash: vec![],
                        external_inputs: vec![],
                    },
                    is_valid: false,
                };
                env::commit(&output);
            }
        }
    }

    /// Example of how to generate a RISC0 proof for email verification
    #[cfg(not(target_arch = "wasm32"))]
    pub fn generate_risc0_proof(email: Email) -> Result<(), Box<dyn std::error::Error>> {
        use risc0_zkvm::{default_prover, ExecutorEnv};

        let input = EmailVerificationInput { email };

        let env = ExecutorEnv::builder().write(&input)?.build()?;

        // In practice, you would load the actual compiled guest ELF
        // let receipt = default_prover().prove(&env, GUEST_ELF)?;

        println!("RISC0 proof generation would happen here");
        Ok(())
    }
}

#[cfg(feature = "sp1")]
pub mod sp1_integration {
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
        pub cycle_count: u64,
    }
    /// SP1 proof generation for email verification
    #[cfg(not(target_arch = "wasm32"))]
    pub fn generate_sp1_proof(_email: Email) -> Result<SP1ProofResult, Box<dyn std::error::Error>> {
        println!("SP1 proof generation initiated...");
        println!("Note: This requires the SP1 program to be built first using:");
        println!("  cd profiling/sp1 && ./build.sh (or build.bat on Windows)");

        // In production with actual ELF and sp1_sdk:
        // use sp1_sdk::{ProverClient, SP1Stdin};
        // let client = ProverClient::new();
        // let mut stdin = SP1Stdin::new();
        // let input = EmailVerificationInput { email };
        // stdin.write(&input);
        // let (pk, vk) = client.setup(ELF);
        // let proof = client.prove(&pk, stdin)?;
        // let result = proof.public_values.read::<EmailVerificationOutput>();

        Ok(SP1ProofResult {
            is_valid: true,
            cycle_count: 500_000, // Placeholder cycle count
            proof_size: 192,      // Placeholder proof size
        })
    }

    /// Result of SP1 proof generation
    pub struct SP1ProofResult {
        pub is_valid: bool,
        pub cycle_count: u64,
        pub proof_size: usize,
    }
    /// Execute SP1 program without proof generation (for benchmarking)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn execute_sp1_program(
        _email: Email,
    ) -> Result<EmailVerificationOutput, Box<dyn std::error::Error>> {
        println!("SP1 execution initiated...");
        println!("Note: This requires the SP1 program to be built first");

        // In production with actual ELF and sp1_sdk:
        // use sp1_sdk::{ProverClient, SP1Stdin};
        // let client = ProverClient::new();
        // let mut stdin = SP1Stdin::new();
        // let input = EmailVerificationInput { email };
        // stdin.write(&input);
        // let (output, report) = client.execute(ELF, stdin).run()?;
        // let result = output.read::<EmailVerificationOutput>();

        // Placeholder result for now
        Ok(EmailVerificationOutput {
            verification_result: EmailVerifierOutput {
                from_domain_hash: vec![],
                public_key_hash: vec![],
                external_inputs: vec![],
            },
            is_valid: true,
            cycle_count: 500_000, // Placeholder
        })
    }
}

/// Common ZKVM performance measurement utilities
pub mod performance {
    use std::time::Instant;
    use zkemail_core::*;

    pub struct ZkvmPerformanceMetrics {
        pub operation: String,
        pub cycles_estimate: u64,
        pub memory_usage_bytes: u64,
        pub execution_time_ms: u64,
    }

    /// Measure performance characteristics of email verification
    /// for ZKVM constraint planning
    pub fn measure_email_verification_performance(email: &Email) -> ZkvmPerformanceMetrics {
        let start = Instant::now();

        // Estimate memory usage
        let memory_usage = estimate_memory_usage(email);

        // Estimate cycles based on operation complexity
        let cycles_estimate = estimate_verification_cycles(email);

        let execution_time = start.elapsed().as_millis() as u64;

        ZkvmPerformanceMetrics {
            operation: "email_verification".to_string(),
            cycles_estimate,
            memory_usage_bytes: memory_usage,
            execution_time_ms: execution_time,
        }
    }

    fn estimate_memory_usage(email: &Email) -> u64 {
        let base_size = std::mem::size_of::<Email>() as u64;
        let email_size = email.raw_email.len() as u64;
        let key_size = email.public_key.key.len() as u64;

        // ZKVM overhead estimation (conservative 3x multiplier)
        (base_size + email_size + key_size) * 3
    }

    fn estimate_verification_cycles(email: &Email) -> u64 {
        // Basic cycle estimation based on email size and complexity
        let base_cycles = 10_000u64; // Base verification overhead
        let email_size_cycles = email.raw_email.len() as u64 * 10; // ~10 cycles per byte
        let crypto_cycles = 50_000u64; // RSA signature verification
        let regex_cycles = email.external_inputs.len() as u64 * 1_000; // Regex processing

        base_cycles + email_size_cycles + crypto_cycles + regex_cycles
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_test_email;

    #[test]
    fn test_performance_estimation() {
        let email = create_test_email(10); // 10KB email
        let metrics = performance::measure_email_verification_performance(&email);

        assert!(metrics.cycles_estimate > 50_000);
        assert!(metrics.memory_usage_bytes > 10_000);
        assert_eq!(metrics.operation, "email_verification");
    }
}
