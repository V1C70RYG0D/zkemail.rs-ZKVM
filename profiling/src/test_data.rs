use std::collections::HashMap;
use zkemail_core::*;

/// Generate a simple email for ZKVM testing
pub fn generate_test_email_simple() -> Vec<u8> {
    let email = r#"From: test@example.com
To: recipient@example.com
Subject: Test Email
Date: Mon, 1 Jan 2024 12:00:00 +0000
DKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed; d=example.com; s=default; h=from:to:subject:date; bh=test; b=test

This is a simple test email body.
"#;
    email.as_bytes().to_vec()
}

/// Generate a complex email with multiple headers for ZKVM testing
pub fn generate_test_email_complex() -> Vec<u8> {
    let email = r#"From: sender@example.com
To: recipient@example.com
Cc: cc@example.com
Bcc: bcc@example.com
Subject: Complex Test Email with Multiple Headers
Date: Mon, 1 Jan 2024 12:00:00 +0000
Message-ID: <test@example.com>
Reply-To: reply@example.com
Return-Path: <sender@example.com>
X-Custom-Header: Custom Value
X-Priority: 1
DKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed; d=example.com; s=default; h=from:to:subject:date:message-id:reply-to; bh=test; b=test

This is a more complex test email body with multiple headers.
It contains multiple lines and various content types.

Best regards,
Test Sender
"#;
    email.as_bytes().to_vec()
}

/// Generate a multipart email for ZKVM testing
pub fn generate_test_email_multipart() -> Vec<u8> {
    let email = r#"From: sender@example.com
To: recipient@example.com
Subject: Multipart Test Email
Date: Mon, 1 Jan 2024 12:00:00 +0000
MIME-Version: 1.0
Content-Type: multipart/alternative; boundary="boundary123"
DKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed; d=example.com; s=default; h=from:to:subject:date:mime-version:content-type; bh=test; b=test

--boundary123
Content-Type: text/plain; charset=utf-8

This is the plain text version of the email.

--boundary123
Content-Type: text/html; charset=utf-8

<html>
<body>
<p>This is the <strong>HTML</strong> version of the email.</p>
</body>
</html>

--boundary123--
"#;
    email.as_bytes().to_vec()
}

/// Generate test email with valid DKIM signature for circuit testing
pub fn generate_test_email_with_dkim() -> Email {
    let raw_email = generate_test_email_simple();

    // Generate a valid test RSA public key in PKCS#1 format for testing
    // This is a minimal valid RSA public key (512-bit for testing purposes)
    // PKCS#1 format: SEQUENCE { modulus INTEGER, publicExponent INTEGER }
    let public_key = PublicKey {
        key: vec![
            // SEQUENCE tag and length
            0x30, 0x48, // SEQUENCE, 72 bytes total
            // Modulus (n) - INTEGER
            0x02, 0x41, 0x00, // INTEGER, 65 bytes (with leading zero for positive)
            // 512-bit modulus
            0xd4, 0x38, 0x6a, 0x5c, 0x9e, 0x54, 0x7a, 0x9b, 0x9f, 0x7c, 0x8b, 0x2c, 0x4d, 0x3e,
            0x5f, 0x6a, 0x7b, 0x8c, 0x9d, 0x1e, 0x2f, 0x3a, 0x4b, 0x5c, 0x6d, 0x7e, 0x8f, 0x9a,
            0x0b, 0x1c, 0x2d, 0x3e, 0x4f, 0x5a, 0x6b, 0x7c, 0x8d, 0x9e, 0x0f, 0x1a, 0x2b, 0x3c,
            0x4d, 0x5e, 0x6f, 0x7a, 0x8b, 0x9c, 0x0d, 0x1e, 0x2f, 0x3a, 0x4b, 0x5c, 0x6d, 0x7e,
            0x8f, 0x9a, 0x0b, 0x1c, 0x2d, 0x3e, 0x4f, 0x5a,
            // Public exponent (e) - INTEGER, typically 65537 (0x010001)
            0x02, 0x03, 0x01, 0x00, 0x01,
        ],
        key_type: "rsa".to_string(),
    };

    Email {
        from_domain: "example.com".to_string(),
        raw_email,
        public_key,
        external_inputs: vec![ExternalInput {
            name: "test_input".to_string(),
            value: Some("test_value".to_string()),
            max_length: 100,
        }],
    }
}

