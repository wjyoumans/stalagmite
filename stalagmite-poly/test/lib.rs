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

pub mod generic;
pub mod poly;
pub mod zz_poly;

// Integration tests that test interactions between modules
#[cfg(test)]
mod integration_tests {
    use stalagmite_poly::zz_poly::ZZPoly;

    #[test]
    fn test_polynomial_workflow() {
        // Create polynomials using different conversion methods
        let p1 = ZZPoly::from(vec![1, 2, 3]);
        let p2 = ZZPoly::from(&[4, 5]);
        let p3 = ZZPoly::from([0, 1]); // x
        
        // Test operations
        let sum = &p1 + &p2; // Should be [5, 7, 3]
        assert_eq!(sum, ZZPoly::from(vec![5, 7, 3]));
        
        // Test with generator polynomial
        assert!(p3.is_gen());
        
        // Test zero polynomial
        let zero = ZZPoly::zero();
        assert!(zero.is_zero());
        assert_eq!(&p1 + &zero, p1);
        
        // Test one polynomial
        let one = ZZPoly::from(vec![1]);
        assert!(one.is_one());
    }

    #[test]
    fn test_normalization_in_operations() {
        // Create polynomials that will result in normalization after addition
        let p1 = ZZPoly::from(vec![1, 2, 3]);
        let p2 = ZZPoly::from(vec![0, 0, -3]);
        
        let result = p1 + p2;
        assert_eq!(result, ZZPoly::from(vec![1, 2]));
        assert_eq!(result.length(), 2);
    }

    #[test] 
    fn test_sum_with_mixed_types() {
        let polys = vec![
            ZZPoly::from([1, 0]),
            ZZPoly::from(vec![0, 1]),
            ZZPoly::from(&[2, 3]),
        ];
        
        let result: ZZPoly = polys.iter().sum();
        assert_eq!(result, ZZPoly::from(vec![3, 4]));
    }
}
