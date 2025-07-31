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

pub mod arithmetic;
pub mod comparison;
pub mod conversion;

use stalagmite_poly2::intpoly::IntPoly;
use malachite::Integer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let poly = IntPoly::default();
        assert_eq!(poly.length(), 0);
        assert!(poly.coeffs.is_empty());
        assert!(poly.is_zero());
    }

    #[test]
    fn test_new() {
        let coeffs = vec![1, 2, 3];
        let poly = IntPoly::new(coeffs);
        assert_eq!(poly.length(), 3);
        assert_eq!(poly.coeffs.len(), 3);
    }

    #[test]
    fn test_from_raw() {
        let coeffs = vec![Integer::from(1), Integer::from(2), Integer::from(0)];
        let poly = IntPoly::from_raw(coeffs);
        // Should normalize, removing trailing zero
        assert_eq!(poly.length(), 2);
    }

    #[test]
    fn test_normalize() {
        let mut poly = IntPoly {
            coeffs: vec![Integer::from(1), Integer::from(2), Integer::from(0), Integer::from(0)],
        };
        poly.normalize();
        assert_eq!(poly.length(), 2);
    }

    #[test]
    fn test_normalize_all_zeros() {
        let mut poly = IntPoly {
            coeffs: vec![Integer::from(0), Integer::from(0), Integer::from(0)],
        };
        poly.normalize();
        assert_eq!(poly.length(), 0);
        assert!(poly.is_zero());
    }

    #[test]
    fn test_with_capacity() {
        let poly = IntPoly::with_capacity(10);
        assert_eq!(poly.length(), 0);
        assert!(poly.coeffs.capacity() >= 10);
        assert!(poly.is_zero());
    }

    #[test]
    fn test_zero() {
        let poly = IntPoly::zero();
        assert_eq!(poly.length(), 0);
        assert!(poly.coeffs.is_empty());
        assert!(poly.is_zero());
    }

    #[test]
    fn test_is_zero() {
        let zero_poly = IntPoly::zero();
        assert!(zero_poly.is_zero());

        let non_zero_poly = IntPoly::from(vec![1]);
        assert!(!non_zero_poly.is_zero());
    }

    #[test]
    fn test_is_one() {
        let one_poly = IntPoly::from(vec![1]);
        assert!(one_poly.is_one());

        let not_one_poly = IntPoly::from(vec![2]);
        assert!(!not_one_poly.is_one());

        let zero_poly = IntPoly::zero();
        assert!(!zero_poly.is_one());

        let polynomial_poly = IntPoly::from(vec![1, 2]);
        assert!(!polynomial_poly.is_one());
    }

    #[test]
    fn test_is_gen() {
        // Generator polynomial is x (i.e., [0, 1])
        let gen_poly = IntPoly::from(vec![0, 1]);
        assert!(gen_poly.is_gen());

        let not_gen_poly = IntPoly::from(vec![1, 1]);
        assert!(!not_gen_poly.is_gen());

        let zero_poly = IntPoly::zero();
        assert!(!zero_poly.is_gen());

        let one_poly = IntPoly::from(vec![1]);
        assert!(!one_poly.is_gen());

        let higher_degree_poly = IntPoly::from(vec![0, 1, 1]);
        assert!(!higher_degree_poly.is_gen());
    }
}