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

use crate::{Build, Ring};
use inertia_core::{IntPolyRing, IntegerRing};

mod generic;
mod intpoly;

use generic::*;

// Builder does heap allocation and constructs generic PolyRings via autoref specialization
pub struct PolyRingBuilder<'a, T: Ring> {
    base_ring: &'a T,
    var: &'a str,
}

impl<'a, T: Ring> PolyRingBuilder<'a, T> {
    pub fn new(base_ring: &'a T, var: &'a str) -> Self {
        PolyRingBuilder { base_ring, var }
    }
}

impl<T: Ring> Build for &PolyRingBuilder<'_, T> {
    type Output = GenericPolyRing<T>;
    fn build(self) -> Self::Output {
        GenericPolyRing::init(&self.base_ring, &self.var)
    }
}

impl Build for PolyRingBuilder<'_, IntegerRing> {
    type Output = IntPolyRing;
    fn build(self) -> Self::Output {
        IntPolyRing::init(&self.var)
    }
}

// Constructor macros to hide autoref specialization
#[macro_export]
macro_rules! polynomial_ring {
    (&$ring:expr, $var:expr) => {
        PolyRingBuilder::new(&$ring, $var).build()
    };
    ($ring:expr, $var:expr) => {
        polynomial_ring!(&$ring, $var)
    };
    (&$ring:expr) => {
        PolyRingBuilder::new(&$ring, "x").build()
    };
    ($ring:expr) => {
        polynomial_ring!(&$ring)
    };
}

#[cfg(test)]
mod tests {
    use crate::{Build, PolyRingBuilder};
    use inertia_core::{IntegerRing, RationalField};

    #[test]
    fn main() {
        let zz = IntegerRing {};
        let _ = polynomial_ring!(&zz, "y");
        let _ = polynomial_ring!(&zz);
        let _ = polynomial_ring!(zz, "z");

        let rr = RationalField {};
        let _ = polynomial_ring!(rr, "z");
    }
}
