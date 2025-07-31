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
    Rem,
    RemAssign,
};

/// Polynomial division with remainder.
/// 
/// Computes the remainder when dividing `dividend` by `divisor`.
/// This implements polynomial long division where we find polynomials
/// q and r such that dividend = q * divisor + r and degree(r) < degree(divisor).
/// 
/// Returns the remainder r.
/// 
/// # Panics
/// 
/// Panics if the divisor is zero or if the leading coefficient of the divisor
/// does not divide evenly into the leading coefficient of terms during division.
/// This is because we're working over integers, not a field.
fn polynomial_division_remainder(dividend: &[Integer], divisor: &[Integer]) -> Vec<Integer> {
    if divisor.is_empty() {
        panic!("Division by zero polynomial");
    }
    
    if dividend.is_empty() {
        return Vec::new();
    }
    
    let divisor_degree = divisor.len() - 1;
    let dividend_degree = dividend.len() - 1;
    
    // If dividend degree < divisor degree, remainder is the dividend
    if dividend_degree < divisor_degree {
        return dividend.to_vec();
    }
    
    let mut remainder = dividend.to_vec();
    let leading_coeff = &divisor[divisor_degree];
    
    if *leading_coeff == 0 {
        panic!("Division by polynomial with zero leading coefficient");
    }
    
    // Polynomial long division
    while remainder.len() > divisor_degree && !remainder.is_empty() {
        let remainder_degree = remainder.len() - 1;
        let remainder_leading = &remainder[remainder_degree];
        
        // Check if leading coefficient divides evenly
        if remainder_leading % leading_coeff != 0 {
            panic!("Leading coefficient {} does not divide {}", leading_coeff, remainder_leading);
        }
        
        let coeff = remainder_leading / leading_coeff;
        let degree_diff = remainder_degree - divisor_degree;
        
        // Subtract coeff * x^degree_diff * divisor from remainder
        for (i, divisor_coeff) in divisor.iter().enumerate() {
            let pos = i + degree_diff;
            if pos < remainder.len() {
                remainder[pos] -= &coeff * divisor_coeff;
            }
        }
        
        // Remove leading zeros
        while let Some(last) = remainder.last() {
            if *last == 0 {
                remainder.pop();
            } else {
                break;
            }
        }
    }
    
    remainder
}

/// Compute the remainder when dividing two owned `IntPoly` polynomials.
///
/// This operation performs polynomial long division and returns the remainder.
/// For polynomials A(x) and B(x), computes A(x) mod B(x) such that 
/// A(x) = Q(x) * B(x) + R(x) where degree(R) < degree(B).
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
///
/// // x² + 3x + 2 = (x + 1)(x + 2), so (x² + 3x + 2) mod (x + 1) = 0
/// let dividend = IntPoly::from(vec![2, 3, 1]);  // 2 + 3x + x²
/// let divisor = IntPoly::from(vec![1, 1]);      // 1 + x
/// let remainder = dividend % divisor;
/// assert!(remainder.is_zero());
///
/// // x² mod (x + 1) = 1 (since x² = (x+1)(x-1) + 1)
/// let dividend = IntPoly::from(vec![0, 0, 1]);  // x²
/// let divisor = IntPoly::from(vec![1, 1]);      // x + 1
/// let remainder = dividend % divisor;
/// assert_eq!(remainder, IntPoly::from(vec![1]));
///
/// // Lower degree dividend
/// let dividend = IntPoly::from(vec![3]);        // 3
/// let divisor = IntPoly::from(vec![1, 1]);      // 1 + x
/// let remainder = dividend % divisor;
/// assert_eq!(remainder, IntPoly::from(vec![3]));
/// ```
///
/// # Panics
///
/// Panics if the divisor is zero or if exact division is not possible
/// (when working over integers, some divisions may not be exact).
impl Rem for IntPoly {
    type Output = IntPoly;
    fn rem(self, rhs: IntPoly) -> IntPoly {
        if rhs.is_zero() {
            panic!("Division by zero polynomial");
        }
        if self.is_zero() {
            return IntPoly::zero();
        }

        let remainder = polynomial_division_remainder(&self.coeffs, &rhs.coeffs);
        IntPoly::from_raw(remainder)
    }
}

