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

use crate::*;

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
    type MatrixSpace = IntMatSpace;
    
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
