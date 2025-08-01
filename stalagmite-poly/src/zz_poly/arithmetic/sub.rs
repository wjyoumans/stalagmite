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
use malachite::base::num::arithmetic::traits::NegAssign;
use crate::zz_poly::ZZPoly;
use std::ops::{
    Sub,
    SubAssign,
};
use std::mem::swap;

/// Subtract two owned `ZZPoly` polynomials.
///
/// This operation subtracts corresponding coefficients and handles polynomials
/// of different lengths by treating missing coefficients as zero.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let p1 = ZZPoly::from(vec![5, 7, 9, 7]);     // 5 + 7x + 9x² + 7x³
/// let p2 = ZZPoly::from(vec![1, 2, 3]);        // 1 + 2x + 3x²
/// let result = p1 - p2;                         // 4 + 5x + 6x² + 7x³
/// assert_eq!(result, ZZPoly::from(vec![4, 5, 6, 7]));
///
/// // Subtracting polynomials of different lengths
/// let p1 = ZZPoly::from(vec![1, 2]);
/// let p2 = ZZPoly::from(vec![3, 4, 5]);
/// let result = p1 - p2;
/// assert_eq!(result, ZZPoly::from(vec![-2, -2, -5]));
///
/// // Subtracting from self gives zero
/// let p = ZZPoly::from(vec![1, 2, 3]);
/// let result = &p - &p;
/// assert!(result.is_zero());
/// ```
impl Sub for ZZPoly {
    type Output = ZZPoly;
    fn sub(mut self, rhs: ZZPoly) -> ZZPoly {
        if self.length() >= rhs.length() {
            self -= rhs;
            self
        } else {
            self + (-rhs)
        }
    }
}

/// Subtract an `ZZPoly` reference from an owned `ZZPoly`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let p1 = ZZPoly::from(vec![5, 7, 3]);
/// let p2 = ZZPoly::from(vec![1, 2]);
/// let result = p1 - &p2;
/// assert_eq!(result, ZZPoly::from(vec![4, 5, 3]));
///
/// // Subtracting zero doesn't change polynomial
/// let p1 = ZZPoly::from(vec![1, 2]);
/// let zero = ZZPoly::zero();
/// let result = p1 - &zero;
/// assert_eq!(result, ZZPoly::from(vec![1, 2]));
/// ```
impl Sub<&ZZPoly> for ZZPoly {
    type Output = ZZPoly;
    #[inline]
    fn sub(mut self, rhs: &ZZPoly) -> ZZPoly {
        self -= rhs;
        self
    }
}

/// Subtract an owned `ZZPoly` from an `ZZPoly` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let p1 = ZZPoly::from(vec![5, 7, 9, 4]);
/// let p2 = ZZPoly::from(vec![1, 2, 3]);
/// let result = &p1 - p2;
/// assert_eq!(result, ZZPoly::from(vec![4, 5, 6, 4]));
/// ```
impl Sub<ZZPoly> for &ZZPoly {
    type Output = ZZPoly;
    #[inline]
    fn sub(self, rhs: ZZPoly) -> ZZPoly {
        self + (-rhs)
    }
}

/// Subtract two `ZZPoly` references.
///
/// This is the most memory-efficient subtraction as it doesn't take ownership
/// of either polynomial and creates a new result.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let p1 = ZZPoly::from(vec![5, 7, 9, -5]);
/// let p2 = ZZPoly::from(vec![1, 2, 3]);
/// let result = &p1 - &p2;
/// assert_eq!(result, ZZPoly::from(vec![4, 5, 6, -5]));
///
/// // Both polynomials remain unchanged
/// assert_eq!(p1, ZZPoly::from(vec![5, 7, 9, -5]));
/// assert_eq!(p2, ZZPoly::from(vec![1, 2, 3]));
///
/// // Zero polynomial handling
/// let p = ZZPoly::from(vec![1, 2, 3]);
/// let zero = ZZPoly::zero();
/// assert_eq!(&p - &zero, p);
/// assert_eq!(&zero - &p, -&p);
/// ```
impl Sub<&ZZPoly> for &ZZPoly {
    type Output = ZZPoly;
    fn sub(self, rhs: &ZZPoly) -> ZZPoly {
        if rhs.is_zero() {
            return self.clone();
        }
        if self.is_zero() {
            return -rhs;
        }

        let n = self.length().max(rhs.length());
        let mut result = Vec::with_capacity(n);
        for i in 0..n {
            let a = if i < self.length() { &self.coeffs[i] } else { &Integer::from(0) };
            let b = if i < rhs.length() { &rhs.coeffs[i] } else { &Integer::from(0) };
            result.push(a - b);
        }
        ZZPoly::from_raw(result)
    }
}

