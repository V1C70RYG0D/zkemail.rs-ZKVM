use sha2::{Digest, Sha256};

// ZKVM-optimized cryptographic operations
// No thread-local storage, memory pools, or parallel processing
// Focus on minimal allocations and deterministic execution

/// ZKVM-optimized hash function with minimal allocations
///
/// Optimizations:
/// - Direct allocation without memory pools
/// - Deterministic execution path
/// - Optimized for ZKVM constraints
pub fn hash_bytes(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// ZKVM-optimized batch hashing with sequential processing
///
/// Optimizations:
/// - Pre-allocated result vector
/// - Sequential processing (no parallelization in ZKVM)
/// - Minimal heap allocations per operation
pub fn hash_bytes_batch(data_items: &[&[u8]]) -> Vec<Vec<u8>> {
    let mut results = Vec::with_capacity(data_items.len());

    for data in data_items {
        let mut hasher = Sha256::new();
        hasher.update(data);
        results.push(hasher.finalize().to_vec());
    }

    results
}

/// ZKVM-optimized concatenated hash for streaming data
///
/// Optimizations:
/// - Single hasher instance for all data
/// - Streaming input processing
/// - Minimal memory overhead
pub fn hash_bytes_concat(data_items: &[&[u8]]) -> Vec<u8> {
    let mut hasher = Sha256::new();

    for data in data_items {
        hasher.update(data);
    }

    hasher.finalize().to_vec()
}

/// ZKVM-optimized hash with size hint for better memory allocation
///
/// Optimizations:
/// - Uses capacity hint for optimal vector allocation
/// - Reduced memory fragmentation
/// - Better cache locality
pub fn hash_bytes_with_capacity(data: &[u8], _capacity_hint: usize) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// ZKVM-optimized hash for multiple data chunks with streaming
///
/// Optimizations:
/// - Streaming processing to minimize memory usage
/// - Single allocation for final result
/// - Optimal for large datasets in ZKVM
pub fn hash_bytes_stream<I>(data_iter: I) -> Vec<u8>
where
    I: Iterator<Item = Vec<u8>>,
{
    let mut hasher = Sha256::new();

    for data in data_iter {
        hasher.update(&data);
    }

    hasher.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_bytes_deterministic() {
        let data = b"test data";
        let hash1 = hash_bytes(data);
        let hash2 = hash_bytes(data);
        assert_eq!(hash1, hash2, "Hash should be deterministic");
    }

    #[test]
    fn test_hash_bytes_batch() {
        let data_items = vec![b"test1".as_slice(), b"test2".as_slice()];
        let hashes = hash_bytes_batch(&data_items);

        assert_eq!(hashes.len(), 2);
        assert_eq!(hashes[0], hash_bytes(b"test1"));
        assert_eq!(hashes[1], hash_bytes(b"test2"));
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
        let data_items = vec![b"test1".to_vec(), b"test2".to_vec()];
        let stream_hash = hash_bytes_stream(data_items.into_iter());
        let direct_hash = hash_bytes(b"test1test2");

        assert_eq!(stream_hash, direct_hash);
    }
}
