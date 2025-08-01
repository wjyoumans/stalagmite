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

use malachite::Integer;
use crate::zz_poly::ZZPoly;
use std::ops::{
    Add,
    AddAssign,
};
use std::iter::Sum;
use std::mem::swap;


/// Add two `ZZPoly` polynomials together.
///
/// This operation adds corresponding coefficients and handles polynomials
/// of different lengths by treating missing coefficients as zero.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let p1 = ZZPoly::from(vec![1, 2, 3]);     // 1 + 2x + 3x²
/// let p2 = ZZPoly::from(vec![4, 5, 6, 7]); // 4 + 5x + 6x² + 7x³
/// let result = p1 + p2;                     // 5 + 7x + 9x² + 7x³
/// assert_eq!(result, ZZPoly::from(vec![5, 7, 9, 7]));
/// ```
impl Add for ZZPoly {
    type Output = ZZPoly;
    fn add(mut self, mut rhs: ZZPoly) -> ZZPoly {
        if self.length() >= rhs.length() {
            self += rhs;
            self
        } else {
            rhs += self;
            rhs
        }
    }
}

/// Add an `ZZPoly` reference to an owned `ZZPoly`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let p1 = ZZPoly::from(vec![1, 2, 3]);
/// let p2 = ZZPoly::from(vec![4, 5]);
/// let result = p1 + &p2;
/// assert_eq!(result, ZZPoly::from(vec![5, 7, 3]));
/// ```
impl Add<&ZZPoly> for ZZPoly {
    type Output = ZZPoly;
    #[inline]
    fn add(mut self, rhs: &ZZPoly) -> ZZPoly {
        self += rhs;
        self
    }
}

/// Add an owned `ZZPoly` to an `ZZPoly` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let p1 = ZZPoly::from(vec![1, 2, 3, 4]);
/// let p2 = ZZPoly::from(vec![4, 5, 6]);
/// let result = &p1 + p2;
/// assert_eq!(result, ZZPoly::from(vec![5, 7, 9, 4]));
/// ```
impl Add<ZZPoly> for &ZZPoly {
    type Output = ZZPoly;
    #[inline]
    fn add(self, mut rhs: ZZPoly) -> ZZPoly {
        rhs += self;
        rhs
    }
}

/// Add two `ZZPoly` references together.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let p1 = ZZPoly::from(vec![1, 2, 3]);
/// let p2 = ZZPoly::from(vec![4, 5, 6, -5]);
/// let result = &p1 + &p2;
/// assert_eq!(result, ZZPoly::from(vec![5, 7, 9, -5]));
/// ```
impl Add<&ZZPoly> for &ZZPoly {
    type Output = ZZPoly;
    fn add(self, rhs: &ZZPoly) -> ZZPoly {
        if rhs.is_zero() {
            return self.clone();
        }
        if self.is_zero() {
            return rhs.clone();
        }

        let n = self.length().max(rhs.length());
        let mut result = Vec::with_capacity(n);
        for i in 0..n {
            let a = if i < self.length() { &self.coeffs[i] } else { &Integer::from(0) };
            let b = if i < rhs.length() { &rhs.coeffs[i] } else { &Integer::from(0) };
            result.push(a + b);
        }
        ZZPoly::from_raw(result)
    }
}


/// Add an owned `ZZPoly` to this polynomial with assignment.
///
/// This re-uses the memory of the larger polynomial to avoid allocating
/// a new polynomial.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let mut p1 = ZZPoly::from(vec![1, 2, 3]);
/// let p2 = ZZPoly::from(vec![4, 5, 6]);
/// p1 += p2;
/// assert_eq!(p1, ZZPoly::from(vec![5, 7, 9]));
/// ```
impl AddAssign<ZZPoly> for ZZPoly {
    fn add_assign(&mut self, mut rhs: ZZPoly) {
        if rhs.is_zero() {
            return;
        } else if self.is_zero() {
            *self = rhs;
        } else {
            if self.length() < rhs.length() {
                swap(self, &mut rhs);
            }
        
            for i in 0..rhs.length() {
                self.coeffs[i] += &rhs.coeffs[i];
            }
        }
        self.normalize();
    }
}

