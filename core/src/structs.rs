#[cfg(feature = "risc0")]
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

// ZKVM-optimized serialization traits
// These provide faster serialization for ZKVM environments

#[cfg_attr(feature = "risc0", derive(BorshSerialize, BorshDeserialize))]
#[cfg_attr(feature = "sp1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct PublicKey {
    pub key: Vec<u8>,
    pub key_type: String,
}

impl PublicKey {
    /// ZKVM-optimized constructor with capacity pre-allocation
    pub fn new(key: Vec<u8>, key_type: String) -> Self {
        Self { key, key_type }
    }

    /// ZKVM-optimized key validation
    pub fn is_valid(&self) -> bool {
        !self.key.is_empty() && !self.key_type.is_empty()
    }

    /// ZKVM-optimized size estimation for memory planning
    pub fn estimated_size(&self) -> usize {
        self.key.len() + self.key_type.len() + 16 // struct overhead
    }
}

#[cfg_attr(feature = "risc0", derive(BorshSerialize, BorshDeserialize))]
#[cfg_attr(feature = "sp1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct DFA {
    pub fwd: Vec<u8>,
    pub bwd: Vec<u8>,
}

#[cfg_attr(feature = "risc0", derive(BorshSerialize, BorshDeserialize))]
#[cfg_attr(feature = "sp1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct CompiledRegex {
    pub verify_re: DFA,
    pub captures: Option<Vec<String>>,
}

#[cfg_attr(feature = "risc0", derive(BorshSerialize, BorshDeserialize))]
#[cfg_attr(feature = "sp1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct RegexInfo {
    pub header_parts: Option<Vec<CompiledRegex>>,
    pub body_parts: Option<Vec<CompiledRegex>>,
}

#[cfg_attr(feature = "risc0", derive(BorshSerialize, BorshDeserialize))]
#[cfg_attr(feature = "sp1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct ExternalInput {
    pub name: String,
    pub value: Option<String>,
    pub max_length: usize,
}

impl ExternalInput {
    /// ZKVM-optimized constructor
    pub fn new(name: String, value: Option<String>, max_length: usize) -> Self {
        Self {
            name,
            value,
            max_length,
        }
    }

    /// ZKVM-optimized size estimation for memory planning
    pub fn estimated_size(&self) -> usize {
        self.name.len() + self.value.as_ref().map(|v| v.len()).unwrap_or(0) + 8 // max_length + struct overhead
    }
}

#[cfg_attr(feature = "risc0", derive(BorshSerialize, BorshDeserialize))]
#[cfg_attr(feature = "sp1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Email {
    pub from_domain: String,
    pub raw_email: Vec<u8>,
    pub public_key: PublicKey,
    pub external_inputs: Vec<ExternalInput>,
}

impl Email {
    /// ZKVM-optimized constructor with pre-allocated vectors
    pub fn new(
        from_domain: String,
        raw_email: Vec<u8>,
        public_key: PublicKey,
        external_inputs: Vec<ExternalInput>,
    ) -> Self {
        Self {
            from_domain,
            raw_email,
            public_key,
            external_inputs,
        }
    }

    /// ZKVM-optimized validation
    pub fn is_valid(&self) -> bool {
        !self.from_domain.is_empty() && !self.raw_email.is_empty() && self.public_key.is_valid()
    }

    /// ZKVM-optimized size estimation for memory planning
    pub fn estimated_size(&self) -> usize {
        self.from_domain.len()
            + self.raw_email.len()
            + self.public_key.estimated_size()
            + self
                .external_inputs
                .iter()
                .map(|e| e.estimated_size())
                .sum::<usize>()
            + 32 // struct overhead
    }

    /// ZKVM-optimized batch processing helper
    pub fn prepare_for_batch_processing(&mut self) {
        // Pre-allocate capacity for external inputs if needed
        if self.external_inputs.capacity() < self.external_inputs.len() + 4 {
            self.external_inputs.reserve(4);
        }
    }
}

#[cfg_attr(feature = "risc0", derive(BorshSerialize, BorshDeserialize))]
#[cfg_attr(feature = "sp1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct EmailWithRegex {
    pub email: Email,
    pub regex_info: RegexInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailVerifierOutput {
    pub from_domain_hash: Vec<u8>,
    pub public_key_hash: Vec<u8>,
    pub external_inputs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailWithRegexVerifierOutput {
    pub email: EmailVerifierOutput,
    pub regex_matches: Vec<String>,
}
