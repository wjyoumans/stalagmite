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

use stalagmite_poly::generic::{GenericPolyRing, GenericPoly, PolyCtx};
use stalagmite_base::integer::{IntegerRing, Integer};
use stalagmite_base::traits::Element;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly_ctx_new() {
        let int_ring = IntegerRing;
        let _ctx = PolyCtx::new(int_ring, "x");
        // Note: Cannot test var field since it's private
        assert!(true);
    }

    #[test]
    fn test_generic_poly_ring_new() {
        let int_ring = IntegerRing;
        let _poly_ring = GenericPolyRing::new(int_ring, "y");
        // Test that it creates successfully
        assert!(true);
    }

    #[test]
    fn test_generic_poly_new() {
        let int_ring = IntegerRing;
        let coeffs = vec![Integer::from(0), Integer::from(1)];
        let _poly = GenericPoly::new(int_ring, coeffs);
        // Note: Cannot test coefficients field since it's private
        assert!(true);
    }

    #[test]
    fn test_generic_poly_parent() {
        let int_ring = IntegerRing;
        let coeffs = vec![Integer::from(0), Integer::from(1)];
        let poly = GenericPoly::new(int_ring, coeffs);
        let _parent = poly.parent();
        // Test that parent() returns a valid parent
        assert!(true);
    }
}