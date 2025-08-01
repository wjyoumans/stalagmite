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

use std::ops::{Add, AddAssign};
use crate::{ZnElem, check_moduli};

/// Add two integers modulo n.
///
/// This operation adds the values and reduces the result modulo n.
///
/// # Examples
///
/// ```
/// use stalagmite_zn::{ZnRing, ZnElem};
/// use malachite::Natural;
///
/// let zn = ZnRing::init(Natural::from(7u32));
/// let a = zn.new(Natural::from(5u32));
/// let b = zn.new(Natural::from(4u32));
/// let result = a + b; // (5 + 4) mod 7 = 2
/// assert_eq!(result.value(), Natural::from(2u32));
/// ```
impl Add for ZnElem {
    type Output = ZnElem;
    
    fn add(self, rhs: ZnElem) -> ZnElem {
        // Check that both elements have the same modulus
        check_moduli!(self, rhs);
        
        let sum = &self.value + &rhs.value;
        let reduced = sum % &self.ctx.modulus;
        
        ZnElem::from_ctx(reduced, self.ctx)
    }
}

/// Add another integer modulo n to this one, modifying this element in place.
impl AddAssign for ZnElem {
    fn add_assign(&mut self, rhs: ZnElem) {
        // Check that both elements have the same modulus
        check_moduli!(self, rhs);
        
        self.value += &rhs.value;
        self.value %= &self.ctx.modulus;
    }
}

/// Add a reference to another integer modulo n.
impl Add<&ZnElem> for ZnElem {
    type Output = ZnElem;
    
    fn add(self, rhs: &ZnElem) -> ZnElem {
        check_moduli!(self, rhs);
        
        let sum = &self.value + &rhs.value;
        let reduced = sum % &self.ctx.modulus;
        
        ZnElem::from_ctx(reduced, self.ctx)
    }
}

/// Add another integer modulo n to a reference to this one.
impl Add<ZnElem> for &ZnElem {
    type Output = ZnElem;
    
    fn add(self, rhs: ZnElem) -> ZnElem {
        check_moduli!(self, rhs);
        
        let sum = &self.value + &rhs.value;
        let reduced = sum % &self.ctx.modulus;
        
        ZnElem::from_ctx(reduced, rhs.ctx)
    }
}

/// Add two references to integers modulo n.
impl Add<&ZnElem> for &ZnElem {
    type Output = ZnElem;
    
    fn add(self, rhs: &ZnElem) -> ZnElem {
        check_moduli!(self, rhs);
        
        let sum = &self.value + &rhs.value;
        let reduced = sum % &self.ctx.modulus;
        
        ZnElem::from_ctx(reduced, self.ctx.clone())
    }
}

/// Add a reference to another integer modulo n to this one.
impl AddAssign<&ZnElem> for ZnElem {
    fn add_assign(&mut self, rhs: &ZnElem) {
        check_moduli!(self, rhs);
        
        self.value += &rhs.value;
        self.value %= &self.ctx.modulus;
    }
}