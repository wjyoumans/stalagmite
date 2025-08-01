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

//! Kronecker substitution polynomial multiplication.
//!
//! This module implements polynomial multiplication using Kronecker substitution,
//! which converts polynomial multiplication into integer multiplication.
//!
//! The idea is to evaluate both polynomials at a sufficiently large base B,
//! multiply the resulting integers, and then extract the coefficients from
//! the product by computing remainders and quotients in base B.
//!
//! This is particularly efficient when polynomials have many terms but
//! relatively small coefficients, as it can leverage fast integer multiplication
//! algorithms.

use malachite::Integer;
use crate::zz_poly::ZZPoly;
use crate::zz_poly::arithmetic::mul_classical::classical_mul;

/// Minimum length threshold for using Kronecker substitution.
/// Below this threshold, classical multiplication is more efficient.
const KS_THRESHOLD: usize = 16;

/// Evaluate a polynomial at a given base using Horner's method.
/// 
/// For polynomial f(x) = a₀ + a₁x + a₂x² + ... + aₙxⁿ,
/// computes f(base) = a₀ + a₁·base + a₂·base² + ... + aₙ·baseⁿ.
/// 
/// # Arguments
/// 
/// * `poly` - Polynomial coefficients  
/// * `len` - Length of the polynomial
/// * `base` - Base to evaluate at
/// 
/// # Returns
/// 
/// The value of the polynomial evaluated at the given base.
/// 
/// # Examples
/// 
/// ```
/// use stalagmite_poly::zz_poly::arithmetic::mul_ks::evaluate_at_base;
/// use malachite::Integer;
/// 
/// let poly = vec![Integer::from(1), Integer::from(2), Integer::from(3)]; // 1 + 2x + 3x²
/// let result = evaluate_at_base(&poly, poly.len(), &Integer::from(10));
/// // 1 + 2·10 + 3·100 = 1 + 20 + 300 = 321
/// assert_eq!(result, Integer::from(321));
/// ```
pub fn evaluate_at_base(poly: &[Integer], len: usize, base: &Integer) -> Integer {
    if len == 0 {
        return Integer::from(0);
    }
    
    let mut result = poly[len - 1].clone();
    for i in (0..len - 1).rev() {
        result = result * base + &poly[i];
    }
    result
}

/// Extract polynomial coefficients from an integer in a given base.
/// 
/// This is the inverse operation of `evaluate_at_base`. Given an integer
/// that represents a polynomial evaluated at a base, extract the original
/// polynomial coefficients.
/// 
/// # Arguments
/// 
/// * `value` - Integer value representing the evaluated polynomial
/// * `base` - Base used for evaluation
/// * `expected_len` - Expected number of coefficients
/// 
/// # Returns
/// 
/// Vector of coefficients extracted from the integer.
/// 
/// # Examples
/// 
/// ```
/// use stalagmite_poly::zz_poly::arithmetic::mul_ks::extract_coefficients;
/// use malachite::Integer;
/// 
/// let value = Integer::from(321); // Represents 1 + 2x + 3x²
/// let result = extract_coefficients(&value, &Integer::from(10), 3);
/// assert_eq!(result, vec![Integer::from(1), Integer::from(2), Integer::from(3)]);
/// ```
pub fn extract_coefficients(value: &Integer, base: &Integer, expected_len: usize) -> Vec<Integer> {
    if expected_len == 0 {
        return Vec::new();
    }
    
    let mut coeffs = Vec::with_capacity(expected_len);
    let mut remaining = value.clone();
    
    for _ in 0..expected_len {
        let remainder = &remaining % base;
        let quotient = &remaining / base;
        coeffs.push(remainder);
        remaining = quotient;
        
        if remaining == 0 {
            break;
        }
    }
    
    // Ensure we have the expected length (pad with zeros if necessary)
    coeffs.resize(expected_len, Integer::from(0));
    coeffs
}

