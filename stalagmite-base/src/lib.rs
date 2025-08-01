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

pub mod traits;
pub mod integer;
pub mod natural;
pub mod rational;

use malachite::Natural;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StalagmiteError {
    DivisionByZero,
    InvalidModulus(Natural),
}

impl fmt::Display for StalagmiteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StalagmiteError::DivisionByZero => write!(f, "Division by zero"),
            StalagmiteError::InvalidModulus(modulus) => write!(f, "Invalid modulus: {}", modulus),
        }
    }
}

impl Error for StalagmiteError {}