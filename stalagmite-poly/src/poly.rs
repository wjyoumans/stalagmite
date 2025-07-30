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

use std::marker::PhantomData;
use stalagmite_base::traits::{
    Parent,
    Element,
    Ring,
    RingElement, 
    PolynomialRing,
    Polynomial
};

// Public interface for polynomials

pub struct PolyRing<R, T> 
where
    R: Ring,
    T: PolynomialRing<R>
{
    inner: T,
    phantom: PhantomData<R>
}

impl<R: Ring, T: PolynomialRing<R>> PolyRing<R, T> {
    fn new(base_ring: R) {}
}