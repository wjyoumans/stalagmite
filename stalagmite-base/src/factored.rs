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

use malachite::base::num::arithmetic::traits::*;


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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factored_elem_multiplication_same_base() {
        let factored1 = FactoredElem::from([(ZZElem::from(2), 3u32)]);
        let factored2 = FactoredElem::from([(ZZElem::from(2), 2u32)]);
        let result = factored1 * factored2;
        
        assert_eq!(result.len(), 1);
        assert_eq!(result.get(&ZZElem::from(2)), Some(&5u32));
    }

    #[test]
    fn test_factored_elem_multiplication_different_bases() {
        let factored1 = FactoredElem::from([(ZZElem::from(2), 3u32)]);
        let factored2 = FactoredElem::from([(ZZElem::from(3), 2u32)]);
        let result = factored1 * factored2;
        
        assert_eq!(result.len(), 2);
        assert_eq!(result.get(&ZZElem::from(2)), Some(&3u32));
        assert_eq!(result.get(&ZZElem::from(3)), Some(&2u32));
    }

    #[test]
    fn test_factored_elem_multiplication_complex() {
        let factored1 = FactoredElem::from([
            (ZZElem::from(2), 3u32),
            (ZZElem::from(5), 1u32)
        ]);
        let factored2 = FactoredElem::from([
            (ZZElem::from(2), 1u32),
            (ZZElem::from(3), 2u32),
            (ZZElem::from(5), 2u32)
        ]);
        let result = factored1 * factored2;
        
        assert_eq!(result.len(), 3);
        assert_eq!(result.get(&ZZElem::from(2)), Some(&4u32));
        assert_eq!(result.get(&ZZElem::from(3)), Some(&2u32));
        assert_eq!(result.get(&ZZElem::from(5)), Some(&3u32));
    }

    #[test]
    fn test_factored_elem_multiplication_with_default() {
        let factored1 = FactoredElem::from([(ZZElem::from(2), 3u32)]);
        let default_factored: FactoredZZElem = FactoredElem::default();
        
        let result = factored1 * default_factored;
        
        // Default (empty) factored element doesn't add any factors
        assert_eq!(result.len(), 1);
        assert_eq!(result.get(&ZZElem::from(2)), Some(&3u32));
    }

    #[test]
    fn test_factored_qq_basic() {
        let factored: FactoredQQElem = FactoredElem::from([
            (QQElem::from(2), 2i32),
            (QQElem::from(3), -1i32)
        ]);
        
        assert_eq!(factored.len(), 2);
        assert_eq!(factored.get(&QQElem::from(2)), Some(&2i32));
        assert_eq!(factored.get(&QQElem::from(3)), Some(&-1i32));
    }

    #[test]
    fn test_factored_elem_constructors() {
        let empty: FactoredZZElem = FactoredElem::new();
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);
        
        let from_array = FactoredElem::from([(ZZElem::from(7), 3u32)]);
        assert_eq!(from_array.len(), 1);
        assert_eq!(from_array.get(&ZZElem::from(7)), Some(&3u32));
        
        let from_vec = FactoredElem::from(vec![
            (ZZElem::from(2), 2u32),
            (ZZElem::from(3), 1u32)
        ]);
        assert_eq!(from_vec.len(), 2);
        assert_eq!(from_vec.get(&ZZElem::from(2)), Some(&2u32));
        assert_eq!(from_vec.get(&ZZElem::from(3)), Some(&1u32));
        
        let pairs = [(ZZElem::from(5), 2u32), (ZZElem::from(11), 1u32)];
        let from_iter: FactoredZZElem = pairs.into_iter().collect();
        assert_eq!(from_iter.len(), 2);
        assert_eq!(from_iter.get(&ZZElem::from(5)), Some(&2u32));
        assert_eq!(from_iter.get(&ZZElem::from(11)), Some(&1u32));
        
        let with_cap: FactoredZZElem = FactoredElem::with_capacity(10);
        assert!(with_cap.is_empty());
    }

    #[test]
    fn test_factored_elem_operations() {
        let mut factored = FactoredElem::from([
            (ZZElem::from(2), 3u32),
            (ZZElem::from(5), 1u32)
        ]);
        
        let old_exp = factored.insert(ZZElem::from(3), 2u32);
        assert_eq!(old_exp, None);
        assert_eq!(factored.get(&ZZElem::from(3)), Some(&2u32));
        
        let old_exp = factored.insert(ZZElem::from(2), 4u32);
        assert_eq!(old_exp, Some(3u32));
        assert_eq!(factored.get(&ZZElem::from(2)), Some(&4u32));
        
        let mut pairs: Vec<_> = factored.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        pairs.sort_by_key(|(k, _)| k.clone());
        
        let expected = vec![
            (ZZElem::from(2), 4u32),
            (ZZElem::from(3), 2u32),
            (ZZElem::from(5), 1u32)
        ];
        assert_eq!(pairs, expected);
        
        assert!(factored.contains_key(&ZZElem::from(2)));
        assert!(!factored.contains_key(&ZZElem::from(7)));
        
        let removed = factored.remove(&ZZElem::from(5));
        assert_eq!(removed, Some(1u32));
        assert_eq!(factored.len(), 2);
    }

    #[test]
    fn test_eval_trait() {
        let factored_zz = FactoredElem::from([
            (ZZElem::from(2), 3u32),
            (ZZElem::from(3), 2u32)
        ]);
        let result = factored_zz.eval();
        assert_eq!(result, ZZElem::from(2u32.pow(3) * 3u32.pow(2)));
        
        let factored_qq = FactoredElem::from([
            (QQElem::from(2), 2i32),
            (QQElem::from(3), -1i32)
        ]);
        let result = factored_qq.eval();
        assert_eq!(result, QQElem::from(4) / QQElem::from(3));
    }
}
