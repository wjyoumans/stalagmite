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

pub trait Parent {
    type Element;
}
pub trait Element {
    type Parent: Parent;
    fn parent(&self) -> Self::Parent;
}

pub trait Ring: Parent {
    type Element: RingElement;
}
pub trait RingElement: Element {
    type Parent: Ring;
}

pub trait Field: Ring {
    type Element: FieldElement;
}
pub trait FieldElement: RingElement {
    type Parent: Field;
}

pub trait PolynomialRing<R: Ring> {}
pub trait Polynomial<R: Ring> {}
