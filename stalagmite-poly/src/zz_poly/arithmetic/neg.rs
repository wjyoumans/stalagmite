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

use crate::zz_poly::ZZPoly;
use std::ops::Neg;
use malachite::base::num::arithmetic::traits::NegAssign;

/// Negate an owned `ZZPoly` by negating all coefficients.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let poly = ZZPoly::from(vec![1, -2, 3]);  // 1 - 2x + 3x²
/// let result = -poly;                        // -1 + 2x - 3x²
/// assert_eq!(result, ZZPoly::from(vec![-1, 2, -3]));
///
/// // Negating zero polynomial gives zero
/// let zero = ZZPoly::zero();
/// let neg_zero = -zero;
/// assert!(neg_zero.is_zero());
///
/// // Double negation returns original
/// let poly = ZZPoly::from(vec![5, -7, 2]);
/// let double_neg = -(-&poly);
/// assert_eq!(double_neg, poly);
/// ```
impl Neg for ZZPoly {
    type Output = ZZPoly;
    
    fn neg(mut self) -> ZZPoly {
        for coeff in &mut self.coeffs {
            *coeff = -&*coeff;
        }
        self
    }
}

/// Negate an `ZZPoly` reference by negating all coefficients.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let poly = ZZPoly::from(vec![1, -2, 3]);
/// let result = -&poly;
/// assert_eq!(result, ZZPoly::from(vec![-1, 2, -3]));
/// // Original polynomial is unchanged
/// assert_eq!(poly, ZZPoly::from(vec![1, -2, 3]));
///
/// // Negating a constant polynomial
/// let constant = ZZPoly::from(vec![42]);
/// let neg_constant = -&constant;
/// assert_eq!(neg_constant, ZZPoly::from(vec![-42]));
/// ```
impl Neg for &ZZPoly {
    type Output = ZZPoly;
    
    fn neg(self) -> ZZPoly {
        if self.is_zero() {
            return ZZPoly::zero();
        }
        
        let coeffs = self.coeffs.iter().map(|c| -c).collect();
        ZZPoly::from_raw(coeffs)
    }
}

impl NegAssign for ZZPoly {
    fn neg_assign(&mut self) {
        for coeff in &mut self.coeffs {
            *coeff = -&*coeff;
        }
    }
}