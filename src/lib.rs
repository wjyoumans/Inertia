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
use std::io;
use std::fmt;
use std::fs::File;
use std::hash::Hash;
use serde::{ser, de};
use thiserror::Error;
use inertia_core::*;


pub mod poly;
pub use poly::*;

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

impl<T> ReadWriteBincode for T where
    T: ser::Serialize + for<'de> de::Deserialize<'de>
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

trait Build {
    type Output;
    fn build(self) -> Self::Output;
}

// hash, serialize/deserialize, display, Eq, PartialEq
pub trait BaseTrait: Clone + fmt::Debug + fmt::Display + Eq + Hash {}

pub trait Parent: BaseTrait {
    type Element: BaseTrait;

    fn default(&self) -> Self::Element;
}

pub trait Element: BaseTrait {
    type Parent: BaseTrait ;
}

pub trait Ring: Parent {}

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
impl BaseTrait for Integer {}
impl Element for Integer {
    type Parent = IntegerRing;
}

impl BaseTrait for IntegerRing {}
impl Parent for IntegerRing {
    type Element = Integer;
    
    #[inline]
    fn default(&self) -> Self::Element {
        self.default()
    }
}

impl Ring for IntegerRing {}

// Rational impls
impl BaseTrait for Rational {}
impl Element for Rational {
    type Parent = RationalField;
}

impl BaseTrait for RationalField {}
impl Parent for RationalField {
    type Element = Rational;
    
    #[inline]
    fn default(&self) -> Self::Element {
        self.default()
    }
}

impl Ring for RationalField {}


