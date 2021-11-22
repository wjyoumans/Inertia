/*
 *  Copyright (C) 2021 William Youmans
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */


#![feature(min_specialization)]

#[macro_use]
pub(crate) mod macros;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod traits;
//pub use rug::Assign;

pub mod integer;
pub use crate::integer::src::*;
pub use crate::integer::macros::*;

pub mod rational;
pub use crate::rational::src::*;
pub use crate::rational::macros::*;

pub mod intpol;
pub use crate::intpol::src::*;
pub use crate::intpol::macros::*;

pub mod ratpol;
pub use crate::ratpol::src::*;
pub use crate::ratpol::macros::*;