/// Add a reference to an `ZZPoly` to this polynomial with assignment.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
///
/// let mut p1 = ZZPoly::from(vec![1, 2, 3]);
/// let p2 = ZZPoly::from(vec![4, 5, 6]);
/// p1 += &p2;
/// assert_eq!(p1, ZZPoly::from(vec![5, 7, 9]));
/// ```
impl AddAssign<&ZZPoly> for ZZPoly {
    fn add_assign(&mut self, rhs: &ZZPoly) {
        if rhs.is_zero() {
            return;
        } else if self.is_zero() {
            *self = rhs.clone();
        } else if self.length() < rhs.length() {
            // add the common coefficients
            for i in 0..self.length() {
                self.coeffs[i] += &rhs.coeffs[i];
            }

            // push the remaining coefficients from rhs
            for i in self.length()..rhs.length() {
                self.coeffs.push(rhs.coeffs[i].clone());
            }
        } else {
            for i in 0..rhs.length() {
                self.coeffs[i] += &rhs.coeffs[i];
            }
        }
        self.normalize();
    }
}

/// Sum an iterator of owned `ZZPoly` polynomials.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use std::iter::Sum;
///
/// let polys = vec![
///     ZZPoly::from(vec![1, 2]),
///     ZZPoly::from(vec![3, 4]),
///     ZZPoly::from(vec![5, 6]),
/// ];
/// let result: ZZPoly = polys.into_iter().sum();
/// assert_eq!(result, ZZPoly::from(vec![9, 12]));
/// ```
impl Sum for ZZPoly {
    fn sum<I: Iterator<Item = ZZPoly>>(iter: I) -> Self {
        iter.fold(ZZPoly::zero(), |acc, x| acc + x)
    }
}

/// Sum an iterator of `ZZPoly` references.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use std::iter::Sum;
///
/// let polys = vec![
///     ZZPoly::from(vec![1, 2]),
///     ZZPoly::from(vec![3, 4]),
///     ZZPoly::from(vec![5, 6]),
/// ];
/// let result: ZZPoly = polys.iter().sum();
/// assert_eq!(result, ZZPoly::from(vec![9, 12]));
/// ```
impl<'a> Sum<&'a ZZPoly> for ZZPoly {
    fn sum<I: Iterator<Item = &'a ZZPoly>>(iter: I) -> Self {
        iter.fold(ZZPoly::zero(), |acc, x| acc + x)
    }
}

/// Add an owned `Integer` to an `ZZPoly`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![1, 2, 3]); // 1 + 2x + 3x²
/// let result = poly + Integer::from(5);    // (1 + 5) + 2x + 3x² = 6 + 2x + 3x²
/// assert_eq!(result, ZZPoly::from(vec![6, 2, 3]));
/// ```
impl Add<Integer> for ZZPoly {
    type Output = ZZPoly;
    #[inline]
    fn add(mut self, rhs: Integer) -> ZZPoly {
        self += rhs;
        self
    }
}

/// Add a reference to an `Integer` to an `ZZPoly`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![1, 2, 3]); // 1 + 2x + 3x²
/// let value = Integer::from(5);
/// let result = poly + &value;              // (1 + 5) + 2x + 3x² = 6 + 2x + 3x²
/// assert_eq!(result, ZZPoly::from(vec![6, 2, 3]));
/// ```
impl Add<&Integer> for ZZPoly {
    type Output = ZZPoly;
    #[inline]
    fn add(mut self, rhs: &Integer) -> ZZPoly {
        self += rhs;
        self
    }
}

/// Add an owned `Integer` to an `ZZPoly` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![1, 2, 3]);
/// let result = &poly + Integer::from(5);
/// assert_eq!(result, ZZPoly::from(vec![6, 2, 3]));
/// ```
impl Add<Integer> for &ZZPoly {
    type Output = ZZPoly;
    fn add(self, rhs: Integer) -> ZZPoly {
        self.clone() + rhs
    }
}

