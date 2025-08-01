use malachite::Natural;
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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct IntegerModContext {
    modulus: Natural
} 

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
        Self { modulus }
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

    pub fn modulus(&self) -> Natural {
        self.ctx.modulus.clone()
    }
}

impl ZnElem {
    pub fn new(value: Natural, modulus: Natural) -> Self {
        Self { value, ctx: Rc::new(IntegerModContext::new(modulus)) }
    }

    pub(crate) fn from_ctx(value: Natural, ctx: Rc<IntegerModContext>) -> Self {
        Self { value, ctx }
    }

    pub fn modulus(&self) -> Natural {
        self.ctx.modulus.clone()
    }

    pub fn value(&self) -> Natural {
        self.value.clone()
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