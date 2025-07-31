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

use stalagmite_poly2::intpoly::IntPoly;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_owned_owned() {
        let a = IntPoly::from(vec![1, 2, 3]);
        let b = IntPoly::from(vec![4, 5, 6, 7]);
        let c = a + b;
        assert_eq!(c, IntPoly::from(vec![5, 7, 9, 7]));
    }

    #[test]
    fn test_add_owned_ref() {
        let a = IntPoly::from(vec![1, 2, -10]);
        let b = IntPoly::from(vec![4, 5, 6, 7, -4]);
        let c = a + &b;
        assert_eq!(c, IntPoly::from(vec![5, 7, -4, 7, -4]));
    }

    #[test]
    fn test_add_ref_owned() {
        let a = IntPoly::from(vec![1, 2, 3, 4]);
        let b = IntPoly::from(vec![4, 5, 6]);
        let c = &a + b;
        assert_eq!(c, IntPoly::from(vec![5, 7, 9, 4]));
    }

    #[test]
    fn test_add_ref_ref() {
        let a = IntPoly::from(vec![1, 2, 3]);
        let b = IntPoly::from(vec![4, 5, 6, -5]);
        let c = &a + &b;
        assert_eq!(c, IntPoly::from(vec![5, 7, 9, -5]));
    }

    #[test]
    fn test_add_different_lengths() {
        let a = IntPoly::from(vec![1, 2]);
        let b = IntPoly::from(vec![3, 4, 5]);
        let c = a + b;
        assert_eq!(c, IntPoly::from(vec![4, 6, 5]));
    }

    #[test]
    fn test_add_with_zero() {
        let a = IntPoly::from(vec![1, 2, 3]);
        let zero = IntPoly::zero();
        let c1 = &a + &zero;
        let c2 = &zero + &a;
        assert_eq!(c1, a);
        assert_eq!(c2, a);
    }

    #[test]
    fn test_add_resulting_in_zero() {
        let a = IntPoly::from(vec![1, 2, 3]);
        let b = IntPoly::from(vec![-1, -2, -3]);
        let c = a + b;
        assert!(c.is_zero());
    }

    #[test]
    fn test_add_with_normalization() {
        let a = IntPoly::from(vec![1, 2, 3]);
        let b = IntPoly::from(vec![0, 0, -3]);
        let c = a + b;
        assert_eq!(c, IntPoly::from(vec![1, 2]));
        assert_eq!(c.length(), 2);
    }

    #[test]
    fn test_add_assign_owned() {
        let mut a = IntPoly::from(vec![1, 2, 3]);
        let b = IntPoly::from(vec![4, 5, 6]);
        a += b;
        assert_eq!(a, IntPoly::from(vec![5, 7, 9]));
    }

    #[test]
    fn test_add_assign_ref() {
        let mut a = IntPoly::from(vec![1, 2, 3]);
        let b = IntPoly::from(vec![4, 5, 6]);
        a += &b;
        assert_eq!(a, IntPoly::from(vec![5, 7, 9]));
    }

    #[test]
    fn test_add_assign_with_zero_lhs() {
        let mut a = IntPoly::zero();
        let b = IntPoly::from(vec![1, 2, 3]);
        a += &b;
        assert_eq!(a, b);
    }

    #[test]
    fn test_add_assign_with_zero_rhs() {
        let mut a = IntPoly::from(vec![1, 2, 3]);
        let zero = IntPoly::zero();
        let expected = a.clone();
        a += &zero;
        assert_eq!(a, expected);
    }

    #[test]
    fn test_add_assign_different_lengths() {
        let mut a = IntPoly::from(vec![1, 2]);
        let b = IntPoly::from(vec![3, 4, 5]);
        a += b;
        assert_eq!(a, IntPoly::from(vec![4, 6, 5]));
    }

    #[test]
    fn test_sum_iterator() {
        let polys = vec![
            IntPoly::from(vec![1, 2]),
            IntPoly::from(vec![3, 4]),
            IntPoly::from(vec![5, 6]),
        ];
        let result: IntPoly = polys.into_iter().sum();
        assert_eq!(result, IntPoly::from(vec![9, 12]));
    }

    #[test]
    fn test_sum_ref_iterator() {
        let polys = vec![
            IntPoly::from(vec![1, 2]),
            IntPoly::from(vec![3, 4]),
            IntPoly::from(vec![5, 6]),
        ];
        let result: IntPoly = polys.iter().sum();
        assert_eq!(result, IntPoly::from(vec![9, 12]));
    }

    #[test]
    fn test_sum_empty_iterator() {
        let polys: Vec<IntPoly> = vec![];
        let result: IntPoly = polys.into_iter().sum();
        assert!(result.is_zero());
    }

    #[test]
    fn test_sum_single_element() {
        let polys = vec![IntPoly::from(vec![1, 2, 3])];
        let result: IntPoly = polys.into_iter().sum();
        assert_eq!(result, IntPoly::from(vec![1, 2, 3]));
    }
}