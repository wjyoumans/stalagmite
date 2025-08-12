use crate::integer::ZZElem;
use crate::rational::QQElem;
use std::collections::{
    HashMap,
    hash_map::Entry,
};
//use std::cmp::Eq;
use std::hash::Hash;
use std::mem::swap;
use std::ops::{Deref, DerefMut, Mul};
use malachite::base::num::basic::integers::PrimitiveInt;
use malachite::base::num::basic::traits::One;
use malachite::base::num::arithmetic::traits::Pow;


#[derive(Debug, Default, Clone)]
pub struct FactoredElem<T, E: PrimitiveInt> {
    pub factors: HashMap<T, E>
}

impl<T, E> FactoredElem<T, E> where
    //T: Eq + Hash,
    E: PrimitiveInt
{
    /// Creates an empty FactoredElem.
    #[inline]
    pub fn new() -> Self {
        FactoredElem { factors: HashMap::new() }
    }

    /// Creates a FactoredElem with the given capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        FactoredElem { 
            factors: HashMap::with_capacity(capacity) 
        }
    }
}

impl<T, E: PrimitiveInt> Deref for FactoredElem<T, E> {
    type Target = HashMap<T, E>;
    fn deref(&self) -> &Self::Target {
        &self.factors
    }
}

impl<T, E: PrimitiveInt> DerefMut for FactoredElem<T, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.factors
    }
}

// From implementations for convenient construction
impl<T, E> From<HashMap<T, E>> for FactoredElem<T, E> where
    T: Eq + Hash,
    E: PrimitiveInt
{
    fn from(factors: HashMap<T, E>) -> Self {
        FactoredElem { factors }
    }
}

impl<T, E, const N: usize> From<[(T, E); N]> for FactoredElem<T, E> where
    T: Eq + Hash,
    E: PrimitiveInt
{
    fn from(arr: [(T, E); N]) -> Self {
        FactoredElem { 
            factors: HashMap::from(arr) 
        }
    }
}

impl<T, E> From<Vec<(T, E)>> for FactoredElem<T, E> where
    T: Eq + Hash,
    E: PrimitiveInt
{
    fn from(vec: Vec<(T, E)>) -> Self {
        FactoredElem { 
            factors: HashMap::from_iter(vec) 
        }
    }
}

impl<T, E> FromIterator<(T, E)> for FactoredElem<T, E> where
    T: Eq + Hash,
    E: PrimitiveInt
{
    fn from_iter<I: IntoIterator<Item = (T, E)>>(iter: I) -> Self {
        FactoredElem { 
            factors: HashMap::from_iter(iter) 
        }
    }
}

// Merge hash maps, combining exponents and removing entries with exponent 0
impl<T, E> Mul for FactoredElem<T, E> where
    T: Eq + Hash,
    E: PrimitiveInt
{
    type Output = Self;
    fn mul(mut self, mut rhs: Self) -> Self {
        if self.len() < rhs.len() {
            swap(&mut self, &mut rhs);
        }
        let mut factors = self.factors;
        let other_factors = rhs.factors;
        
        for (fac, exp) in other_factors {
            match factors.entry(fac) {
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() += exp;
                    if *entry.get() == E::ZERO {
                        let _ = entry.remove_entry();
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(exp);
                }
            }
        }
        FactoredElem { factors }
    }
}


pub type FactoredZZElem = FactoredElem<ZZElem, u32>;
pub type FactoredQQElem = FactoredElem<QQElem, i32>;


pub trait Eval {
    type Output;
    fn eval(self) -> Self::Output;
}

impl Eval for FactoredZZElem {
    type Output = ZZElem;
    fn eval(self) -> Self::Output {        
        self.factors.into_iter().fold(ZZElem::ONE, |acc, (fac, exp)| acc * fac.pow(exp as u64))
    }
}

impl Eval for FactoredQQElem {
    type Output = QQElem;
    fn eval(self) -> Self::Output {                
        self.factors.into_iter().fold(QQElem::ONE, |acc, (fac, exp)| acc * fac.pow(exp as i64))
    }
}
