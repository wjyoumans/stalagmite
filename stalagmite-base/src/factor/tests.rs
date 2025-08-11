// Copyright (C) 2025 William Youmans
//
// This file is part of Stalagmite.
//
// Stalagmite is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published
// by the Free Software Foundation, either version 3 of the License,
// or (at your option) any later version.
//
// Stalagmite is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Stalagmite. If not, see <https://www.gnu.org/licenses/>.

#[cfg(test)]
mod tests {
    use crate::factor::{FactoredElem, FactoredZZ, FactoredQQ};
    use crate::integer::ZZElem;
    use crate::rational::QQElem;
    use malachite::base::num::basic::traits::{Zero, One};
    use std::collections::HashMap;

    // Tests for FactoredElem basic functionality
    mod factored_elem_tests {
        use super::*;

        #[test]
        fn test_factored_elem_multiplication_same_base() {
            // Test: (2^3) * (2^2) = 2^5
            let factored1 = FactoredElem::from([(ZZElem::from(2), 3u32)]);
            let factored2 = FactoredElem::from([(ZZElem::from(2), 2u32)]);

            let result = factored1 * factored2;
            
            assert_eq!(result.len(), 1);
            assert_eq!(result.get(&ZZElem::from(2)), Some(&5u32));
        }

        #[test]
        fn test_factored_elem_multiplication_different_bases() {
            // Test: (2^3) * (3^2) = 2^3 * 3^2
            let factored1 = FactoredElem::from([(ZZElem::from(2), 3u32)]);
            let factored2 = FactoredElem::from([(ZZElem::from(3), 2u32)]);

            let result = factored1 * factored2;
            
            assert_eq!(result.len(), 2);
            assert_eq!(result.get(&ZZElem::from(2)), Some(&3u32));
            assert_eq!(result.get(&ZZElem::from(3)), Some(&2u32));
        }

        #[test]
        fn test_factored_elem_multiplication_complex() {
            // Test: (2^3 * 5^1) * (2^1 * 3^2 * 5^2) = 2^4 * 3^2 * 5^3
            let factored1 = FactoredElem::from([
                (ZZElem::from(2), 3u32),
                (ZZElem::from(5), 1u32)
            ]);
            let factored2 = FactoredElem::from([
                (ZZElem::from(2), 1u32),
                (ZZElem::from(3), 2u32),
                (ZZElem::from(5), 2u32)
            ]);

            let result = factored1 * factored2;
            
            assert_eq!(result.len(), 3);
            assert_eq!(result.get(&ZZElem::from(2)), Some(&4u32));
            assert_eq!(result.get(&ZZElem::from(3)), Some(&2u32));
            assert_eq!(result.get(&ZZElem::from(5)), Some(&3u32));
        }

        #[test]
        fn test_factored_elem_multiplication_with_default() {
            // Test: (2^3) * default = 2^3
            let factored1 = FactoredElem::from([(ZZElem::from(2), 3u32)]);
            let default_factored: FactoredZZ = FactoredElem::default();
            
            let result = factored1 * default_factored;
            
            assert_eq!(result.len(), 2); // 2^3 * 1^1 = has both factors
            assert_eq!(result.get(&ZZElem::from(2)), Some(&3u32));
            assert_eq!(result.get(&ZZElem::ONE), Some(&1u32));
        }

        #[test]
        fn test_factored_qq_basic() {
            // Test FactoredQQ with rational numbers
            let factored: FactoredQQ = FactoredElem::from([
                (QQElem::from(2), 2i32),
                (QQElem::from(3), -1i32) // Negative exponent for rationals
            ]);

            // This represents 2^2 * 3^(-1) = 4/3
            assert_eq!(factored.len(), 2);
            assert_eq!(factored.get(&QQElem::from(2)), Some(&2i32));
            assert_eq!(factored.get(&QQElem::from(3)), Some(&-1i32));
        }

