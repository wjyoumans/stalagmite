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

use malachite::{Natural, Integer};
use crate::zz_poly::ZZPoly;
use std::ops::{
    Mul,
    MulAssign,
};

// Import all multiplication algorithms
use crate::zz_poly::arithmetic::mul_classical;
use crate::zz_poly::arithmetic::mul_karatsuba;
use crate::zz_poly::arithmetic::mul_ks;
use crate::zz_poly::arithmetic::sqr;

/// Get the maximum number of bits needed to represent any coefficient.
/// 
/// This is used for algorithm selection - larger coefficients may benefit
/// from different multiplication algorithms.
fn max_coefficient_bits(poly: &[Integer]) -> usize {
    poly.iter()
        .map(|coeff| {
            // Simple approximation: convert to string length * 3.32 (log base 2 of 10)
            // This gives a rough estimate of bit size
            let str_len = coeff.to_string().len();
            if str_len <= 1 { 1 } else { (str_len as f64 * 3.32) as usize }
        })
        .max()
        .unwrap_or(1)
}

/// Intelligent algorithm selection for polynomial multiplication.
/// 
/// This function implements algorithm selection logic similar to flint,
/// choosing the most appropriate multiplication algorithm based on:
/// - Polynomial lengths
/// - Coefficient bit sizes
/// - Efficiency characteristics of each algorithm
/// 
/// The selection follows this general strategy:
/// - Very small polynomials: classical multiplication
/// - Medium polynomials with large coefficients: Karatsuba  
/// - Large polynomials with small coefficients: Kronecker substitution
///
/// Eventually Schönhage-Strassen and NTT, hopefully.
/// 
/// # Arguments
/// 
/// * `poly1` - First polynomial coefficients
/// * `len1` - Length of first polynomial
/// * `poly2` - Second polynomial coefficients
/// * `len2` - Length of second polynomial
/// 
/// # Returns
/// 
/// Vector of coefficients for the product polynomial.
fn auto_mul(poly1: &[Integer], len1: usize, poly2: &[Integer], len2: usize) -> Vec<Integer> {
    if len1 == 0 || len2 == 0 {
        return Vec::new();
    }
    
    // Handle scalar multiplication
    if len1 == 1 || len2 == 1 {
        return mul_classical::classical_mul(poly1, len1, poly2, len2);
    }
    
    // Use squaring algorithm if polynomials are identical
    if len1 == len2 && poly1 == poly2 {
        return sqr::auto_sqr(poly1, len1);
    }
    
    let min_len = len1.min(len2);
    let max_len = len1.max(len2);
    
    // Get coefficient bit sizes for algorithm selection
    let bits1 = max_coefficient_bits(&poly1[..len1]);
    let bits2 = max_coefficient_bits(&poly2[..len2]);
    let total_bits = bits1 + bits2;
    
    // Algorithm selection logic based on flint's approach
    if max_len <= 6 && total_bits <= 5000 {
        // Very small polynomials: classical is most efficient
        mul_classical::classical_mul(poly1, len1, poly2, len2)
    } else if max_len <= 8 && total_bits >= 1500 && total_bits <= 10000 {
        // Medium size with large coefficients: Karatsuba
        mul_karatsuba::karatsuba_mul(poly1, len1, poly2, len2)
    // } else if max_len >= 8 && max_len <= 75 && total_bits >= 800 && total_bits <= 4000 {
    //     // Medium-large with medium coefficients: Schönhage-Strassen region
    } else if min_len < 16 && (bits1 > 1000 || bits2 > 1000) {
        // One small polynomial with large coefficients: Karatsuba
        mul_karatsuba::karatsuba_mul(poly1, len1, poly2, len2)
    } else if total_bits <= 800 || max_len < 50 {
        // Small total bit complexity or medium size: Kronecker substitution
        mul_ks::ks_mul(poly1, len1, poly2, len2)
    // } else if max_len >= 1000 {
    //     // Very large polynomials: Schönhage-Strassen
    } else {
        // Default fallback: Karatsuba for medium cases
        mul_karatsuba::karatsuba_mul(poly1, len1, poly2, len2)
    }
}

