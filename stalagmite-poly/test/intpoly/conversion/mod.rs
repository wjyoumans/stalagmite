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

use stalagmite_poly::intpoly::IntPoly;
use malachite::Integer;

#[cfg(test)]
mod tests {
    use super::*;

    // Test From<Vec<T>> implementations
    #[test]
    fn test_from_vec_i32() {
        let coeffs = vec![1, 2, 3];
        let poly = IntPoly::from(coeffs);
        assert_eq!(poly.length, 3);
        assert_eq!(poly.coeffs[0], Integer::from(1));
        assert_eq!(poly.coeffs[1], Integer::from(2));
        assert_eq!(poly.coeffs[2], Integer::from(3));
    }

    #[test]
    fn test_from_vec_i64() {
        let coeffs = vec![1i64, 2i64, 3i64];
        let poly = IntPoly::from(coeffs);
        assert_eq!(poly.length, 3);
        assert_eq!(poly.coeffs[0], Integer::from(1));
        assert_eq!(poly.coeffs[1], Integer::from(2));
        assert_eq!(poly.coeffs[2], Integer::from(3));
    }

    #[test]
    fn test_from_vec_integer() {
        let coeffs = vec![Integer::from(1), Integer::from(2), Integer::from(3)];
        let poly = IntPoly::from(coeffs);
        assert_eq!(poly.length, 3);
        assert_eq!(poly.coeffs[0], Integer::from(1));
        assert_eq!(poly.coeffs[1], Integer::from(2));
        assert_eq!(poly.coeffs[2], Integer::from(3));
    }

    #[test]
    fn test_from_vec_with_trailing_zeros() {
        let coeffs = vec![1, 2, 0, 0];
        let poly = IntPoly::from(coeffs);
        assert_eq!(poly.length, 2); // Should normalize
    }

    #[test]
    fn test_from_empty_vec() {
        let coeffs: Vec<i32> = vec![];
        let poly = IntPoly::from(coeffs);
        assert_eq!(poly.length, 0);
        assert!(poly.is_zero());
    }

    // Test From<&[T]> implementations
    #[test]
    fn test_from_slice_integer() {
        let coeffs = vec![Integer::from(1), Integer::from(2), Integer::from(3)];
        let poly = IntPoly::from(coeffs);
        assert_eq!(poly.length, 3);
        assert_eq!(poly.coeffs[0], Integer::from(1));
        assert_eq!(poly.coeffs[1], Integer::from(2));
        assert_eq!(poly.coeffs[2], Integer::from(3));
    }

    #[test]
    fn test_from_slice_integer_vec() {
        let coeffs = vec![Integer::from(1i64), Integer::from(2i64), Integer::from(3i64)];
        let poly = IntPoly::from(coeffs);
        assert_eq!(poly.length, 3);
        assert_eq!(poly.coeffs[0], Integer::from(1));
        assert_eq!(poly.coeffs[1], Integer::from(2));
        assert_eq!(poly.coeffs[2], Integer::from(3));
    }

    #[test]
    fn test_from_slice_with_trailing_zeros() {
        let coeffs = vec![Integer::from(1), Integer::from(2), Integer::from(0), Integer::from(0)];
        let poly = IntPoly::from(coeffs);
        assert_eq!(poly.length, 2); // Should normalize
    }

    #[test]
    fn test_from_empty_slice() {
        let coeffs: Vec<Integer> = vec![];
        let poly = IntPoly::from(coeffs);
        assert_eq!(poly.length, 0);
        assert!(poly.is_zero());
    }

    // Test From<[T; CAP]> implementations
    #[test]
    fn test_from_array_i32() {
        let coeffs = [1, 2, 3];
        let poly = IntPoly::from(coeffs);
        assert_eq!(poly.length, 3);
        assert_eq!(poly.coeffs[0], Integer::from(1));
        assert_eq!(poly.coeffs[1], Integer::from(2));
        assert_eq!(poly.coeffs[2], Integer::from(3));
    }

    #[test]
    fn test_from_array_i64() {
        let coeffs = [1i64, 2i64, 3i64];
        let poly = IntPoly::from(coeffs);
        assert_eq!(poly.length, 3);
        assert_eq!(poly.coeffs[0], Integer::from(1));
        assert_eq!(poly.coeffs[1], Integer::from(2));
        assert_eq!(poly.coeffs[2], Integer::from(3));
    }

    #[test]
    fn test_from_array_with_trailing_zeros() {
        let coeffs = [1, 2, 0, 0];
        let poly = IntPoly::from(coeffs);
        assert_eq!(poly.length, 2); // Should normalize
    }

    #[test]
    fn test_from_array_single_element() {
        let coeffs = [42];
        let poly = IntPoly::from(coeffs);
        assert_eq!(poly.length, 1);
        assert_eq!(poly.coeffs[0], Integer::from(42));
    }

    #[test]
    fn test_from_array_all_zeros() {
        let coeffs = [0, 0, 0];
        let poly = IntPoly::from(coeffs);
        assert_eq!(poly.length, 0); // Should normalize to zero
        assert!(poly.is_zero());
    }

    // Test From<&[T; CAP]> implementations
    #[test]
    fn test_from_array_ref_i32() {
        let coeffs = &[1, 2, 3];
        let poly = IntPoly::from(coeffs);
        assert_eq!(poly.length, 3);
        assert_eq!(poly.coeffs[0], Integer::from(1));
        assert_eq!(poly.coeffs[1], Integer::from(2));
        assert_eq!(poly.coeffs[2], Integer::from(3));
    }

    #[test]
    fn test_from_array_ref_i64() {
        let coeffs = &[1i64, 2i64, 3i64];
        let poly = IntPoly::from(coeffs);
        assert_eq!(poly.length, 3);
        assert_eq!(poly.coeffs[0], Integer::from(1));
        assert_eq!(poly.coeffs[1], Integer::from(2));
        assert_eq!(poly.coeffs[2], Integer::from(3));
    }

    #[test]
    fn test_from_array_ref_with_trailing_zeros() {
        let coeffs = &[1, 2, 0, 0];
        let poly = IntPoly::from(coeffs);
        assert_eq!(poly.length, 2); // Should normalize
    }

    #[test]
    fn test_from_array_ref_single_element() {
        let coeffs = &[42];
        let poly = IntPoly::from(coeffs);
        assert_eq!(poly.length, 1);
        assert_eq!(poly.coeffs[0], Integer::from(42));
    }

    #[test]
    fn test_from_array_ref_all_zeros() {
        let coeffs = &[0, 0, 0];
        let poly = IntPoly::from(coeffs);
        assert_eq!(poly.length, 0); // Should normalize to zero
        assert!(poly.is_zero());
    }

    // Test mixed usage patterns
    #[test]
    fn test_conversion_consistency() {
        let data = vec![1, 2, 3];
        
        let from_vec = IntPoly::from(data.clone());
        let from_array = IntPoly::from([1, 2, 3]);
        let from_array_ref = IntPoly::from(&[1, 2, 3]);
        
        assert_eq!(from_vec, from_array);
        assert_eq!(from_array, from_array_ref);
    }
}