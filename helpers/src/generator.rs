use zkemail_core::*;

/// ZKVM-compatible email data generator
///
/// Generates email structures suitable for ZKVM processing
/// without any async or network dependencies
pub fn generate_email_inputs(
    domain: &str,
    raw_email: Vec<u8>,
    public_key_bytes: Vec<u8>,
    key_type: &str,
    external_inputs: Vec<ExternalInput>,
) -> Email {
    Email {
        from_domain: domain.to_string(),
        raw_email,
        public_key: PublicKey {
            key: public_key_bytes,
            key_type: key_type.to_string(),
        },
        external_inputs,
    }
}

/// ZKVM-compatible email with regex generator
///
/// Generates EmailWithRegex structures for ZKVM regex processing
pub fn generate_email_with_regex_inputs(
    email: Email,
    header_regexes: Option<Vec<CompiledRegex>>,
    body_regexes: Option<Vec<CompiledRegex>>,
) -> EmailWithRegex {
    EmailWithRegex {
        email,
        regex_info: RegexInfo {
            header_parts: header_regexes,
            body_parts: body_regexes,
        },
    }
}

/// ZKVM-compatible DFA generator for testing
///
/// Creates simple DFA patterns for ZKVM regex testing
pub fn generate_test_dfa() -> DFA {
    DFA {
        fwd: vec![
            // Simple DFA bytes for testing
            0x30, 0x10, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        bwd: vec![
            // Reverse DFA bytes for testing
            0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x10, 0x30,
        ],
    }
}

/// ZKVM-compatible regex compilation for testing
///
/// Creates CompiledRegex structures for ZKVM testing
pub fn generate_test_compiled_regex(captures: Option<Vec<String>>) -> CompiledRegex {
    CompiledRegex {
        verify_re: generate_test_dfa(),
        captures,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_email_inputs() {
        let email = generate_email_inputs(
            "example.com",
            b"test email".to_vec(),
            vec![0u8; 64],
            "rsa",
            vec![ExternalInput {
                name: "test".to_string(),
                value: Some("value".to_string()),
                max_length: 100,
            }],
        );

        assert_eq!(email.from_domain, "example.com");
        assert_eq!(email.raw_email, b"test email");
        assert_eq!(email.public_key.key_type, "rsa");
        assert_eq!(email.external_inputs.len(), 1);
    }

    #[test]
    fn test_generate_email_with_regex_inputs() {
        let base_email = generate_email_inputs(
            "example.com",
            b"test".to_vec(),
            vec![0u8; 32],
            "rsa",
            vec![],
        );

        let regex_email = generate_email_with_regex_inputs(
            base_email,
            Some(vec![generate_test_compiled_regex(None)]),
            None,
        );

        assert!(regex_email.regex_info.header_parts.is_some());
        assert!(regex_email.regex_info.body_parts.is_none());
    }

    #[test]
    fn test_generate_test_dfa() {
        let dfa = generate_test_dfa();
        assert_eq!(dfa.fwd.len(), 16);
        assert_eq!(dfa.bwd.len(), 16);
    }
}
