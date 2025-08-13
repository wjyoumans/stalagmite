use crate::factor::prime_cache::get_nth_prime_using_cache;
use crate::factored::FactoredNatural;
use malachite::Natural;
use malachite::base::num::arithmetic::traits::{
    DivExactAssign, DivisibleBy, Square,
};
use malachite::base::num::basic::traits::Two;
use malachite::base::num::factorization::traits::Factor;
use std::cmp::{max, min};

/// Factors a ZZElem using trial division within a specified prime range.
/// Returns the (potentially partial) factorization and remaining cofactor.
pub fn factor_trial_range(
    n: &mut Natural,
    start: usize,
    num_primes: usize,
) -> Option<FactoredNatural> {
    if *n == 0 {
        return None;
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
    if start == 0 {
        if let Some(exp) = n.trailing_zeros() {
            if exp != 0 {
                factors.insert(Natural::TWO, exp);
                *n >>= exp;
            }
        }
    }

    // return if we've completely factored the number
    if *n == 1 {
        return Some(factors);
    }

    let trial_start = max(1, start);
    let trial_stop = min(3512, start + num_primes);

    for i in trial_start..trial_stop {
        let p = Natural::from(get_nth_prime_using_cache(i));

        // Check if p divides n
        if (&*n).divisible_by(&p) {
            n.div_exact_assign(&p);
            let mut exp = 1;

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

        // Break if we've completely factored the number
        if *n == 1 {
            break;
        }
    }

    Some(factors)
}

pub fn remove_power(n: &mut Natural, p: Natural) -> Option<u64> {
    if *n == 0 {
        return None;
    }

    if p > *n {
        return Some(0);
    }

    if p == 2 {
        if let Some(exp) = n.trailing_zeros() {
            if exp != 0 {
                *n >>= exp;
            }
            return Some(exp);
        }
    }

    let mut exp: u64 = 0;

    // first check
    if (&*n).divisible_by(&p) {
        n.div_exact_assign(&p);
        exp += 1;
    } else {
        return Some(0);
    }

    exp += remove_power_ascending(n, p);
    Some(exp)
}

pub fn remove_power_ascending(n: &mut Natural, p: Natural) -> u64 {
    let mut exp: u64 = 0;

    // Store precomputed squares: square[i] = p^(2^i)
    let mut squares = Vec::new();
    squares.push(p.clone()); // square[0] = p^1

    // Phase 1: Ascending powers (binary lifting)
    // Try dividing by p^1, p^2, p^4, p^8, p^16, ...
    let mut i = 0;

    loop {
        // Check if we can divide by p^(2^i)
        if (&*n).divisible_by(&squares[i]) {
            n.div_exact_assign(&squares[i]);
        } else {
            i = i.saturating_sub(1);
            break;
        }
        // Successfully divided by p^(2^i)

        // Check for overflow?
        exp += 1 << i;

        let next_square = (&squares[i]).square();

        // Stop if the next square would be larger than remaining n
        if next_square > *n {
            break;
        }

        squares.push(next_square);
        i += 1;
    }

    // Phase 2: Descending powers (cleanup)
    loop {
        if squares[i] <= *n && (&*n).divisible_by(&squares[i]) {
            n.div_exact_assign(&squares[i]);
            exp += 1 << i
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    exp
}

#[cfg(test)]
mod tests {
    use super::*;
    use malachite::base::num::basic::traits::{One, Zero};

    #[test]
    fn test_factor_trial_range_basic() {
        let (factors, cofactor) = factor_trial_range(ZZElem::from(12), 0, 10);

        // 12 = 2^2 * 3
        assert_eq!(factors.get(&ZZElem::from(2)), Some(&2u32));
        assert_eq!(factors.get(&ZZElem::from(3)), Some(&1u32));
        assert_eq!(cofactor, ZZElem::ONE);
    }

    #[test]
    fn test_factor_trial_range_power_of_two() {
        let (factors, cofactor) = factor_trial_range(ZZElem::from(8), 0, 10);

        // 8 = 2^3
        assert_eq!(factors.get(&ZZElem::from(2)), Some(&3u32));
        assert_eq!(cofactor, ZZElem::ONE);
    }

    #[test]
    fn test_factor_trial_range_prime() {
        let (factors, cofactor) = factor_trial_range(ZZElem::from(7), 0, 10);

        // 7 is prime
        assert_eq!(factors.get(&ZZElem::from(7)), Some(&1u32));
        assert_eq!(cofactor, ZZElem::ONE);
    }

    #[test]
    fn test_factor_trial_range_special_cases() {
        let (factors, cofactor) = factor_trial_range(ZZElem::ONE, 0, 10);
        assert!(factors.is_empty());
        assert_eq!(cofactor, ZZElem::ONE);

        let (factors, cofactor) = factor_trial_range(ZZElem::ZERO, 0, 10);
        assert!(factors.is_empty());
        assert_eq!(cofactor, ZZElem::ZERO);
    }

    #[test]
    fn test_factor_trial_range_negative() {
        let (factors, cofactor) = factor_trial_range(ZZElem::from(-6), 0, 10);

        // -6 = -1 * 2 * 3
        assert_eq!(factors.get(&ZZElem::from(2)), Some(&1u32));
        assert_eq!(factors.get(&ZZElem::from(3)), Some(&1u32));
        assert_eq!(cofactor, ZZElem::from(-1));
    }

    #[test]
    fn test_factor_trial_range_partial_factorization() {
        // Use a number with a large prime factor outside our range
        let (factors, cofactor) = factor_trial_range(ZZElem::from(6 * 101), 0, 10);

        // Should factor out 2 and 3, leaving 101 as cofactor
        assert_eq!(factors.get(&ZZElem::from(2)), Some(&1u32));
        assert_eq!(factors.get(&ZZElem::from(3)), Some(&1u32));
        assert_eq!(cofactor, ZZElem::from(101));
    }

    #[test]
    fn test_factor_trial_range_skip_start() {
        let (factors, cofactor) = factor_trial_range(ZZElem::from(6), 1, 5);

        // Skip factor 2, should only find 3
        assert!(factors.get(&ZZElem::from(2)).is_none());
        assert_eq!(factors.get(&ZZElem::from(3)), Some(&1u32));
        assert_eq!(cofactor, ZZElem::from(2)); // 2 remains as cofactor
    }
}
