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

pub use malachite::Integer;
use crate::traits::{
    Parent,
    Element,
    Ring,
    RingElement
};


pub struct IntegerRing;

impl Parent for IntegerRing {
    type Element = Integer;
}

impl Ring for IntegerRing {
    type Element = Integer;
}

impl Element for Integer {
    type Parent = IntegerRing;
    fn parent(&self) -> Self::Parent {
        IntegerRing
    }
}

impl RingElement for Integer {
    type Parent = IntegerRing;
}

