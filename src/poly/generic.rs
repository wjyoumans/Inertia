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
use crate::{
    BaseTrait, 
    Build, 
    PolyRingBuilder,
    Parent, 
    Element, 
    Ring,
    PolynomialRing,
    PolyRing
};

// Builder for autoref specialization
impl<T: Ring> Build for &PolyRingBuilder<'_, T> {
    type Output = PolyRing<T>;
    fn build(self) -> Self::Output {
        PolyRing::Generic(
            GenericPolyRing {
                base_ring: Arc::clone(&self.base_ring)
            }
        )
    }
}

// Generic polynomial implementation
#[derive(Clone, Debug)]
pub struct GenericPoly<T: Ring> {
    base_ring: Arc<T>,
    coeffs: Vec<T::Element>,
}
impl<T: Ring> BaseTrait for GenericPoly<T> {}

impl<T: Ring> Element for GenericPoly<T> {
    type Parent = GenericPolyRing<T>;
}

// Generic polynomial boilerplate

// Generic polynomial ring implementation
#[derive(Clone, Debug)]
pub struct GenericPolyRing<T: Ring> {
    base_ring: Arc<T>,
}

impl<T: Ring> BaseTrait for GenericPolyRing<T> {}

impl<T: Ring> Parent for GenericPolyRing<T> {
    type Element = GenericPoly<T>;
}

impl<T: Ring> Ring for GenericPolyRing<T> {}

// Generic polynomial ring boilerplate
impl<T: Ring> PolynomialRing for GenericPolyRing<T> {
    type BaseRing = T;
    fn test(&self) {
        println!("generic");
    }
}


