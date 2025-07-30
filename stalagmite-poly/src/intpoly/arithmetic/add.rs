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

use malachite::Integer;
use crate::intpoly::IntPoly;
use std::ops::{
    Add,
    AddAssign,
};
use std::iter::Sum;
use std::mem::swap;


impl Add for IntPoly {
    type Output = IntPoly;
    fn add(mut self, mut rhs: IntPoly) -> IntPoly {
        if self.coeffs.capacity() >= rhs.coeffs.capacity() {
            self += rhs;
            self
        } else {
            rhs += self;
            rhs
        }
    }
}

impl Add<&IntPoly> for IntPoly {
    type Output = IntPoly;
    #[inline]
    fn add(mut self, rhs: &IntPoly) -> IntPoly {
        self += rhs;
        self
    }
}

impl Add<IntPoly> for &IntPoly {
    type Output = IntPoly;
    #[inline]
    fn add(self, mut rhs: IntPoly) -> IntPoly {
        rhs += self;
        rhs
    }
}

impl Add<&IntPoly> for &IntPoly {
    type Output = IntPoly;
    fn add(self, rhs: &IntPoly) -> IntPoly {
        if rhs.is_zero() {
            return self.clone();
        }
        if self.is_zero() {
            return rhs.clone();
        }

        let n = self.length.max(rhs.length);
        let mut result = Vec::with_capacity(n);
        for i in 0..n {
            let a = if i < self.length { &self.coeffs[i] } else { &Integer::from(0) };
            let b = if i < rhs.length { &rhs.coeffs[i] } else { &Integer::from(0) };
            result.push(a + b);
        }
        IntPoly::from_raw(result, n)
    }
}


impl AddAssign<IntPoly> for IntPoly {
    fn add_assign(&mut self, mut rhs: IntPoly) {
        if rhs.is_zero() {
            return;
        } else if self.is_zero() {
            *self = rhs;
        } else {
            if self.length < rhs.length {
                swap(self, &mut rhs);
            }

            for i in 0..rhs.length {
                self.coeffs[i] += &rhs.coeffs[i];
            }
            self.normalize();
        }
    }
}

impl AddAssign<&IntPoly> for IntPoly {
    fn add_assign(&mut self, rhs: &IntPoly) {
        if rhs.is_zero() {
            return;
        } else if self.is_zero() {
            *self = rhs.clone();
        } else if self.length < rhs.length {
            // add the common coefficients
            for i in 0..self.length {
                self.coeffs[i] += &rhs.coeffs[i];
            }

            // copy the coefficients from rhs for the 
            // entries where self is initialized
            let n = self.coeffs.len().min(rhs.length);
            for i in self.length..n {
                self.coeffs[i] = rhs.coeffs[i].clone();
            }

            // push the remaining coefficients from rhs
            for i in n..rhs.length {
                self.coeffs.push(rhs.coeffs[i].clone());
            }
        } else {
            for i in 0..rhs.length {
                self.coeffs[i] += &rhs.coeffs[i];
            }
        }
        self.normalize();
    }
}

impl Sum for IntPoly {
    fn sum<I: Iterator<Item = IntPoly>>(iter: I) -> Self {
        iter.fold(IntPoly::zero(), |acc, x| acc + x)
    }
}

impl<'a> Sum<&'a IntPoly> for IntPoly {
    fn sum<I: Iterator<Item = &'a IntPoly>>(iter: I) -> Self {
        iter.fold(IntPoly::zero(), |acc, x| acc + x)
    }
}