/// Generate test email for testing scenarios that should fail DKIM verification
pub fn generate_test_email_invalid_dkim() -> Email {
    let raw_email = generate_test_email_simple();

    // Use an invalid key format to test error handling
    let public_key = PublicKey {
        key: vec![0u8; 64], // Invalid key format
        key_type: "rsa".to_string(),
    };

    Email {
        from_domain: "example.com".to_string(),
        raw_email,
        public_key,
        external_inputs: vec![ExternalInput {
            name: "test_input".to_string(),
            value: Some("test_value".to_string()),
            max_length: 100,
        }],
    }
}

/// Generate test email with regex patterns for circuit testing
pub fn generate_test_email_with_regex() -> EmailWithRegex {
    let email = generate_test_email_with_dkim();

    // Create test DFA patterns (simplified)
    let test_dfa = DFA {
        fwd: vec![0x01, 0x02, 0x03, 0x04], // Simplified DFA bytes
        bwd: vec![0x04, 0x03, 0x02, 0x01], // Simplified DFA bytes
    };

    let compiled_regex = CompiledRegex {
        verify_re: test_dfa,
        captures: Some(vec!["test".to_string()]),
    };

    let regex_info = RegexInfo {
        header_parts: Some(vec![compiled_regex.clone()]),
        body_parts: Some(vec![compiled_regex]),
    };

    EmailWithRegex { email, regex_info }
}

/// Generate test cases for regex processing benchmarks
pub fn generate_regex_test_cases() -> Vec<(String, Vec<CompiledRegex>, Vec<u8>)> {
    let mut test_cases = Vec::new();

    // Simple pattern test case
    let simple_dfa = DFA {
        fwd: vec![0x01, 0x02, 0x03, 0x04],
        bwd: vec![0x04, 0x03, 0x02, 0x01],
    };

    let simple_regex = CompiledRegex {
        verify_re: simple_dfa,
        captures: Some(vec!["test".to_string()]),
    };

    test_cases.push((
        "simple".to_string(),
        vec![simple_regex],
        b"This is a test string for regex matching".to_vec(),
    ));

    // Complex pattern test case
    let complex_dfa = DFA {
        fwd: vec![0x05, 0x06, 0x07, 0x08, 0x09, 0x0a],
        bwd: vec![0x0a, 0x09, 0x08, 0x07, 0x06, 0x05],
    };

    let complex_regex = CompiledRegex {
        verify_re: complex_dfa,
        captures: Some(vec!["complex".to_string(), "pattern".to_string()]),
    };

    test_cases.push((
        "complex".to_string(),
        vec![complex_regex],
        b"This is a complex pattern matching test with multiple captures".to_vec(),
    ));

    // Multiple patterns test case
    let multi_regexes = vec![
        CompiledRegex {
            verify_re: DFA {
                fwd: vec![0x01, 0x02],
                bwd: vec![0x02, 0x01],
            },
            captures: Some(vec!["first".to_string()]),
        },
        CompiledRegex {
            verify_re: DFA {
                fwd: vec![0x03, 0x04],
                bwd: vec![0x04, 0x03],
            },
            captures: Some(vec!["second".to_string()]),
        },
    ];

    test_cases.push((
        "multiple".to_string(),
        multi_regexes,
        b"Testing multiple regex patterns in sequence for ZKVM optimization".to_vec(),
    ));

    test_cases
}