/// Subtract-assign an owned `ZZPoly` from this polynomial.
///
/// This modifies the left-hand side polynomial in place by subtracting the
/// right-hand side polynomial from it.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let mut p1 = ZZPoly::from(vec![5, 7]);
/// let p2 = ZZPoly::from(vec![1, 2, 3]);
/// p1 -= p2;
/// assert_eq!(p1, ZZPoly::from(vec![4, 5, -3]));
///
/// // Subtracting zero doesn't change the polynomial
/// let mut p = ZZPoly::from(vec![1, 2]);
/// p -= ZZPoly::zero();
/// assert_eq!(p, ZZPoly::from(vec![1, 2]));
///
/// // Subtracting from zero polynomial
/// let mut zero = ZZPoly::zero();
/// zero -= ZZPoly::from(vec![3, 4]);
/// assert_eq!(zero, ZZPoly::from(vec![-3, -4]));
/// ```
impl SubAssign<ZZPoly> for ZZPoly {
    fn sub_assign(&mut self, mut rhs: ZZPoly) {
        if rhs.is_zero() {
            return;
        } else if self.is_zero() {
            *self = -rhs;
        } else {
            if self.length() < rhs.length() {
                rhs.neg_assign();
                swap(self, &mut rhs);
                for i in 0..rhs.length() {
                    self.coeffs[i] += &rhs.coeffs[i];
                }
            } else {
                for i in 0..rhs.length() {
                    self.coeffs[i] -= &rhs.coeffs[i];
                }
            }
        }
        self.normalize();
    }
}

/// Subtract-assign an `ZZPoly` reference from this polynomial.
///
/// This modifies the left-hand side polynomial in place by subtracting the
/// right-hand side polynomial from it, without taking ownership of the RHS.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let mut p1 = ZZPoly::from(vec![5, 7, 9]);
/// let p2 = ZZPoly::from(vec![1, 2, 3]);
/// p1 -= &p2;
/// assert_eq!(p1, ZZPoly::from(vec![4, 5, 6]));
/// // p2 is still available for use
/// assert_eq!(p2, ZZPoly::from(vec![1, 2, 3]));
///
/// // Different length polynomials
/// let mut p1 = ZZPoly::from(vec![1, 2]);
/// let p2 = ZZPoly::from(vec![3, 4, 5]);
/// p1 -= &p2;
/// assert_eq!(p1, ZZPoly::from(vec![-2, -2, -5]));
/// ```
impl SubAssign<&ZZPoly> for ZZPoly {
    fn sub_assign(&mut self, rhs: &ZZPoly) {
        if rhs.is_zero() {
            return;
        } else if self.is_zero() {
            *self = -rhs;
        } else {
            if self.length() < rhs.length() {
                // subtract the common coefficients
                for i in 0..self.length() {
                    self.coeffs[i] -= &rhs.coeffs[i];
                }
                // push the remaining coefficients from rhs
                for i in self.length()..rhs.length() {
                    self.coeffs.push(-rhs.coeffs[i].clone());
                }
            } else {
                for i in 0..rhs.length() {
                    self.coeffs[i] -= &rhs.coeffs[i];
                }
            }
        }
        self.normalize();
    }
}

// Subtracting Integer from ZZPoly
/// Subtract an owned `Integer` from an `ZZPoly`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![5, 2, 3]); // 5 + 2x + 3x²
/// let result = poly - Integer::from(2);    // (5 - 2) + 2x + 3x² = 3 + 2x + 3x²
/// assert_eq!(result, ZZPoly::from(vec![3, 2, 3]));
///
/// // Subtracting from zero polynomial
/// let zero = ZZPoly::zero();
/// let result = zero - Integer::from(42);
/// assert_eq!(result, ZZPoly::from(vec![-42]));
/// ```
impl Sub<Integer> for ZZPoly {
    type Output = ZZPoly;
    #[inline]
    fn sub(mut self, rhs: Integer) -> ZZPoly {
        self -= rhs;
        self
    }
}

