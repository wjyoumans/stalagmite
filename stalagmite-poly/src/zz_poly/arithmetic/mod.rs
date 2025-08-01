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

pub mod add;
pub mod neg;
pub mod sub;
pub mod mul;
pub mod mul_classical;
pub mod mul_karatsuba;
pub mod mul_ks;
pub mod mul_ss;
pub mod sqr;
pub mod mullow;
pub mod rem;

// TODO: Generated macros promote pimitive integers, but should instead work with 
// Integer coefficients and primitive integer types directly.


/*
/// Macro for implementing binary operations between ZZPoly and primitive integer types.
/// 
/// This macro generates implementations for Add, Sub, Mul, Div operations and their
/// assignment variants (AddAssign, SubAssign, etc.) between ZZPoly and primitive
/// integer types (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize).
///
/// # Arguments
/// * `$op_trait` - The operation trait (Add, Sub, Mul, Div)
/// * `$op_method` - The method name (add, sub, mul, div)
/// * `$op_assign_trait` - The assignment operation trait (AddAssign, SubAssign, etc.)
/// * `$op_assign_method` - The assignment method name (add_assign, sub_assign, etc.)
/// * `$prim_type` - The primitive integer type (u8, i16, etc.)
///
/// # Generated implementations
/// For each primitive type, this generates:
/// - `ZZPoly $op PrimType -> ZZPoly`
/// - `&ZZPoly $op PrimType -> ZZPoly` 
/// - `ZZPoly $op &PrimType -> ZZPoly`
/// - `&ZZPoly $op &PrimType -> ZZPoly`
/// - `PrimType $op ZZPoly -> ZZPoly`
/// - `&PrimType $op ZZPoly -> ZZPoly`
/// - `PrimType $op &ZZPoly -> ZZPoly`
/// - `&PrimType $op &ZZPoly -> ZZPoly`
/// - `ZZPoly $op= PrimType`
/// - `ZZPoly $op= &PrimType`
#[macro_export]
macro_rules! impl_zz_poly_primitive_binop {
    ($op_trait:ident, $op_method:ident, $op_assign_trait:ident, $op_assign_method:ident, $prim_type:ty) => {
        // ZZPoly op PrimType -> ZZPoly
        impl std::ops::$op_trait<$prim_type> for crate::zz_poly::ZZPoly {
            type Output = crate::zz_poly::ZZPoly;
            fn $op_method(self, rhs: $prim_type) -> crate::zz_poly::ZZPoly {
                self.$op_method(malachite::Integer::from(rhs))
            }
        }

        // &ZZPoly op PrimType -> ZZPoly
        impl std::ops::$op_trait<$prim_type> for &crate::zz_poly::ZZPoly {
            type Output = crate::zz_poly::ZZPoly;
            fn $op_method(self, rhs: $prim_type) -> crate::zz_poly::ZZPoly {
                self.$op_method(malachite::Integer::from(rhs))
            }
        }

        // // ZZPoly op &PrimType -> ZZPoly
        // impl std::ops::$op_trait<&$prim_type> for crate::zz_poly::ZZPoly {
        //     type Output = crate::zz_poly::ZZPoly;
        //     fn $op_method(self, rhs: &$prim_type) -> crate::zz_poly::ZZPoly {
        //         self.$op_method(malachite::Integer::from(*rhs))
        //     }
        // }

        // // &ZZPoly op &PrimType -> ZZPoly
        // impl std::ops::$op_trait<&$prim_type> for &crate::zz_poly::ZZPoly {
        //     type Output = crate::zz_poly::ZZPoly;
        //     fn $op_method(self, rhs: &$prim_type) -> crate::zz_poly::ZZPoly {
        //         self.$op_method(malachite::Integer::from(*rhs))
        //     }
        // }

        // PrimType op ZZPoly -> ZZPoly
        impl std::ops::$op_trait<crate::zz_poly::ZZPoly> for $prim_type {
            type Output = crate::zz_poly::ZZPoly;
            fn $op_method(self, rhs: crate::zz_poly::ZZPoly) -> crate::zz_poly::ZZPoly {
                malachite::Integer::from(self).$op_method(rhs)
            }
        }

        // // &PrimType op ZZPoly -> ZZPoly
        // impl std::ops::$op_trait<crate::zz_poly::ZZPoly> for &$prim_type {
        //     type Output = crate::zz_poly::ZZPoly;
        //     fn $op_method(self, rhs: crate::zz_poly::ZZPoly) -> crate::zz_poly::ZZPoly {
        //         malachite::Integer::from(*self).$op_method(rhs)
        //     }
        // }

        // PrimType op &ZZPoly -> ZZPoly
        impl std::ops::$op_trait<&crate::zz_poly::ZZPoly> for $prim_type {
            type Output = crate::zz_poly::ZZPoly;
            fn $op_method(self, rhs: &crate::zz_poly::ZZPoly) -> crate::zz_poly::ZZPoly {
                malachite::Integer::from(self).$op_method(rhs)
            }
        }

        // // &PrimType op &ZZPoly -> ZZPoly
        // impl std::ops::$op_trait<&crate::zz_poly::ZZPoly> for &$prim_type {
        //     type Output = crate::zz_poly::ZZPoly;
        //     fn $op_method(self, rhs: &crate::zz_poly::ZZPoly) -> crate::zz_poly::ZZPoly {
        //         malachite::Integer::from(*self).$op_method(rhs)
        //     }
        // }

        // ZZPoly op= PrimType
        impl std::ops::$op_assign_trait<$prim_type> for crate::zz_poly::ZZPoly {
            fn $op_assign_method(&mut self, rhs: $prim_type) {
                self.$op_assign_method(malachite::Integer::from(rhs));
            }
        }

        // // ZZPoly op= &PrimType
        // impl std::ops::$op_assign_trait<&$prim_type> for crate::zz_poly::ZZPoly {
        //     fn $op_assign_method(&mut self, rhs: &$prim_type) {
        //         self.$op_assign_method(malachite::Integer::from(*rhs));
        //     }
        // }
    };
}

/// Convenience macro to implement all binary operations for a primitive type.
/// 
/// This macro calls `impl_zz_poly_primitive_binop!` for Add, Sub, Mul, and Div operations.
#[macro_export]
macro_rules! impl_zz_poly_primitive_ops {
    ($prim_type:ty) => {
        impl_zz_poly_primitive_binop!(Add, add, AddAssign, add_assign, $prim_type);
        impl_zz_poly_primitive_binop!(Sub, sub, SubAssign, sub_assign, $prim_type);
        impl_zz_poly_primitive_binop!(Mul, mul, MulAssign, mul_assign, $prim_type);
        impl_zz_poly_primitive_binop!(Div, div, DivAssign, div_assign, $prim_type);
    };
}

/// Macro to implement operations for all standard primitive integer types.

// impl_zz_poly_primitive_ops!(u8);
// impl_zz_poly_primitive_ops!(u16);
// impl_zz_poly_primitive_ops!(u32);
// impl_zz_poly_primitive_ops!(u64);
// impl_zz_poly_primitive_ops!(u128);
// impl_zz_poly_primitive_ops!(usize);
// impl_zz_poly_primitive_ops!(i8);
// impl_zz_poly_primitive_ops!(i16);
// impl_zz_poly_primitive_ops!(i32);
// impl_zz_poly_primitive_ops!(i64);
// impl_zz_poly_primitive_ops!(i128);
// impl_zz_poly_primitive_ops!(isize);
// */