/// Choose an appropriate base for Kronecker substitution.
/// 
/// The base must be large enough to prevent overflow between coefficients
/// in the product polynomial. A safe choice is to make the base larger than
/// the maximum possible coefficient in the product.
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
/// A base suitable for Kronecker substitution.
fn choose_base(poly1: &[Integer], len1: usize, poly2: &[Integer], len2: usize) -> Integer {
    // Find maximum absolute coefficients using simple approach
    let max1 = poly1.iter().take(len1).map(|x| if *x >= 0 { x.clone() } else { -x }).max()
        .unwrap_or_else(|| Integer::from(1));
    let max2 = poly2.iter().take(len2).map(|x| if *x >= 0 { x.clone() } else { -x }).max()
        .unwrap_or_else(|| Integer::from(1));
    
    // In the product, each coefficient is a sum of at most min(len1, len2) terms,
    // each term being a product of coefficients from the input polynomials.
    // So the maximum coefficient magnitude is approximately:
    // min(len1, len2) * max1 * max2
    let max_terms = Integer::from(len1.min(len2));
    let max_product_coeff = &max_terms * &max1 * &max2;
    
    // Choose base to be larger than the maximum possible coefficient
    // We multiply by 2 for safety margin
    &max_product_coeff * Integer::from(2) + Integer::from(1)
}

/// Kronecker substitution multiplication algorithm.
/// 
/// Converts polynomial multiplication to integer multiplication by:
/// 1. Choosing a suitable base B
/// 2. Evaluating both polynomials at B to get integers
/// 3. Multiplying the integers
/// 4. Extracting coefficients from the product
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
/// 
/// # Examples
/// 
/// ```
/// use stalagmite_poly::zz_poly::arithmetic::mul_ks::ks_mul;
/// use malachite::Integer;
/// 
/// let poly1 = vec![Integer::from(1), Integer::from(2)]; // 1 + 2x
/// let poly2 = vec![Integer::from(3), Integer::from(4)]; // 3 + 4x
/// let result = ks_mul(&poly1, poly1.len(), &poly2, poly2.len());
/// // (1 + 2x)(3 + 4x) = 3 + 10x + 8x²
/// assert_eq!(result, vec![Integer::from(3), Integer::from(10), Integer::from(8)]);
/// ```
pub fn ks_mul(poly1: &[Integer], len1: usize, poly2: &[Integer], len2: usize) -> Vec<Integer> {
    if len1 == 0 || len2 == 0 {
        return Vec::new();
    }
    
    // Use classical multiplication for small polynomials
    if len1 < KS_THRESHOLD || len2 < KS_THRESHOLD {
        return classical_mul(poly1, len1, poly2, len2);
    }
    
    // Choose appropriate base
    let base = choose_base(poly1, len1, poly2, len2);
    
    // Evaluate polynomials at the base
    let val1 = evaluate_at_base(poly1, len1, &base);
    let val2 = evaluate_at_base(poly2, len2, &base);
    
    // Multiply the evaluated values
    let product = val1 * val2;
    
    // Extract coefficients from the product
    let result_len = len1 + len2 - 1;
    extract_coefficients(&product, &base, result_len)
}

