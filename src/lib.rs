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
