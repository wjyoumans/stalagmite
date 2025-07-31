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
    Mul,
    MulAssign,
};

/// Classical polynomial multiplication algorithm.
/// 
/// This is based on the flint `_fmpz_poly_mul_tiny1` algorithm.
/// For polynomials A(x) = Σ a_i x^i and B(x) = Σ b_j x^j,
/// computes C(x) = A(x) * B(x) = Σ c_k x^k where c_k = Σ a_i * b_{k-i}.
fn classical_mul(poly1: &[Integer], poly2: &[Integer]) -> Vec<Integer> {
    if poly1.is_empty() || poly2.is_empty() {
        return Vec::new();
    }

    let result_len = poly1.len() + poly2.len() - 1;
    let mut result = vec![Integer::from(0); result_len];

    for (i, a) in poly1.iter().enumerate() {
        if *a != 0 {
            for (j, b) in poly2.iter().enumerate() {
                result[i + j] += a * b;
            }
        }
    }

    result
}

/// Multiply two owned `IntPoly` polynomials.
///
/// This operation multiplies two polynomials using the classical algorithm.
/// For polynomials A(x) and B(x), computes A(x) * B(x).
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
///
/// let p1 = IntPoly::from(vec![1, 2]);     // 1 + 2x
/// let p2 = IntPoly::from(vec![3, 4]);     // 3 + 4x
/// let result = p1 * p2;                   // (1 + 2x)(3 + 4x) = 3 + 10x + 8x²
/// assert_eq!(result, IntPoly::from(vec![3, 10, 8]));
///
/// // Multiplying by constant polynomial
/// let poly = IntPoly::from(vec![1, 2, 3]);
/// let constant = IntPoly::from(vec![5]);
/// let result = poly * constant;
/// assert_eq!(result, IntPoly::from(vec![5, 10, 15]));
///
/// // Multiplying by zero gives zero
/// let poly = IntPoly::from(vec![1, 2, 3]);
/// let zero = IntPoly::zero();
/// let result = poly * zero;
/// assert!(result.is_zero());
/// ```
impl Mul for IntPoly {
    type Output = IntPoly;
    fn mul(self, rhs: IntPoly) -> IntPoly {
        if self.is_zero() || rhs.is_zero() {
            return IntPoly::zero();
        }

        let coeffs = classical_mul(&self.coeffs, &rhs.coeffs);
        IntPoly::from_raw(coeffs)
    }
}

/// Multiply an owned `IntPoly` by an `IntPoly` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
///
/// let p1 = IntPoly::from(vec![1, 1]);     // 1 + x
/// let p2 = IntPoly::from(vec![1, -1]);    // 1 - x
/// let result = p1 * &p2;                  // (1 + x)(1 - x) = 1 - x²
/// assert_eq!(result, IntPoly::from(vec![1, 0, -1]));
/// ```
impl Mul<&IntPoly> for IntPoly {
    type Output = IntPoly;
    #[inline]
    fn mul(self, rhs: &IntPoly) -> IntPoly {
        if self.is_zero() || rhs.is_zero() {
            return IntPoly::zero();
        }

        let coeffs = classical_mul(&self.coeffs, &rhs.coeffs);
        IntPoly::from_raw(coeffs)
    }
}

/// Multiply an `IntPoly` reference by an owned `IntPoly`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
///
/// let p1 = IntPoly::from(vec![2, 1]);     // 2 + x
/// let p2 = IntPoly::from(vec![1, 1]);     // 1 + x
/// let result = &p1 * p2;                  // (2 + x)(1 + x) = 2 + 3x + x²
/// assert_eq!(result, IntPoly::from(vec![2, 3, 1]));
/// ```
impl Mul<IntPoly> for &IntPoly {
    type Output = IntPoly;
    #[inline]
    fn mul(self, rhs: IntPoly) -> IntPoly {
        if self.is_zero() || rhs.is_zero() {
            return IntPoly::zero();
        }

        let coeffs = classical_mul(&self.coeffs, &rhs.coeffs);
        IntPoly::from_raw(coeffs)
    }
}

/// Multiply two `IntPoly` references.
///
/// This is the most memory-efficient multiplication as it doesn't take ownership
/// of either polynomial and creates a new result.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
///
/// let p1 = IntPoly::from(vec![1, 2, 1]);  // 1 + 2x + x²
/// let p2 = IntPoly::from(vec![1, -1]);    // 1 - x
/// let result = &p1 * &p2;                 // (1 + 2x + x²)(1 - x) = 1 + x - x² - x³
/// assert_eq!(result, IntPoly::from(vec![1, 1, -1, -1]));
///
/// // Both polynomials remain unchanged
/// assert_eq!(p1, IntPoly::from(vec![1, 2, 1]));
/// assert_eq!(p2, IntPoly::from(vec![1, -1]));
///
/// // Multiplication with one polynomial
/// let one = IntPoly::from(vec![1]);
/// let poly = IntPoly::from(vec![1, 2, 3]);
/// assert_eq!(&poly * &one, poly);
/// assert_eq!(&one * &poly, poly);
/// ```
impl Mul<&IntPoly> for &IntPoly {
    type Output = IntPoly;
    fn mul(self, rhs: &IntPoly) -> IntPoly {
        if self.is_zero() || rhs.is_zero() {
            return IntPoly::zero();
        }

        let coeffs = classical_mul(&self.coeffs, &rhs.coeffs);
        IntPoly::from_raw(coeffs)
    }
}

