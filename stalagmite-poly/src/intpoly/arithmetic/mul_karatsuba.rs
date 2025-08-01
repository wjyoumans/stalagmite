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

//! Karatsuba polynomial multiplication algorithms.
//!
//! This module implements the Karatsuba algorithm for polynomial multiplication,
//! which achieves O(n^log₂3) ≈ O(n^1.585) complexity compared to the classical
//! O(n²) algorithm. The algorithm is most effective when polynomials have
//! similar lengths and large coefficients.
//!
//! The implementation includes both traditional Karatsuba and a threshold-based
//! hybrid approach that falls back to classical multiplication for small inputs.

use malachite::Integer;
use crate::intpoly::IntPoly;
use crate::intpoly::arithmetic::mul_classical::classical_mul;

/// Threshold below which we use classical multiplication instead of Karatsuba.
/// This avoids the overhead of recursion for small polynomials.
const KARATSUBA_THRESHOLD: usize = 8;

/// Add polynomial `src` to `dst` starting at position `offset`.
/// 
/// This is equivalent to `dst += src * x^offset` where `x^offset` represents
/// shifting the polynomial by `offset` positions.
fn add_at_offset(dst: &mut [Integer], src: &[Integer], offset: usize) {
    for (i, coeff) in src.iter().enumerate() {
        if offset + i < dst.len() {
            dst[offset + i] += coeff;
        }
    }
}

/// Subtract polynomial `src` from `dst` starting at position `offset`.
/// 
/// This is equivalent to `dst -= src * x^offset`.
fn sub_at_offset(dst: &mut [Integer], src: &[Integer], offset: usize) {
    for (i, coeff) in src.iter().enumerate() {
        if offset + i < dst.len() {
            dst[offset + i] -= coeff;
        }
    }
}