/// Multiply two owned `ZZPoly` polynomials.
///
/// This operation automatically selects the most efficient multiplication
/// algorithm based on polynomial characteristics. For polynomials A(x) and B(x),
/// computes A(x) * B(x).
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let p1 = ZZPoly::from(vec![1, 2]);     // 1 + 2x
/// let p2 = ZZPoly::from(vec![3, 4]);     // 3 + 4x
/// let result = p1 * p2;                   // (1 + 2x)(3 + 4x) = 3 + 10x + 8x²
/// assert_eq!(result, ZZPoly::from(vec![3, 10, 8]));
///
/// // Multiplying by constant polynomial
/// let poly = ZZPoly::from(vec![1, 2, 3]);
/// let constant = ZZPoly::from(vec![5]);
/// let result = poly * constant;
/// assert_eq!(result, ZZPoly::from(vec![5, 10, 15]));
///
/// // Multiplying by zero gives zero
/// let poly = ZZPoly::from(vec![1, 2, 3]);
/// let zero = ZZPoly::zero();
/// let result = poly * zero;
/// assert!(result.is_zero());
/// 
/// // Large polynomial multiplication automatically uses efficient algorithms
/// let large1: Vec<i32> = (1..=50).collect();
/// let large2: Vec<i32> = (51..=100).collect();
/// let p1 = ZZPoly::from(large1);
/// let p2 = ZZPoly::from(large2);
/// let result = p1 * p2; // Automatically selects appropriate algorithm
/// ```
impl Mul for ZZPoly {
    type Output = ZZPoly;
    fn mul(self, rhs: ZZPoly) -> ZZPoly {
        if self.is_zero() || rhs.is_zero() {
            return ZZPoly::zero();
        }

        let coeffs = auto_mul(&self.coeffs, self.length(), &rhs.coeffs, rhs.length());
        ZZPoly::from_raw(coeffs)
    }
}

/// Multiply an owned `ZZPoly` by an `ZZPoly` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let p1 = ZZPoly::from(vec![1, 1]);     // 1 + x
/// let p2 = ZZPoly::from(vec![1, -1]);    // 1 - x
/// let result = p1 * &p2;                  // (1 + x)(1 - x) = 1 - x²
/// assert_eq!(result, ZZPoly::from(vec![1, 0, -1]));
/// ```
impl Mul<&ZZPoly> for ZZPoly {
    type Output = ZZPoly;
    #[inline]
    fn mul(self, rhs: &ZZPoly) -> ZZPoly {
        if self.is_zero() || rhs.is_zero() {
            return ZZPoly::zero();
        }

        let coeffs = auto_mul(&self.coeffs, self.length(), &rhs.coeffs, rhs.length());
        ZZPoly::from_raw(coeffs)
    }
}

/// Multiply an `ZZPoly` reference by an owned `ZZPoly`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let p1 = ZZPoly::from(vec![2, 1]);     // 2 + x
/// let p2 = ZZPoly::from(vec![1, 1]);     // 1 + x
/// let result = &p1 * p2;                  // (2 + x)(1 + x) = 2 + 3x + x²
/// assert_eq!(result, ZZPoly::from(vec![2, 3, 1]));
/// ```
impl Mul<ZZPoly> for &ZZPoly {
    type Output = ZZPoly;
    #[inline]
    fn mul(self, rhs: ZZPoly) -> ZZPoly {
        if self.is_zero() || rhs.is_zero() {
            return ZZPoly::zero();
        }

        let coeffs = auto_mul(&self.coeffs, self.length(), &rhs.coeffs, rhs.length());
        ZZPoly::from_raw(coeffs)
    }
}