/// Multiply-assign an owned `IntPoly` to this polynomial.
///
/// This modifies the left-hand side polynomial in place by multiplying it
/// with the right-hand side polynomial.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
///
/// let mut p1 = IntPoly::from(vec![1, 2]);
/// let p2 = IntPoly::from(vec![3, 4]);
/// p1 *= p2;
/// assert_eq!(p1, IntPoly::from(vec![3, 10, 8]));
///
/// // Multiplying by zero makes polynomial zero
/// let mut p = IntPoly::from(vec![1, 2]);
/// p *= IntPoly::zero();
/// assert!(p.is_zero());
///
/// // Multiplying by one doesn't change polynomial
/// let mut p = IntPoly::from(vec![1, 2, 3]);
/// p *= IntPoly::from(vec![1]);
/// assert_eq!(p, IntPoly::from(vec![1, 2, 3]));
/// ```
impl MulAssign<IntPoly> for IntPoly {
    fn mul_assign(&mut self, rhs: IntPoly) {
        *self = std::mem::take(self) * rhs;
    }
}

/// Multiply-assign an `IntPoly` reference to this polynomial.
///
/// This modifies the left-hand side polynomial in place by multiplying it
/// with the right-hand side polynomial, without taking ownership of the RHS.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
///
/// let mut p1 = IntPoly::from(vec![1, 2]);
/// let p2 = IntPoly::from(vec![3, 4]);
/// p1 *= &p2;
/// assert_eq!(p1, IntPoly::from(vec![3, 10, 8]));
/// // p2 is still available for use
/// assert_eq!(p2, IntPoly::from(vec![3, 4]));
///
/// // Self-multiplication (squaring)
/// let mut p = IntPoly::from(vec![1, 1]);  // 1 + x
/// p *= &p.clone();                        // (1 + x)²
/// assert_eq!(p, IntPoly::from(vec![1, 2, 1]));
/// ```
impl MulAssign<&IntPoly> for IntPoly {
    fn mul_assign(&mut self, rhs: &IntPoly) {
        *self = std::mem::take(self) * rhs;
    }
}

// Multiplying IntPoly with Integer
/// Multiply an owned `IntPoly` by an owned `Integer`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
/// use malachite::Integer;
///
/// let poly = IntPoly::from(vec![1, 2, 3]); // 1 + 2x + 3x²
/// let result = poly * Integer::from(5);    // 5 + 10x + 15x²
/// assert_eq!(result, IntPoly::from(vec![5, 10, 15]));
///
/// // Multiplying by zero
/// let poly = IntPoly::from(vec![1, 2, 3]);
/// let result = poly * Integer::from(0);
/// assert!(result.is_zero());
///
/// // Multiplying by one
/// let poly = IntPoly::from(vec![1, 2, 3]);
/// let result = poly * Integer::from(1);
/// assert_eq!(result, IntPoly::from(vec![1, 2, 3]));
/// ```
impl Mul<Integer> for IntPoly {
    type Output = IntPoly;
    fn mul(mut self, rhs: Integer) -> IntPoly {
        if rhs == 0 {
            return IntPoly::zero();
        }
        if rhs == 1 {
            return self;
        }

        for coeff in &mut self.coeffs {
            *coeff *= &rhs;
        }
        self
    }
}

/// Multiply an owned `IntPoly` by an `Integer` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
/// use malachite::Integer;
///
/// let poly = IntPoly::from(vec![2, 4, 6]);
/// let num = Integer::from(3);
/// let result = poly * &num;
/// assert_eq!(result, IntPoly::from(vec![6, 12, 18]));
/// ```
impl Mul<&Integer> for IntPoly {
    type Output = IntPoly;
    fn mul(mut self, rhs: &Integer) -> IntPoly {
        if *rhs == 0 {
            return IntPoly::zero();
        }
        if *rhs == 1 {
            return self;
        }

        for coeff in &mut self.coeffs {
            *coeff *= rhs;
        }
        self
    }
}

/// Multiply an owned `Integer` by an owned `IntPoly`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
/// use malachite::Integer;
///
/// let poly = IntPoly::from(vec![1, 2, 3]);
/// let result = Integer::from(4) * poly;
/// assert_eq!(result, IntPoly::from(vec![4, 8, 12]));
/// ```
impl Mul<IntPoly> for Integer {
    type Output = IntPoly;
    #[inline]
    fn mul(self, rhs: IntPoly) -> IntPoly {
        rhs * self
    }
}

/// Multiply an `Integer` reference by an owned `IntPoly`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
/// use malachite::Integer;
///
/// let poly = IntPoly::from(vec![7, 8]);
/// let num = Integer::from(2);
/// let result = &num * poly;
/// assert_eq!(result, IntPoly::from(vec![14, 16]));
/// ```
impl Mul<IntPoly> for &Integer {
    type Output = IntPoly;
    #[inline]
    fn mul(self, rhs: IntPoly) -> IntPoly {
        rhs * self
    }
}

