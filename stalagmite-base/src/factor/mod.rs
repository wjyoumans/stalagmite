use crate::factor::trial_division::trial_range::factor_trial_range;
use crate::factored::FactoredNatural;

use malachite::Natural;
pub mod prime_cache;
pub mod trial_division;

// Redefine malachite::base::num::factorization::traits::Factor
pub trait Factor {
    type FACTORS;

    fn factor(&mut self) -> Self::FACTORS;
}

impl Factor for Natural {
    type FACTORS = FactoredNatural;

    fn factor(&mut self) -> FactoredNatural {
        // FLINT does this in batches of 1000. If trial division finds a factor
        // it continues to the next 1000, otherwise switch to non-trial factoring.
        let mut factors = factor_trial_range(self, 0, 3512);

        // factor_no_trial:
        // 1. check if prime
        // 2. check if perfect power
        //     - is_perfect_power modifies n in place
        //     - call factor_no_trial on base (may not be prime)
        // 3. factor smooth (ECM)
        // 4. check if remaining cofactor is perfect power (FLINT qsieve cant factor perfect powers)
        //     - if so, continue
        //     - otherwise call qsieve
        // 5. call factor_no_trial on each FACTOR, since they might not be prime

        //factors *= factor_no_trial(n);
        factors.unwrap()
    }
}

/*
fn factor_trial_range() {}
fn factor_trial() {}
fn factor_no_trial() {}

fn factor() {}
fn factor_smooth() {}

fn factor_pp1() {}
fn factor_refine() {}

fn factor_pollard_brent_single() {}
fn factor_pollard_brent() {}

fn factor_ecm_double() {}
fn factor_ecm_add() {}
fn factor_ecm_mul_montogomery_ladder() {}
fn factor_ecm_select_curve() {}
fn factor_ecm_stage_1() {}
fn factor_ecm_stage_2() {}
fn factor_ecm() {}

*/
