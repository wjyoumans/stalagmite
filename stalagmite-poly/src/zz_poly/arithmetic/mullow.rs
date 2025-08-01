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

//! Truncated polynomial multiplication algorithms.
//!
//! This module implements truncated (or "low") polynomial multiplication,
//! where only the first n coefficients of the product are computed.
//! This is useful in many applications where higher-degree terms are not needed,
//! such as power series computations or modular polynomial arithmetic.
//!
//! The algorithms can be significantly more efficient than computing the full
//! product when the truncation point is much smaller than the full product length.

use malachite::Integer;
use crate::zz_poly::ZZPoly;
use crate::zz_poly::arithmetic::mul_karatsuba::karatsuba_mul;

/// Threshold for using Karatsuba in truncated multiplication.
const KARATSUBA_MULLOW_THRESHOLD: usize = 16;

/// Classical truncated multiplication algorithm.
/// 
/// Computes only the first `n` coefficients of the product of two polynomials.
/// For coefficient k < n, computes sum of a[i] * b[k-i] for valid i.
/// 
/// This is more efficient than full multiplication when n << len1 + len2 - 1.
/// 
/// # Arguments
/// 
/// * `poly1` - First polynomial coefficients
/// * `len1` - Length of first polynomial
/// * `poly2` - Second polynomial coefficients
/// * `len2` - Length of second polynomial
/// * `n` - Number of low coefficients to compute
/// 
/// # Returns
/// 
/// Vector of the first `n` coefficients of the product polynomial.
/// 
/// # Examples
/// 
/// ```
/// use stalagmite_poly::zz_poly::arithmetic::mullow::classical_mullow;
/// use malachite::Integer;
/// 
/// let poly1 = vec![Integer::from(1), Integer::from(2), Integer::from(3)]; // 1 + 2x + 3x²
/// let poly2 = vec![Integer::from(4), Integer::from(5), Integer::from(6)]; // 4 + 5x + 6x²
/// let result = classical_mullow(&poly1, 3, &poly2, 3, 3);
/// // (1 + 2x + 3x²)(4 + 5x + 6x²) = 4 + 13x + 28x² + 27x³ + 18x⁴
/// // Truncated to first 3 coefficients: [4, 13, 28]
/// assert_eq!(result, vec![Integer::from(4), Integer::from(13), Integer::from(28)]);
/// ```
pub fn classical_mullow(poly1: &[Integer], len1: usize, poly2: &[Integer], len2: usize, n: usize) -> Vec<Integer> {
    if len1 == 0 || len2 == 0 || n == 0 {
        return vec![Integer::from(0); n];
    }
    
    let mut result = vec![Integer::from(0); n];
    
    for k in 0..n {
        // For coefficient k, sum a[i] * b[k-i] for valid i
        let start_i = k.saturating_sub(len2 - 1);
        let end_i = (len1 - 1).min(k);
        
        for i in start_i..=end_i {
            let j = k - i;
            if i < len1 && j < len2 {
                result[k] += &poly1[i] * &poly2[j];
            }
        }
    }
    
    result
}

/// Karatsuba-based truncated multiplication.
/// 
/// Uses a modified Karatsuba algorithm that avoids computing unnecessary
/// high-degree terms. This can be more efficient than classical truncated
/// multiplication for larger polynomials.
/// 
/// # Arguments
/// 
/// * `poly1` - First polynomial coefficients
/// * `len1` - Length of first polynomial
/// * `poly2` - Second polynomial coefficients
/// * `len2` - Length of second polynomial
/// * `n` - Number of low coefficients to compute
/// 
/// # Returns
/// 
/// Vector of the first `n` coefficients of the product polynomial.
/// 
/// # Examples
/// 
/// ```
/// use stalagmite_poly::zz_poly::arithmetic::mullow::karatsuba_mullow;
/// use malachite::Integer;
/// 
/// let poly1: Vec<Integer> = (1..=20).map(Integer::from).collect();
/// let poly2: Vec<Integer> = (21..=40).map(Integer::from).collect();
/// let result = karatsuba_mullow(&poly1, 20, &poly2, 20, 10);
/// 
/// // Verify against classical truncated multiplication
/// use stalagmite_poly::zz_poly::arithmetic::mullow::classical_mullow;
/// let expected = classical_mullow(&poly1, 20, &poly2, 20, 10);
/// assert_eq!(result, expected);
/// ```
pub fn karatsuba_mullow(poly1: &[Integer], len1: usize, poly2: &[Integer], len2: usize, n: usize) -> Vec<Integer> {
    if len1 < KARATSUBA_MULLOW_THRESHOLD || len2 < KARATSUBA_MULLOW_THRESHOLD || n < KARATSUBA_MULLOW_THRESHOLD {
        return classical_mullow(poly1, len1, poly2, len2, n);
    }
    
    let split = len1.max(len2) / 2;
    
    // If we only need low coefficients and split is large enough,
    // we might be able to skip some high-degree computations
    if n <= split {
        // Only need to compute low parts
        let b1 = &poly1[0..len1.min(split)];
        let b2 = &poly2[0..len2.min(split)];
        return karatsuba_mullow(b1, b1.len(), b2, b2.len(), n);
    }
    
    // Otherwise, fall back to regular truncated computation
    // A full optimized implementation would handle more cases
    let full_result = karatsuba_mul(poly1, len1, poly2, len2);
    let mut result = vec![Integer::from(0); n];
    for i in 0..n.min(full_result.len()) {
        result[i] = full_result[i].clone();
    }
    result
}

