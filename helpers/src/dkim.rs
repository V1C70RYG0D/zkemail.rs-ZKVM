use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use rsa::{pkcs1::EncodeRsaPublicKey, pkcs8::DecodePublicKey, RsaPublicKey};

/// ZKVM-compatible DKIM key format conversion
///
/// Converts PKCS#8 base64-encoded public keys to PKCS#1 format
/// suitable for ZKVM DKIM verification
pub fn convert_to_pkcs1(key_b64: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let pkcs8_der = STANDARD.decode(key_b64)?;
    let rsa_key = RsaPublicKey::from_public_key_der(&pkcs8_der)?;
    let pkcs1_der = rsa_key.to_pkcs1_der()?;
    Ok(pkcs1_der.as_bytes().to_vec())
}

/// ZKVM-compatible key validation
///
/// Validates that a key is in proper format for ZKVM processing
pub fn validate_key_format(key_bytes: &[u8], key_type: &str) -> bool {
    match key_type {
        "rsa" => {
            // Basic validation for PKCS#1 format
            if key_bytes.len() < 10 {
                return false;
            }
            // Check for SEQUENCE tag (0x30) at start
            key_bytes[0] == 0x30
        }
        "ed25519" => {
            // Ed25519 keys should be 32 bytes
            key_bytes.len() == 32
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_key_format_rsa() {
        // Valid PKCS#1 format (starts with SEQUENCE and sufficient length)
        let valid_key = vec![0x30, 0x48, 0x02, 0x41, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
        assert!(validate_key_format(&valid_key, "rsa"));

        // Invalid format (wrong start byte)
        let invalid_key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
        assert!(!validate_key_format(&invalid_key, "rsa"));

        // Too short
        let short_key = vec![0x30, 0x48];
        assert!(!validate_key_format(&short_key, "rsa"));
    }

    #[test]
    fn test_validate_key_format_ed25519() {
        // Valid Ed25519 key (32 bytes)
        let valid_key = vec![0u8; 32];
        assert!(validate_key_format(&valid_key, "ed25519"));

        // Invalid length
        let invalid_key = vec![0u8; 16];
        assert!(!validate_key_format(&invalid_key, "ed25519"));
    }

    #[test]
    fn test_validate_key_format_unknown() {
        let key = vec![0u8; 32];
        assert!(!validate_key_format(&key, "unknown"));
    }
}
