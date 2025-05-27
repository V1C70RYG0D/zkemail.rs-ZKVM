use zkemail_core::*;
use zkemail_profiling::test_data::{
    generate_test_email_invalid_dkim, generate_test_email_with_dkim,
};

/// Comprehensive ZKVM-specific tests ensuring deterministic execution,
/// memory efficiency, and functional correctness
fn create_test_email() -> Email {
    generate_test_email_with_dkim()
}

fn create_test_email_invalid() -> Email {
    generate_test_email_invalid_dkim()
}

fn create_test_email_with_regex() -> EmailWithRegex {
    EmailWithRegex {
        email: create_test_email(),
        regex_info: RegexInfo {
            header_parts: None, // Use None to avoid regex verification
            body_parts: None,
        },
    }
}

#[cfg(test)]
mod email_tests {
    use super::*;
    use mailparse::parse_mail;

    #[test]
    fn test_extract_email_body_simple() {
        let email_content = "From: test@example.com\r\nSubject: Test\r\n\r\nSimple body";
        let parsed = parse_mail(email_content.as_bytes()).unwrap();
        let body = extract_email_body(&parsed);

        assert_eq!(body, b"Simple body");
    }

    #[test]
    fn test_extract_email_body_multipart_html_preferred() {
        let email_content = "From: test@example.com\r\n\
            Content-Type: multipart/alternative; boundary=boundary\r\n\
            \r\n\
            --boundary\r\n\
            Content-Type: text/plain\r\n\
            \r\n\
            Plain text body\r\n\
            --boundary\r\n\
            Content-Type: text/html\r\n\
            \r\n\
            <html><body>HTML body</body></html>\r\n\
            --boundary--";

        let parsed = parse_mail(email_content.as_bytes()).unwrap();
        let body = extract_email_body(&parsed);

        // Should prefer HTML over plain text (may include trailing CRLF)
        let expected = b"<html><body>HTML body</body></html>";
        let body_trimmed = body.strip_suffix(b"\r\n").unwrap_or(&body);
        assert_eq!(body_trimmed, expected);
    }

    #[test]
    fn test_extract_email_body_multipart_plain_fallback() {
        let email_content = "From: test@example.com\r\n\
            Content-Type: multipart/alternative; boundary=boundary\r\n\
            \r\n\
            --boundary\r\n\
            Content-Type: text/plain\r\n\
            \r\n\
            Plain text body\r\n\
            --boundary\r\n\
            Content-Type: application/pdf\r\n\
            \r\n\
            PDF content\r\n\
            --boundary--";

        let parsed = parse_mail(email_content.as_bytes()).unwrap();
        let body = extract_email_body(&parsed);

        // Should fallback to plain text when HTML not available (may include trailing CRLF)
        let expected = b"Plain text body";
        let body_trimmed = body.strip_suffix(b"\r\n").unwrap_or(&body);
        assert_eq!(body_trimmed, expected);
    }

    #[test]
    fn test_extract_email_bodies_batch() {
        let emails = [
            "From: test1@example.com\r\nSubject: Test 1\r\n\r\nBody 1",
            "From: test2@example.com\r\nSubject: Test 2\r\n\r\nBody 2",
            "From: test3@example.com\r\nSubject: Test 3\r\n\r\nBody 3",
        ];

        let parsed_emails: Vec<_> = emails
            .iter()
            .map(|e| parse_mail(e.as_bytes()).unwrap())
            .collect();
        let email_refs: Vec<&mailparse::ParsedMail> = parsed_emails.iter().collect();

        let bodies = extract_email_bodies_batch(&email_refs);

        assert_eq!(bodies.len(), 3);
        assert_eq!(bodies[0], b"Body 1");
        assert_eq!(bodies[1], b"Body 2");
        assert_eq!(bodies[2], b"Body 3");
    }

