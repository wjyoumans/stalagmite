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

//! Polynomial squaring algorithms.
//!
//! This module implements efficient algorithms for computing the square of a polynomial.
//! Squaring is more efficient than general multiplication because we can avoid 
//! computing duplicate cross terms.
//!
//! For a polynomial f(x) = Σ aᵢxⁱ, we have:
//! f(x)² = Σ aᵢ²x^(2i) + 2 Σᵢ<ⱼ aᵢaⱼx^(i+j)
//!
//! This allows us to compute each coefficient more efficiently than general multiplication.

use malachite::Integer;
use crate::intpoly::IntPoly;
use crate::intpoly::arithmetic::mul_classical::classical_mul;
use crate::intpoly::arithmetic::mul_karatsuba::karatsuba_mul;

/// Threshold below which we use the tiny squaring algorithm.
const TINY_SQR_THRESHOLD: usize = 8;

/// Threshold below which we use classical squaring instead of Karatsuba.
const KARATSUBA_SQR_THRESHOLD: usize = 16;

/// Tiny squaring algorithm for small polynomials.
/// 
/// This is based on the flint `_fmpz_poly_sqr_tiny1` algorithm.
/// It's optimized for polynomials with small coefficients and short length.
/// 
/// The algorithm computes:
/// - Diagonal terms: aᵢ² for coefficient i contributing to x^(2i)
/// - Cross terms: 2·aᵢ·aⱼ for i < j contributing to x^(i+j)
/// 
/// # Arguments
/// 
/// * `poly` - Polynomial coefficients to square
/// * `len` - Length of the polynomial
/// 
/// # Returns
/// 
/// Vector of coefficients for the squared polynomial of length `2*len - 1`.
/// 
/// # Examples
/// 
/// ```
/// use stalagmite_poly::intpoly::arithmetic::sqr::tiny_sqr;
/// use malachite::Integer;
/// 
/// let poly = vec![Integer::from(1), Integer::from(2), Integer::from(3)]; // 1 + 2x + 3x²
/// let result = tiny_sqr(&poly, poly.len());
/// // (1 + 2x + 3x²)² = 1 + 4x + 10x² + 12x³ + 9x⁴
/// assert_eq!(result, vec![
///     Integer::from(1), Integer::from(4), Integer::from(10), 
///     Integer::from(12), Integer::from(9)
/// ]);
/// ```
pub fn tiny_sqr(poly: &[Integer], len: usize) -> Vec<Integer> {
    if len == 0 {
        return Vec::new();
    }
    
    let result_len = 2 * len - 1;
    let mut result = vec![Integer::from(0); result_len];
    
    for i in 0..len {
        let coeff = &poly[i];
        
        if *coeff != 0 {
            // Square term: aᵢ² contributes to coefficient 2i
            result[2 * i] += coeff * coeff;
            
            // Cross terms: 2·aᵢ·aⱼ contributes to coefficient i+j for j > i
            let double_coeff = coeff * &Integer::from(2);
            for j in (i + 1)..len {
                result[i + j] += &poly[j] * &double_coeff;
            }
        }
    }
    
    result
}

/// Classical squaring algorithm.
/// 
/// This algorithm is more efficient than general multiplication for medium-sized
/// polynomials where the tiny algorithm becomes inefficient but Karatsuba
/// overhead is not yet justified.
/// 
/// # Examples
/// 
/// ```
/// use stalagmite_poly::intpoly::arithmetic::sqr::classical_sqr;
/// use malachite::Integer;
/// 
/// let poly = vec![Integer::from(2), Integer::from(3), Integer::from(1)]; // 2 + 3x + x²
/// let result = classical_sqr(&poly, poly.len());
/// // (2 + 3x + x²)² = 4 + 12x + 13x² + 6x³ + x⁴
/// assert_eq!(result, vec![
///     Integer::from(4), Integer::from(12), Integer::from(13), 
///     Integer::from(6), Integer::from(1)
/// ]);
/// ```
pub fn classical_sqr(poly: &[Integer], len: usize) -> Vec<Integer> {
    if len <= TINY_SQR_THRESHOLD {
        return tiny_sqr(poly, len);
    }
    
    // For now, use classical multiplication for simplicity
    // This can be optimized further with specialized squaring logic
    classical_mul(poly, len, poly, len)
}

