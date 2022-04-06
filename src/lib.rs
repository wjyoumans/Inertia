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

#![allow(dead_code)]
use serde::{de, ser};
use std::fmt;
use std::fs::File;
use std::hash::Hash;
use std::io;
use thiserror::Error;

#[macro_use]
pub mod poly;
pub mod map;

#[derive(Error, Debug)]
pub enum InertiaError {
    #[error("IO error.")]
    ReadWriteError(#[from] io::Error),

    #[error("Serialization error.")]
    BincodeError(#[from] bincode::Error),

    #[error("InertiaError::Other: `{0}`")]
    Other(String),
}

pub trait ReadWriteBincode: Sized {
    type Error;
    fn read_bincode(filename: &str) -> Result<Self, Self::Error>;
    fn write_bincode(self, filename: &str) -> Result<(), Self::Error>;
}

impl<T> ReadWriteBincode for T
where
    T: ser::Serialize + for<'de> de::Deserialize<'de>,
{
    type Error = InertiaError;
    fn read_bincode(filename: &str) -> Result<Self, Self::Error> {
        let f = io::BufReader::new(File::open(filename)?);
        Ok(bincode::deserialize_from(f)?)
    }
    fn write_bincode(self, filename: &str) -> Result<(), Self::Error> {
        let mut f = io::BufWriter::new(File::create(filename)?);
        Ok(bincode::serialize_into(&mut f, &self)?)
    }
}

pub trait Build {
    type Output;
    fn build(self) -> Self::Output;
}

// hash, serialize/deserialize, display, Eq, PartialEq
pub trait BaseTrait: Clone + fmt::Debug + fmt::Display + Eq + Hash {}

pub trait Parent: BaseTrait {
    type Element: BaseTrait;
    fn default(&self) -> Self::Element;
}

pub trait Monoid: Parent {
    //fn identity(&self) -> Self::Element;
}
/*
pub trait AdditiveMonoid: Monoid {
    fn zero(&self) -> Self::Element;
}
pub trait MultiplicativeMonoid: Monoid {
    fn one(&self) -> Self::Element;
}

pub trait AdditiveGroup: Monoid {
    fn identity(&self) -> Self::Element;

    #[inline]
    fn one(&self) -> Self::Element {
        self.identity()
    }
}

pub trait MultiplicativeGroup: Monoid {
    fn identity(&self) -> Self::Element;

    #[inline]
    fn zero(&self) -> Self::Element {
        self.identity()
    }
}
*/
pub trait Group: Monoid {}

pub trait Ring: Group {}

pub trait Element: BaseTrait {
    type Parent: BaseTrait;
}

pub trait RingElement: Element {
    /*
    //type Parent: BaseTrait + Ring;
    fn is_zero(&self) -> bool;
    fn is_one(&self) -> bool;
    */
}

pub trait New<T>: Parent {
    fn new(&self, x: T) -> Self::Element;
}

/*TODO: move to poly/mod
pub trait PolynomialRing: Ring {
    type BaseRing: Ring;
    fn test(&self);

    // new should take: T, &[T], Vec<T>?
    //fn new(&self, x: T) -> Self::Element;
/*
    // fn default move to parent
    // fn new
    // fn nvars = 1
    // fn var
    // fn set_var
    // fn base_ring
    // */
}*/

// Integer impls
impl BaseTrait for IntegerRing {}
impl Parent for IntegerRing {
    type Element = Integer;

    #[inline]
    fn default(&self) -> Self::Element {
        self.default()
    }
}

impl Monoid for IntegerRing {}

impl Group for IntegerRing {}

impl Ring for IntegerRing {}

impl BaseTrait for Integer {}
impl Element for Integer {
    type Parent = IntegerRing;
}

impl RingElement for Integer {
    /*
    fn is_zero(&self) -> bool {
        self.is_zero()
    }
    fn is_one(&self) -> bool {
        self.is_one()
    }*/
}

// Rational impls
impl BaseTrait for Rational {}
impl Element for Rational {
    type Parent = RationalField;
}
impl RingElement for Rational {
    /*
    #[inline]
    fn is_zero(&self) -> bool {
        self == 0
    }

    #[inline]
    fn is_one(&self) -> bool {
        self == 1
    }*/
}

impl BaseTrait for RationalField {}
impl Parent for RationalField {
    type Element = Rational;

    #[inline]
    fn default(&self) -> Self::Element {
        self.default()
    }
}

impl Monoid for RationalField {}

impl Group for RationalField {}

impl Ring for RationalField {}

pub use inertia_core::*;
pub use poly::*;
