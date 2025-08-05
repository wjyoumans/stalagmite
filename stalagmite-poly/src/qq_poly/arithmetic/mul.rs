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

use std::ops::{Mul, MulAssign};
use std::mem::swap;
use malachite::rational::Rational;
use malachite::{Integer, Natural};
use crate::qq_poly::QQPoly;

// Multiplication: QQPoly * QQPoly
impl Mul<QQPoly> for QQPoly {
    type Output = QQPoly;

    fn mul(mut self, rhs: QQPoly) -> QQPoly {
        self *= rhs;
        self
    }
}

// Multiplication: QQPoly * &QQPoly
impl Mul<&QQPoly> for QQPoly {
    type Output = QQPoly;

    fn mul(mut self, rhs: &QQPoly) -> QQPoly {
        self *= rhs;
        self
    }
}

// Multiplication: &QQPoly * QQPoly
impl Mul<QQPoly> for &QQPoly {
    type Output = QQPoly;

    fn mul(self, mut rhs: QQPoly) -> QQPoly {
        rhs *= self;
        rhs
    }
}

// Multiplication: &QQPoly * &QQPoly
impl Mul<&QQPoly> for &QQPoly {
    type Output = QQPoly;

    fn mul(self, rhs: &QQPoly) -> QQPoly {
        if self.numerator.capacity() <= rhs.numerator.capacity() {
            self.clone() * rhs
        } else {
            rhs.clone() * self
        }
    }
}

// MulAssign: QQPoly *= QQPoly
impl MulAssign<QQPoly> for QQPoly {
    fn mul_assign(&mut self, mut rhs: QQPoly) {
        if self.numerator.capacity() < rhs.numerator.capacity() {
            swap(self, &mut rhs);
        }
        self.numerator *= rhs.numerator;
        self.denominator *= rhs.denominator;
        self.normalize();
    }
}

// MulAssign: QQPoly *= &QQPoly
impl MulAssign<&QQPoly> for QQPoly {
    fn mul_assign(&mut self, rhs: &QQPoly) {
        self.numerator *= &rhs.numerator;
        self.denominator *= &rhs.denominator;
        self.normalize();
    }
}

// Multiplication: QQPoly * Integer
impl Mul<Integer> for QQPoly {
    type Output = QQPoly;

    fn mul(mut self, rhs: Integer) -> QQPoly {
        if rhs == 0 {
            return QQPoly::zero();
        }
        
        // Multiply numerator by the integer
        self.numerator *= rhs;
        self.normalize();
        self
    }
}

// Multiplication: QQPoly * &Integer
impl Mul<&Integer> for QQPoly {
    type Output = QQPoly;

    fn mul(mut self, rhs: &Integer) -> QQPoly {
        if *rhs == 0 {
            return QQPoly::zero();
        }
        
        // Multiply numerator by the integer
        self.numerator *= rhs;
        self.normalize();
        self
    }
}

// Multiplication: Integer * QQPoly
impl Mul<QQPoly> for Integer {
    type Output = QQPoly;

    fn mul(self, rhs: QQPoly) -> QQPoly {
        rhs * self
    }
}

// Multiplication: &Integer * QQPoly
impl Mul<QQPoly> for &Integer {
    type Output = QQPoly;

    fn mul(self, rhs: QQPoly) -> QQPoly {
        rhs * self
    }
}

// Multiplication: QQPoly * Rational
impl Mul<Rational> for QQPoly {
    type Output = QQPoly;

    fn mul(mut self, rhs: Rational) -> QQPoly {
        if rhs == 0 {
            return QQPoly::zero();
        }
        
        // Multiply numerator by the integer
        self.numerator *= rhs.numerator_ref();
        self.denominator *= rhs.denominator_ref();
        self.normalize();
        self
    }
}

// Multiplication: QQPoly * &Rational
impl Mul<&Rational> for QQPoly {
    type Output = QQPoly;

    fn mul(mut self, rhs: &Rational) -> QQPoly {
        if *rhs == 0 {
            return QQPoly::zero();
        }
        
        self.numerator *= rhs.numerator_ref();
        self.denominator *= rhs.denominator_ref();
        self.normalize();
        self
    }
}

// Multiplication: Rational * QQPoly
impl Mul<QQPoly> for Rational {
    type Output = QQPoly;

    fn mul(self, rhs: QQPoly) -> QQPoly {
        rhs * self
    }
}

// Multiplication: &Rational * QQPoly
impl Mul<QQPoly> for &Rational {
    type Output = QQPoly;

    fn mul(self, rhs: QQPoly) -> QQPoly {
        rhs * self
    }
}
