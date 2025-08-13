use malachite::Natural;

fn _is_prime(n: Natural, proved: bool) -> bool {
    // 1. if small use n_is_prime (NOT n_is_probable_prime)
    // 2. else:
    //    check if 0 mod 2
    //    trial_primes = bit_size * LIMB_BITS + (?)
    //    brute force check divisibility by primes up to trial_primes

    // 3. check if square, or maybe if perfect power
    // 4. if !proved return is_probable_prime_BPSW

    // 5. deterministic Miller-Rabin test up to ~81 bits (is_strong_probable_prime)
    // 6. is_strong_probably_prime(n, 2) rules out most composites (?)

    // 7. multi-modular reduction?
    // 8. is_prime_pocklington
    // 9. p-1, p+1 tests
    // 10. is_prime_morrison
    // 11. aprcl_is_prime

    false
}

fn is_prime(n: Natural) -> bool {
    _is_prime(n, true)
}

fn is_probable_prime(n: Natural) -> bool {
    _is_prime(n, false)
}

// Deterministic tests

pub fn is_prime_pocklington() -> bool {
    false
}

pub fn is_prime_morrison() -> bool {
    false
}

pub fn is_prime_aprcl() -> bool {
    false
}

// Probabalistic

use crate::factor::prime_cache::PRIME_CACHE;
pub fn is_prime_using_cache() -> bool {
    false
}

pub fn is_strong_probable_prime() -? bool {
    false
}

pub fn is_probable_prime_bpsw() -? bool {
    false
}