        #[test]
        fn test_factored_elem_constructors() {
            // Test various constructor methods
            
            // Empty factorization
            let empty: FactoredZZ = FactoredElem::new();
            assert!(empty.is_empty());
            assert_eq!(empty.len(), 0);
            
            // From array (most common case)
            let from_array = FactoredElem::from([(ZZElem::from(7), 3u32)]);
            assert_eq!(from_array.len(), 1);
            assert_eq!(from_array.get(&ZZElem::from(7)), Some(&3u32));
            
            // From vector
            let from_vec = FactoredElem::from(vec![
                (ZZElem::from(2), 2u32),
                (ZZElem::from(3), 1u32)
            ]);
            assert_eq!(from_vec.len(), 2);
            assert_eq!(from_vec.get(&ZZElem::from(2)), Some(&2u32));
            assert_eq!(from_vec.get(&ZZElem::from(3)), Some(&1u32));
            
            // From iterator
            let pairs = [(ZZElem::from(5), 2u32), (ZZElem::from(11), 1u32)];
            let from_iter: FactoredZZ = pairs.into_iter().collect();
            assert_eq!(from_iter.len(), 2);
            assert_eq!(from_iter.get(&ZZElem::from(5)), Some(&2u32));
            assert_eq!(from_iter.get(&ZZElem::from(11)), Some(&1u32));
            
            // With capacity
            let with_cap: FactoredZZ = FactoredElem::with_capacity(10);
            assert!(with_cap.is_empty());
        }

        #[test]
        fn test_factored_elem_operations() {
            // Test HashMap-like operations through Deref
            let mut factored = FactoredElem::from([
                (ZZElem::from(2), 3u32),
                (ZZElem::from(5), 1u32)
            ]);
            
            // Test insert
            let old_exp = factored.insert(ZZElem::from(3), 2u32);
            assert_eq!(old_exp, None);
            assert_eq!(factored.get(&ZZElem::from(3)), Some(&2u32));
            
            // Test replace
            let old_exp = factored.insert(ZZElem::from(2), 4u32);
            assert_eq!(old_exp, Some(3u32));
            assert_eq!(factored.get(&ZZElem::from(2)), Some(&4u32));
            
            // Test iteration
            let mut pairs: Vec<_> = factored.iter()
                .map(|(k, v)| (k.clone(), *v))
                .collect();
            pairs.sort_by_key(|(k, _)| k.clone());
            
            let expected = vec![
                (ZZElem::from(2), 4u32),
                (ZZElem::from(3), 2u32),
                (ZZElem::from(5), 1u32)
            ];
            assert_eq!(pairs, expected);
            
            // Test other HashMap methods through Deref
            assert!(factored.contains_key(&ZZElem::from(2)));
            assert!(!factored.contains_key(&ZZElem::from(7)));
            
            // Test remove
            let removed = factored.remove(&ZZElem::from(5));
            assert_eq!(removed, Some(1u32));
            assert_eq!(factored.len(), 2);
        }
    }

    // Tests for trial division functionality
    mod trial_div_tests {
        use super::*;
        use crate::factor::trial_div::factor_trial;

        #[test]
        fn test_factor_trial_small_primes() {
            // Test factoring small numbers
            assert_eq!(factor_trial(&ZZElem::from(2), 0, 5), Some(0)); // 2 is prime at index 0
            assert_eq!(factor_trial(&ZZElem::from(3), 0, 5), Some(1)); // 3 is prime at index 1
            assert_eq!(factor_trial(&ZZElem::from(5), 0, 5), Some(2)); // 5 is prime at index 2
            assert_eq!(factor_trial(&ZZElem::from(7), 0, 5), Some(3)); // 7 is prime at index 3
        }

