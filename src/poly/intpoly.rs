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

use inertia_core::IntPolyRing;
use crate::{BaseTrait, Parent, Element, Ring, IntPoly};

// Integer polynomials
impl BaseTrait for IntPoly {}

impl Element for IntPoly {
    type Parent = IntPolyRing;
}

// Integer polynomial ring implementation
impl BaseTrait for IntPolyRing {}

impl Parent for IntPolyRing {
    type Element = IntPoly; 
    
    #[inline]
    fn default(&self) -> Self::Element {
        self.default()
    }
}

impl Ring for IntPolyRing {}