/// Traditional Karatsuba multiplication algorithm.
/// 
/// Uses the identity:
/// ```text
/// f(x) * g(x) = ac*x^{2m} + (ad + bc)*x^m + bd
/// ```
/// where:
/// - `f(x) = a*x^m + b` (split at position m)
/// - `g(x) = c*x^m + d` (split at position m)
/// - `ad + bc = (a+b)(c+d) - ac - bd` (computed with only 3 multiplications)
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
/// use stalagmite_poly::intpoly::arithmetic::mul_karatsuba::karatsuba_mul;
/// use malachite::Integer;
/// 
/// let poly1 = vec![Integer::from(1), Integer::from(2), Integer::from(3), Integer::from(4)];
/// let poly2 = vec![Integer::from(5), Integer::from(6), Integer::from(7), Integer::from(8)];
/// let result = karatsuba_mul(&poly1, poly1.len(), &poly2, poly2.len());
/// // Verify the result matches classical multiplication
/// ```
pub fn karatsuba_mul(poly1: &[Integer], len1: usize, poly2: &[Integer], len2: usize) -> Vec<Integer> {
    if len1 == 0 || len2 == 0 {
        return Vec::new();
    }
    
    // Use classical multiplication for small inputs
    if len1 < KARATSUBA_THRESHOLD || len2 < KARATSUBA_THRESHOLD {
        return classical_mul(poly1, len1, poly2, len2);
    }
    
    let result_len = len1 + len2 - 1;
    let mut result = vec![Integer::from(0); result_len];
    
    // For simplicity, handle the case where lengths are very different
    // by falling back to classical multiplication
    if len1 < len2 / 2 || len2 < len1 / 2 {
        return classical_mul(poly1, len1, poly2, len2);
    }
    
    // Choose split point
    let split = len1.max(len2) / 2;
    
    // Split polynomials: f = a*x^split + b, g = c*x^split + d
    let (a, b) = if len1 > split { 
        (&poly1[split..len1], &poly1[0..split]) 
    } else { 
        (&poly1[len1..len1], &poly1[0..len1]) // Empty slice
    };
    
    let (c, d) = if len2 > split { 
        (&poly2[split..len2], &poly2[0..split]) 
    } else { 
        (&poly2[len2..len2], &poly2[0..len2]) // Empty slice
    };
    
    // Compute bd
    let bd = if !b.is_empty() && !d.is_empty() {
        karatsuba_mul(b, b.len(), d, d.len())
    } else {
        Vec::new()
    };
    
    // Compute ac  
    let ac = if !a.is_empty() && !c.is_empty() {
        karatsuba_mul(a, a.len(), c, c.len())
    } else {
        Vec::new()
    };
    
    // Compute (a+b) and (c+d)
    let max_low_len = split;
    let mut a_plus_b = vec![Integer::from(0); max_low_len.max(a.len())];
    let mut c_plus_d = vec![Integer::from(0); max_low_len.max(c.len())];
    
    // a_plus_b = a + b (with appropriate zero padding)
    for (i, coeff) in b.iter().enumerate() {
        if i < a_plus_b.len() {
            a_plus_b[i] = coeff.clone();
        }
    }
    for (i, coeff) in a.iter().enumerate() {
        if i < a_plus_b.len() {
            a_plus_b[i] += coeff;
        }
    }
    
    // c_plus_d = c + d (with appropriate zero padding)  
    for (i, coeff) in d.iter().enumerate() {
        if i < c_plus_d.len() {
            c_plus_d[i] = coeff.clone();
        }
    }
    for (i, coeff) in c.iter().enumerate() {
        if i < c_plus_d.len() {
            c_plus_d[i] += coeff;
        }
    }
    
    // Remove trailing zeros
    while let Some(&ref last) = a_plus_b.last() {
        if *last == 0 {
            a_plus_b.pop();
        } else {
            break;
        }
    }
    while let Some(&ref last) = c_plus_d.last() {
        if *last == 0 {
            c_plus_d.pop();
        } else {
            break;
        }
    }
    
    // Compute (a+b)(c+d)
    let ab_cd = if !a_plus_b.is_empty() && !c_plus_d.is_empty() {
        karatsuba_mul(&a_plus_b, a_plus_b.len(), &c_plus_d, c_plus_d.len())
    } else {
        Vec::new()
    };
    
    // result = bd + (ab_cd - ac - bd)*x^split + ac*x^{2*split}
    
    // Add bd (at position 0)
    if !bd.is_empty() {
        add_at_offset(&mut result, &bd, 0);
    }
    
    // Add ac*x^{2*split} (at position 2*split)
    if !ac.is_empty() {
        add_at_offset(&mut result, &ac, 2 * split);
    }
    
    // Add (ab_cd - ac - bd)*x^split (at position split)
    if !ab_cd.is_empty() {
        add_at_offset(&mut result, &ab_cd, split);
        
        // Subtract ac*x^split
        if !ac.is_empty() {
            sub_at_offset(&mut result, &ac, split);
        }
        
        // Subtract bd*x^split  
        if !bd.is_empty() {
            sub_at_offset(&mut result, &bd, split);
        }
    }
    
    result
}