/// Karatsuba-based squaring algorithm.
/// 
/// Uses a modified Karatsuba algorithm optimized for squaring.
/// For a polynomial f(x) = a·x^m + b, we have:
/// f(x)² = a²·x^(2m) + 2ab·x^m + b²
/// 
/// This requires only 3 operations: computing a², b², and ab.
/// 
/// # Examples
/// 
/// ```
/// use stalagmite_poly::intpoly::arithmetic::sqr::karatsuba_sqr;
/// use malachite::Integer;
/// 
/// let poly: Vec<Integer> = (1..=10).map(Integer::from).collect();
/// let result = karatsuba_sqr(&poly, poly.len());
/// 
/// // Verify against classical squaring
/// use stalagmite_poly::intpoly::arithmetic::sqr::classical_sqr;
/// let expected = classical_sqr(&poly, poly.len());
/// assert_eq!(result, expected);
/// ```
pub fn karatsuba_sqr(poly: &[Integer], len: usize) -> Vec<Integer> {
    if len < KARATSUBA_SQR_THRESHOLD {
        return classical_sqr(poly, len);
    }
    
    let result_len = 2 * len - 1;
    let mut result = vec![Integer::from(0); result_len];
    
    let split = len / 2;
    
    // Split polynomial: f = a*x^split + b
    let a = &poly[split..len];
    let b = &poly[0..split];
    
    // Compute b²
    let b_sqr = karatsuba_sqr(b, b.len());
    
    // Compute a² (if a is non-empty)
    let a_sqr = if !a.is_empty() {
        karatsuba_sqr(a, a.len())
    } else {
        Vec::new()
    };
    
    // Compute ab using regular multiplication
    let ab = if !a.is_empty() {
        karatsuba_mul(a, a.len(), b, b.len())
    } else {
        Vec::new()
    };
    
    // result = b² + 2ab*x^split + a²*x^{2*split}
    
    // Add b² (at position 0)
    for (i, coeff) in b_sqr.iter().enumerate() {
        if i < result.len() {
            result[i] += coeff;
        }
    }
    
    // Add a²*x^{2*split} (at position 2*split)
    if !a_sqr.is_empty() {
        for (i, coeff) in a_sqr.iter().enumerate() {
            let pos = 2 * split + i;
            if pos < result.len() {
                result[pos] += coeff;
            }
        }
    }
    
    // Add 2ab*x^split (at position split)
    if !ab.is_empty() {
        for (i, coeff) in ab.iter().enumerate() {
            let pos = split + i;
            if pos < result.len() {
                result[pos] += coeff * &Integer::from(2);
            }
        }
    }
    
    result
}

/// Automatic squaring algorithm selection.
/// 
/// Selects the most appropriate squaring algorithm based on polynomial length
/// and coefficient characteristics, similar to flint's approach.
/// 
/// # Examples
/// 
/// ```
/// use stalagmite_poly::intpoly::arithmetic::sqr::auto_sqr;
/// use malachite::Integer;
/// 
/// let small_poly = vec![Integer::from(1), Integer::from(2)];
/// let result = auto_sqr(&small_poly, small_poly.len());
/// assert_eq!(result, vec![Integer::from(1), Integer::from(4), Integer::from(4)]);
/// 
/// let medium_poly: Vec<Integer> = (1..=12).map(Integer::from).collect();
/// let result = auto_sqr(&medium_poly, medium_poly.len());
/// // Should automatically select appropriate algorithm
/// ```
pub fn auto_sqr(poly: &[Integer], len: usize) -> Vec<Integer> {
    if len == 0 {
        return Vec::new();
    }
    
    if len <= TINY_SQR_THRESHOLD {
        tiny_sqr(poly, len)
    } else if len < KARATSUBA_SQR_THRESHOLD {
        classical_sqr(poly, len)
    } else {
        karatsuba_sqr(poly, len)
    }
}

