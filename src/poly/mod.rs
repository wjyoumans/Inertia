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

use std::sync::Arc;
use inertia_core::{
    IntPoly,
    IntPolyRing
};
use crate::{BaseTrait, Element, Parent, Ring, PolynomialRing};

mod generic;
mod intpoly;

use generic::*;

// Builder does heap allocation and constructs generic PolyRings via autoref specialization
pub struct PolyRingBuilder<'a, T: Ring> {
    base_ring: Arc<T>,
    var: &'a str,
}

// Generic or specialized polynomials
#[derive(Clone, Debug)]
pub enum Poly<T: Ring> {
    Generic(GenericPoly<T>),
    Integer(IntPoly),
}

impl<T: Ring> BaseTrait for Poly<T> {}

impl<T: Ring> Element for Poly<T> {
    type Parent = PolyRing<T>;
}

// Generic or specialized polynomial rings
#[derive(Clone, Debug)]
pub enum PolyRing<T: Ring> {
    Generic(GenericPolyRing<T>),
    Integer(IntPolyRing),
}

impl<T: Ring> BaseTrait for PolyRing<T> {}

impl<T: Ring> Parent for PolyRing<T> {
    type Element = Poly<T>;
}

impl<T: Ring> Ring for PolyRing<T> {}

// Polynomial ring boilerplate
impl<T: Ring> PolynomialRing for PolyRing<T> {
    type BaseRing = T;
    fn test(&self) {
        match self {
            PolyRing::Generic(ring) => ring.test(),
            PolyRing::Integer(ring) => ring.test(),
        }
    }
}

// Constructor macros to hide autoref specialization
#[allow(unused_macros)]
macro_rules! polynomial_ring {
    (&$ring:expr, $var:expr) => {
        polynomial_ring!($ring.clone(), $var)
    };
    ($ring:expr, $var:expr) => {
        {
            let builder = PolyRingBuilder {
                base_ring: Arc::new($ring),
                var: $var,
            };
            builder.build()
        }
    };
    (&$ring:expr) => {
        polynomial_ring!($ring.clone())
    };
    ($ring:expr) => {
        {
            let builder = PolyRingBuilder {
                base_ring: Arc::new($ring),
                var: "x",
            };
            builder.build()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use inertia_core::{IntegerRing, RationalField};
    use crate::{Build, PolyRingBuilder, PolynomialRing};

    #[test]
    fn main() {
        let zz = IntegerRing {};
        let _ = polynomial_ring!(&zz, "y");
        let _ = polynomial_ring!(&zz);
        let zp = polynomial_ring!(zz, "z");
        zp.test();

        
        let rr = RationalField {};
        let zp = polynomial_ring!(rr, "z");
        zp.test();
    }
}
