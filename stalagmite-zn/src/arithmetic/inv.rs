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

use malachite::base::num::arithmetic::traits::ModInverse;
use crate::ZnElem;

pub trait Inv {
    type Output;

    fn inv(self) -> Self::Output;
}

impl Inv for ZnElem {
    type Output = ZnElem;

    fn inv(self) -> Self::Output {
        let inverse = self.value().mod_inverse(self.modulus())
            .expect("Element has no modular inverse");
        ZnElem::from_ctx(inverse, self.ctx.clone())
    }
}

impl Inv for &ZnElem {
    type Output = ZnElem;

    fn inv(self) -> Self::Output {
        let inverse = self.value().mod_inverse(self.modulus())
            .expect("Element has no modular inverse");
        ZnElem::from_ctx(inverse, self.ctx.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ZnRing;
    use malachite::Natural;

    #[test]
    fn test_inverse_prime_modulus() {
        let ring = ZnRing::init(Natural::from(7u32));
        
        // 3^(-1) ≡ 5 (mod 7) since 3 * 5 = 15 ≡ 1 (mod 7)
        let three = ring.new(Natural::from(3u32));
        let inv_three = three.inv();
        assert_eq!(*inv_three.value(), Natural::from(5u32));
    }

    #[test]
    fn test_inverse_composite_modulus() {
        let ring = ZnRing::init(Natural::from(15u32));
        
        // 4^(-1) ≡ 4 (mod 15) since 4 * 4 = 16 ≡ 1 (mod 15)
        let four = ring.new(Natural::from(4u32));
        let inv_four = four.inv();
        assert_eq!(*inv_four.value(), Natural::from(4u32));
    }

    #[test]
    #[should_panic(expected = "Element has no modular inverse")]
    fn test_no_inverse() {
        let ring = ZnRing::init(Natural::from(15u32));
        
        // 6 has no inverse mod 15 since gcd(6, 15) = 3 ≠ 1
        let six = ring.new(Natural::from(6u32));
        six.inv(); // This should panic
    }

    #[test]
    fn test_inverse_reference() {
        let ring = ZnRing::init(Natural::from(7u32));
        
        let three = ring.new(Natural::from(3u32));
        let inv_three = (&three).inv();
        assert_eq!(*inv_three.value(), Natural::from(5u32));
    }
}