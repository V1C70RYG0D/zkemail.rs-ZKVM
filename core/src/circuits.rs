use mailparse::parse_mail;
use slog::{o, Discard, Logger};

use crate::{
    extract_email_body, hash_bytes, process_regex_parts, verify_dkim, DkimError, Email,
    EmailVerifierOutput, EmailWithRegex, EmailWithRegexVerifierOutput,
};

use crate::verify_dkim_test_only;

/// ZKVM-optimized email verification with error handling
///
/// Returns Result to handle DKIM verification failures gracefully
pub fn verify_email(email: &Email) -> Result<EmailVerifierOutput, DkimError> {
    let logger = Logger::root(Discard, o!());

    let verified = verify_dkim(email, &logger)?;

    if !verified {
        return Err(DkimError::VerificationError(
            "DKIM verification failed".to_string(),
        ));
    }

    Ok(EmailVerifierOutput {
        from_domain_hash: hash_bytes(email.from_domain.as_bytes()),
        public_key_hash: hash_bytes(&email.public_key.key),
        external_inputs: email
            .external_inputs
            .iter()
            .flat_map(|inputs| {
                vec![
                    inputs.name.clone(),
                    inputs.value.clone().expect("Value cannot be null"),
                ]
            })
            .collect(),
    })
}

/// Test-only email verification that bypasses DKIM for testing ZKVM optimizations
pub fn verify_email_test_only(email: &Email) -> Result<EmailVerifierOutput, DkimError> {
    let logger = Logger::root(Discard, o!());

    let verified = verify_dkim_test_only(email, &logger)?;

    if !verified {
        return Err(DkimError::VerificationError(
            "DKIM verification failed".to_string(),
        ));
    }

    Ok(EmailVerifierOutput {
        from_domain_hash: hash_bytes(email.from_domain.as_bytes()),
        public_key_hash: hash_bytes(&email.public_key.key),
        external_inputs: email
            .external_inputs
            .iter()
            .flat_map(|inputs| {
                vec![
                    inputs.name.clone(),
                    inputs.value.clone().expect("Value cannot be null"),
                ]
            })
            .collect(),
    })
}

/// ZKVM-optimized email with regex verification with error handling
pub fn verify_email_with_regex(
    input: &EmailWithRegex,
) -> Result<EmailWithRegexVerifierOutput, DkimError> {
    let email_verifier_output = verify_email(&input.email)?;

    let parsed_email = parse_mail(&input.email.raw_email)
        .map_err(|e| DkimError::EmailParseError(e.to_string()))?;

    let header_bytes = parsed_email.get_headers().get_raw_bytes();
    let email_body = extract_email_body(&parsed_email);

    let header_matches = input
        .regex_info
        .header_parts
        .as_ref()
        .map(|parts| process_regex_parts(parts, header_bytes))
        .map(|(verified, matches)| {
            if !verified {
                return Err(DkimError::VerificationError(
                    "Header regex verification failed".to_string(),
                ));
            }
            Ok(matches)
        })
        .transpose()?;

    let body_matches = input
        .regex_info
        .body_parts
        .as_ref()
        .map(|parts| process_regex_parts(parts, &email_body))
        .map(|(verified, matches)| {
            if !verified {
                return Err(DkimError::VerificationError(
                    "Body regex verification failed".to_string(),
                ));
            }
            Ok(matches)
        })
        .transpose()?;

    let regex_matches = header_matches
        .into_iter()
        .chain(body_matches)
        .flatten()
        .collect();

    Ok(EmailWithRegexVerifierOutput {
        email: email_verifier_output,
        regex_matches,
    })
}

/// Test-only email with regex verification that bypasses DKIM for testing ZKVM optimizations
pub fn verify_email_with_regex_test_only(
    input: &EmailWithRegex,
) -> Result<EmailWithRegexVerifierOutput, DkimError> {
    let email_verifier_output = verify_email_test_only(&input.email)?;

    let parsed_email = parse_mail(&input.email.raw_email)
        .map_err(|e| DkimError::EmailParseError(e.to_string()))?;

    let header_bytes = parsed_email.get_headers().get_raw_bytes();
    let email_body = extract_email_body(&parsed_email);

    let header_matches = input
        .regex_info
        .header_parts
        .as_ref()
        .map(|parts| process_regex_parts(parts, header_bytes))
        .map(|(verified, matches)| {
            if !verified {
                return Err(DkimError::VerificationError(
                    "Header regex verification failed".to_string(),
                ));
            }
            Ok(matches)
        })
        .transpose()?;

    let body_matches = input
        .regex_info
        .body_parts
        .as_ref()
        .map(|parts| process_regex_parts(parts, &email_body))
        .map(|(verified, matches)| {
            if !verified {
                return Err(DkimError::VerificationError(
                    "Body regex verification failed".to_string(),
                ));
            }
            Ok(matches)
        })
        .transpose()?;

    let regex_matches = header_matches
        .into_iter()
        .chain(body_matches)
        .flatten()
        .collect();

    Ok(EmailWithRegexVerifierOutput {
        email: email_verifier_output,
        regex_matches,
    })
}
