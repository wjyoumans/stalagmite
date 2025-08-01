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


// derive PartialEq and Eq instead

// use crate::zz_poly::ZZPoly;
// use std::cmp::PartialEq;
// use malachite::Integer;

// impl PartialEq for ZZPoly {
//     fn eq(&self, other: &Self) -> bool {
//         if self.length() != other.length() {
//             return false;
//         }
//         self.coeffs == other.coeffs
//     }
// }

// impl PartialEq<Integer> for ZZPoly {
//     fn eq(&self, other: &Integer) -> bool {
//         self.length() == 0 && *other == 0 ||
//             self.length() == 1 && self.coeffs[0] == *other
//     }
// }

// impl PartialEq<ZZPoly> for Integer {
//     fn eq(&self, other: &ZZPoly) -> bool {
//         other == self
//     }
// }