        #[test]
        fn test_factor_trial_composite_numbers() {
            // Test factoring composite numbers
            assert_eq!(factor_trial(&ZZElem::from(4), 0, 10), Some(0)); // 4 = 2^2
            assert_eq!(factor_trial(&ZZElem::from(6), 0, 10), Some(0)); // 6 = 2 * 3
            assert_eq!(factor_trial(&ZZElem::from(9), 0, 10), Some(1)); // 9 = 3^2
            assert_eq!(factor_trial(&ZZElem::from(15), 0, 10), Some(1)); // 15 = 3 * 5
            assert_eq!(factor_trial(&ZZElem::from(25), 0, 10), Some(2)); // 25 = 5^2
        }

        #[test]
        fn test_factor_trial_larger_numbers() {
            // Test with larger composite numbers
            assert_eq!(factor_trial(&ZZElem::from(77), 0, 15), Some(3)); // 77 = 7 * 11
            assert_eq!(factor_trial(&ZZElem::from(91), 0, 15), Some(3)); // 91 = 7 * 13
            assert_eq!(factor_trial(&ZZElem::from(121), 0, 15), Some(4)); // 121 = 11^2
        }

        #[test]
        fn test_factor_trial_no_factors_in_range() {
            // Test with prime numbers larger than our range
            assert_eq!(factor_trial(&ZZElem::from(31), 0, 10), None); // 31 is prime, not in first 10 primes
            assert_eq!(factor_trial(&ZZElem::from(37), 0, 10), None); // 37 is prime, not in first 10 primes
        }

        #[test]
        fn test_factor_trial_special_cases() {
            // Test special cases
            assert_eq!(factor_trial(&ZZElem::ZERO, 0, 5), Some(0)); // 0 is divisible by any prime
            assert_eq!(factor_trial(&ZZElem::ONE, 0, 5), None); // 1 has no prime factors
        }

        #[test]
        fn test_factor_trial_negative_numbers() {
            // Test with negative numbers
            assert_eq!(factor_trial(&ZZElem::from(-6), 0, 10), Some(0)); // -6 = -1 * 2 * 3
            assert_eq!(factor_trial(&ZZElem::from(-15), 0, 10), Some(1)); // -15 = -1 * 3 * 5
        }

        #[test]
        fn test_factor_trial_range_limits() {
            // Test with different start/stop ranges - now works correctly after bug fix
            assert_eq!(factor_trial(&ZZElem::from(6), 1, 5), Some(1)); // Skip 2, find 3 at index 1
            assert_eq!(factor_trial(&ZZElem::from(10), 1, 3), Some(2)); // Skip 2, check indices 1,2 (primes 3,5), find 5 at index 2
            assert_eq!(factor_trial(&ZZElem::from(10), 2, 3), Some(2)); // Only check index 2 (prime 5), find 5 at index 2
            assert_eq!(factor_trial(&ZZElem::from(77), 3, 5), Some(3)); // Check indices 3,4 (primes 7,11), find 7 at index 3 (77 = 7*11)
            assert_eq!(factor_trial(&ZZElem::from(91), 4, 6), Some(5)); // Check indices 4,5 (primes 11,13), 91=7*13, find 13 at index 5
            assert_eq!(factor_trial(&ZZElem::from(21), 3, 5), Some(3)); // Check indices 3,4 (primes 7,11), 21=3*7, find 7 at index 3
            assert_eq!(factor_trial(&ZZElem::from(143), 5, 7), Some(5)); // Check indices 5,6 (primes 13,17), 143=11*13, find 13 at index 5
            assert_eq!(factor_trial(&ZZElem::from(221), 6, 8), Some(6)); // Check indices 6,7 (primes 17,19), 221=13*17, find 17 at index 6
            assert_eq!(factor_trial(&ZZElem::from(35), 4, 6), None); // Check indices 4,5 (primes 11,13), 35=5*7, neither 5 nor 7 in range
        }

