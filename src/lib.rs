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

pub trait BaseTrait: Clone + fmt::Debug {}

pub trait Parent: BaseTrait {
    type Element: BaseTrait;
}

pub trait Element: BaseTrait {
    type Parent: BaseTrait ;
}

pub trait Ring: Parent {}

pub trait PolynomialRing: Ring {
    type BaseRing: Ring;

    fn test(&self);
}

// Integer impls
impl BaseTrait for Integer {}
impl Element for Integer {
    type Parent = IntegerRing;
}

impl BaseTrait for IntegerRing {}
impl Parent for IntegerRing {
    type Element = Integer;
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
}

impl Ring for RationalField {}


