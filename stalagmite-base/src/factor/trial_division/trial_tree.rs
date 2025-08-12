
use malachite::base::num::arithmetic::traits::*;
use malachite::base::num::basic::traits::*;

use crate::LIMB_BITS;
use crate::factor::prime_cache::{
    ensure_primes_computed,
    get_prime_cache
};

use std::cmp::{min, max};
use std::sync::{OnceLock, RwLock};
use malachite::Natural;

const FACTOR_TREE_LEVELS: usize = 13 - (LIMB_BITS / 32);
const FACTOR_TREE_ENTRIES_PER_LEVEL: usize = 4096 / (LIMB_BITS / 16);


// Should be an easy optimization target: maybe use Boxed slice instead since size is const?
// E.g. as an array of arrays it could be [[0; ENTRIES_PER_LEVEL]; LEVELS]
#[derive(Debug)]
pub struct FactorTrialTree {
    pub tree: Vec<Vec<Natural>>,
    initialized: bool,
}

impl FactorTrialTree {
    const ENTRIES: usize = FACTOR_TREE_ENTRIES_PER_LEVEL;
    const LEVELS: usize = FACTOR_TREE_LEVELS;

    pub fn new() -> Self {
        Self {
            tree: vec![vec![Natural::ZERO; Self::ENTRIES]; Self::LEVELS],
            initialized: false,
        }
    }

    pub fn initialize(&mut self) {
        if self.initialized {
            return;
        }

        ensure_primes_computed(3512);

        self.build_first_layer();
        self.build_remaining_layers();
        self.initialized = true;
    }

    fn build_first_layer(&mut self) {
        let mut j = 0;
        let step = LIMB_BITS / 16;
        let cache = get_prime_cache().read().unwrap();
        
        for i in (0..3512).step_by(step) {
            if LIMB_BITS == 64 {
                self.tree[0][j] = Natural::from(cache[i]*cache[i+1]*cache[i+2]*cache[i+3]);
            } else {
                self.tree[0][j] = Natural::from(cache[i]*cache[i+1]);
            }
            j += 1;
        }
    }

    fn build_remaining_layers(&mut self) {
        let max_levels = Self::LEVELS - 1;
        let mut entries_count = 3512 / (LIMB_BITS / 16);

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

    // fn get_num_levels(&self) -> usize {
    //     self.tree.len()
    // }
    
    // fn get_level_size(&self, level: usize) -> usize {
    //     self.tree.get(level).map(|v| v.len()).unwrap_or(0)
    // }
}


static TRIAL_TREE: OnceLock<RwLock<FactorTrialTree>> = OnceLock::new();

pub fn get_trial_tree() -> &'static RwLock<FactorTrialTree> {
    TRIAL_TREE.get_or_init(|| {
        let mut tree = FactorTrialTree::new();
        tree.initialize();
        RwLock::new(tree)
    })
}

pub fn factor_trial_tree(x: Natural, num_primes: usize) -> Option<Vec<usize>> {
    if x <= 1 {
        return if x == 1 { Some(vec![]) } else { None };
    }
    
    let tree = get_trial_tree().read().unwrap();
    let cache = get_prime_cache().read().unwrap();
    
    let m = max(bit_count(num_primes).saturating_sub(LIMB_BITS / 32), 0);
    
    let entries_to_check = (num_primes + (LIMB_BITS / 16) - 1) / (LIMB_BITS / 16);
    let mut factors = Vec::new();
    
    for i in 0..entries_to_check {
        if should_check_group(&x, &tree, i, m) {
            // Check individual primes in this group
            for j in 0..(LIMB_BITS / 16) {
                let prime_idx = (LIMB_BITS / 16) * i + j;
                
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
    if n == 0 {
        0
    } else {
        (n.floor_log_base_2() + 1) as usize
    }
}
