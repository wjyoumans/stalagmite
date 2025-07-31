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

use malachite::Integer;

pub use arithmetic::*;

#[derive(Debug, Clone)]
pub struct IntPoly {
    pub coeffs: Vec<Integer>,
}

impl Default for IntPoly {
    #[inline]
    fn default() -> Self {
        IntPoly::from_raw(vec![])
    }
}

// static ZERO: IntPoly = IntPoly { coeffs: vec![], length: 0 };

impl IntPoly {
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
    pub fn with_capacity(capacity: usize) -> Self {
        IntPoly::from_raw(Vec::with_capacity(capacity))
    }
    #[inline]
    pub fn zero() -> Self {
        IntPoly::default()
    }
    pub fn length(&self) -> usize {
        self.coeffs.len()
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

}