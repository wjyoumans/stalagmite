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
pub mod comparison;
pub mod conversion;

use std::fmt;
use malachite::Integer;

pub use arithmetic::*;

#[derive(Debug, Clone)]
pub struct IntPoly {
    coeffs: Vec<Integer>,
}

impl Default for IntPoly {
    #[inline]
    fn default() -> Self {
        IntPoly::from_raw(vec![])
    }
}

// Impl Deref but NOT DerefMut.
impl std::ops::Deref for IntPoly {
    type Target = Vec<Integer>;

    fn deref(&self) -> &Self::Target {
        &self.coeffs
    }
}

impl fmt::Display for IntPoly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let len = self.length();
        if len == 0 {
            return write!(f, "0");
        } else if len == 1 {
            return write!(f, "{}", self.coeffs[0]);
        }

        let mut s = String::new();
        let mut iter = self.iter().enumerate().filter(|(_, coeff)| **coeff != 0).rev();
        
        // Dont print sign of highest order term if it is positive.
        // We can assume i > 1 and coeff != 0.
        if let Some((i, coeff)) = iter.next() {
            if *coeff == 1 {
                s.push_str(&format!("x^{}", i));
            } else if *coeff == -1 {
                s.push_str(&format!("-x^{}", i));
            } else {
                s.push_str(&format!("{}*x^{}", coeff, i));
            }
        }

        // now just avoid printing the exponent if it is 1 or 0.
        for (i, coeff) in iter {
            if i == 0 {
                if *coeff == 1 { s.push_str(" + 1"); }
                else if *coeff == -1 { s.push_str(" - 1"); }
                else if *coeff > 0 { s.push_str(&format!(" + {}", coeff)); } 
                else { s.push_str(&format!(" - {}", coeff.unsigned_abs_ref())); }
            } else if i == 1 {
                if *coeff == 1 { s.push_str(" + x"); }
                else if *coeff == -1 { s.push_str(" - x"); }
                else if *coeff > 0 { s.push_str(&format!(" + {}*x", coeff)); } 
                else { s.push_str(&format!(" - {}*x", coeff.unsigned_abs_ref())); }
            } else {
                if *coeff == 1 { s.push_str(&format!(" + x^{}", i)); }
                else if *coeff == -1 { s.push_str(&format!(" - x^{}", i)); }
                else if *coeff > 0 { s.push_str(&format!(" + {}*x^{}", coeff, i));} 
                else { s.push_str(&format!(" - {}*x^{}", coeff.unsigned_abs_ref(), i)); }
            }
        }
        write!(f, "{}", s)
    }
}

impl IntPoly {
    #[inline]
    pub fn new<T: Into<IntPoly>>(input: T) -> Self {
        input.into()
    }
    
    pub fn normalize(&mut self) {
        let mut new_len = self.coeffs.len();
        while new_len > 0 && self.coeffs[new_len - 1] == 0 {
            new_len -= 1;
        }
        self.coeffs.truncate(new_len);
    }
    
    pub fn from_raw(coeffs: Vec<Integer>) -> Self {
            let mut p = IntPoly { coeffs };
            p.normalize();
            p
    }
    
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        IntPoly::from_raw(Vec::with_capacity(capacity))
    }
    
    #[inline]
    pub fn zero() -> Self {
        IntPoly::default()
    }

    #[inline]
    pub fn one() -> Self {
        IntPoly::from(vec![1])
    }

    #[inline]
    pub fn r#gen() -> Self {
        IntPoly::from(vec![0, 1])
    }
    
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.length() == 0
    }
    
    pub fn is_one(&self) -> bool {
        self.length() == 1 && self.coeffs[0] == Integer::from(1)
    }
    
    pub fn is_gen(&self) -> bool {
        self.length() == 2 && self.coeffs[0] == 0 && self.coeffs[1] == Integer::from(1)
    }

    #[inline]
    pub fn length(&self) -> usize {
        self.coeffs.len()
    }

    #[inline]
    pub fn degree(&self) -> usize {
        if self.length() == 0 {
            return 0
        }
        self.length() - 1
    }

    // unsafe? user needs to be sure to normalize if needed.
    pub fn coeff_mut(&mut self, i: usize) -> &mut Integer {
        &mut self.coeffs[i]
    }

    // unsafe? user needs to be sure to normalize if needed.
    pub fn coeffs_mut(&mut self) -> &mut [Integer] {
        &mut self.coeffs
    }

    pub fn set_coeff(mut self, i: usize, coeff: Integer) {
        self.coeffs[i] = coeff;
    }
}
