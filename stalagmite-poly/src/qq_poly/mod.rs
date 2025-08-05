// Copyright (C) 2025 William Youmans
//
// This file is part of Stalagmite.
//
// Stalagmite is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published
// by the Free Software Foundation, either version 3 of the License,
// or (at your option) any later version.
//
// Stalagmite is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Stalagmite. If not, see <https://www.gnu.org/licenses/>.

pub mod arithmetic;
pub mod conversion;

use std::fmt;
use std::str::FromStr;
use malachite::{Integer, Natural};
use malachite::base::num::arithmetic::traits::{Abs, Gcd};
use crate::zz_poly::ZZPoly;

pub use arithmetic::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RationalPolynomialRing;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RationalPolynomial {
    numerator: ZZPoly,
    denominator: Natural,
}

pub type QQPolyRing = RationalPolynomialRing;
pub type QQPoly = RationalPolynomial;

impl Default for QQPoly {
    #[inline]
    fn default() -> Self {
        QQPoly::from_raw(ZZPoly::zero(), Natural::from(1u32))
    }
}

impl fmt::Display for QQPoly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl QQPoly {
    #[inline]
    pub fn new<T: Into<QQPoly>>(input: T) -> Self {
        input.into()
    }
    
    pub fn normalize(&mut self) {
        // Normalize the numerator polynomial (removes leading zeros)
        self.numerator.normalize();
        
        // If polynomial is zero, set denominator to 1
        if self.numerator.is_zero() {
            self.denominator = Natural::from(1u32);
            return;
        }
        
        // Find GCD of all coefficients and denominator for simplification
        let mut gcd = self.denominator.clone();
        for coeff in self.numerator.iter() {
            if *coeff != 0 {
                // Convert abs Integer to Natural by taking abs and then converting
                let abs_coeff = coeff.abs();
                let nat_coeff = Natural::from_str(&abs_coeff.to_string()).unwrap();
                gcd = Natural::gcd(gcd.clone(), nat_coeff);
            }
        }
        
        // Simplify by dividing by GCD
        if gcd > 1 {
            // We need to create a new ZZPoly with simplified coefficients
            let mut new_coeffs = Vec::new();
            for coeff in self.numerator.iter() {
                new_coeffs.push(coeff / Integer::from(&gcd));
            }
            self.numerator = ZZPoly::from_raw(new_coeffs);
            self.denominator /= &gcd;
        }
    }
    
    pub fn from_raw(numerator: ZZPoly, denominator: Natural) -> Self {
        let mut p = QQPoly { numerator, denominator };
        p.normalize();
        p
    }
    
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        QQPoly::from_raw(ZZPoly::with_capacity(capacity), Natural::from(1u32))
    }
    
    #[inline]
    pub fn zero() -> Self {
        QQPoly::default()
    }

    #[inline]
    pub fn one() -> Self {
        QQPoly::from_raw(ZZPoly::one(), Natural::from(1u32))
    }

    #[inline]
    pub fn r#gen() -> Self {
        QQPoly::from_raw(ZZPoly::r#gen(), Natural::from(1u32))
    }
    
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.numerator.is_zero()
    }
    
    pub fn is_one(&self) -> bool {
        self.numerator.is_one() && self.denominator == 1
    }
    
    pub fn is_gen(&self) -> bool {
        self.numerator.is_gen() && self.denominator == 1
    }

    #[inline]
    pub fn length(&self) -> usize {
        self.numerator.length()
    }

    #[inline]
    pub fn degree(&self) -> usize {
        self.numerator.degree()
    }

    #[inline]
    pub fn numerator(&self) -> &ZZPoly {
        &self.numerator
    }

    #[inline]
    pub fn denominator(&self) -> &Natural {
        &self.denominator
    }

    // Get coefficient at given index (returns 0/1 if out of bounds)
    pub fn coeff(&self, i: usize) -> (Integer, Natural) {
        if i >= self.numerator.length() {
            (Integer::from(0), Natural::from(1u32))
        } else {
            (self.numerator[i].clone(), self.denominator.clone())
        }
    }
}