/// Square an IntPoly using the most appropriate algorithm.
/// 
/// This is a high-level interface that automatically selects the best
/// squaring algorithm based on the polynomial characteristics.
/// 
/// # Examples
/// 
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
/// use stalagmite_poly::intpoly::arithmetic::sqr::sqr;
/// 
/// let poly = IntPoly::from(vec![1, 2, 3]); // 1 + 2x + 3x²
/// let result = sqr(&poly);
/// // (1 + 2x + 3x²)² = 1 + 4x + 10x² + 12x³ + 9x⁴
/// assert_eq!(result, IntPoly::from(vec![1, 4, 10, 12, 9]));
/// 
/// // Squaring zero polynomial
/// let zero = IntPoly::zero();
/// let result = sqr(&zero);
/// assert!(result.is_zero());
/// 
/// // Squaring constant polynomial
/// let constant = IntPoly::from(vec![5]);
/// let result = sqr(&constant);
/// assert_eq!(result, IntPoly::from(vec![25]));
/// 
/// // Verify squaring matches multiplication
/// let poly = IntPoly::from(vec![1, 2, 3, 4, 5]);
/// let sqr_result = sqr(&poly);
/// let mul_result = &poly * &poly;
/// assert_eq!(sqr_result, mul_result);
/// ```
pub fn sqr(poly: &IntPoly) -> IntPoly {
    if poly.is_zero() {
        return IntPoly::zero();
    }
    
    let coeffs = auto_sqr(&poly.coeffs, poly.length());
    IntPoly::from_raw(coeffs)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tiny_sqr_basic() {
        let poly = vec![Integer::from(1), Integer::from(2)]; // 1 + 2x
        let result = tiny_sqr(&poly, 2);
        // (1 + 2x)² = 1 + 4x + 4x²
        assert_eq!(result, vec![Integer::from(1), Integer::from(4), Integer::from(4)]);
    }
    
    #[test]
    fn test_tiny_sqr_longer() {
        let poly = vec![Integer::from(1), Integer::from(2), Integer::from(3)]; // 1 + 2x + 3x²
        let result = tiny_sqr(&poly, 3);
        // (1 + 2x + 3x²)² = 1 + 4x + 10x² + 12x³ + 9x⁴
        assert_eq!(result, vec![
            Integer::from(1), Integer::from(4), Integer::from(10), 
            Integer::from(12), Integer::from(9)
        ]);
    }
    
    #[test]
    fn test_classical_sqr_vs_multiplication() {
        let poly: Vec<Integer> = (1..=6).map(Integer::from).collect();
        let sqr_result = classical_sqr(&poly, 6);
        let mul_result = classical_mul(&poly, 6, &poly, 6);
        assert_eq!(sqr_result, mul_result);
    }
    
    #[test]
    fn test_karatsuba_sqr_vs_multiplication() {
        let poly: Vec<Integer> = (1..=20).map(Integer::from).collect();
        let sqr_result = karatsuba_sqr(&poly, 20);
        let mul_result = karatsuba_mul(&poly, 20, &poly, 20);
        assert_eq!(sqr_result, mul_result);
    }
    
    #[test]
    fn test_auto_sqr_selection() {
        // Test tiny threshold
        let tiny_poly = vec![Integer::from(1), Integer::from(2)];
        let result = auto_sqr(&tiny_poly, 2);
        assert_eq!(result, tiny_sqr(&tiny_poly, 2));
        
        // Test medium size
        let medium_poly: Vec<Integer> = (1..=10).map(Integer::from).collect();
        let result = auto_sqr(&medium_poly, 10);
        assert_eq!(result, classical_sqr(&medium_poly, 10));
        
        // Test large size  
        let large_poly: Vec<Integer> = (1..=25).map(Integer::from).collect();
        let result = auto_sqr(&large_poly, 25);
        assert_eq!(result, karatsuba_sqr(&large_poly, 25));
    }
    
    #[test]
    fn test_sqr_intpoly() {
        let poly = IntPoly::from(vec![1, 2, 3]);
        let result = sqr(&poly);
        assert_eq!(result, IntPoly::from(vec![1, 4, 10, 12, 9]));
        
        // Verify against multiplication
        let mul_result = &poly * &poly;
        assert_eq!(result, mul_result);
    }
    
    #[test]
    fn test_sqr_edge_cases() {
        // Zero polynomial
        let zero = IntPoly::zero();
        assert!(sqr(&zero).is_zero());
        
        // Constant polynomial
        let constant = IntPoly::from(vec![3]);
        assert_eq!(sqr(&constant), IntPoly::from(vec![9]));
        
        // Single term
        let single = IntPoly::from(vec![0, 5]); // 5x
        assert_eq!(sqr(&single), IntPoly::from(vec![0, 0, 25])); // 25x²
    }
}