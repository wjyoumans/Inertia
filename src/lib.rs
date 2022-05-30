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

#![allow(dead_code)]
use serde::{de, ser};
use std::fs::File;
use std::io;
use thiserror::Error;

//#[macro_use]
pub mod traits;
pub mod map;
pub mod prod;
pub mod poly;

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

///////////////////////////////////////////////////////////////////////
// Integer traits
///////////////////////////////////////////////////////////////////////

impl Parent for IntegerRing {
    type Element = Integer;

    #[inline]
    fn default(&self) -> Self::Element {
        self.default()
    }
}

impl Ring for IntegerRing {
    type Element = Integer;
    type PolynomialRing = IntPolyRing;
    //type PolynomialRingElement = IntPoly;
    
    #[inline]
    fn default(&self) -> <Self as Ring>::Element {
        self.default()
    }
}

impl Element for Integer {
    type Parent = IntegerRing;
    
    #[inline]
    fn parent(&self) -> Self::Parent {
        IntegerRing {}
    }
}

impl RingElement for Integer {
    type Parent = IntegerRing;
    
    #[inline]
    fn parent(&self) -> <Self as RingElement>::Parent {
        IntegerRing {}
    }
    
    #[inline]
    fn is_zero(&self) -> bool {
        self == 0
    }
}

///////////////////////////////////////////////////////////////////////
// Rational traits
///////////////////////////////////////////////////////////////////////

impl Parent for RationalField {
    type Element = Rational;

    #[inline]
    fn default(&self) -> Self::Element {
        self.default()
    }
}

impl Ring for RationalField {
    type Element = Rational;
    type PolynomialRing = RatPolyRing;
    
    #[inline]
    fn default(&self) -> <Self as Ring>::Element {
        self.default()
    }
}

impl Element for Rational {
    type Parent = RationalField;
    
    #[inline]
    fn parent(&self) -> Self::Parent {
        RationalField {}
    }
}

impl RingElement for Rational {
    type Parent = RationalField;
    
    #[inline]
    fn parent(&self) -> <Self as RingElement>::Parent {
        RationalField {}
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self == 0
    }
}


// re-exports
#[doc(no_inline)]
pub use inertia_core::*;
pub use traits::*;
pub use map::*;
pub use prod::*;
pub use poly::*;
