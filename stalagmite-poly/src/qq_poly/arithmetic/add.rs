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

use std::ops::{Add, AddAssign};
use malachite::{Integer, Natural};
use malachite::base::num::arithmetic::traits::Lcm;
use crate::qq_poly::QQPoly;

// Addition: QQPoly + QQPoly
impl Add<QQPoly> for QQPoly {
    type Output = QQPoly;

    fn add(self, rhs: QQPoly) -> QQPoly {
        // Find common denominator
        let lcm_denom = Natural::lcm(self.denominator.clone(), rhs.denominator.clone());
        
        // Scale numerators to common denominator
        let left_scale = &lcm_denom / &self.denominator;
        let right_scale = &lcm_denom / &rhs.denominator;
        
        // Scale the numerator polynomials
        let mut left_num = self.numerator;
        let mut right_num = rhs.numerator;
        
        // Multiply coefficients by scaling factors
        if left_scale != 1 {
            let left_scale_int = Integer::from(&left_scale);
            for coeff in left_num.coeffs_mut() {
                *coeff *= &left_scale_int;
            }
        }
        
        if right_scale != 1 {
            let right_scale_int = Integer::from(&right_scale);
            for coeff in right_num.coeffs_mut() {
                *coeff *= &right_scale_int;
            }
        }
        
        // Add the scaled numerators
        let result_num = left_num + right_num;
        
        QQPoly::from_raw(result_num, lcm_denom)
    }
}

// Addition: QQPoly + &QQPoly
impl Add<&QQPoly> for QQPoly {
    type Output = QQPoly;

    fn add(self, rhs: &QQPoly) -> QQPoly {
        self + rhs.clone()
    }
}

// Addition: &QQPoly + QQPoly
impl Add<QQPoly> for &QQPoly {
    type Output = QQPoly;

    fn add(self, rhs: QQPoly) -> QQPoly {
        self.clone() + rhs
    }
}

// Addition: &QQPoly + &QQPoly
impl Add<&QQPoly> for &QQPoly {
    type Output = QQPoly;

    fn add(self, rhs: &QQPoly) -> QQPoly {
        self.clone() + rhs.clone()
    }
}

// AddAssign: QQPoly += QQPoly
impl AddAssign<QQPoly> for QQPoly {
    fn add_assign(&mut self, rhs: QQPoly) {
        *self = self.clone() + rhs;
    }
}

// AddAssign: QQPoly += &QQPoly
impl AddAssign<&QQPoly> for QQPoly {
    fn add_assign(&mut self, rhs: &QQPoly) {
        *self = self.clone() + rhs;
    }
}