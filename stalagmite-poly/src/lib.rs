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

pub mod poly;
pub mod intpoly;
pub mod generic;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intpoly::IntPoly;

    #[test]
    fn test_add() {
        let a = IntPoly::from(&[1, 2, 3]);
        let b = IntPoly::from([4, 5, 6]);
        let c = a + b;
        assert_eq!(c, IntPoly::from([5, 7, 9]));
    }
}