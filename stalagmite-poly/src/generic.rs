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

use std::rc::Rc;
use stalagmite_base::traits::{
    Parent,
    Element,
    Ring,
    RingElement,
    PolynomialRing,
    Polynomial
};

pub struct PolyCtx<R: Ring> {
    base_ring: R,
    var: String,
}

impl<R: Ring> PolyCtx<R> {
    pub fn new(base_ring: R, var: &str) -> Self {
        Self { base_ring, var: var.to_owned()}
    }
}

pub struct GenericPolyRing<R: Ring> {
    ctx: Rc<PolyCtx<R>>
}

impl<R: Ring> Parent for GenericPolyRing<R> {
    type Element = GenericPoly<R>;
}

impl<R: Ring> Ring for GenericPolyRing<R> {
    type Element = GenericPoly<R>;
}

impl<R: Ring> PolynomialRing<R> for GenericPolyRing<R> {}


pub struct GenericPoly<R: Ring> {
    ctx: Rc<PolyCtx<R>>,
    coefficients: Vec<<R as Ring>::Element>,
}

impl<R: Ring> Element for GenericPoly<R> {
    type Parent = GenericPolyRing<R>;

    fn parent(&self) -> Self::Parent {
        GenericPolyRing { ctx: self.ctx.clone() } 
    }
}

impl<R: Ring> RingElement for GenericPoly<R> {
    type Parent = GenericPolyRing<R>;
}

impl<R: Ring> Polynomial<R> for GenericPoly<R> {}




impl<R: Ring> GenericPolyRing<R> {
    pub fn new(base_ring: R, var: &str) -> Self {
        Self { ctx: Rc::new(PolyCtx::new(base_ring, var)) }
    }
}

impl<R: Ring> GenericPoly<R> {
    pub fn new(base_ring: R, coefficients: Vec<<R as Ring>::Element>) -> Self {
        Self { ctx: Rc::new(PolyCtx::new(base_ring, "x")), coefficients }
    }
}