/// Generate test data for memory profiling
pub fn generate_memory_test_data() -> HashMap<String, Vec<u8>> {
    let mut test_data = HashMap::new();

    // Small data (typical header size)
    test_data.insert("small".to_string(), vec![0u8; 1024]);

    // Medium data (typical email body size)
    test_data.insert("medium".to_string(), vec![0u8; 16384]);

    // Large data (large email with attachments)
    test_data.insert("large".to_string(), vec![0u8; 262144]);

    test_data
}

/// Generate realistic email workloads for ZKVM testing
pub fn generate_realistic_workloads() -> Vec<(String, Vec<u8>)> {
    vec![
        ("newsletter".to_string(), generate_newsletter_email()),
        ("transaction".to_string(), generate_transaction_email()),
        ("notification".to_string(), generate_notification_email()),
        ("marketing".to_string(), generate_marketing_email()),
    ]
}

fn generate_newsletter_email() -> Vec<u8> {
    let email = r#"From: newsletter@company.com
To: subscriber@example.com
Subject: Weekly Newsletter - Tech Updates
Date: Mon, 1 Jan 2024 12:00:00 +0000
Content-Type: text/html; charset=utf-8
DKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed; d=company.com; s=default; h=from:to:subject:date:content-type; bh=test; b=test

<html>
<body>
<h1>Weekly Tech Newsletter</h1>
<p>Here are this week's top tech stories...</p>
<ul>
<li>AI breakthrough in natural language processing</li>
<li>New quantum computing milestone achieved</li>
<li>Blockchain technology adoption increases</li>
</ul>
</body>
</html>
"#;
    email.as_bytes().to_vec()
}

fn generate_transaction_email() -> Vec<u8> {
    let email = r#"From: noreply@bank.com
To: customer@example.com
Subject: Transaction Confirmation - $150.00
Date: Mon, 1 Jan 2024 12:00:00 +0000
DKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed; d=bank.com; s=default; h=from:to:subject:date; bh=test; b=test

Dear Customer,

This email confirms your transaction:
Amount: $150.00
Merchant: Online Store
Date: January 1, 2024
Transaction ID: TXN123456789

Thank you for your business.

Best regards,
Your Bank
"#;
    email.as_bytes().to_vec()
}

fn generate_notification_email() -> Vec<u8> {
    let email = r#"From: notifications@service.com
To: user@example.com
Subject: Account Activity Alert
Date: Mon, 1 Jan 2024 12:00:00 +0000
DKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed; d=service.com; s=default; h=from:to:subject:date; bh=test; b=test

Hello,

We detected new activity on your account:
- Login from new device
- Location: New York, NY
- Time: 12:00 PM EST

If this wasn't you, please secure your account immediately.

Security Team
"#;
    email.as_bytes().to_vec()
}

fn generate_marketing_email() -> Vec<u8> {
    let email = r#"From: marketing@retailer.com
To: customer@example.com
Subject: Special Offer - 50% Off Everything!
Date: Mon, 1 Jan 2024 12:00:00 +0000
Content-Type: text/html; charset=utf-8
DKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed; d=retailer.com; s=default; h=from:to:subject:date:content-type; bh=test; b=test

<html>
<body>
<h1>New Year Sale!</h1>
<p>Get 50% off everything in our store!</p>
<p>Use code: NEWYEAR50</p>
<p>Valid until January 31, 2024</p>
<a href="https://retailer.com/sale">Shop Now</a>
</body>
</html>
"#;
    email.as_bytes().to_vec()
}

/// Generate test data for ZKVM constraint validation
pub fn generate_constraint_test_data() -> Vec<(String, usize, Vec<u8>)> {
    vec![
        ("min_size".to_string(), 64, vec![0u8; 64]),
        ("typical_size".to_string(), 4096, vec![0u8; 4096]),
        ("max_size".to_string(), 65536, vec![0u8; 65536]),
    ]
}

