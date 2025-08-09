use malachite::Natural;
use malachite::base::num::arithmetic::traits::ModMulPrecomputed;
use malachite::natural::arithmetic::mod_mul::ModMulData;
use malachite::natural::random::get_random_natural_less_than;
use malachite::base::num::random::{RandomPrimitiveInts, HasRandomPrimitiveInts};

use std::rc::Rc;

use stalagmite_base::traits::{
    Parent, Element,
    Ring, RingElement,
};

#[macro_export]
macro_rules! check_moduli {
    ($a:expr, $b:expr) => {
        if $a.modulus() != $b.modulus() {
            panic!(
                "Cannot perform operation on elements from different rings: Zn({}) and Zn({})", 
                $a.modulus(), $b.modulus()
            );
        }
    };
}

#[derive(Debug, Clone, Hash)]
pub(crate) struct IntegerModContext {
    modulus: Natural,
    mod_mul_data: ModMulData,
}

impl PartialEq for IntegerModContext {
    fn eq(&self, other: &Self) -> bool {
        self.modulus == other.modulus
    }
}

impl Eq for IntegerModContext {}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct IntegerModRing {
    ctx: Rc<IntegerModContext>
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct IntegerMod {
    value: Natural,
    ctx: Rc<IntegerModContext>
}

// Shortcuts
pub(crate) type ZnContext = IntegerModContext;
pub type ZnRing = IntegerModRing;
pub type ZnElem = IntegerMod;


impl std::fmt::Display for ZnRing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Zn({})", self.modulus())
    }
}

impl std::fmt::Display for ZnElem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl ZnContext {
    pub fn new(modulus: Natural) -> Self {
        let mod_mul_data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&modulus);
        Self { modulus, mod_mul_data }
    }

    pub fn mod_mul_data(&self) -> &ModMulData {
        &self.mod_mul_data
    }

    pub fn modulus(&self) -> &Natural {
        &self.modulus
    }
}

impl ZnRing {
    pub fn init(modulus: Natural) -> Self {
        Self { ctx: Rc::new(IntegerModContext::new(modulus)) }
    }

    pub fn new(&self, value: Natural) -> ZnElem {
        ZnElem::from_ctx(value, self.ctx.clone())
    }

    pub(crate) fn from_ctx(ctx: Rc<IntegerModContext>) -> Self {
        Self { ctx }
    }

    pub fn modulus(&self) -> &Natural {
        self.ctx.modulus()
    }

    pub fn mod_mul_data(&self) -> &ModMulData {
        self.ctx.mod_mul_data()
    }

    pub fn random_element(&self, rng: &mut RandomPrimitiveInts<u64>) -> ZnElem {
        let value = get_random_natural_less_than(rng, self.modulus());
        self.new(value)
    }
    
    pub fn random_elements(&self, rng: &mut RandomPrimitiveInts<u64>, count: usize) -> Vec<ZnElem> {
        (0..count).map(|_| self.random_element(rng)).collect()
    }
}

impl ZnElem {
    pub fn new(value: Natural, modulus: Natural) -> Self {
        Self { value, ctx: Rc::new(IntegerModContext::new(modulus)) }
    }

    pub(crate) fn from_ctx(value: Natural, ctx: Rc<IntegerModContext>) -> Self {
        Self { value, ctx }
    }

    pub fn value(&self) -> &Natural {
        &self.value
    }

    pub fn as_natural(&self) -> &Natural {
        &self.value
    }

    pub fn to_natural(self) -> Natural {
        self.value
    }

    pub fn into_natural(&self) -> Natural {
        self.value.clone()
    }

    pub fn modulus(&self) -> &Natural {
        self.ctx.modulus()
    }

    pub fn mod_mul_data(&self) -> &ModMulData {
        self.ctx.mod_mul_data()
    }
}

impl Parent for ZnRing {
    type Element = ZnElem;
}

impl Element for ZnElem {
    type Parent = ZnRing;
    fn parent(&self) -> Self::Parent {
        ZnRing::from_ctx(self.ctx.clone())  
    }
}

impl Ring for ZnRing {
    type Element = ZnElem;
}

impl RingElement for ZnElem {
    type Parent = ZnRing;
}

pub mod arithmetic;