/// Multiply two `ZZPoly` references.
///
/// This is the most memory-efficient multiplication as it doesn't take ownership
/// of either polynomial and creates a new result. Automatically selects the
/// most efficient algorithm.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let p1 = ZZPoly::from(vec![1, 2, 1]);  // 1 + 2x + x²
/// let p2 = ZZPoly::from(vec![1, -1]);    // 1 - x
/// let result = &p1 * &p2;                 // (1 + 2x + x²)(1 - x) = 1 + x - x² - x³
/// assert_eq!(result, ZZPoly::from(vec![1, 1, -1, -1]));
///
/// // Both polynomials remain unchanged
/// assert_eq!(p1, ZZPoly::from(vec![1, 2, 1]));
/// assert_eq!(p2, ZZPoly::from(vec![1, -1]));
///
/// // Multiplication with one polynomial
/// let one = ZZPoly::from(vec![1]);
/// let poly = ZZPoly::from(vec![1, 2, 3]);
/// assert_eq!(&poly * &one, poly);
/// assert_eq!(&one * &poly, poly);
/// ```
impl Mul<&ZZPoly> for &ZZPoly {
    type Output = ZZPoly;
    fn mul(self, rhs: &ZZPoly) -> ZZPoly {
        if self.is_zero() || rhs.is_zero() {
            return ZZPoly::zero();
        }

        let coeffs = auto_mul(&self.coeffs, self.length(), &rhs.coeffs, rhs.length());
        ZZPoly::from_raw(coeffs)
    }
}

/// Multiply-assign an owned `ZZPoly` to this polynomial.
///
/// This modifies the left-hand side polynomial in place by multiplying it
/// with the right-hand side polynomial.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let mut p1 = ZZPoly::from(vec![1, 2]);
/// let p2 = ZZPoly::from(vec![3, 4]);
/// p1 *= p2;
/// assert_eq!(p1, ZZPoly::from(vec![3, 10, 8]));
///
/// // Multiplying by zero makes polynomial zero
/// let mut p = ZZPoly::from(vec![1, 2]);
/// p *= ZZPoly::zero();
/// assert!(p.is_zero());
///
/// // Multiplying by one doesn't change polynomial
/// let mut p = ZZPoly::from(vec![1, 2, 3]);
/// p *= ZZPoly::from(vec![1]);
/// assert_eq!(p, ZZPoly::from(vec![1, 2, 3]));
/// ```
impl MulAssign<ZZPoly> for ZZPoly {
    fn mul_assign(&mut self, rhs: ZZPoly) {
        // Use automatic algorithm selection
        if self.is_zero() || rhs.is_zero() {
            *self = ZZPoly::zero();
            return;
        }
        
        let coeffs = auto_mul(&self.coeffs, self.length(), &rhs.coeffs, rhs.length());
        *self = ZZPoly::from_raw(coeffs);
    }
}

/// Multiply-assign an `ZZPoly` reference to this polynomial.
///
/// This modifies the left-hand side polynomial in place by multiplying it
/// with the right-hand side polynomial, without taking ownership of the RHS.
/// Automatically selects the most efficient algorithm.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let mut p1 = ZZPoly::from(vec![1, 2]);
/// let p2 = ZZPoly::from(vec![3, 4]);
/// p1 *= &p2;
/// assert_eq!(p1, ZZPoly::from(vec![3, 10, 8]));
/// // p2 is still available for use
/// assert_eq!(p2, ZZPoly::from(vec![3, 4]));
///
/// // Self-multiplication (squaring automatically detected)
/// let mut p = ZZPoly::from(vec![1, 1]);  // 1 + x
/// let p_clone = p.clone();
/// p *= &p_clone;                          // Uses squaring algorithm
/// assert_eq!(p, ZZPoly::from(vec![1, 2, 1]));
/// ```
impl MulAssign<&ZZPoly> for ZZPoly {
    fn mul_assign(&mut self, rhs: &ZZPoly) {
        // Use automatic algorithm selection
        if self.is_zero() || rhs.is_zero() {
            *self = ZZPoly::zero();
            return;
        }
        
        let coeffs = auto_mul(&self.coeffs, self.length(), &rhs.coeffs, rhs.length());
        *self = ZZPoly::from_raw(coeffs);
    }
}

