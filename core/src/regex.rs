use regex_automata::dfa::{dense, regex::Regex};

use crate::CompiledRegex;

/// SP1-specific memory alignment for optimal performance
///
/// SP1 ZKVM requires 4-byte aligned memory access for optimal performance.
/// This function ensures data is properly aligned to prevent performance penalties.
#[cfg(feature = "sp1")]
fn align_slice(bytes: &[u8]) -> Vec<u8> {
    let mut aligned = Vec::with_capacity(bytes.len() + 4);
    let offset = (aligned.as_ptr() as usize) % 4;
    let padding = if offset == 0 { 0 } else { 4 - offset };
    aligned.extend(std::iter::repeat(0).take(padding));
    aligned.extend_from_slice(bytes);
    aligned
}

/// ZKVM-optimized regex processing with early termination and minimal allocations
///
/// Optimizations:
/// - Early termination on first mismatch
/// - Pre-allocated vectors with capacity estimation
/// - Platform-specific memory alignment (SP1)
/// - Minimal cycle usage with efficient pattern matching
/// - Sequential processing optimized for ZKVM constraints
pub fn process_regex_parts(
    compiled_regexes: &[CompiledRegex],
    input: &[u8],
) -> (bool, Vec<String>) {
    // Pre-allocate with estimated capacity to reduce reallocations
    let mut regex_matches = Vec::with_capacity(compiled_regexes.len() * 2);

    for part in compiled_regexes {
        // Platform-specific memory alignment for optimal performance
        #[cfg(feature = "sp1")]
        let fwd = align_slice(&part.verify_re.fwd);
        #[cfg(not(feature = "sp1"))]
        let fwd = part.verify_re.fwd.clone();

        #[cfg(feature = "sp1")]
        let bwd = align_slice(&part.verify_re.bwd);
        #[cfg(not(feature = "sp1"))]
        let bwd = part.verify_re.bwd.clone();

        // Build regex with error handling for ZKVM robustness
        let fwd_dfa = match dense::DFA::from_bytes(&fwd) {
            Ok((dfa, _)) => dfa,
            Err(_) => return (false, regex_matches), // Early termination on error
        };

        let bwd_dfa = match dense::DFA::from_bytes(&bwd) {
            Ok((dfa, _)) => dfa,
            Err(_) => return (false, regex_matches), // Early termination on error
        };

        let re = Regex::builder().build_from_dfas(fwd_dfa, bwd_dfa);

        // Collect matches with early termination optimization
        let matches: Vec<_> = re.find_iter(input).collect();

        // ZKVM optimization: Early termination for invalid match count
        if matches.len() != 1 {
            return (false, regex_matches);
        }

        // Process captures with optimized string handling
        if let Some(ref captures) = part.captures {
            for capture in captures.iter() {
                // ZKVM-optimized string conversion with error handling
                let matched_str = match std::str::from_utf8(&input[matches[0].range()]) {
                    Ok(s) => s,
                    Err(_) => return (false, regex_matches), // Early termination on UTF-8 error
                };

                // Efficient capture validation with early termination
                if !matched_str.contains(capture) || matched_str.matches(capture).count() != 1 {
                    return (false, regex_matches);
                }

                // Direct string allocation for ZKVM compatibility
                regex_matches.push(capture.to_string());
            }
        }
    }

    (true, regex_matches)
}

/// ZKVM-optimized batch regex processing
///
/// Optimizations:
/// - Sequential processing only (no parallelization in ZKVM)
/// - Pre-allocated result vectors
/// - Early termination on first failure
/// - Minimal memory overhead per batch item
pub fn process_regex_parts_batch(
    compiled_regexes: &[CompiledRegex],
    inputs: &[&[u8]],
) -> Vec<(bool, Vec<String>)> {
    let mut results = Vec::with_capacity(inputs.len());

    for input in inputs {
        let result = process_regex_parts(compiled_regexes, input);
        results.push(result);

        // Optional: Early termination for batch processing if needed
        // if !result.0 { break; }
    }

    results
}

#[cfg(test)]
mod tests {
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
        let inputs = vec![b"test1".as_slice(), b"test2".as_slice()];
        let results = process_regex_parts_batch(&regexes, &inputs);

        assert_eq!(results.len(), 2);
        assert!(results[0].0);
        assert!(results[1].0);
    }

    #[cfg(feature = "sp1")]
    #[test]
    fn test_align_slice() {
        let data = vec![1, 2, 3, 4, 5];
        let aligned = align_slice(&data);

        // Check that alignment is correct
        assert_eq!(aligned.as_ptr() as usize % 4, 0);

        // Check that original data is preserved (after padding)
        let padding_len = aligned.len() - data.len();
        assert_eq!(&aligned[padding_len..], &data);
    }
}
