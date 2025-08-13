use crate::factor::prime_cache::{ensure_primes_computed, get_prime_cache};
use crate::factor::trial_division::trial_range::remove_power_ascending;
use crate::factor::trial_division::trial_tree::factor_trial_tree;
use crate::factored::FactoredNatural;
use malachite::Natural;
use malachite::base::num::arithmetic::traits::{
    DivAssignRem, DivExactAssign, DivRem, DivisibleBy, NegAssign, Square,
};
use malachite::base::num::basic::traits::{Two, Zero};
use malachite::base::num::factorization::traits::Factor;
use malachite::natural::logic::trailing_zeros;

pub fn factor_trial(n: &mut Natural, num_primes: usize) -> Option<FactoredNatural> {
    if *n == 0 {
        return None;
    }

    if num_primes > 3512 {
        panic!("Number of primes must be in 0..3512");
    }

    let mut factors = FactoredNatural::new();

    if n.limb_count() == 1 {
        let fac = n.to_limbs_asc()[0].factor();
        for (p, exp) in fac {
            factors.insert(Natural::from(p), exp as u64);
        }
        return Some(factors);
    }

    // factor out powers of two
    if let Some(exp) = n.trailing_zeros() {
        if exp != 0 {
            factors.insert(Natural::TWO, exp);
            *n >>= exp;
        }
    }

    // return if we've completely factored the number
    if *n == 1 {
        return Some(factors);
    }

    let found = factor_trial_tree(n, num_primes).unwrap();

    // gcd tree will have expanded the prime cache up to num_primes
    let cache = get_prime_cache().read().unwrap();

    println!("{n} {found:?}");
    // remove powers of p for each p found using the gcd tree
    for i in found {
        // don't need to ensure the cache is expanded, so we access it directly
        let p = Natural::from(cache[i]);
        let mut exp = 1;

        // don't need to check divisibility
        n.div_exact_assign(&p);

        // Check if p^2 divides n
        if (&*n).divisible_by(&p) {
            n.div_exact_assign(&p);
            exp += 1;
        }

        // Check if p^3 divides n, then switch to specialized algorithm for higher powers
        if exp == 2 && (&*n).divisible_by(&p) {
            n.div_exact_assign(&p);
            exp += remove_power_ascending(n, p.clone()) + 1;
        }

        factors.insert(p, exp);
    }

    Some(factors)
}

/*
/// Rust translation of fmpz_factor_trial
/// Returns the index of the first prime that divides x, or None if no factor found
pub fn factor_trial(n: &Natural, start: usize, stop: usize) -> Option<usize> {
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

    let found = factor_trial_tree(n);

    for i in found {
        n.div_exact_assign()
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
        if n.divisible_by(Natural::from(prime)) {
            return Some(i);
        }
    }

    None // No factors found in the range
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use malachite::base::num::basic::traits::{One, Zero};

    #[test]
    fn test_factor_trial_small_primes() {
        assert_eq!(factor_trial(&ZZElem::from(2), 0, 5), Some(0)); // 2 is prime at index 0
        assert_eq!(factor_trial(&ZZElem::from(3), 0, 5), Some(1)); // 3 is prime at index 1
        assert_eq!(factor_trial(&ZZElem::from(5), 0, 5), Some(2)); // 5 is prime at index 2
        assert_eq!(factor_trial(&ZZElem::from(7), 0, 5), Some(3)); // 7 is prime at index 3
    }

    #[test]
    fn test_factor_trial_composite_numbers() {
        assert_eq!(factor_trial(&ZZElem::from(4), 0, 10), Some(0)); // 4 = 2^2
        assert_eq!(factor_trial(&ZZElem::from(6), 0, 10), Some(0)); // 6 = 2 * 3
        assert_eq!(factor_trial(&ZZElem::from(9), 0, 10), Some(1)); // 9 = 3^2
        assert_eq!(factor_trial(&ZZElem::from(15), 0, 10), Some(1)); // 15 = 3 * 5
        assert_eq!(factor_trial(&ZZElem::from(25), 0, 10), Some(2)); // 25 = 5^2
    }

    #[test]
    fn test_factor_trial_special_cases() {
        assert_eq!(factor_trial(&ZZElem::ZERO, 0, 5), Some(0)); // 0 is divisible by any prime
        assert_eq!(factor_trial(&ZZElem::ONE, 0, 5), None); // 1 has no prime factors
    }

    #[test]
    fn test_factor_trial_negative_numbers() {
        assert_eq!(factor_trial(&ZZElem::from(-6), 0, 10), Some(0)); // -6 = -1 * 2 * 3
        assert_eq!(factor_trial(&ZZElem::from(-15), 0, 10), Some(1)); // -15 = -1 * 3 * 5
    }

    #[test]
    fn test_factor_trial_range_limits() {
        assert_eq!(factor_trial(&ZZElem::from(6), 1, 5), Some(1)); // Skip 2, find 3 at index 1
        assert_eq!(factor_trial(&ZZElem::from(10), 2, 3), Some(2)); // Only check index 2 (prime 5)
    }

    #[test]
    fn test_factor_trial_invalid_range() {
        assert_eq!(factor_trial(&ZZElem::from(6), 5, 3), None); // start >= stop
        assert_eq!(factor_trial(&ZZElem::from(6), 5, 5), None); // start == stop
    }

    #[test]
    fn test_factor_trial_no_factors_in_range() {
        // Test with prime numbers larger than our range
        assert_eq!(factor_trial(&ZZElem::from(31), 0, 10), None); // 31 is prime, not in first 10 primes
        assert_eq!(factor_trial(&ZZElem::from(37), 0, 10), None); // 37 is prime, not in first 10 primes
    }
}