/// Karatsuba multiplication for IntPoly.
/// 
/// This is a high-level interface to the Karatsuba multiplication algorithm
/// that handles IntPoly types and their internal representation.
/// 
/// # Examples
/// 
/// ```
/// use stalagmite_poly::intpoly::IntPoly;
/// use stalagmite_poly::intpoly::arithmetic::mul_karatsuba::mul_karatsuba;
/// 
/// let poly1 = IntPoly::from(vec![1, 2, 3, 4]); // 1 + 2x + 3x² + 4x³
/// let poly2 = IntPoly::from(vec![5, 6, 7, 8]); // 5 + 6x + 7x² + 8x³  
/// let result = mul_karatsuba(&poly1, &poly2);
/// 
/// // Verify against classical multiplication
/// use stalagmite_poly::intpoly::arithmetic::mul_classical::mul_classical;
/// let expected = mul_classical(&poly1, &poly2);
/// assert_eq!(result, expected);
/// 
/// // Multiplication with zero polynomial
/// let zero = IntPoly::zero();
/// let result = mul_karatsuba(&poly1, &zero);
/// assert!(result.is_zero());
/// 
/// // Small polynomials (should fall back to classical)
/// let small1 = IntPoly::from(vec![1, 2]);
/// let small2 = IntPoly::from(vec![3, 4]);
/// let result = mul_karatsuba(&small1, &small2);
/// assert_eq!(result, IntPoly::from(vec![3, 10, 8]));
/// ```
pub fn mul_karatsuba(poly1: &IntPoly, poly2: &IntPoly) -> IntPoly {
    if poly1.is_zero() || poly2.is_zero() {
        return IntPoly::zero();
    }
    
    let coeffs = karatsuba_mul(&poly1.coeffs, poly1.length(), &poly2.coeffs, poly2.length());
    IntPoly::from_raw(coeffs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intpoly::arithmetic::mul_classical::classical_mul;
    
    #[test]
    fn test_karatsuba_vs_classical_small() {
        let poly1 = vec![Integer::from(1), Integer::from(2)];
        let poly2 = vec![Integer::from(3), Integer::from(4)];
        
        let karatsuba_result = karatsuba_mul(&poly1, 2, &poly2, 2);
        let classical_result = classical_mul(&poly1, 2, &poly2, 2);
        
        assert_eq!(karatsuba_result, classical_result);
    }
    
    #[test]
    fn test_karatsuba_vs_classical_medium() {
        let poly1: Vec<Integer> = (1..=10).map(Integer::from).collect();
        let poly2: Vec<Integer> = (11..=20).map(Integer::from).collect();
        
        let karatsuba_result = karatsuba_mul(&poly1, 10, &poly2, 10);
        let classical_result = classical_mul(&poly1, 10, &poly2, 10);
        
        assert_eq!(karatsuba_result, classical_result);
    }
    
    #[test]
    fn test_karatsuba_vs_classical_different_lengths() {
        let poly1: Vec<Integer> = (1..=15).map(Integer::from).collect();
        let poly2: Vec<Integer> = (1..=8).map(Integer::from).collect();
        
        let karatsuba_result = karatsuba_mul(&poly1, 15, &poly2, 8);
        let classical_result = classical_mul(&poly1, 15, &poly2, 8);
        
        assert_eq!(karatsuba_result, classical_result);
    }
    
    #[test]
    fn test_karatsuba_empty() {
        let poly1 = vec![Integer::from(1), Integer::from(2)];
        let poly2: Vec<Integer> = vec![];
        
        let result = karatsuba_mul(&poly1, 2, &poly2, 0);
        assert!(result.is_empty());
    }
    
    #[test]
    fn test_mul_karatsuba_intpoly() {
        let poly1 = IntPoly::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        let poly2 = IntPoly::from(vec![8, 7, 6, 5, 4, 3, 2, 1]);
        
        let karatsuba_result = mul_karatsuba(&poly1, &poly2);
        
        // Verify it matches classical multiplication
        use crate::intpoly::arithmetic::mul_classical::mul_classical;
        let classical_result = mul_classical(&poly1, &poly2);
        assert_eq!(karatsuba_result, classical_result);
    }
    
    #[test]
    fn test_add_at_offset() {
        let mut dst = vec![Integer::from(1), Integer::from(2), Integer::from(3), Integer::from(4)];
        let src = vec![Integer::from(10), Integer::from(20)];
        
        add_at_offset(&mut dst, &src, 1);
        assert_eq!(dst, vec![Integer::from(1), Integer::from(12), Integer::from(23), Integer::from(4)]);
    }
    
    #[test]
    fn test_sub_at_offset() {
        let mut dst = vec![Integer::from(15), Integer::from(25), Integer::from(35), Integer::from(4)];
        let src = vec![Integer::from(10), Integer::from(20)];
        
        sub_at_offset(&mut dst, &src, 1);
        assert_eq!(dst, vec![Integer::from(15), Integer::from(15), Integer::from(15), Integer::from(4)]);
    }
}