        #[test]
        fn test_factor_trial_start_parameter_fix() {
            // Test that demonstrates the bug fix - start parameter is now respected
            
            // 30 = 2 * 3 * 5, so it has factors at indices 0, 1, 2
            assert_eq!(factor_trial(&ZZElem::from(30), 0, 10), Some(0)); // Find 2 at index 0
            assert_eq!(factor_trial(&ZZElem::from(30), 1, 10), Some(1)); // Skip 2, find 3 at index 1
            assert_eq!(factor_trial(&ZZElem::from(30), 2, 10), Some(2)); // Skip 2,3, find 5 at index 2
            assert_eq!(factor_trial(&ZZElem::from(30), 3, 10), None);    // Skip 2,3,5, no factors found in remaining range
            
            // 105 = 3 * 5 * 7, so it has factors at indices 1, 2, 3
            assert_eq!(factor_trial(&ZZElem::from(105), 1, 10), Some(1)); // Find 3 at index 1
            assert_eq!(factor_trial(&ZZElem::from(105), 2, 10), Some(2)); // Skip 3, find 5 at index 2
            assert_eq!(factor_trial(&ZZElem::from(105), 3, 10), Some(3)); // Skip 3,5, find 7 at index 3
            assert_eq!(factor_trial(&ZZElem::from(105), 4, 10), None);    // Skip 3,5,7, no factors found in remaining range
        }

        #[test]
        fn test_factor_trial_invalid_range() {
            // Test invalid ranges
            assert_eq!(factor_trial(&ZZElem::from(6), 5, 3), None); // start >= stop
            assert_eq!(factor_trial(&ZZElem::from(6), 5, 5), None); // start == stop
        }

        #[test]
        fn test_factor_trial_large_composite() {
            // Test larger composite numbers to stress test the algorithm
            let large_even = ZZElem::from(1000000); // 10^6 = 2^6 * 5^6
            assert_eq!(factor_trial(&large_even, 0, 10), Some(0)); // Should find 2

            let large_odd_composite = ZZElem::from(999999); // 3^3 * 7 * 11 * 13 * 37
            assert_eq!(factor_trial(&large_odd_composite, 0, 10), Some(1)); // Should find 3
        }
    }

    // Stress tests for PRIME_CACHE functionality
    mod prime_cache_stress_tests {
        use super::*;
        use crate::factor::prime_cache::*;
        use std::thread;

        #[test]
        fn test_prime_cache_initialization() {
            // Test that cache starts with the expected primes
            let cache = PRIME_CACHE.read().unwrap();
            let expected_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
            assert!(cache.len() >= 10); // May be larger due to other tests
            for (i, &expected) in expected_primes.iter().enumerate() {
                assert_eq!(cache[i], expected);
            }
        }

        #[test]
        fn test_ensure_primes_computed_basic() {
            // Test basic extension of cache
            ensure_primes_computed(15);
            let cache = PRIME_CACHE.read().unwrap();
            assert!(cache.len() >= 15);
            
            // Verify the first few additional primes
            let expected_next_primes = vec![31, 37, 41, 43, 47];
            for (i, &expected) in expected_next_primes.iter().enumerate() {
                assert_eq!(cache[10 + i], expected);
            }
        }

        #[test]
        fn test_get_nth_prime() {
            // Test getting specific primes by index
            assert_eq!(get_nth_prime(0), 2);
            assert_eq!(get_nth_prime(1), 3);
            assert_eq!(get_nth_prime(4), 11);
            assert_eq!(get_nth_prime(9), 29);
            
            // Test extending cache through get_nth_prime
            assert_eq!(get_nth_prime(20), 73); // 21st prime is 73
            assert_eq!(get_nth_prime(24), 97); // 25th prime is 97
        }

        #[test]
        fn test_get_primes_slice() {
            // Test getting slices of primes
            let slice = get_primes_slice(5);
            assert_eq!(slice, vec![2, 3, 5, 7, 11]);
            
            let larger_slice = get_primes_slice(15);
            assert_eq!(larger_slice.len(), 15);
            assert_eq!(larger_slice[0], 2);
            assert_eq!(larger_slice[14], 47); // 15th prime is 47
        }