/// Automatic algorithm selection for truncated multiplication.
/// 
/// Chooses the most appropriate algorithm based on input sizes and
/// truncation point.
/// 
/// # Examples
/// 
/// ```
/// use stalagmite_poly::zz_poly::arithmetic::mullow::auto_mullow;
/// use malachite::Integer;
/// 
/// let poly1 = vec![Integer::from(1), Integer::from(2), Integer::from(3)];
/// let poly2 = vec![Integer::from(4), Integer::from(5), Integer::from(6)];
/// let result = auto_mullow(&poly1, 3, &poly2, 3, 2);
/// // Should return first 2 coefficients: [4, 13]
/// assert_eq!(result, vec![Integer::from(4), Integer::from(13)]);
/// ```
pub fn auto_mullow(poly1: &[Integer], len1: usize, poly2: &[Integer], len2: usize, n: usize) -> Vec<Integer> {
    if len1 == 0 || len2 == 0 || n == 0 {
        return vec![Integer::from(0); n];
    }
    
    // For very small truncation or small polynomials, use classical
    if n < KARATSUBA_MULLOW_THRESHOLD || len1 < KARATSUBA_MULLOW_THRESHOLD || len2 < KARATSUBA_MULLOW_THRESHOLD {
        classical_mullow(poly1, len1, poly2, len2, n)
    } else {
        karatsuba_mullow(poly1, len1, poly2, len2, n)
    }
}

/// Truncated multiplication for ZZPoly.
/// 
/// Computes only the first `n` coefficients of the product of two polynomials.
/// This is more efficient than full multiplication when `n` is much smaller
/// than the full product length.
/// 
/// # Examples
/// 
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use stalagmite_poly::zz_poly::arithmetic::mullow::mullow;
/// 
/// let poly1 = ZZPoly::from(vec![1, 2, 3, 4, 5]);
/// let poly2 = ZZPoly::from(vec![6, 7, 8, 9, 10]);
/// let result = mullow(&poly1, &poly2, 3);
/// 
/// // Compare with full multiplication truncated
/// let full_result = &poly1 * &poly2;
/// let expected = ZZPoly::from_raw(full_result.coeffs[0..3].to_vec());
/// assert_eq!(result, expected);
/// 
/// // Truncated multiplication with zero polynomial
/// let zero = ZZPoly::zero();
/// let result = mullow(&poly1, &zero, 5);
/// assert!(result.is_zero());
/// 
/// // Truncation larger than result
/// let small1 = ZZPoly::from(vec![1, 2]);
/// let small2 = ZZPoly::from(vec![3, 4]);
/// let result = mullow(&small1, &small2, 10);
/// // Should be same as full multiplication, padded with zeros if needed
/// let full = &small1 * &small2;
/// assert_eq!(result.coeffs[0..full.length()], full.coeffs);
/// ```
pub fn mullow(poly1: &ZZPoly, poly2: &ZZPoly, n: usize) -> ZZPoly {
    if n == 0 {
        return ZZPoly::zero();
    }
    
    if poly1.is_zero() || poly2.is_zero() {
        return ZZPoly::from_raw(vec![Integer::from(0); n]);
    }
    
    let coeffs = auto_mullow(&poly1.coeffs, poly1.length(), &poly2.coeffs, poly2.length(), n);
    ZZPoly::from_raw(coeffs)
}