/// Multiply an owned `Integer` by an `IntPoly` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
/// use malachite::Integer;
///
/// let poly = IntPoly::from(vec![1, 2, 3]);
/// let result = Integer::from(5) * &poly;
/// assert_eq!(result, IntPoly::from(vec![5, 10, 15]));
///
/// // Multiplying zero polynomial
/// let zero = IntPoly::zero();
/// let result = Integer::from(42) * &zero;
/// assert!(result.is_zero());
/// ```
impl Mul<&IntPoly> for Integer {
    type Output = IntPoly;
    fn mul(self, rhs: &IntPoly) -> IntPoly {
        if self == 0 || rhs.is_zero() {
            return IntPoly::zero();
        }
        if self == 1 {
            return rhs.clone();
        }

        let coeffs = rhs.coeffs.iter().map(|c| &self * c).collect();
        IntPoly::from_raw(coeffs)
    }
}

/// Multiply an `Integer` reference by an `IntPoly` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
/// use malachite::Integer;
///
/// let poly = IntPoly::from(vec![2, 4, 6]);
/// let num = Integer::from(3);
/// let result = &num * &poly;
/// assert_eq!(result, IntPoly::from(vec![6, 12, 18]));
/// ```
impl Mul<&IntPoly> for &Integer {
    type Output = IntPoly;
    fn mul(self, rhs: &IntPoly) -> IntPoly {
        if *self == 0 || rhs.is_zero() {
            return IntPoly::zero();
        }
        if *self == 1 {
            return rhs.clone();
        }

        let coeffs = rhs.coeffs.iter().map(|c| self * c).collect();
        IntPoly::from_raw(coeffs)
    }
}

/// Multiply an `IntPoly` reference by an owned `Integer`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
/// use malachite::Integer;
///
/// let poly = IntPoly::from(vec![1, 2, 3]);
/// let result = &poly * Integer::from(5);
/// assert_eq!(result, IntPoly::from(vec![5, 10, 15]));
/// ```
impl Mul<Integer> for &IntPoly {
    type Output = IntPoly;
    fn mul(self, rhs: Integer) -> IntPoly {
        if rhs == 0 || self.is_zero() {
            return IntPoly::zero();
        }
        if rhs == 1 {
            return self.clone();
        }

        let coeffs = self.coeffs.iter().map(|c| c * &rhs).collect();
        IntPoly::from_raw(coeffs)
    }
}

/// Multiply an `IntPoly` reference by an `Integer` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
/// use malachite::Integer;
///
/// let poly = IntPoly::from(vec![7, 8, 9]);
/// let num = Integer::from(2);
/// let result = &poly * &num;
/// assert_eq!(result, IntPoly::from(vec![14, 16, 18]));
/// ```
impl Mul<&Integer> for &IntPoly {
    type Output = IntPoly;
    fn mul(self, rhs: &Integer) -> IntPoly {
        if *rhs == 0 || self.is_zero() {
            return IntPoly::zero();
        }
        if *rhs == 1 {
            return self.clone();
        }

        let coeffs = self.coeffs.iter().map(|c| c * rhs).collect();
        IntPoly::from_raw(coeffs)
    }
}

/// Multiply-assign an owned `Integer` to an `IntPoly`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
/// use malachite::Integer;
///
/// let mut poly = IntPoly::from(vec![1, 2, 3]);
/// poly *= Integer::from(5);
/// assert_eq!(poly, IntPoly::from(vec![5, 10, 15]));
///
/// // Multiplying by zero makes polynomial zero
/// let mut poly = IntPoly::from(vec![5, 6]);
/// poly *= Integer::from(0);
/// assert!(poly.is_zero());
///
/// // Multiplying by one doesn't change polynomial
/// let mut poly = IntPoly::from(vec![1, 2, 3]);
/// poly *= Integer::from(1);
/// assert_eq!(poly, IntPoly::from(vec![1, 2, 3]));
/// ```
impl MulAssign<Integer> for IntPoly {
    fn mul_assign(&mut self, rhs: Integer) {
        if rhs == 0 {
            *self = IntPoly::zero();
            return;
        }
        if rhs == 1 {
            return;
        }

        for coeff in &mut self.coeffs {
            *coeff *= &rhs;
        }
    }
}

/// Multiply-assign an `Integer` reference to an `IntPoly`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
/// use malachite::Integer;
///
/// let mut poly = IntPoly::from(vec![3, 4, 5]);
/// let num = Integer::from(2);
/// poly *= &num;
/// assert_eq!(poly, IntPoly::from(vec![6, 8, 10]));
/// ```
impl MulAssign<&Integer> for IntPoly {
    fn mul_assign(&mut self, rhs: &Integer) {
        if *rhs == 0 {
            *self = IntPoly::zero();
            return;
        }
        if *rhs == 1 {
            return;
        }

        for coeff in &mut self.coeffs {
            *coeff *= rhs;
        }
    }
}