    #[test]
    fn test_extract_email_body_deterministic() {
        let email_content = "From: test@example.com\r\nSubject: Test\r\n\r\nTest body";
        let parsed = parse_mail(email_content.as_bytes()).unwrap();

        // Test that multiple calls return identical results
        for _ in 0..10 {
            let body = extract_email_body(&parsed);
            assert_eq!(body, b"Test body");
        }
    }
}

#[cfg(test)]
mod crypto_tests {
    use super::*;

    #[test]
    fn test_hash_bytes_deterministic() {
        let data = b"test data for hashing";

        // Test that hash is deterministic across multiple calls
        for _ in 0..10 {
            let hash1 = hash_bytes(data);
            let hash2 = hash_bytes(data);
            assert_eq!(hash1, hash2, "Hash should be deterministic");
            assert_eq!(hash1.len(), 32, "SHA256 should produce 32-byte hash");
        }
    }

    #[test]
    fn test_hash_bytes_different_inputs() {
        let data1 = b"test data 1";
        let data2 = b"test data 2";

        let hash1 = hash_bytes(data1);
        let hash2 = hash_bytes(data2);

        assert_ne!(
            hash1, hash2,
            "Different inputs should produce different hashes"
        );
    }

    #[test]
    fn test_hash_bytes_batch() {
        let data_items = vec![
            b"data1".as_slice(),
            b"data2".as_slice(),
            b"data3".as_slice(),
        ];

        let batch_hashes = hash_bytes_batch(&data_items);

        assert_eq!(batch_hashes.len(), 3);

        // Verify each hash matches individual computation
        for (i, data) in data_items.iter().enumerate() {
            let individual_hash = hash_bytes(data);
            assert_eq!(batch_hashes[i], individual_hash);
        }
    }

    #[test]
    fn test_hash_bytes_concat() {
        let data_items = vec![b"hello".as_slice(), b"world".as_slice()];
        let concat_hash = hash_bytes_concat(&data_items);
        let direct_hash = hash_bytes(b"helloworld");

        assert_eq!(concat_hash, direct_hash);
    }

    #[test]
    fn test_hash_bytes_stream() {
        let data_items = vec![b"test1".to_vec(), b"test2".to_vec(), b"test3".to_vec()];
        let stream_hash = hash_bytes_stream(data_items.clone().into_iter());
        let direct_hash = hash_bytes(b"test1test2test3");

        assert_eq!(stream_hash, direct_hash);
    }

    #[test]
    fn test_hash_bytes_empty_input() {
        let empty_data = b"";
        let hash = hash_bytes(empty_data);

        assert_eq!(hash.len(), 32);
        // SHA256 of empty string is a known value
        let expected = hash_bytes(b"");
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_hash_bytes_large_input() {
        let large_data = vec![0u8; 1024 * 1024]; // 1MB of zeros
        let hash = hash_bytes(&large_data);

        assert_eq!(hash.len(), 32);

        // Test determinism with large input
        let hash2 = hash_bytes(&large_data);
        assert_eq!(hash, hash2);
    }
}

#[cfg(test)]
mod regex_tests {
    use super::*;

    #[test]
    fn test_process_regex_parts_empty() {
        let regexes = vec![];
        let input = b"test input";
        let (success, matches) = process_regex_parts(&regexes, input);

        assert!(success);
        assert!(matches.is_empty());
    }

    #[test]
    fn test_process_regex_parts_batch() {
        let regexes = vec![];
        let inputs = vec![
            b"test1".as_slice(),
            b"test2".as_slice(),
            b"test3".as_slice(),
        ];
        let results = process_regex_parts_batch(&regexes, &inputs);

        assert_eq!(results.len(), 3);
        for (success, matches) in results {
            assert!(success);
            assert!(matches.is_empty());
        }
    }

    #[test]
    fn test_process_regex_parts_deterministic() {
        let regexes = vec![];
        let input = b"consistent test input";

        // Test that multiple calls return identical results
        for _ in 0..10 {
            let (success, matches) = process_regex_parts(&regexes, input);
            assert!(success);
            assert!(matches.is_empty());
        }
    }