/// Compute the remainder when dividing an owned `IntPoly` by an `IntPoly` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
///
/// let dividend = IntPoly::from(vec![1, 2, 1]);  // 1 + 2x + x²
/// let divisor = IntPoly::from(vec![1, 1]);      // 1 + x
/// let remainder = dividend % &divisor;
/// assert!(remainder.is_zero());
/// ```
impl Rem<&IntPoly> for IntPoly {
    type Output = IntPoly;
    #[inline]
    fn rem(self, rhs: &IntPoly) -> IntPoly {
        if rhs.is_zero() {
            panic!("Division by zero polynomial");
        }
        if self.is_zero() {
            return IntPoly::zero();
        }

        let remainder = polynomial_division_remainder(&self.coeffs, &rhs.coeffs);
        IntPoly::from_raw(remainder)
    }
}

/// Compute the remainder when dividing an `IntPoly` reference by an owned `IntPoly`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
///
/// let dividend = IntPoly::from(vec![4, 4, 1]);  // 4 + 4x + x²
/// let divisor = IntPoly::from(vec![2, 1]);      // 2 + x
/// let remainder = &dividend % divisor;
/// assert!(remainder.is_zero());
/// ```
impl Rem<IntPoly> for &IntPoly {
    type Output = IntPoly;
    #[inline]
    fn rem(self, rhs: IntPoly) -> IntPoly {
        if rhs.is_zero() {
            panic!("Division by zero polynomial");
        }
        if self.is_zero() {
            return IntPoly::zero();
        }

        let remainder = polynomial_division_remainder(&self.coeffs, &rhs.coeffs);
        IntPoly::from_raw(remainder)
    }
}

/// Compute the remainder when dividing two `IntPoly` references.
///
/// This is the most memory-efficient remainder operation as it doesn't take ownership
/// of either polynomial and creates a new result.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
///
/// let dividend = IntPoly::from(vec![6, 7, 2]);  // 6 + 7x + 2x²
/// let divisor = IntPoly::from(vec![3, 1]);      // 3 + x
/// let remainder = &dividend % &divisor;
/// assert_eq!(remainder, IntPoly::from(vec![3]));  // remainder is 3
///
/// // Both polynomials remain unchanged
/// assert_eq!(dividend, IntPoly::from(vec![6, 7, 2]));
/// assert_eq!(divisor, IntPoly::from(vec![3, 1]));
///
/// // x³ mod (x² + 1) = x (since x³ = x(x² + 1) - x)
/// let dividend = IntPoly::from(vec![0, 0, 0, 1]);  // x³
/// let divisor = IntPoly::from(vec![1, 0, 1]);      // 1 + x²
/// let remainder = &dividend % &divisor;
/// assert_eq!(remainder, IntPoly::from(vec![0, -1]));  // -x
/// ```
impl Rem<&IntPoly> for &IntPoly {
    type Output = IntPoly;
    fn rem(self, rhs: &IntPoly) -> IntPoly {
        if rhs.is_zero() {
            panic!("Division by zero polynomial");
        }
        if self.is_zero() {
            return IntPoly::zero();
        }

        let remainder = polynomial_division_remainder(&self.coeffs, &rhs.coeffs);
        IntPoly::from_raw(remainder)
    }
}

/// Remainder-assign an owned `IntPoly` to this polynomial.
///
/// This modifies the left-hand side polynomial in place by computing
/// the remainder when dividing it by the right-hand side polynomial.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
///
/// let mut dividend = IntPoly::from(vec![2, 3, 1]);  // 2 + 3x + x²
/// let divisor = IntPoly::from(vec![1, 1]);          // 1 + x
/// dividend %= divisor;
/// assert!(dividend.is_zero());
///
/// // Another example: x² mod (x + 2)
/// let mut dividend = IntPoly::from(vec![0, 0, 1]);  // x²
/// let divisor = IntPoly::from(vec![2, 1]);          // 2 + x
/// dividend %= divisor;
/// assert_eq!(dividend, IntPoly::from(vec![4]));     // 4
/// ```
impl RemAssign<IntPoly> for IntPoly {
    fn rem_assign(&mut self, rhs: IntPoly) {
        *self = std::mem::take(self) % rhs;
    }
}

/// Remainder-assign an `IntPoly` reference to this polynomial.
///
/// This modifies the left-hand side polynomial in place by computing
/// the remainder when dividing it by the right-hand side polynomial,
/// without taking ownership of the RHS.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
///
/// let mut dividend = IntPoly::from(vec![6, 7, 2]);  // 6 + 7x + 2x²
/// let divisor = IntPoly::from(vec![3, 1]);          // 3 + x
/// dividend %= &divisor;
/// assert_eq!(dividend, IntPoly::from(vec![3]));
/// // divisor is still available for use
/// assert_eq!(divisor, IntPoly::from(vec![3, 1]));
/// ```
impl RemAssign<&IntPoly> for IntPoly {
    fn rem_assign(&mut self, rhs: &IntPoly) {
        *self = std::mem::take(self) % rhs;
    }
}