/// Truncated squaring algorithm.
/// 
/// Computes only the first `n` coefficients of the square of a polynomial.
/// This is more efficient than full squaring when `n` is much smaller
/// than the full square length.
/// 
/// # Examples
/// 
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use stalagmite_poly::zz_poly::arithmetic::mullow::sqrlow;
/// 
/// let poly = ZZPoly::from(vec![1, 2, 3, 4]);
/// let result = sqrlow(&poly, 4);
/// 
/// // Compare with full squaring truncated
/// use stalagmite_poly::zz_poly::arithmetic::sqr::sqr;
/// let full_sqr = sqr(&poly);
/// let expected = ZZPoly::from_raw(full_sqr.coeffs[0..4].to_vec());
/// assert_eq!(result, expected);
/// ```
pub fn sqrlow(poly: &ZZPoly, n: usize) -> ZZPoly {
    // For squaring, we can use the same truncated multiplication
    mullow(poly, poly, n)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_classical_mullow_basic() {
        let poly1 = vec![Integer::from(1), Integer::from(2)]; // 1 + 2x
        let poly2 = vec![Integer::from(3), Integer::from(4)]; // 3 + 4x
        let result = classical_mullow(&poly1, 2, &poly2, 2, 2);
        // (1 + 2x)(3 + 4x) = 3 + 10x + 8x², truncated to first 2: [3, 10]
        assert_eq!(result, vec![Integer::from(3), Integer::from(10)]);
    }
    
    #[test]
    fn test_classical_mullow_vs_full() {
        let poly1 = vec![Integer::from(1), Integer::from(2), Integer::from(3)];
        let poly2 = vec![Integer::from(4), Integer::from(5), Integer::from(6)];
        
        let truncated = classical_mullow(&poly1, 3, &poly2, 3, 3);
        let full = crate::zz_poly::arithmetic::mul_classical::classical_mul(&poly1, 3, &poly2, 3);
        
        // First 3 coefficients should match
        assert_eq!(truncated, full[0..3]);
    }
    
    #[test]
    fn test_karatsuba_mullow_vs_classical() {
        let poly1: Vec<Integer> = (1..=20).map(Integer::from).collect();
        let poly2: Vec<Integer> = (21..=40).map(Integer::from).collect();
        
        let karatsuba_result = karatsuba_mullow(&poly1, 20, &poly2, 20, 10);
        let classical_result = classical_mullow(&poly1, 20, &poly2, 20, 10);
        
        assert_eq!(karatsuba_result, classical_result);
    }
    
    #[test]
    fn test_auto_mullow_selection() {
        // Small case should use classical
        let small1 = vec![Integer::from(1), Integer::from(2)];
        let small2 = vec![Integer::from(3), Integer::from(4)];
        let result = auto_mullow(&small1, 2, &small2, 2, 2);
        let expected = classical_mullow(&small1, 2, &small2, 2, 2);
        assert_eq!(result, expected);
        
        // Large case should use Karatsuba (or fallback)
        let large1: Vec<Integer> = (1..=25).map(Integer::from).collect();
        let large2: Vec<Integer> = (26..=50).map(Integer::from).collect();
        let result = auto_mullow(&large1, 25, &large2, 25, 15);
        let expected = classical_mullow(&large1, 25, &large2, 25, 15);
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_mullow_zz_poly() {
        let poly1 = ZZPoly::from(vec![1, 2, 3, 4, 5]);
        let poly2 = ZZPoly::from(vec![6, 7, 8, 9, 10]);
        let result = mullow(&poly1, &poly2, 4);
        
        // Verify against full multiplication truncated
        let full = &poly1 * &poly2;
        assert_eq!(result.coeffs, full.coeffs[0..4]);
    }
    
    #[test]
    fn test_mullow_edge_cases() {
        let poly = ZZPoly::from(vec![1, 2, 3]);
        
        // Zero truncation
        let result = mullow(&poly, &poly, 0);
        assert!(result.is_zero());
        
        // Zero polynomial
        let zero = ZZPoly::zero();
        let result = mullow(&poly, &zero, 5);
        assert_eq!(result.coeffs, vec![Integer::from(0); 5]);
        
        // Truncation larger than full result
        let small1 = ZZPoly::from(vec![1, 2]);
        let small2 = ZZPoly::from(vec![3]);
        let result = mullow(&small1, &small2, 10);
        // Should match full multiplication in the meaningful positions
        let full = &small1 * &small2;
        assert_eq!(result.coeffs[0..full.length()], full.coeffs);
    }
    
    #[test]
    fn test_sqrlow() {
        let poly = ZZPoly::from(vec![1, 2, 3]);
        let result = sqrlow(&poly, 3);
        
        // Verify against full squaring truncated
        use crate::zz_poly::arithmetic::sqr::sqr;
        let full_sqr = sqr(&poly);
        assert_eq!(result.coeffs, full_sqr.coeffs[0..3]);
    }
}