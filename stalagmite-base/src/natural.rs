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

pub use malachite::Natural;
use crate::traits::{
    Parent,
    Element,
};

pub struct Naturals;

impl Parent for Naturals {
    type Element = Natural;
}

impl Element for Natural {
    type Parent = Naturals;
    fn parent(&self) -> Self::Parent {
        Naturals
    }
}
