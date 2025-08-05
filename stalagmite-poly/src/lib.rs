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

pub mod poly;
pub mod zz_poly;
pub mod qq_poly;
pub mod generic;

#[cfg(test)]
mod tests {
    use crate::zz_poly::ZZPoly;
    use crate::qq_poly::QQPoly;
    use malachite::{Integer, Natural};

    #[test]
    fn test_add() {
        let a = ZZPoly::from(&[1, 2, 3]);
        let b = ZZPoly::from([4, 5, 6]);
        let c = a + b;
        assert_eq!(c, ZZPoly::from([5, 7, 9]));
    }

    #[test]
    fn test_qq_poly_basic() {
        // Test basic creation and operations
        let a = QQPoly::from(vec![1, 2, 3]);  // 1 + 2x + 3x²
        let b = QQPoly::from(vec![4, 5]);     // 4 + 5x
        
        // Test addition
        let c = &a + &b;
        assert_eq!(c.numerator(), &ZZPoly::from(vec![5, 7, 3]));
        assert_eq!(c.denominator(), &Natural::from(1u32));
        
        // Test multiplication
        let d = &a * &b;
        // (1 + 2x + 3x²) * (4 + 5x) = 4 + 5x + 8x + 10x² + 12x² + 15x³ = 4 + 13x + 22x² + 15x³
        assert_eq!(d.numerator(), &ZZPoly::from(vec![4, 13, 22, 15]));
        assert_eq!(d.denominator(), &Natural::from(1u32));
    }

    #[test]
    fn test_qq_poly_with_denominator() {
        // Test polynomial with denominator
        let a = QQPoly::from((vec![1, 2], Natural::from(3u32)));  // (1 + 2x)/3
        let b = QQPoly::from((vec![3, 6], Natural::from(2u32)));  // (3 + 6x)/2
        
        // Test addition: (1 + 2x)/3 + (3 + 6x)/2 = (2 + 4x)/6 + (9 + 18x)/6 = (11 + 22x)/6
        let c = &a + &b;
        assert_eq!(c.numerator(), &ZZPoly::from(vec![11, 22]));
        assert_eq!(c.denominator(), &Natural::from(6u32));
    }
}
