use crate::factored::FactoredZZElem;
use crate::integer::ZZElem;

pub mod prime_cache;
pub mod trial_division;

// Redefine malachite::base::num::factorization::traits::Factor
pub trait Factor {
    type FACTORS;

    fn factor(&self) -> Self::FACTORS;
}

impl Factor for ZZElem {
    type FACTORS = FactoredZZElem;

    fn factor(&self) -> FactoredZZElem {
        FactoredZZElem::new()
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
