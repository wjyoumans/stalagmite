
use malachite::base::num::arithmetic::traits::{Parity, DivisibleBy};
use crate::factor::*;
use crate::factor::prime_cache::{
    PRIME_CACHE,
    ensure_primes_computed,
};

/// Rust translation of flint_mpn_factor_trial
/// Returns the index of the first prime that divides x, or None if no factor found
pub fn factor_trial(
    n: &ZZElem, 
    start: usize, 
    stop: usize
) -> Option<usize> {
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
    let prime_cache = PRIME_CACHE.read().unwrap();
    
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


// TESTING //

/// Factors a ZZElem using trial division within a specified prime range.
/// Returns true if the number is completely factored, false if there's a remaining composite factor.
pub fn factor_trial_range(
    n: &ZZElem,
    start: usize,
    num_primes: usize
) -> FactoredZZ {
    /* TODO: Check if n is word-size, use existing code
    if n.is_small() {
        factor_small_integer(factor, n.to_i64().unwrap());
        return true;
    }
    */

    let factors = FactoredZZ::new();

    /*
    // Create a mutable copy of the limbs
    let mut limbs = n.limbs().to_vec();
    let mut limb_count = limbs.len();
    */
    
    // Handle sign
    factor.sign = if n.is_negative() {
        limbs = n.abs().limbs().to_vec();
        limb_count = limbs.len();
        -1
    } else {
        1
    };

    /*
    // Factor out powers of two if starting from the beginning
    if start == 0 {
        let exp = remove_powers_of_two(&mut limbs, &mut limb_count);
        if exp != 0 {
            factor.append(2, exp);
        }
    }
    */

    // Get read-only access to prime cache
    let prime_cache = PRIME_CACHE.read().unwrap();

    let trial_start = std::cmp::max(1, start);
    let mut current_start = trial_start;
    let end_range = start + num_primes;

    loop {
        let trial_stop = std::cmp::min(current_start + 1000, end_range);
        
        if current_start >= trial_stop {
            break;
        }

        let found_index = factor_trial(&n, current_start, trial_stop);

        if let Some(prime_idx) = found_index {
            // Get the prime that was found
            let p = prime_cache.get_nth_prime(prime_idx);
            let mut exp = 1;

            // Divide out the prime
            divide_limbs_by_word(&mut limbs, &mut limb_count, p);

            // Check for higher powers of the same prime
            if is_divisible_by_odd(&limbs, limb_count, p) {
                divide_limbs_by_word(&mut limbs, &mut limb_count, p);
                exp = 2;

                // If we found p^2, check for p^3 and higher powers
                if is_divisible_by_odd(&limbs, limb_count, p) {
                    divide_limbs_by_word(&mut limbs, &mut limb_count, p);
                    let additional_exp = remove_power_ascending(&mut limbs, &mut limb_count, p);
                    exp += 3 + additional_exp;
                }
            }

            factor.append(p, exp);

            // Continue trial division from where we found success
            current_start = prime_idx + 1;
        } else {
            // No factor found in this range, move to next block
            current_start = trial_stop;
        }

        // Break if we've completely factored the number
        if limb_count == 1 && limbs[0] == 1 {
            break;
        }
        
        // Break if we've exhausted our range
        if current_start >= end_range {
            break;
        }
    }

    // Return true if completely factored (remainder is 1)
    limb_count == 1 && limbs[0] == 1
}

/// Helper function to remove powers of 2 from the limbs
fn remove_powers_of_two(limbs: &mut Vec<u64>, limb_count: &mut usize) -> usize {
    if *limb_count == 0 {
        return 0;
    }

    let mut total_exp = 0;
    let mut i = 0;

    // Remove trailing zero limbs
    while i < *limb_count && limbs[i] == 0 {
        total_exp += 64;
        i += 1;
    }

    if i < *limb_count {
        // Remove trailing zeros from the first non-zero limb
        let trailing_zeros = limbs[i].trailing_zeros() as usize;
        total_exp += trailing_zeros;
        
        if trailing_zeros > 0 {
            limbs[i] >>= trailing_zeros;
            
            // Shift remaining limbs if needed
            if trailing_zeros < 64 {
                let shift = trailing_zeros;
                for j in i..*limb_count {
                    if j + 1 < *limb_count {
                        limbs[j] |= limbs[j + 1] << (64 - shift);
                    }
                    if j > i {
                        limbs[j] >>= shift;
                    }
                }
            }
        }
    }

    // Remove any leading zero limbs and shift down
    if i > 0 {
        for j in 0..(*limb_count - i) {
            limbs[j] = limbs[j + i];
        }
        *limb_count -= i;
    }

    // Remove any trailing zero limbs
    while *limb_count > 1 && limbs[*limb_count - 1] == 0 {
        *limb_count -= 1;
    }

    total_exp
}

/// Helper function to divide limbs by a single word
fn divide_limbs_by_word(limbs: &mut Vec<u64>, limb_count: &mut usize, divisor: u64) {
    if *limb_count == 0 {
        return;
    }

    let mut remainder = 0u64;
    for i in (0..*limb_count).rev() {
        let dividend = ((remainder as u128) << 64) | (limbs[i] as u128);
        limbs[i] = (dividend / divisor as u128) as u64;
        remainder = (dividend % divisor as u128) as u64;
    }

    // Remove leading zero limb if present
    if *limb_count > 1 && limbs[*limb_count - 1] == 0 {
        *limb_count -= 1;
    }
}

/// Helper function to check if limbs are divisible by an odd number
fn is_divisible_by_odd(limbs: &[u64], limb_count: usize, divisor: u64) -> bool {
    if limb_count == 0 {
        return true;
    }

    let mut remainder = 0u64;
    for i in (0..limb_count).rev() {
        let dividend = ((remainder as u128) << 64) | (limbs[i] as u128);
        remainder = (dividend % divisor as u128) as u64;
    }
    remainder == 0
}

/// Helper function to remove additional powers of a prime
fn remove_power_ascending(limbs: &mut Vec<u64>, limb_count: &mut usize, prime: u64) -> usize {
    let mut additional_exp = 0;
    
    while *limb_count > 1 || limbs[0] != 1 {
        if !is_divisible_by_odd(limbs, *limb_count, prime) {
            break;
        }
        divide_limbs_by_word(limbs, limb_count, prime);
        additional_exp += 1;
    }
    
    additional_exp
}

/// Helper function to factor small integers
fn factor_small_integer(factor: &mut FactoredZZ, n: i64) {
    // Implementation depends on your ZZFactorization structure
    // This is a placeholder - you'll need to implement based on your actual types
    unimplemented!()
}
