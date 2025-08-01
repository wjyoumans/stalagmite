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

// From vectors
impl<T> From<Vec<T>> for ZZPoly 
where
    T: Into<Integer>
{
    fn from(coeffs: Vec<T>) -> Self {
        let coeffs: Vec<Integer> = coeffs.into_iter().map(|x| x.into()).collect();
        ZZPoly::from_raw(coeffs)
    }
}

// From slices
impl<'a, T> From<&'a [T]> for ZZPoly 
where
    &'a T: Into<Integer>
{
    fn from(coeffs: &'a [T]) -> ZZPoly {
        let coeffs: Vec<Integer> = coeffs.iter().map(|x| x.into()).collect();
        ZZPoly::from_raw(coeffs)
    }
}

// From arrays
impl<T, const CAP: usize> From<[T; CAP]> for ZZPoly
where
    T: Into<Integer>
{
    fn from(coeffs: [T; CAP]) -> ZZPoly {
        let coeffs: Vec<Integer> = coeffs.into_iter().map(|x| x.into()).collect();
        let mut p = ZZPoly { coeffs };
        p.normalize();
        p
    }
}

// From slices of arrays, assuming copyable (i.e `&[1, 2, 3]`).
impl<'a, T, const CAP: usize> From<&'a [T; CAP]> for ZZPoly
where
    T: Copy + Into<Integer>
{
    fn from(coeffs: &'a [T; CAP]) -> ZZPoly {
        let coeffs: Vec<Integer> = coeffs.iter().map(|&x| x.into()).collect();
        let mut p = ZZPoly { coeffs };
        p.normalize();
        p
    }
}