/// Add an `Integer` reference to an `ZZPoly` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![7, 8, 9]);
/// let num = Integer::from(3);
/// let result = &poly + &num;
/// assert_eq!(result, ZZPoly::from(vec![10, 8, 9]));
/// ```
impl Add<&Integer> for &ZZPoly {
    type Output = ZZPoly;
    fn add(self, rhs: &Integer) -> ZZPoly {
        self.clone() + rhs
    }
}

/// Add an `ZZPoly` to an `Integer`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![1, 2, 3]);
/// let result = Integer::from(10) + poly;
/// assert_eq!(result, ZZPoly::from(vec![11, 2, 3]));
/// ```
impl Add<ZZPoly> for Integer {
    type Output = ZZPoly;
    #[inline]
    fn add(self, mut rhs: ZZPoly) -> ZZPoly {
        rhs += self;
        rhs
    }
}

/// Add an `ZZPoly` to an `Integer` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![7, 8]);
/// let num = Integer::from(3);
/// let result = &num + poly;
/// assert_eq!(result, ZZPoly::from(vec![10, 8]));
/// ```
impl Add<ZZPoly> for &Integer {
    type Output = ZZPoly;
    #[inline]
    fn add(self, mut rhs: ZZPoly) -> ZZPoly {
        rhs += self;
        rhs
    }
}

/// Add an `ZZPoly` reference to an `Integer`.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![1, 2, 3]);
/// let result = Integer::from(5) + &poly;
/// assert_eq!(result, ZZPoly::from(vec![6, 2, 3]));
/// ```
impl Add<&ZZPoly> for Integer {
    type Output = ZZPoly;
    fn add(self, rhs: &ZZPoly) -> ZZPoly {
        if rhs.is_zero() {
            return ZZPoly::from_raw(vec![self]);
        }
        
        let mut res = rhs.clone();
        res += self;
        res
    }
}

/// Add an `ZZPoly` reference to an `Integer` reference.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let poly = ZZPoly::from(vec![2, 4, 6]);
/// let num = Integer::from(8);
/// let result = &num + &poly;
/// assert_eq!(result, ZZPoly::from(vec![10, 4, 6]));
/// ```
impl Add<&ZZPoly> for &Integer {
    type Output = ZZPoly;
    fn add(self, rhs: &ZZPoly) -> ZZPoly {
        if rhs.is_zero() {
            return ZZPoly::from_raw(vec![self.clone()]);
        }
        
        let mut res = rhs.clone();
        res += self;
        res
    }
}

/// Add an `Integer` to an `ZZPoly` with assignment.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let mut poly = ZZPoly::from(vec![1, 2, 3]);
/// poly += Integer::from(10);
/// assert_eq!(poly, ZZPoly::from(vec![11, 2, 3]));
/// ```
impl AddAssign<Integer> for ZZPoly {
    fn add_assign(&mut self, rhs: Integer) {
        if rhs == 0 {
            return;
        }
        
        if self.is_zero() {
            self.coeffs.push(rhs);
        } else {
            self.coeffs[0] += rhs;
            self.normalize();
        }
    }
}

/// Add a reference to an `Integer` to an `ZZPoly` with assignment.
///
/// # Examples
///
/// ```
/// use stalagmite_poly::zz_poly::ZZPoly;
/// use malachite::Integer;
///
/// let mut poly = ZZPoly::from(vec![3, 4, 5]);
/// let num = Integer::from(7);
/// poly += &num;
/// assert_eq!(poly, ZZPoly::from(vec![10, 4, 5]));
/// ```
impl AddAssign<&Integer> for ZZPoly {
    fn add_assign(&mut self, rhs: &Integer) {
        if *rhs == 0 {
            return;
        }
        
        if self.is_zero() {
            self.coeffs.push(rhs.clone());
        } else {
            self.coeffs[0] += rhs;
            self.normalize();
        }
    }
}

