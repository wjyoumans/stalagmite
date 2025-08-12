use crate::factor::prime_cache::{ensure_primes_computed, get_prime_cache};
use crate::integer::ZZElem;
use malachite::base::num::arithmetic::traits::{DivisibleBy, Parity};

/// Rust translation of flint_mpn_factor_trial
/// Returns the index of the first prime that divides x, or None if no factor found
pub fn factor_trial(n: &ZZElem, start: usize, stop: usize) -> Option<usize> {
    if start >= stop {
        return None;
    }

    if *n == 0 {
        return Some(start); // 0 is divisible by any prime
    }

    if *n == 1 {
        return None; // 1 has no prime factors
    }

    // Ensure we have enough primes computed
    ensure_primes_computed(stop);

    // Get read-only access to prime cache
    let prime_cache = get_prime_cache().read().unwrap();

    // Special case: check divisibility by 2 if start = 0
    if start == 0 && stop > 0 {
        if n.even() {
            return Some(0); // Found factor 2 at index 0
        }
    }

    // Check primes starting from the requested start index
    let loop_start = if start == 0 { 1 } else { start }; // Skip 2 if we already checked it
    for i in loop_start..stop {
        if i >= prime_cache.len() {
            break; // Shouldn't happen if ensure_primes_computed worked correctly
        }

        let prime = prime_cache[i];

        // TODO: if moved into malachite, use Natural::divisible_by_limb (or make public).
        // Or impl Rem<u64> for Integer/&Integer?
        //
        // In any case ZZElem::from(prime) should be cheap since prime fits in a limb.
        if n.divisible_by(ZZElem::from(prime)) {
            return Some(i);
        }
    }

    None // No factors found in the range
}