/// Subtract an `Integer` reference from an `ZZPoly`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![10, 20]);
/// let num = Integer::from(5);
/// let result = poly - &num;
/// assert_eq!(result, ZZPoly::from(vec![5, 20]));
/// ```
impl Sub<&Integer> for ZZPoly {
    type Output = ZZPoly;
    #[inline]
    fn sub(mut self, rhs: &Integer) -> ZZPoly {
        self -= rhs;
        self
    }
}

/// Subtract an `ZZPoly` from an owned `Integer`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![1, 2, 3]);
/// let result = Integer::from(10) - poly;
/// assert_eq!(result, ZZPoly::from(vec![9, -2, -3]));
/// ```
impl Sub<ZZPoly> for Integer {
    type Output = ZZPoly;
    #[inline]
    fn sub(self, rhs: ZZPoly) -> ZZPoly {
        (-rhs) + self
    }
}

/// Subtract an `ZZPoly` from an `Integer` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![7, 8]);
/// let num = Integer::from(3);
/// let result = &num - poly;
/// assert_eq!(result, ZZPoly::from(vec![-4, -8]));
/// ```
impl Sub<ZZPoly> for &Integer {
    type Output = ZZPoly;
    #[inline]
    fn sub(self, rhs: ZZPoly) -> ZZPoly {
        (-rhs) + self
    }
}

/// Subtract an `ZZPoly` reference from an owned `Integer`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![1, 2, 3]);
/// let result = Integer::from(5) - &poly;
/// assert_eq!(result, ZZPoly::from(vec![4, -2, -3]));
///
/// // Subtracting zero polynomial
/// let zero = ZZPoly::zero();
/// let result = Integer::from(42) - &zero;
/// assert_eq!(result, ZZPoly::from(vec![42]));
/// ```
impl Sub<&ZZPoly> for Integer {
    type Output = ZZPoly;
    fn sub(self, rhs: &ZZPoly) -> ZZPoly {
        (-rhs) + self
    }
}

/// Subtract an `ZZPoly` reference from an `Integer` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![2, 4, 6]);
/// let num = Integer::from(8);
/// let result = &num - &poly;
/// assert_eq!(result, ZZPoly::from(vec![6, -4, -6]));
/// ```
impl Sub<&ZZPoly> for &Integer {
    type Output = ZZPoly;
    fn sub(self, rhs: &ZZPoly) -> ZZPoly {
        (-rhs) + self
    }
}

/// Subtract an owned `Integer` from an `ZZPoly` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![1, 2, 3]);
/// let result = &poly - Integer::from(5);
/// assert_eq!(result, ZZPoly::from(vec![-4, 2, 3]));
/// ```
impl Sub<Integer> for &ZZPoly {
    type Output = ZZPoly;
    fn sub(self, rhs: Integer) -> ZZPoly {
        (-rhs) + self
    }
}

/// Subtract an `Integer` reference from an `ZZPoly` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![7, 8, 9]);
/// let num = Integer::from(3);
/// let result = &poly - &num;
/// assert_eq!(result, ZZPoly::from(vec![4, 8, 9]));
/// ```
impl Sub<&Integer> for &ZZPoly {
    type Output = ZZPoly;
    fn sub(self, rhs: &Integer) -> ZZPoly {
        (-rhs) + self
    }
}

/// Subtract-assign an owned `Integer` from an `ZZPoly`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let mut poly = ZZPoly::from(vec![11, 2, 3]);
/// poly -= Integer::from(10);
/// assert_eq!(poly, ZZPoly::from(vec![1, 2, 3]));
///
/// // Subtracting zero doesn't change the polynomial
/// let mut poly = ZZPoly::from(vec![5, 6]);
/// poly -= Integer::from(0);
/// assert_eq!(poly, ZZPoly::from(vec![5, 6]));
///
/// // Subtracting from zero polynomial
/// let mut zero = ZZPoly::zero();
/// zero -= Integer::from(42);
/// assert_eq!(zero, ZZPoly::from(vec![-42]));
/// ```
impl SubAssign<Integer> for ZZPoly {
    fn sub_assign(&mut self, rhs: Integer) {
        *self += -rhs;
    }
}

/// Subtract-assign an `Integer` reference from an `ZZPoly`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let mut poly = ZZPoly::from(vec![10, 4, 5]);
/// let num = Integer::from(7);
/// poly -= &num;
/// assert_eq!(poly, ZZPoly::from(vec![3, 4, 5]));
/// ```
impl SubAssign<&Integer> for ZZPoly {
    fn sub_assign(&mut self, rhs: &Integer) {
        *self += -rhs;
    }
}