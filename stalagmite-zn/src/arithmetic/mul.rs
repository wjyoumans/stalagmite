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

use std::ops::{Mul, MulAssign};
use crate::{ZnElem, check_moduli};

/// Multiply two integers modulo n.
///
/// This operation multiplies the values and reduces the result modulo n.
///
/// # Examples
///
/// ```
/// use stalagmite_zn::{ZnRing, ZnElem};
/// use malachite::Natural;
///
/// let zn = ZnRing::init(Natural::from(7u32));
/// let a = zn.new(Natural::from(3u32));
/// let b = zn.new(Natural::from(5u32));
/// let result = a * b; // (3 * 5) mod 7 = 1
/// assert_eq!(result.value(), Natural::from(1u32));
/// ```
impl Mul for ZnElem {
    type Output = ZnElem;
    
    fn mul(self, rhs: ZnElem) -> ZnElem {
        // Check that both elements have the same modulus
        check_moduli!(self, rhs);
        
        let product = &self.value * &rhs.value;
        let reduced = product % &self.ctx.modulus;
        
        ZnElem::from_ctx(reduced, self.ctx)
    }
}

/// Multiply this integer modulo n by another one, modifying this element in place.
impl MulAssign for ZnElem {
    fn mul_assign(&mut self, rhs: ZnElem) {
        // Check that both elements have the same modulus
        check_moduli!(self, rhs);
        
        self.value *= &rhs.value;
        self.value %= &self.ctx.modulus;
    }
}

/// Multiply by a reference to another integer modulo n.
impl Mul<&ZnElem> for ZnElem {
    type Output = ZnElem;
    
    fn mul(self, rhs: &ZnElem) -> ZnElem {
        check_moduli!(self, rhs);
        
        let product = &self.value * &rhs.value;
        let reduced = product % &self.ctx.modulus;
        
        ZnElem::from_ctx(reduced, self.ctx)
    }
}

/// Multiply a reference to this integer modulo n by another one.
impl Mul<ZnElem> for &ZnElem {
    type Output = ZnElem;
    
    fn mul(self, rhs: ZnElem) -> ZnElem {
        check_moduli!(self, rhs);
        
        let product = &self.value * &rhs.value;
        let reduced = product % &self.ctx.modulus;
        
        ZnElem::from_ctx(reduced, rhs.ctx)
    }
}

/// Multiply two references to integers modulo n.
impl Mul<&ZnElem> for &ZnElem {
    type Output = ZnElem;
    
    fn mul(self, rhs: &ZnElem) -> ZnElem {
        check_moduli!(self, rhs);
        
        let product = &self.value * &rhs.value;
        let reduced = product % &self.ctx.modulus;
        
        ZnElem::from_ctx(reduced, self.ctx.clone())
    }
}

/// Multiply this integer modulo n by a reference to another one.
impl MulAssign<&ZnElem> for ZnElem {
    fn mul_assign(&mut self, rhs: &ZnElem) {
        check_moduli!(self, rhs);
        
        self.value *= &rhs.value;
        self.value %= &self.ctx.modulus;
    }
}