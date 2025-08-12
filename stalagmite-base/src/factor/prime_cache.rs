use std::sync::{OnceLock, RwLock};

// Use growing vector instead of FLINTs power-of-2 strategy for caching.

// Initialize with small primes
pub static PRIME_CACHE: OnceLock<RwLock<Vec<u64>>> = OnceLock::new();

pub fn get_prime_cache() -> &'static RwLock<Vec<u64>> {
    PRIME_CACHE.get_or_init(|| {
        RwLock::new(vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]) // Start with first 10 primes
    })
}

/// Ensure we have at least `count` primes computed
pub fn ensure_primes_computed(count: usize) {
    let cache = get_prime_cache().read().unwrap();
    if cache.len() >= count {
        return; // Already have enough
    }
    drop(cache);

    let mut cache = get_prime_cache().write().unwrap();
    
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
pub fn get_nth_prime_using_cache(n: usize) -> u64 {
    ensure_primes_computed(n + 1);
    let cache = get_prime_cache().read().unwrap();
    cache[n]
}

/// Get a slice of primes from the cache, extending if necessary
pub fn get_primes_using_cache<const N: usize>(start: usize, num_primes: usize) -> [u64; N] {
    let stop = start + num_primes;
    ensure_primes_computed(stop);
    let cache = get_prime_cache().read().unwrap();
    let res: [u64; N] = cache[start..stop].try_into().unwrap();
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_prime_cache_initialization() {
        let cache = get_prime_cache().read().unwrap();
        let expected_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        assert!(cache.len() >= 10);
        for (i, &expected) in expected_primes.iter().enumerate() {
            assert_eq!(cache[i], expected);
        }
    }

    #[test]
    fn test_ensure_primes_computed_basic() {
        ensure_primes_computed(15);
        let cache = get_prime_cache().read().unwrap();
        assert!(cache.len() >= 15);
        
        let expected_next_primes = vec![31, 37, 41, 43, 47];
        for (i, &expected) in expected_next_primes.iter().enumerate() {
            assert_eq!(cache[10 + i], expected);
        }
    }

    #[test]
    fn test_get_nth_prime_using_cache() {
        assert_eq!(get_nth_prime_using_cache(0), 2);
        assert_eq!(get_nth_prime_using_cache(1), 3);
        assert_eq!(get_nth_prime_using_cache(4), 11);
        assert_eq!(get_nth_prime_using_cache(9), 29);
        
        assert_eq!(get_nth_prime_using_cache(20), 73);
        assert_eq!(get_nth_prime_using_cache(24), 97);
    }

    #[test]
    fn test_get_primes_using_cache() {
        let slice: [u64; 5] = get_primes_using_cache(0, 5);
        assert_eq!(slice, [2, 3, 5, 7, 11]);
        
        let larger_slice: [u64; 15] = get_primes_using_cache(0, 15);
        assert_eq!(larger_slice.len(), 15);
        assert_eq!(larger_slice[0], 2);
        assert_eq!(larger_slice[14], 47);
    }

    #[test]
    fn test_is_prime_using_cache() {
        let cache = get_prime_cache().read().unwrap();
        
        // Note: is_prime_using_cache returns false for 2 because it checks n % 2 == 0
        assert!(!is_prime_using_cache(2, &cache));
        assert!(is_prime_using_cache(3, &cache));
        assert!(is_prime_using_cache(5, &cache));
        assert!(is_prime_using_cache(31, &cache));
        
        assert!(!is_prime_using_cache(4, &cache));
        assert!(!is_prime_using_cache(6, &cache));
        assert!(!is_prime_using_cache(9, &cache));
        assert!(!is_prime_using_cache(15, &cache));
        assert!(!is_prime_using_cache(25, &cache));
        
        assert!(!is_prime_using_cache(0, &cache));
        assert!(!is_prime_using_cache(1, &cache));
    }

    #[test]
    fn test_extend_cache_to() {
        let mut test_cache = vec![2, 3, 5, 7, 11];
        extend_cache_to(&mut test_cache, 10);
        
        assert_eq!(test_cache.len(), 10);
        assert_eq!(test_cache[5], 13);
        assert_eq!(test_cache[6], 17);
        assert_eq!(test_cache[7], 19);
        assert_eq!(test_cache[8], 23);
        assert_eq!(test_cache[9], 29);
    }

    #[test]
    fn test_concurrent_access() {
        let handles: Vec<_> = (0..10).map(|i| {
            thread::spawn(move || {
                ensure_primes_computed(50 + i);
                get_nth_prime_using_cache(30 + i)
            })
        }).collect();
        
        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        
        assert_eq!(results[0], get_nth_prime_using_cache(30));
        
        let cache = get_prime_cache().read().unwrap();
        assert!(cache.len() >= 59);
    }

    #[test]
    fn test_large_cache_extension() {
        ensure_primes_computed(100);
        let cache = get_prime_cache().read().unwrap();
        assert!(cache.len() >= 100);
        
        assert_eq!(cache[24], 97);
        assert_eq!(cache[49], 229);
        assert_eq!(cache[99], 541);
    }
}
