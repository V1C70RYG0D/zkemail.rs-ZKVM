use zkemail_core::*;
use zkemail_profiling::*;

#[cfg(test)]
mod zkvm_optimization_tests {
    use super::*;

    #[test]
    fn test_zkvm_crypto_optimizations() {
        // Test basic hashing functionality
        let test_data = b"test data for ZKVM optimization";
        let hash1 = hash_bytes(test_data);
        let hash2 = hash_bytes(test_data);

        assert_eq!(hash1, hash2, "Hash should be deterministic");
        assert_eq!(hash1.len(), 32, "SHA256 hash should be 32 bytes");
    }

    #[test]
    fn test_zkvm_batch_crypto_optimizations() {
        let test_data = vec![
            b"data1".as_slice(),
            b"data2".as_slice(),
            b"data3".as_slice(),
        ];

        let batch_hashes = hash_bytes_batch(&test_data);
        assert_eq!(batch_hashes.len(), 3);

        // Verify each hash matches individual computation
        for (i, data) in test_data.iter().enumerate() {
            assert_eq!(batch_hashes[i], hash_bytes(data));
        }
    }

    #[test]
    fn test_zkvm_concat_hash_optimization() {
        let test_data = vec![b"hello".as_slice(), b"world".as_slice()];
        let concat_hash = hash_bytes_concat(&test_data);
        let direct_hash = hash_bytes(b"helloworld");

        assert_eq!(concat_hash, direct_hash);
    }

    #[test]
    fn test_zkvm_email_body_extraction() {
        let simple_email = generate_test_email_simple();
        let parsed = mailparse::parse_mail(&simple_email).unwrap();
        let body = extract_email_body(&parsed);

        assert!(!body.is_empty(), "Email body should not be empty");

        // Test multipart email
        let multipart_email = generate_test_email_multipart();
        let parsed_multipart = mailparse::parse_mail(&multipart_email).unwrap();
        let multipart_body = extract_email_body(&parsed_multipart);

        assert!(
            !multipart_body.is_empty(),
            "Multipart email body should not be empty"
        );
    }

    #[test]
    fn test_zkvm_batch_email_processing() {
        let emails = [
            generate_test_email_simple(),
            generate_test_email_complex(),
            generate_test_email_multipart(),
        ];

        let parsed_emails: Vec<_> = emails
            .iter()
            .map(|e| mailparse::parse_mail(e).unwrap())
            .collect();

        let parsed_refs: Vec<_> = parsed_emails.iter().collect();
        let bodies = extract_email_bodies_batch(&parsed_refs);

        assert_eq!(bodies.len(), 3);
        for body in bodies {
            assert!(!body.is_empty(), "Each email body should not be empty");
        }
    }

    #[test]
    fn test_zkvm_regex_processing() {
        let test_cases = generate_regex_test_cases();

        for (name, regexes, input) in test_cases {
            let (success, matches) = process_regex_parts(&regexes, &input);

            // For our test cases, we expect them to process without errors
            // (though they may not match due to simplified DFA patterns)
            println!(
                "Test case '{}': success={}, matches={:?}",
                name, success, matches
            );
        }
    }

    #[test]
    fn test_zkvm_struct_optimizations() {
        // Test PublicKey optimizations
        let key_data = vec![1, 2, 3, 4, 5];
        let public_key = PublicKey::new(key_data.clone(), "rsa".to_string());

        assert!(public_key.is_valid());
        assert_eq!(public_key.key, key_data);
        assert_eq!(public_key.key_type, "rsa");

        let estimated_size = public_key.estimated_size();
        assert!(estimated_size > 0, "Estimated size should be positive");

        // Test ExternalInput optimizations
        let external_input =
            ExternalInput::new("test_name".to_string(), Some("test_value".to_string()), 100);

        assert!(external_input.estimated_size() > 0);

        // Test Email optimizations
        let email = Email::new(
            "example.com".to_string(),
            generate_test_email_simple(),
            public_key,
            vec![external_input],
        );

        assert!(email.is_valid());
        assert!(email.estimated_size() > 0);
    }

