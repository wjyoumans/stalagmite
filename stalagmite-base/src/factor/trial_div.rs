
use malachite::base::num::arithmetic::traits::*;
use malachite::base::num::basic::traits::*;
use crate::factor::*;
use crate::factor::prime_cache::{
    PRIME_CACHE,
    ensure_primes_computed,
    get_primes_using_cache
};
use std::cmp::{min, max};

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
/// Returns the (potentially partial) factorization and remaining cofactor.
pub fn factor_trial_range(
    mut n: ZZElem,
    start: usize,
    num_primes: usize
) -> (FactoredZZ, ZZElem) {
    /* TODO: Check if n is word-size, use existing code
    if n.is_small() {
        factor_small_integer(factor, n.to_i64().unwrap());
        return true;
    }
    */

    let mut factors = FactoredZZ::new();

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
    let prime_cache = PRIME_CACHE.read().unwrap();

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

/*
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
*/

use std::sync::{OnceLock, RwLock};
use malachite::Natural;

const FLINT_BITS: usize = malachite::platform::Limb::BITS as usize;
const FACTOR_TREE_LEVELS: usize = 13 - (FLINT_BITS / 32);
const FACTOR_TREE_ENTRIES_PER_LEVEL: usize = 4096 / (FLINT_BITS / 16);

#[derive(Debug)]
pub struct FactorTrialTree {
    pub tree: Vec<Vec<Natural>>,
    initialized: bool,
}

struct FactorTrialTreeArr<const N: usize, const L: usize> {
    tree: [[u64; N]; L],
    initialized: bool,
}

struct FactorTrialTree3 {
    tree: [[u64; FACTOR_TREE_ENTRIES_PER_LEVEL]; FACTOR_TREE_LEVELS],
    initialized: bool,
}

impl FactorTrialTree {
    const LEVELS: usize = 13 - (FLINT_BITS / 32);
    const ENTRIES_PER_LEVEL: usize = 4096 / (FLINT_BITS / 16);
    
    pub fn new() -> Self {
        Self {
            tree: Vec::new(),
            initialized: false,
        }
    }

    pub fn initialize(&mut self) {
        if self.initialized {
            return;
        }

        let levels = 13 - (FLINT_BITS / 32);
        self.tree = vec![Vec::new(); levels];

        // Calculate size for each level
        let entries_per_level = 4096 / (FLINT_BITS / 16);
        
        // Initialize all levels with appropriate capacity
        for i in 0..levels {
            self.tree[i] = vec![Natural::ZERO; entries_per_level];
        }

        ensure_primes_computed(3512);
        self.build_first_layer();
        self.build_remaining_layers();
        
        self.initialized = true;
    }

    fn build_first_layer(&mut self) {
        let mut j = 0;
        let step = FLINT_BITS / 16;
        let cache = PRIME_CACHE.read().unwrap();
        
        for i in (0..3512).step_by(step) {
            if FLINT_BITS == 64 {
                self.tree[0][j] = Natural::from(cache[i]*cache[i+1]*cache[i+2]*cache[i+3]);
            } else {
                self.tree[0][j] = Natural::from(cache[i]*cache[i+1]);
            }
            j += 1;
        }
    }

    fn build_remaining_layers(&mut self) {
        let max_levels = 12 - (FLINT_BITS / 32);
        let mut entries_count = 3512 / (FLINT_BITS / 16);

        for level in 0..max_levels {
            // Multiply adjacent entries in pairs
            let mut output_idx = 0;
            let mut input_idx = 0;

            // If implemented in malachite, use malachite::natural::arithmetic::mul::limbs_mul_same_length_to_out?

            while input_idx + 1 < entries_count {
                let result = &self.tree[level][input_idx] * &self.tree[level][input_idx + 1];
                                
                if level + 1 < self.tree.len() && output_idx < self.tree[level + 1].len() {
                    self.tree[level + 1][output_idx] = result;
                }
                
                input_idx += 2;
                output_idx += 1;
            }

            // Handle odd entries
            if entries_count % 2 == 1 && level + 1 < self.tree.len() {
                let last_idx = entries_count - 1;
                if last_idx < self.tree[level].len() && output_idx < self.tree[level + 1].len() {
                    self.tree[level + 1][output_idx] = self.tree[level][last_idx].clone();
                }
            }

            entries_count = (entries_count + 1) / 2;
        }
    }


    fn get_entry(&self, level: usize, index: usize) -> Option<&Natural> {
        self.tree.get(level)?.get(index)
    }

    fn get_num_levels(&self) -> usize {
        self.tree.len()
    }
    
    fn get_level_size(&self, level: usize) -> usize {
        self.tree.get(level).map(|v| v.len()).unwrap_or(0)
    }
}

static TRIAL_TREE: OnceLock<RwLock<FactorTrialTree>> = OnceLock::new();

fn get_trial_tree() -> &'static RwLock<FactorTrialTree> {
    TRIAL_TREE.get_or_init(|| {
        let mut tree = FactorTrialTree::new();
        tree.initialize();
        RwLock::new(tree)
    })
}


// Alternative implementation if you prefer a more functional style
pub fn factor_trial_tree(x: Natural, num_primes: usize) -> Option<Vec<usize>> {
    if x <= 1 {
        return if x == 1 { Some(vec![]) } else { None };
    }
    
    let tree = get_trial_tree().read().unwrap();
    let cache = PRIME_CACHE.read().unwrap();
    
    let m = max(bit_count(num_primes).saturating_sub(FLINT_BITS / 32), 0);
    
    let entries_to_check = (num_primes + (FLINT_BITS / 16) - 1) / (FLINT_BITS / 16);
    let mut factors = Vec::new();
    
    for i in 0..entries_to_check {
        if should_check_group(&x, &tree, i, m) {
            // Check individual primes in this group
            for j in 0..(FLINT_BITS / 16) {
                let prime_idx = (FLINT_BITS / 16) * i + j;
                
                if prime_idx < cache.len() && prime_idx < num_primes {
                    if (&x).divisible_by(Natural::from(cache[prime_idx])) {
                        factors.push(prime_idx);
                    }
                }
            }
        }
    }
    
    Some(factors)
}

fn should_check_group(x: &Natural, tree: &FactorTrialTree, group_idx: usize, max_level: usize) -> bool {
    let mut current_gcd = x.clone();
    
    for level in (0..=max_level).rev() {
        // FIXED: Proper bit extraction for tree navigation
        let entry_idx = if level == max_level {
            0  // Root is always at index 0
        } else {
            (group_idx >> level) & ((1 << (max_level - level)) - 1)
        };
        
        if let Some(tree_entry) = tree.get_entry(level, entry_idx) {
            current_gcd = current_gcd.gcd(tree_entry);
            if current_gcd == 1 {
                return false;
            }
        } else {
            return false;
        }
    }
    
    true
}


fn bit_count(n: usize) -> usize {
    if n == 0 { 0 } else { 64 - (n.leading_zeros() as usize) }
}

// fn bit_count(n: usize) -> usize {
//     if n == 0 {
//         0
//     } else {
//         (n.floor_log_base_2() + 1) as usize
//     }
// }
