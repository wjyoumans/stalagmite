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

use malachite::{Integer, Natural};
use crate::qq_poly::QQPoly;
use crate::zz_poly::ZZPoly;

// From vectors of integers
impl<T> From<Vec<T>> for QQPoly 
where
    T: Into<Integer>
{
    fn from(coeffs: Vec<T>) -> Self {
        let zz_poly = ZZPoly::from(coeffs);
        QQPoly::from_raw(zz_poly, Natural::from(1u32))
    }
}

// From slices of integers
impl<'a, T> From<&'a [T]> for QQPoly 
where
    &'a T: Into<Integer>
{
    fn from(coeffs: &'a [T]) -> QQPoly {
        let zz_poly = ZZPoly::from(coeffs);
        QQPoly::from_raw(zz_poly, Natural::from(1u32))
    }
}

// From arrays of integers
impl<T, const CAP: usize> From<[T; CAP]> for QQPoly
where
    T: Into<Integer>
{
    fn from(coeffs: [T; CAP]) -> QQPoly {
        let zz_poly = ZZPoly::from(coeffs);
        QQPoly::from_raw(zz_poly, Natural::from(1u32))
    }
}

// From slices of arrays, assuming copyable
impl<'a, T, const CAP: usize> From<&'a [T; CAP]> for QQPoly
where
    T: Copy + Into<Integer>
{
    fn from(coeffs: &'a [T; CAP]) -> QQPoly {
        let zz_poly = ZZPoly::from(coeffs);
        QQPoly::from_raw(zz_poly, Natural::from(1u32))
    }
}

// From tuple (numerator_coeffs, denominator)
impl<T> From<(Vec<T>, Natural)> for QQPoly 
where
    T: Into<Integer>
{
    fn from((coeffs, denom): (Vec<T>, Natural)) -> Self {
        let zz_poly = ZZPoly::from(coeffs);
        QQPoly::from_raw(zz_poly, denom)
    }
}

// From tuple (numerator_coeffs, denominator) with unsigned integer denominator
impl<T> From<(Vec<T>, u32)> for QQPoly 
where
    T: Into<Integer>
{
    fn from((coeffs, denom): (Vec<T>, u32)) -> Self {
        let zz_poly = ZZPoly::from(coeffs);
        QQPoly::from_raw(zz_poly, Natural::from(denom))
    }
}

// From ZZPoly directly (with denominator 1)
impl From<ZZPoly> for QQPoly {
    fn from(poly: ZZPoly) -> Self {
        QQPoly::from_raw(poly, Natural::from(1u32))
    }
}

// From tuple (ZZPoly, denominator)
impl<D> From<(ZZPoly, D)> for QQPoly 
where
    D: Into<Natural>
{
    fn from((poly, denom): (ZZPoly, D)) -> Self {
        QQPoly::from_raw(poly, denom.into())
    }
}

// From single i32 (constant polynomial)
impl From<i32> for QQPoly {
    fn from(value: i32) -> Self {
        if value == 0 {
            QQPoly::zero()
        } else {
            let zz_poly = ZZPoly::from(vec![Integer::from(value)]);
            QQPoly::from_raw(zz_poly, Natural::from(1u32))
        }
    }
}

// From single Integer (constant polynomial)
impl From<Integer> for QQPoly {
    fn from(value: Integer) -> Self {
        if value == 0 {
            QQPoly::zero()
        } else {
            let zz_poly = ZZPoly::from(vec![value]);
            QQPoly::from_raw(zz_poly, Natural::from(1u32))
        }
    }
}

// From rational
impl From<malachite::rational::Rational> for QQPoly {
    fn from(value: malachite::rational::Rational) -> Self {
        let (num, denom) = value.into_numerator_and_denominator();
        let zz_poly = ZZPoly::from(vec![num]);
        QQPoly::from_raw(zz_poly, denom)
    }
}