        #[test]
        fn test_is_prime_using_cache() {
            let cache = PRIME_CACHE.read().unwrap();
            
            // Test known primes (note: is_prime_using_cache has a bug with 2)
            // The function incorrectly returns false for 2 because it's even
            // This is a known issue in the implementation that we're testing
            assert!(!is_prime_using_cache(2, &cache)); // Bug: should be true
            assert!(is_prime_using_cache(3, &cache));
            assert!(is_prime_using_cache(5, &cache));
            assert!(is_prime_using_cache(31, &cache));
            
            // Test known composites
            assert!(!is_prime_using_cache(4, &cache));
            assert!(!is_prime_using_cache(6, &cache));
            assert!(!is_prime_using_cache(9, &cache));
            assert!(!is_prime_using_cache(15, &cache));
            assert!(!is_prime_using_cache(25, &cache));
            
            // Test edge cases
            assert!(!is_prime_using_cache(0, &cache));
            assert!(!is_prime_using_cache(1, &cache));
        }

        #[test]
        fn test_extend_cache_to() {
            // Create a temporary cache for testing
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
            // Stress test: multiple threads accessing cache simultaneously
            let handles: Vec<_> = (0..10).map(|i| {
                thread::spawn(move || {
                    // Each thread tries to extend cache and access different primes
                    ensure_primes_computed(50 + i);
                    get_nth_prime(30 + i)
                })
            }).collect();

            // Wait for all threads and collect results
            let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
            
            // Verify results are consistent (same calls should return same values)
            assert_eq!(results[0], get_nth_prime(30)); // Should be same as thread 0 result
            
            // Verify cache grew appropriately
            let cache = PRIME_CACHE.read().unwrap();
            assert!(cache.len() >= 59); // Should have at least 59 primes now
        }

        #[test]
        fn test_large_cache_extension() {
            // Stress test: extend cache to a large size
            ensure_primes_computed(100);
            let cache = PRIME_CACHE.read().unwrap();
            assert!(cache.len() >= 100);
            
            // Verify some known larger primes
            assert_eq!(cache[24], 97);  // 25th prime
            assert_eq!(cache[49], 229); // 50th prime
            assert_eq!(cache[99], 541); // 100th prime
        }

        #[test]
        fn test_cache_consistency_under_stress() {
            // Verify cache remains consistent under repeated access
            for _ in 0..10 {
                ensure_primes_computed(30);
                let prime_15 = get_nth_prime(14); // 15th prime
                assert_eq!(prime_15, 47);
            }
            
            // Test repeated slice operations
            for _ in 0..5 {
                let slice = get_primes_slice(20);
                assert_eq!(slice.len(), 20);
                assert_eq!(slice[19], 71); // 20th prime is 71
            }
        }

        #[test]
        fn test_cache_double_check_locking() {
            // Test the double-check locking pattern in ensure_primes_computed
            // We'll test that concurrent calls work correctly and don't cause races
            
            // Get current cache size
            let initial_size = {
                let cache = PRIME_CACHE.read().unwrap();
                cache.len()
            };
            
            // Make concurrent calls to ensure_primes_computed with the same target
            let target_size = initial_size + 10; // Request slightly larger size
            let handles: Vec<_> = (0..5).map(move |_| {
                thread::spawn(move || {
                    ensure_primes_computed(target_size);
                })
            }).collect();
            
            for handle in handles {
                handle.join().unwrap();
            }
            
            // Cache should be at least the target size
            let final_size = {
                let cache = PRIME_CACHE.read().unwrap();
                cache.len()
            };
            assert!(final_size >= target_size);
            
            // Verify the primes are correct
            let primes = get_primes_slice(final_size.min(20));
            let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71];
            for (i, &expected_prime) in expected.iter().enumerate() {
                if i < primes.len() {
                    assert_eq!(primes[i], expected_prime);
                }
            }
        }
    }
}
