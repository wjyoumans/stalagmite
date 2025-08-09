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
use std::rc::Rc;
use crate::{ZnElem, IntegerMod};
use malachite::base::num::arithmetic::traits::{ModMulPrecomputed, ModMulPrecomputedAssign};
use stalagmite_base::traits::Element;

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
    
    #[inline]    
    fn mul(mut self, rhs: ZnElem) -> ZnElem {
        self *= rhs;
        self
    }
}

/// Multiply by a reference to another integer modulo n.
impl Mul<&ZnElem> for ZnElem {
    type Output = ZnElem;
    
    #[inline]    
    fn mul(mut self, rhs: &ZnElem) -> ZnElem {
        self *= rhs;
        self
    }
}

/// Multiply a reference to this integer modulo n by another one.
impl Mul<ZnElem> for &ZnElem {
    type Output = ZnElem;

    #[inline]    
    fn mul(self, mut rhs: ZnElem) -> ZnElem {
        rhs *= self;
        rhs
    }
}

/// Multiply two references to integers modulo n.
impl Mul<&ZnElem> for &ZnElem {
    type Output = ZnElem;
    
    fn mul(self, rhs: &ZnElem) -> ZnElem {
        check_moduli!(self, rhs);
        let ctx = Rc::clone(&self.ctx);

        let res = (&self.value).mod_mul_precomputed(&rhs.value, ctx.modulus(), ctx.mod_mul_data());
        ZnElem::from_ctx(res, ctx)
    }
}

/// Multiply this integer modulo n by another one, modifying this element in place.
impl MulAssign for ZnElem {
    fn mul_assign(&mut self, rhs: ZnElem) {
        check_moduli!(self, rhs);
        let IntegerMod { value: rhs_value, ctx: rhs_ctx } = rhs;
        self.value.mod_mul_precomputed_assign(rhs_value, rhs_ctx.modulus(), rhs_ctx.mod_mul_data());
    }
}

/// Multiply this integer modulo n by a reference to another one.
impl MulAssign<&ZnElem> for ZnElem {
    fn mul_assign(&mut self, rhs: &ZnElem) {
        check_moduli!(self, rhs);
        let ctx = Rc::clone(&self.ctx);
        self.value.mod_mul_precomputed_assign(&rhs.value, ctx.modulus(), ctx.mod_mul_data());
    }
}