// Multiplying ZZPoly with Integer
/// Multiply an owned `ZZPoly` by an owned `Integer`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![1, 2, 3]); // 1 + 2x + 3x²
/// let result = poly * Integer::from(5);    // 5 + 10x + 15x²
/// assert_eq!(result, ZZPoly::from(vec![5, 10, 15]));
///
/// // Multiplying by zero
/// let poly = ZZPoly::from(vec![1, 2, 3]);
/// let result = poly * Integer::from(0);
/// assert!(result.is_zero());
///
/// // Multiplying by one
/// let poly = ZZPoly::from(vec![1, 2, 3]);
/// let result = poly * Integer::from(1);
/// assert_eq!(result, ZZPoly::from(vec![1, 2, 3]));
/// ```
impl Mul<Integer> for ZZPoly {
    type Output = ZZPoly;
    fn mul(mut self, rhs: Integer) -> ZZPoly {
        if rhs == 0 {
            return ZZPoly::zero();
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

/// Multiply an owned `ZZPoly` by an `Integer` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![2, 4, 6]);
/// let num = Integer::from(3);
/// let result = poly * &num;
/// assert_eq!(result, ZZPoly::from(vec![6, 12, 18]));
/// ```
impl Mul<&Integer> for ZZPoly {
    type Output = ZZPoly;
    fn mul(mut self, rhs: &Integer) -> ZZPoly {
        if *rhs == 0 {
            return ZZPoly::zero();
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

/// Multiply an owned `Integer` by an owned `ZZPoly`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![1, 2, 3]);
/// let result = Integer::from(4) * poly;
/// assert_eq!(result, ZZPoly::from(vec![4, 8, 12]));
/// ```
impl Mul<ZZPoly> for Integer {
    type Output = ZZPoly;
    #[inline]
    fn mul(self, rhs: ZZPoly) -> ZZPoly {
        rhs * self
    }
}

/// Multiply an `Integer` reference by an owned `ZZPoly`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![7, 8]);
/// let num = Integer::from(2);
/// let result = &num * poly;
/// assert_eq!(result, ZZPoly::from(vec![14, 16]));
/// ```
impl Mul<ZZPoly> for &Integer {
    type Output = ZZPoly;
    #[inline]
    fn mul(self, rhs: ZZPoly) -> ZZPoly {
        rhs * self
    }
}

/// Multiply an owned `Integer` by an `ZZPoly` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![1, 2, 3]);
/// let result = Integer::from(5) * &poly;
/// assert_eq!(result, ZZPoly::from(vec![5, 10, 15]));
///
/// // Multiplying zero polynomial
/// let zero = ZZPoly::zero();
/// let result = Integer::from(42) * &zero;
/// assert!(result.is_zero());
/// ```
impl Mul<&ZZPoly> for Integer {
    type Output = ZZPoly;
    fn mul(self, rhs: &ZZPoly) -> ZZPoly {
        if self == 0 || rhs.is_zero() {
            return ZZPoly::zero();
        }
        if self == 1 {
            return rhs.clone();
        }

        let coeffs = rhs.coeffs.iter().map(|c| &self * c).collect();
        ZZPoly::from_raw(coeffs)
    }
}

/// Multiply an `Integer` reference by an `ZZPoly` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![2, 4, 6]);
/// let num = Integer::from(3);
/// let result = &num * &poly;
/// assert_eq!(result, ZZPoly::from(vec![6, 12, 18]));
/// ```
impl Mul<&ZZPoly> for &Integer {
    type Output = ZZPoly;
    fn mul(self, rhs: &ZZPoly) -> ZZPoly {
        if *self == 0 || rhs.is_zero() {
            return ZZPoly::zero();
        }
        if *self == 1 {
            return rhs.clone();
        }

        let coeffs = rhs.coeffs.iter().map(|c| self * c).collect();
        ZZPoly::from_raw(coeffs)
    }
}

/// Multiply an `ZZPoly` reference by an owned `Integer`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![1, 2, 3]);
/// let result = &poly * Integer::from(5);
/// assert_eq!(result, ZZPoly::from(vec![5, 10, 15]));
/// ```
impl Mul<Integer> for &ZZPoly {
    type Output = ZZPoly;
    fn mul(self, rhs: Integer) -> ZZPoly {
        if rhs == 0 || self.is_zero() {
            return ZZPoly::zero();
        }
        if rhs == 1 {
            return self.clone();
        }

        let coeffs = self.coeffs.iter().map(|c| c * &rhs).collect();
        ZZPoly::from_raw(coeffs)
    }
}

/// Multiply an `ZZPoly` reference by an `Integer` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![7, 8, 9]);
/// let num = Integer::from(2);
/// let result = &poly * &num;
/// assert_eq!(result, ZZPoly::from(vec![14, 16, 18]));
/// ```
impl Mul<&Integer> for &ZZPoly {
    type Output = ZZPoly;
    fn mul(self, rhs: &Integer) -> ZZPoly {
        if *rhs == 0 || self.is_zero() {
            return ZZPoly::zero();
        }
        if *rhs == 1 {
            return self.clone();
        }

        let coeffs = self.coeffs.iter().map(|c| c * rhs).collect();
        ZZPoly::from_raw(coeffs)
    }
}

/// Multiply-assign an owned `Integer` to an `ZZPoly`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let mut poly = ZZPoly::from(vec![1, 2, 3]);
/// poly *= Integer::from(5);
/// assert_eq!(poly, ZZPoly::from(vec![5, 10, 15]));
///
/// // Multiplying by zero makes polynomial zero
/// let mut poly = ZZPoly::from(vec![5, 6]);
/// poly *= Integer::from(0);
/// assert!(poly.is_zero());
///
/// // Multiplying by one doesn't change polynomial
/// let mut poly = ZZPoly::from(vec![1, 2, 3]);
/// poly *= Integer::from(1);
/// assert_eq!(poly, ZZPoly::from(vec![1, 2, 3]));
/// ```
impl MulAssign<Integer> for ZZPoly {
    fn mul_assign(&mut self, rhs: Integer) {
        if rhs == 0 {
            *self = ZZPoly::zero();
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

/// Multiply-assign an `Integer` reference to an `ZZPoly`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let mut poly = ZZPoly::from(vec![3, 4, 5]);
/// let num = Integer::from(2);
/// poly *= &num;
/// assert_eq!(poly, ZZPoly::from(vec![6, 8, 10]));
/// ```
impl MulAssign<&Integer> for ZZPoly {
    fn mul_assign(&mut self, rhs: &Integer) {
        if *rhs == 0 {
            *self = ZZPoly::zero();
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

macro_rules! impl_mul_into_integer {
    ($($t:ty),*) => {
        $(
            impl Mul<$t> for ZZPoly {
                type Output = ZZPoly;
                fn mul(self, rhs: $t) -> ZZPoly {
                    self * Integer::from(rhs)
                }
            }
            impl Mul<&$t> for ZZPoly {
                type Output = ZZPoly;
                fn mul(self, rhs: &$t) -> ZZPoly {
                    self * Integer::from(rhs)
                }
            }
            impl Mul<$t> for &ZZPoly {
                type Output = ZZPoly;
                fn mul(self, rhs: $t) -> ZZPoly {
                    self * Integer::from(rhs)
                }
            }
            impl Mul<&$t> for &ZZPoly { 
                type Output = ZZPoly;
                fn mul(self, rhs: &$t) -> ZZPoly {
                    self * Integer::from(rhs)
                }
            }
            impl Mul<ZZPoly> for $t {
                type Output = ZZPoly;
                fn mul(self, rhs: ZZPoly) -> ZZPoly {
                    Integer::from(self) * rhs
                }
            }
            impl Mul<&ZZPoly> for $t {
                type Output = ZZPoly;
                fn mul(self, rhs: &ZZPoly) -> ZZPoly {
                    Integer::from(self) * rhs
                }
            }
            impl Mul<ZZPoly> for &$t {
                type Output = ZZPoly;
                fn mul(self, rhs: ZZPoly) -> ZZPoly {
                    Integer::from(self) * rhs
                }
            }
            impl Mul<&ZZPoly> for &$t { 
                type Output = ZZPoly;
                fn mul(self, rhs: &ZZPoly) -> ZZPoly {
                    Integer::from(self) * rhs
                }
            }
            impl MulAssign<$t> for ZZPoly {     
                fn mul_assign(&mut self, rhs: $t) {
                    *self *= Integer::from(rhs);
                }
            }
            impl MulAssign<&$t> for ZZPoly {
                fn mul_assign(&mut self, rhs: &$t) {    
                    *self *= Integer::from(rhs);
                }
            }
        )*
    }
}

impl_mul_into_integer!(Natural);