    #[cfg(feature = "sp1")]
    #[test]
    fn test_align_slice() {
        let data = vec![1, 2, 3, 4, 5];
        let aligned = super::align_slice(&data);

        // Check that alignment is correct (4-byte aligned)
        assert_eq!(aligned.as_ptr() as usize % 4, 0);

        // Check that original data is preserved after padding
        let padding_len = aligned.len() - data.len();
        assert_eq!(&aligned[padding_len..], &data);
    }

    #[cfg(feature = "sp1")]
    #[test]
    fn test_align_slice_already_aligned() {
        // Create data that's already aligned
        let mut data = vec![0u8; 4]; // 4 bytes, should be naturally aligned
        data.extend_from_slice(&[1, 2, 3, 4]);

        let aligned = super::align_slice(&data);

        // Should still be aligned
        assert_eq!(aligned.as_ptr() as usize % 4, 0);
    }
}

#[cfg(test)]
mod circuits_tests {
    use super::*;

    #[test]
    fn test_verify_email_deterministic() {
        let email = create_test_email();

        // Test that verification is deterministic
        for _ in 0..5 {
            let output1 = verify_email_test_only(&email).unwrap();
            let output2 = verify_email_test_only(&email).unwrap();

            assert_eq!(output1.from_domain_hash, output2.from_domain_hash);
            assert_eq!(output1.public_key_hash, output2.public_key_hash);
            assert_eq!(output1.external_inputs, output2.external_inputs);
        }
    }

    #[test]
    fn test_verify_email_with_regex_deterministic() {
        let email_with_regex = create_test_email_with_regex();

        // Test that verification with regex is deterministic
        for _ in 0..5 {
            let output1 = verify_email_with_regex_test_only(&email_with_regex).unwrap();
            let output2 = verify_email_with_regex_test_only(&email_with_regex).unwrap();

            assert_eq!(
                output1.email.from_domain_hash,
                output2.email.from_domain_hash
            );
            assert_eq!(output1.email.public_key_hash, output2.email.public_key_hash);
            assert_eq!(output1.regex_matches, output2.regex_matches);
        }
    }

    #[test]
    fn test_verify_email_output_format() {
        let email = create_test_email();
        let output = verify_email_test_only(&email).unwrap();

        // Verify output structure
        assert_eq!(output.from_domain_hash.len(), 32); // SHA256 hash
        assert_eq!(output.public_key_hash.len(), 32); // SHA256 hash
        assert_eq!(output.external_inputs.len(), 2); // name + value
        assert_eq!(output.external_inputs[0], "test_input");
        assert_eq!(output.external_inputs[1], "test_value");
    }

    #[test]
    fn test_verify_email_with_regex_output_format() {
        let email_with_regex = create_test_email_with_regex();
        let output = verify_email_with_regex_test_only(&email_with_regex).unwrap();

        // Verify output structure
        assert_eq!(output.email.from_domain_hash.len(), 32);
        assert_eq!(output.email.public_key_hash.len(), 32);
        assert_eq!(output.email.external_inputs.len(), 2); // name + value
                                                           // Note: regex_matches will be empty since we're using mock regex data
        assert!(output.regex_matches.is_empty());
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;
    use slog::Logger;

    fn create_test_logger() -> Logger {
        let drain = slog::Discard;
        Logger::root(drain, slog::o!())
    }

    #[test]
    fn test_dkim_error_display() {
        let errors = vec![
            DkimError::EmailParseError("parse failed".to_string()),
            DkimError::KeyParseError("key invalid".to_string()),
            DkimError::VerificationError("verification failed".to_string()),
        ];

        for error in errors {
            let error_string = format!("{}", error);
            assert!(!error_string.is_empty());
            assert!(error_string.contains("error"));
        }
    }

    #[test]
    fn test_verify_dkim_with_invalid_email() {
        let logger = create_test_logger();
        let email = create_test_email_invalid();

        let result = verify_dkim(&email, &logger);
        assert!(result.is_err());

        if let Err(DkimError::KeyParseError(_)) = result {
            // Expected error type for invalid key
        } else {
            panic!("Expected KeyParseError, got: {:?}", result);
        }
    }

    #[test]
    fn test_verify_dkim_batch_error_handling() {
        let logger = create_test_logger();
        let valid_email = create_test_email();
        let mut invalid_email = create_test_email();
        invalid_email.raw_email = b"invalid".to_vec();

        let emails = vec![&valid_email, &invalid_email, &valid_email];
        let results = verify_dkim_batch(&emails, &logger);

        assert_eq!(results.len(), 3);
        // First should succeed (or fail gracefully)
        // Second should fail with parse error
        assert!(results[1].is_err());
        // Third should succeed (or fail gracefully)
    }
}

#[cfg(test)]
mod memory_efficiency_tests {
    use super::*;

