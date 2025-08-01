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

//! Classical polynomial multiplication algorithms.
//!
//! This module implements the classical O(n*m) polynomial multiplication algorithm
//! based on the flint `_fmpz_poly_mul_classical` implementation. The classical
//! algorithm is efficient for small polynomials or when coefficient size is large
//! relative to polynomial degree.

use malachite::Integer;
use crate::intpoly::IntPoly;

/// Compute the dot product of two slices with a stride.
/// 
/// This computes the sum of `a[i] * b[stride * i]` for `i` in `0..len`.
fn dot_product(a: &[Integer], b: &[Integer], stride: usize, len: usize) -> Integer {
    let mut result = Integer::from(0);
    for i in 0..len {
        if i < a.len() && stride * i < b.len() {
            result += &a[i] * &b[stride * i];
        }
    }
    result
}

/// Classical polynomial multiplication algorithm.
/// 
/// This is based on the flint `_fmpz_poly_mul_classical` algorithm.
/// Computes the coefficients of the product polynomial using the
/// formula: `(sum a_i x^i) * (sum b_j x^j) = sum (sum a_i * b_{k-i}) x^k`.
/// 
/// The algorithm is optimized for cases where one polynomial is much
/// smaller than the other, and uses efficient dot product computation.
/// 
/// # Arguments
/// 
/// * `poly1` - First polynomial coefficients (length `len1`)
/// * `len1` - Length of first polynomial
/// * `poly2` - Second polynomial coefficients (length `len2`)  
/// * `len2` - Length of second polynomial
/// 
/// # Returns
/// 
/// Vector of coefficients for the product polynomial of length `len1 + len2 - 1`.
/// 
/// # Examples
/// 
/// ```
/// use stalagmite_poly::intpoly::arithmetic::mul_classical::classical_mul;
/// use malachite::Integer;
/// 
/// let poly1 = vec![Integer::from(1), Integer::from(2), Integer::from(3)]; // 1 + 2x + 3x²
/// let poly2 = vec![Integer::from(4), Integer::from(5)]; // 4 + 5x
/// let result = classical_mul(&poly1, poly1.len(), &poly2, poly2.len());
/// // (1 + 2x + 3x²)(4 + 5x) = 4 + 13x + 22x² + 15x³
/// assert_eq!(result, vec![
///     Integer::from(4), Integer::from(13), 
///     Integer::from(22), Integer::from(15)
/// ]);
/// ```
pub fn classical_mul(poly1: &[Integer], len1: usize, poly2: &[Integer], len2: usize) -> Vec<Integer> {
    if len1 == 0 || len2 == 0 {
        return Vec::new();
    }
    
    // Handle trivial cases first
    if len1 == 1 && len2 == 1 {
        return vec![&poly1[0] * &poly2[0]];
    }
    
    if len1 == 1 {
        // Scalar multiplication: poly2 * poly1[0]
        return poly2.iter().take(len2).map(|c| c * &poly1[0]).collect();
    }
    
    if len2 == 1 {
        // Scalar multiplication: poly1 * poly2[0]
        return poly1.iter().take(len1).map(|c| c * &poly2[0]).collect();
    }
    
    let result_len = len1 + len2 - 1;
    let mut result = vec![Integer::from(0); result_len];
    
    // First coefficient: poly1[0] * poly2[0]
    result[0] = &poly1[0] * &poly2[0];
    
    // Middle coefficients using optimized dot product computation
    // For coefficient k, we need sum of poly1[i] * poly2[k-i] for valid i
    for k in 1..result_len - 1 {
        let start_i = k.saturating_sub(len2 - 1);
        let end_i = (len1 - 1).min(k);
        
        for i in start_i..=end_i {
            let j = k - i;
            if i < len1 && j < len2 {
                result[k] += &poly1[i] * &poly2[j];
            }
        }
    }
    
    // Last coefficient: poly1[len1-1] * poly2[len2-1]
    if result_len > 1 {
        result[result_len - 1] = &poly1[len1 - 1] * &poly2[len2 - 1];
    }
    
    result
}

/// Classical multiplication for IntPoly.
/// 
/// This is a high-level interface to the classical multiplication algorithm
/// that handles IntPoly types and their internal representation.
/// 
/// # Examples
/// 
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
/// use stalagmite_poly::intpoly::arithmetic::mul_classical::mul_classical;
/// 
/// let poly1 = IntPoly::from(vec![1, 2, 3]); // 1 + 2x + 3x²
/// let poly2 = IntPoly::from(vec![4, 5]);    // 4 + 5x
/// let result = mul_classical(&poly1, &poly2);
/// // (1 + 2x + 3x²)(4 + 5x) = 4 + 13x + 22x² + 15x³
/// assert_eq!(result, IntPoly::from(vec![4, 13, 22, 15]));
/// 
/// // Multiplication with zero polynomial
/// let zero = IntPoly::zero();
/// let result = mul_classical(&poly1, &zero);
/// assert!(result.is_zero());
/// 
/// // Multiplication with constant polynomial
/// let constant = IntPoly::from(vec![3]);
/// let result = mul_classical(&poly1, &constant);
/// assert_eq!(result, IntPoly::from(vec![3, 6, 9]));
/// ```
pub fn mul_classical(poly1: &IntPoly, poly2: &IntPoly) -> IntPoly {
    if poly1.is_zero() || poly2.is_zero() {
        return IntPoly::zero();
    }
    
    let coeffs = classical_mul(&poly1.coeffs, poly1.length(), &poly2.coeffs, poly2.length());
    IntPoly::from_raw(coeffs)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_classical_mul_basic() {
        let poly1 = vec![Integer::from(1), Integer::from(2)]; // 1 + 2x
        let poly2 = vec![Integer::from(3), Integer::from(4)]; // 3 + 4x
        let result = classical_mul(&poly1, 2, &poly2, 2);
        // (1 + 2x)(3 + 4x) = 3 + 10x + 8x²
        assert_eq!(result, vec![Integer::from(3), Integer::from(10), Integer::from(8)]);
    }
    
    #[test]
    fn test_classical_mul_different_lengths() {
        let poly1 = vec![Integer::from(1), Integer::from(2), Integer::from(3)]; // 1 + 2x + 3x²
        let poly2 = vec![Integer::from(4), Integer::from(5)]; // 4 + 5x
        let result = classical_mul(&poly1, 3, &poly2, 2);
        // (1 + 2x + 3x²)(4 + 5x) = 4 + 13x + 22x² + 15x³
        assert_eq!(result, vec![
            Integer::from(4), Integer::from(13), 
            Integer::from(22), Integer::from(15)
        ]);
    }
    
    #[test]
    fn test_classical_mul_scalar() {
        let poly1 = vec![Integer::from(2), Integer::from(4), Integer::from(6)];
        let poly2 = vec![Integer::from(3)];
        let result = classical_mul(&poly1, 3, &poly2, 1);
        assert_eq!(result, vec![Integer::from(6), Integer::from(12), Integer::from(18)]);
    }
    
    #[test]
    fn test_classical_mul_empty() {
        let poly1 = vec![Integer::from(1), Integer::from(2)];
        let poly2: Vec<Integer> = vec![];
        let result = classical_mul(&poly1, 2, &poly2, 0);
        assert!(result.is_empty());
    }
    
    #[test]
    fn test_mul_classical_intpoly() {
        let poly1 = IntPoly::from(vec![1, 2, 3]);
        let poly2 = IntPoly::from(vec![4, 5]);
        let result = mul_classical(&poly1, &poly2);
        assert_eq!(result, IntPoly::from(vec![4, 13, 22, 15]));
    }
}