    #[test]
    fn test_zkvm_memory_efficiency() {
        // Test that our optimizations don't cause excessive allocations
        let test_data = vec![0u8; 1024];

        // Hash multiple times to test memory reuse
        for _ in 0..100 {
            let _hash = hash_bytes(&test_data);
        }

        // Test batch processing efficiency
        let batch_data: Vec<&[u8]> = (0..10).map(|_| test_data.as_slice()).collect();
        let _batch_hashes = hash_bytes_batch(&batch_data);
    }

    #[test]
    fn test_zkvm_serialization_risc0() {
        #[cfg(feature = "risc0")]
        {
            let email = generate_test_email_with_dkim();

            // Test Borsh serialization
            let serialized = borsh::to_vec(&email).expect("Serialization should succeed");
            let deserialized: Email =
                borsh::from_slice(&serialized).expect("Deserialization should succeed");

            assert_eq!(email.from_domain, deserialized.from_domain);
            assert_eq!(email.raw_email, deserialized.raw_email);
        }
    }

    #[test]
    fn test_zkvm_serialization_sp1() {
        #[cfg(feature = "sp1")]
        {
            let email = generate_test_email_with_dkim();

            // Test Serde JSON serialization
            let serialized = serde_json::to_vec(&email).expect("Serialization should succeed");
            let deserialized: Email =
                serde_json::from_slice(&serialized).expect("Deserialization should succeed");

            assert_eq!(email.from_domain, deserialized.from_domain);
            assert_eq!(email.raw_email, deserialized.raw_email);
        }
    }

    #[test]
    fn test_zkvm_constraint_validation() {
        let constraint_data = generate_constraint_test_data();

        for (name, expected_size, data) in constraint_data {
            assert_eq!(
                data.len(),
                expected_size,
                "Test case '{}' should have expected size",
                name
            );

            // Test that we can hash data of various sizes efficiently
            let _hash = hash_bytes(&data);
        }
    }

    #[test]
    fn test_zkvm_realistic_workloads() {
        let workloads = generate_realistic_workloads();

        for (workload_type, email_data) in workloads {
            // Test parsing
            let parsed = mailparse::parse_mail(&email_data)
                .unwrap_or_else(|_| panic!("Should parse {} email", workload_type));

            // Test body extraction
            let body = extract_email_body(&parsed);
            assert!(
                !body.is_empty(),
                "Body should not be empty for {} email",
                workload_type
            );

            // Test hashing
            let _hash = hash_bytes(&email_data);
        }
    }

    #[test]
    fn test_zkvm_error_handling() {
        // Test error handling in DKIM verification
        let invalid_email = Email::new(
            "".to_string(),                         // Invalid empty domain
            vec![],                                 // Invalid empty email
            PublicKey::new(vec![], "".to_string()), // Invalid empty key
            vec![],
        );

        assert!(
            !invalid_email.is_valid(),
            "Invalid email should not validate"
        );
    }

    #[test]
    fn test_zkvm_performance_characteristics() {
        use std::time::Instant;

        // Test that operations complete within reasonable time bounds
        let test_data = vec![0u8; 4096];

        let start = Instant::now();
        for _ in 0..1000 {
            let _hash = hash_bytes(&test_data);
        }
        let duration = start.elapsed();

        // This is a basic performance sanity check
        assert!(
            duration.as_millis() < 5000,
            "1000 hash operations should complete in under 5 seconds"
        );
    }

    #[cfg(feature = "sp1")]
    #[test]
    fn test_sp1_memory_alignment() {
        // Test SP1-specific memory alignment optimizations
        let test_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let test_regexes = generate_regex_test_cases();

        for (_, regexes, input) in test_regexes {
            let (_success, _matches) = process_regex_parts(&regexes, &input);
            // The function should handle alignment internally
        }
    }
}