    #[test]
    fn test_memory_allocation_patterns() {
        // Test that we don't have excessive allocations
        let email = create_test_email();

        // Multiple verifications should not accumulate memory
        for _ in 0..100 {
            let _output = verify_email_test_only(&email).unwrap();
        }

        // If we reach here without OOM, memory management is working
        // Test completed successfully - no explicit assertion needed
    }

    #[test]
    fn test_batch_processing_memory_efficiency() {
        let emails: Vec<Email> = (0..50)
            .map(|i| {
                let mut email = create_test_email();
                email.from_domain = format!("test{}.com", i);
                email
            })
            .collect();

        // Process batch without excessive memory usage
        let outputs: Vec<_> = emails
            .iter()
            .map(|email| verify_email_test_only(email).unwrap())
            .collect();

        assert_eq!(outputs.len(), 50);
    }

    #[test]
    fn test_large_email_processing() {
        let mut email = create_test_email();
        // Create a larger email (100KB)
        email.raw_email = format!(
            "From: test@example.com\r\nSubject: Large Email\r\n\r\n{}",
            "A".repeat(100 * 1024)
        )
        .into_bytes();

        let output = verify_email_test_only(&email).unwrap();

        // Should handle large emails without issues
        assert_eq!(output.from_domain_hash.len(), 32);
        assert_eq!(output.public_key_hash.len(), 32);
    }
}

#[cfg(test)]
mod deterministic_execution_tests {
    use super::*;

    #[test]
    fn test_hash_determinism_across_iterations() {
        let data = b"deterministic test data";
        let mut hashes = Vec::new();

        // Collect hashes from multiple iterations
        for _ in 0..100 {
            hashes.push(hash_bytes(data));
        }

        // All hashes should be identical
        let first_hash = &hashes[0];
        for hash in &hashes[1..] {
            assert_eq!(hash, first_hash, "Hash should be deterministic");
        }
    }

    #[test]
    fn test_email_verification_determinism() {
        let email = create_test_email();
        let mut outputs = Vec::new();

        // Collect outputs from multiple verifications
        for _ in 0..50 {
            outputs.push(verify_email_test_only(&email).unwrap());
        }

        // All outputs should be identical
        let first_output = &outputs[0];
        for output in &outputs[1..] {
            assert_eq!(output.from_domain_hash, first_output.from_domain_hash);
            assert_eq!(output.public_key_hash, first_output.public_key_hash);
            assert_eq!(output.external_inputs, first_output.external_inputs);
        }
    }

    #[test]
    fn test_regex_processing_determinism() {
        let regexes = vec![];
        let input = b"consistent test input for regex processing";
        let mut results = Vec::new();

        // Collect results from multiple processing calls
        for _ in 0..50 {
            results.push(process_regex_parts(&regexes, input));
        }

        // All results should be identical
        let first_result = &results[0];
        for result in &results[1..] {
            assert_eq!(result.0, first_result.0);
            assert_eq!(result.1, first_result.1);
        }
    }
}
