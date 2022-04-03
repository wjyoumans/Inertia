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

use inertia_core::{IntegerRing, IntPolyRing};
use crate::{Build, Ring};

mod generic;
mod intpoly;

use generic::*;

// Builder does heap allocation and constructs generic PolyRings via autoref specialization
pub struct PolyRingBuilder<'a, T: Ring> {
    base_ring: &'a T,
    var: &'a str,
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
#[allow(unused_macros)]
macro_rules! polynomial_ring {
    (&$ring:expr, $var:expr) => {
        {
            let builder = PolyRingBuilder {
                base_ring: &$ring,
                var: $var,
            };
            builder.build()
        }
    };
    ($ring:expr, $var:expr) => {
        polynomial_ring!(&$ring, $var)
    };
    (&$ring:expr) => {
        {
            let builder = PolyRingBuilder {
                base_ring: &$ring,
                var: "x",
            };
            builder.build()
        }
    };
    ($ring:expr) => {
        polynomial_ring!(&$ring)
    }
}

#[cfg(test)]
mod tests {
    use inertia_core::{IntegerRing, RationalField};
    use crate::{Build, PolyRingBuilder};

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