/// Generate a test email of specified size for ZKVM testing
///
/// This function creates emails of different sizes (in KB) for ZKVM performance testing.
/// The size is approximate and includes both headers and body content.
///
/// # Arguments
/// * `size_kb` - Target size in kilobytes (1024 bytes)
///
/// # Returns
/// * `Email` - A test email with DKIM signature and specified approximate size
pub fn create_test_email(size_kb: usize) -> Email {
    let target_bytes = size_kb * 1024;

    // Base email headers (approximately 400 bytes)
    let headers = format!(
        r#"From: test@example.com
To: recipient@example.com
Subject: ZKVM Test Email - Size {}KB
Date: Mon, 1 Jan 2024 12:00:00 +0000
Message-ID: <test-{}-{}@example.com>
MIME-Version: 1.0
Content-Type: text/plain; charset=utf-8
DKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed; d=example.com; s=default; h=from:to:subject:date:message-id:mime-version:content-type; bh=test; b=test

"#,
        size_kb,
        chrono::Utc::now().timestamp(),
        std::process::id()
    );

    let header_bytes = headers.len();
    let remaining_bytes = target_bytes.saturating_sub(header_bytes);

    // Generate body content to reach target size
    let body = if remaining_bytes > 0 {
        let base_content = "This is test content for ZKVM email verification benchmarking. ";
        let base_len = base_content.len();
        let repetitions = (remaining_bytes / base_len) + 1;

        let mut body_content = String::new();
        body_content.push_str("Email body content for ZKVM testing:\n\n");

        for i in 0..repetitions {
            body_content.push_str(&format!("Line {}: {}", i + 1, base_content));
            if body_content.len() >= remaining_bytes {
                break;
            }
            if i % 10 == 9 {
                body_content.push('\n');
            }
        }

        // Truncate to exact size if needed
        if body_content.len() > remaining_bytes {
            body_content.truncate(remaining_bytes);
        }

        body_content
    } else {
        "Minimal content.".to_string()
    };

    let full_email = format!("{}{}", headers, body);
    let raw_email = full_email.into_bytes();

    // Generate a valid test RSA public key for DKIM verification
    let public_key = PublicKey {
        key: vec![
            // SEQUENCE tag and length
            0x30, 0x48, // SEQUENCE, 72 bytes total
            // Modulus (n) - INTEGER
            0x02, 0x41, 0x00, // INTEGER, 65 bytes (with leading zero for positive)
            // 512-bit modulus for testing
            0xd4, 0x38, 0x6a, 0x5c, 0x9e, 0x54, 0x7a, 0x9b, 0x9f, 0x7c, 0x8b, 0x2c, 0x4d, 0x3e,
            0x5f, 0x6a, 0x7b, 0x8c, 0x9d, 0x1e, 0x2f, 0x3a, 0x4b, 0x5c, 0x6d, 0x7e, 0x8f, 0x9a,
            0x0b, 0x1c, 0x2d, 0x3e, 0x4f, 0x5a, 0x6b, 0x7c, 0x8d, 0x9e, 0x0f, 0x1a, 0x2b, 0x3c,
            0x4d, 0x5e, 0x6f, 0x7a, 0x8b, 0x9c, 0x0d, 0x1e, 0x2f, 0x3a, 0x4b, 0x5c, 0x6d, 0x7e,
            0x8f, 0x9a, 0x0b, 0x1c, 0x2d, 0x3e, 0x4f, 0x5a,
            // Public exponent (e) - INTEGER, typically 65537 (0x010001)
            0x02, 0x03, 0x01, 0x00, 0x01,
        ],
        key_type: "rsa".to_string(),
    };

    Email {
        from_domain: "example.com".to_string(),
        raw_email,
        public_key,
        external_inputs: vec![ExternalInput {
            name: format!("test_input_{}_kb", size_kb),
            value: Some(format!("test_value_for_{}_kb_email", size_kb)),
            max_length: 100,
        }],
    }
}
