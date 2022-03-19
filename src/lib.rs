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


//! Inertia is a computational mathematics library for Rust.
//!
//! ## Features
//!
//! This is a checklist of the main intended features and their current implementation progress.
//! Features marked with an asterisk have their basic implementation done but need work on
//! additional functions, arithmetic, conversion, etc.
//!
//! - [x] arbitrary precision integers
//! - [x] rational numbers
//! - [ ] \*real numbers
//! - [ ] \*complex numbers
//! - [ ] \*integers mod n
//! - [ ] \*finite fields
//! - [ ] \*p-adic/q-adic numbers
//! - [ ] polynomials
//!     - [x] integer polynomials
//!     - [x] rational polynomials
//!     - [ ] real polynomials
//!     - [ ] complex polynomials 
//!     - [ ] \*polynomials over integers mod n
//!     - [ ] \*polynomials over finite fields
//!     - [ ] polynomials over p-adic/q-adics
//! - [ ] matrices
//!     - [x] integer matrices
//!     - [x] rational matrices
//!     - [ ] real matrices
//!     - [ ] complex matrices 
//!     - [ ] \*matrices over integers mod n
//!     - [ ] \*matrices over finite fields
//!     - [ ] matrices over p-adic/q-adics
//! - [ ] multivariate polynomials
//! - [ ] rational functions (currently disabled)
//! - [ ] \*number fields

// doc additions:
// Types generally fall in 2 categories - algebraic structures and their elements.
//
// "All algebraic structures can be initialized using the `init` function which will take a
// different number of arguments depending on the structure. The `default` method will always
// return zero when we have an additive structure, and `new` will always be the standard element
// constructor."
//
// All main types with generics have aliases: Elem<IntegerRing> == Integer, PolyRing<FiniteField>
// == FinFldPolyRing etc
//
// incomplete computation values vs assign_ops
//
// focus on effortless and intuitive conversions and intercompatability(?) of types

use libc::{c_long, c_ulong, c_uint};

const ARB_DEFAULT_PREC: c_long = 10;
const ARB_DEFAULT_NUM_DIGITS: c_long = 20;
const ARB_DEFAULT_PRINT_MODE: c_ulong = 0;

const PADIC_PRINT_TERSE: c_uint = 0;
#[allow(dead_code)]
const PADIC_PRINT_SERIES: c_uint = 1;
#[allow(dead_code)]
const PADIC_PRINT_VAL_UNIT: c_uint = 2;
const PADIC_DEFAULT_PRINT_MODE: c_uint = PADIC_PRINT_TERSE;

#[macro_use]
pub(crate) mod macros;

//#[cfg(test)]
//#[macro_use(quickcheck)]
//extern crate quickcheck_macros;

pub mod traits;
pub mod product;
pub mod base;

pub mod prelude { 
    //! A prelude for glob importing.
    
    pub use rug::ops::*;
    pub use crate::traits::*;

    pub use crate::product::src::*;

    pub use crate::base::integer::src::*;
    pub use super::int;
    pub use crate::base::intpoly::src::*;
    pub use super::intpoly;
    pub use crate::base::intmat::src::*;
    //pub use super::intmat;

    pub use crate::base::rational::src::*;
    pub use super::rat;
    pub use crate::base::ratpoly::src::*;
    pub use super::ratpoly;
    pub use crate::base::ratmat::src::*;
    //pub use super::ratmat;

    pub use crate::base::intmod::src::*;
    pub use super::intmod;
    pub use crate::base::intmodpoly::src::*;
    pub use crate::base::intmodmat::src::*;

    pub use crate::base::finfld::src::*;
    pub use super::finfld;
    pub use crate::base::finfldpoly::src::*;
    pub use crate::base::finfldmat::src::*;

    pub use crate::base::padic::src::*;

    pub use crate::base::qadic::src::*;

    //pub use crate::base::ratfunc::src::*;
    
    //pub use crate::base::numfld::src::*; 
    
    pub use crate::base::real::src::*;
    //pub use crate::real::realpoly::src::*;
    
    pub use crate::base::complex::src::*;
}

pub use prelude::*;
