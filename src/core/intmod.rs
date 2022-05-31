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

impl Parent for IntModRing {
    type Element = IntMod;

    #[inline]
    fn default(&self) -> Self::Element {
        self.default()
    }
}

impl Ring for IntModRing {
    type Element = IntMod;
    //type PolynomialRing = IntModPolyRing;
    type PolynomialRing = GenericPolyRing<Self>;
    //type MatrixSpace = IntModMatSpace;
    type MatrixSpace = GenericMatSpace<Self>;

    #[inline]
    fn default(&self) -> <Self as Ring>::Element {
        self.default()
    }
}

impl Element for IntMod {
    type Parent = IntModRing;

    #[inline]
    fn parent(&self) -> Self::Parent {
        self.parent()
    }
}

impl RingElement for IntMod {
    type Parent = IntModRing;

    #[inline]
    fn parent(&self) -> <Self as RingElement>::Parent {
        self.parent()
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self == 0
    }
}
