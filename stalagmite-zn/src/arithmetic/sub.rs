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

use std::ops::{Sub, SubAssign};
use crate::{ZnElem, check_moduli};

/// Subtract one integer modulo n from another.
///
/// This operation subtracts the values and reduces the result modulo n.
/// If the result would be negative, it wraps around to the positive equivalent.
///
/// # Examples
///
/// ```
/// use stalagmite_zn::{ZnRing, ZnElem};
/// use malachite::Natural;
///
/// let zn = ZnRing::init(Natural::from(7u32));
/// let a = zn.new(Natural::from(2u32));
/// let b = zn.new(Natural::from(5u32));
/// let result = a - b; // (2 - 5) mod 7 = 4
/// assert_eq!(result.value(), Natural::from(4u32));
/// ```
impl Sub for ZnElem {
    type Output = ZnElem;
    
    fn sub(self, rhs: ZnElem) -> ZnElem {
        // Check that both elements have the same modulus
        check_moduli!(self, rhs);
        
        let result = if self.value >= rhs.value {
            &self.value - &rhs.value
        } else {
            // Handle negative result by adding the modulus
            &self.ctx.modulus - (&rhs.value - &self.value)
        };
        
        ZnElem::from_ctx(result, self.ctx)
    }
}

/// Subtract another integer modulo n from this one, modifying this element in place.
impl SubAssign for ZnElem {
    fn sub_assign(&mut self, rhs: ZnElem) {
        // Check that both elements have the same modulus
        check_moduli!(self, rhs);
        
        if self.value >= rhs.value {
            self.value -= &rhs.value;
        } else {
            // Handle negative result by adding the modulus
            self.value = &self.ctx.modulus - (&rhs.value - &self.value);
        }
    }
}

/// Subtract a reference to another integer modulo n.
impl Sub<&ZnElem> for ZnElem {
    type Output = ZnElem;
    
    fn sub(self, rhs: &ZnElem) -> ZnElem {
        check_moduli!(self, rhs);
        
        let result = if self.value >= rhs.value {
            &self.value - &rhs.value
        } else {
            // Handle negative result by adding the modulus
            &self.ctx.modulus - (&rhs.value - &self.value)
        };
        
        ZnElem::from_ctx(result, self.ctx)
    }
}

/// Subtract another integer modulo n from a reference to this one.
impl Sub<ZnElem> for &ZnElem {
    type Output = ZnElem;
    
    fn sub(self, rhs: ZnElem) -> ZnElem {
        check_moduli!(self, rhs);
        
        let result = if self.value >= rhs.value {
            &self.value - &rhs.value
        } else {
            // Handle negative result by adding the modulus
            &self.ctx.modulus - (&rhs.value - &self.value)
        };
        
        ZnElem::from_ctx(result, rhs.ctx)
    }
}

/// Subtract two references to integers modulo n.
impl Sub<&ZnElem> for &ZnElem {
    type Output = ZnElem;
    
    fn sub(self, rhs: &ZnElem) -> ZnElem {
        check_moduli!(self, rhs);
        
        let result = if self.value >= rhs.value {
            &self.value - &rhs.value
        } else {
            // Handle negative result by adding the modulus
            &self.ctx.modulus - (&rhs.value - &self.value)
        };
        
        ZnElem::from_ctx(result, self.ctx.clone())
    }
}

/// Subtract a reference to another integer modulo n from this one.
impl SubAssign<&ZnElem> for ZnElem {
    fn sub_assign(&mut self, rhs: &ZnElem) {
        check_moduli!(self, rhs);
        
        if self.value >= rhs.value {
            self.value -= &rhs.value;
        } else {
            // Handle negative result by adding the modulus
            self.value = &self.ctx.modulus - (&rhs.value - &self.value);
        }
    }
}