/// Kronecker substitution multiplication for ZZPoly.
/// 
/// This is a high-level interface to the Kronecker substitution algorithm
/// that handles ZZPoly types and their internal representation.
/// 
/// # Examples
/// 
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use stalagmite_poly::zz_poly::arithmetic::mul_ks::mul_ks;
/// 
/// let poly1 = ZZPoly::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
/// let poly2 = ZZPoly::from(vec![16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
/// let result = mul_ks(&poly1, &poly2);
/// 
/// // Verify against classical multiplication
/// use stalagmite_poly::zz_poly::arithmetic::mul_classical::mul_classical;
/// let expected = mul_classical(&poly1, &poly2);
/// assert_eq!(result, expected);
/// 
/// // Multiplication with zero polynomial
/// let zero = ZZPoly::zero();
/// let result = mul_ks(&poly1, &zero);
/// assert!(result.is_zero());
/// 
/// // Small polynomials (should fall back to classical)
/// let small1 = ZZPoly::from(vec![1, 2, 3]);
/// let small2 = ZZPoly::from(vec![4, 5, 6]);
/// let result = mul_ks(&small1, &small2);
/// let expected = mul_classical(&small1, &small2);
/// assert_eq!(result, expected);
/// ```
pub fn mul_ks(poly1: &ZZPoly, poly2: &ZZPoly) -> ZZPoly {
    if poly1.is_zero() || poly2.is_zero() {
        return ZZPoly::zero();
    }
    
    let coeffs = ks_mul(&poly1.coeffs, poly1.length(), &poly2.coeffs, poly2.length());
    ZZPoly::from_raw(coeffs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zz_poly::arithmetic::mul_classical::classical_mul;
    
    #[test]
    fn test_evaluate_at_base() {
        let poly = vec![Integer::from(1), Integer::from(2), Integer::from(3)]; // 1 + 2x + 3x²
        let result = evaluate_at_base(&poly, 3, &Integer::from(10));
        // 1 + 2·10 + 3·100 = 321
        assert_eq!(result, Integer::from(321));
    }
    
    #[test]
    fn test_extract_coefficients() {
        let value = Integer::from(321);
        let result = extract_coefficients(&value, &Integer::from(10), 3);
        assert_eq!(result, vec![Integer::from(1), Integer::from(2), Integer::from(3)]);
    }
    
    #[test]
    fn test_choose_base() {
        let poly1 = vec![Integer::from(1), Integer::from(2)];
        let poly2 = vec![Integer::from(3), Integer::from(4)];
        let base = choose_base(&poly1, 2, &poly2, 2);
        
        // Base should be large enough to handle the multiplication
        // Maximum coefficient in (1+2x)(3+4x) = 3+10x+8x² is 10
        // So base should be > 10
        assert!(base > Integer::from(10));
    }
    
    #[test]
    fn test_ks_mul_basic() {
        let poly1 = vec![Integer::from(1), Integer::from(2)]; // 1 + 2x
        let poly2 = vec![Integer::from(3), Integer::from(4)]; // 3 + 4x
        let result = ks_mul(&poly1, 2, &poly2, 2);
        // (1 + 2x)(3 + 4x) = 3 + 10x + 8x²
        assert_eq!(result, vec![Integer::from(3), Integer::from(10), Integer::from(8)]);
    }
    
    #[test]
    fn test_ks_mul_vs_classical_medium() {
        // Create polynomials large enough to trigger KS algorithm
        let poly1: Vec<Integer> = (1..=20).map(Integer::from).collect();
        let poly2: Vec<Integer> = (21..=40).map(Integer::from).collect();
        
        let ks_result = ks_mul(&poly1, 20, &poly2, 20);
        let classical_result = classical_mul(&poly1, 20, &poly2, 20);
        
        assert_eq!(ks_result, classical_result);
    }
    
    #[test]
    fn test_ks_mul_small_fallback() {
        // Small polynomials should fall back to classical
        let poly1 = vec![Integer::from(1), Integer::from(2), Integer::from(3)];
        let poly2 = vec![Integer::from(4), Integer::from(5)];
        
        let ks_result = ks_mul(&poly1, 3, &poly2, 2);
        let classical_result = classical_mul(&poly1, 3, &poly2, 2);
        
        assert_eq!(ks_result, classical_result);
    }
    
    #[test]
    fn test_ks_mul_empty() {
        let poly1 = vec![Integer::from(1), Integer::from(2)];
        let poly2: Vec<Integer> = vec![];
        
        let result = ks_mul(&poly1, 2, &poly2, 0);
        assert!(result.is_empty());
    }
    
    #[test]
    fn test_mul_ks_zz_poly() {
        // Use large enough polynomials to trigger KS
        let poly1: Vec<i32> = (1..=18).collect();
        let poly2: Vec<i32> = (19..=36).collect();
        
        let ipoly1 = ZZPoly::from(poly1);
        let ipoly2 = ZZPoly::from(poly2);
        
        let ks_result = mul_ks(&ipoly1, &ipoly2);
        
        // Verify against classical multiplication
        use crate::zz_poly::arithmetic::mul_classical::mul_classical;
        let classical_result = mul_classical(&ipoly1, &ipoly2);
        assert_eq!(ks_result, classical_result);
    }
    
    #[test]
    fn test_evaluate_extract_roundtrip() {
        let original = vec![Integer::from(5), Integer::from(3), Integer::from(7), Integer::from(2)];
        let base = Integer::from(100); // Large enough base
        
        let evaluated = evaluate_at_base(&original, 4, &base);
        let extracted = extract_coefficients(&evaluated, &base, 4);
        
        assert_eq!(original, extracted);
    }
}