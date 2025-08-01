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

use std::ops::Neg;
use malachite::Natural;
use malachite::base::num::basic::traits::Zero;
use crate::ZnElem;

/// Negate an integer modulo n.
///
/// This operation returns the additive inverse of the element modulo n.
/// For a non-zero element a, the result is n - a.
/// For zero, the result is zero.
///
/// # Examples
///
/// ```
/// use stalagmite_zn::{ZnRing, ZnElem};
/// use malachite::Natural;
///
/// let zn = ZnRing::init(Natural::from(7u32));
/// let a = zn.new(Natural::from(3u32));
/// let result = -a; // -3 mod 7 = 4
/// assert_eq!(result.value(), Natural::from(4u32));
///
/// let zero = zn.new(Natural::from(0u32));
/// let neg_zero = -zero; // -0 mod 7 = 0
/// assert_eq!(neg_zero.value(), Natural::from(0u32));
/// ```
impl Neg for ZnElem {
    type Output = ZnElem;
    
    fn neg(self) -> ZnElem {
        if self.value == Natural::ZERO {
            // -0 = 0
            self
        } else {
            // -a = n - a for non-zero a
            let neg_value = &self.ctx.modulus - &self.value;
            ZnElem::from_ctx(neg_value, self.ctx)
        }
    }
}

/// Negate a reference to an integer modulo n.
impl Neg for &ZnElem {
    type Output = ZnElem;
    
    fn neg(self) -> ZnElem {
        if self.value == Natural::ZERO {
            // -0 = 0
            ZnElem::from_ctx(Natural::ZERO, self.ctx.clone())
        } else {
            // -a = n - a for non-zero a
            let neg_value = &self.ctx.modulus - &self.value;
            ZnElem::from_ctx(neg_value, self.ctx.clone())
        }
    }
}