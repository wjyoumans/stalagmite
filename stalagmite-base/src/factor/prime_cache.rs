use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;

// Use growing vector instead of FLINTs power-of-2 strategy for caching.

// Initialize with small primes
pub static PRIME_CACHE: Lazy<RwLock<Vec<u64>>> = Lazy::new(|| {
    RwLock::new(vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]) // Start with first 10 primes
});

/// Ensure we have at least `count` primes computed
pub fn ensure_primes_computed(count: usize) {
    let cache = PRIME_CACHE.read().unwrap();
    if cache.len() >= count {
        return; // Already have enough
    }
    drop(cache);

    let mut cache = PRIME_CACHE.write().unwrap();
    
    // Double-check after acquiring write lock
    if cache.len() >= count {
        return;
    }
    
    // Extend cache to the required count
    extend_cache_to(&mut cache, count);
}

pub fn extend_cache_to(cache: &mut Vec<u64>, target_size: usize) {
    let mut candidate = cache.last().unwrap() + 2; // Start from next odd number
    
    while cache.len() < target_size {
        if is_prime_using_cache(candidate, cache) {
            cache.push(candidate);
        }
        candidate += 2; // Only check odd numbers
    }
}

pub fn is_prime_using_cache(n: u64, cache: &[u64]) -> bool {
    if n < 2 { return false; }
    if n % 2 == 0 { return false; }
    
    let sqrt_n = (n as f64).sqrt() as u64;
    
    for &prime in cache {
        if prime > sqrt_n {
            break;
        }
        if n % prime == 0 {
            return false;
        }
    }
    true
}

/// Get the nth prime (0-indexed), extending cache if necessary
pub fn get_nth_prime(n: usize) -> u64 {
    ensure_primes_computed(n + 1);
    let cache = PRIME_CACHE.read().unwrap();
    cache[n]
}

/// Get a slice of primes from the cache, extending if necessary
pub fn get_primes_slice(up_to_index: usize) -> Vec<u64> {
    ensure_primes_computed(up_to_index);
    let cache = PRIME_CACHE.read().unwrap();
    cache[..up_to_index].to_vec()
}
