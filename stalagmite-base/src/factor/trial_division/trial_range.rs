use crate::factor::FactoredZZElem;
use crate::integer::ZZElem;
use malachite::base::num::arithmetic::traits::NegAssign;
use malachite::base::num::basic::traits::Two;

/// Factors a ZZElem using trial division within a specified prime range.
/// Returns the (potentially partial) factorization and remaining cofactor.
pub fn factor_trial_range(
    mut n: ZZElem,
    start: usize,
    num_primes: usize,
) -> (FactoredZZElem, ZZElem) {
    /* TODO: Check if n is word-size, use existing code
    if n.is_small() {
        factor_small_integer(factor, n.to_i64().unwrap());
        return true;
    }
    */

    let mut factors = FactoredZZElem::new();

    // handle sign
    if n < 0 {
        factors.insert(ZZElem::from(-1), 1);
        n.neg_assign();
    }

    // factor out powers of two
    if start == 0 {
        if let Some(exp) = n.trailing_zeros() {
            if exp != 0 {
                factors.insert(ZZElem::TWO, exp as u32);
                n >>= exp;
            }
        }
    }

    /*

    // Get read-only access to prime cache
    let prime_cache = get_prime_cache().read().unwrap();

    let trial_start = max(1, start);
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
    */
    (factors, n)
}
