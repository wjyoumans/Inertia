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

impl Parent for IntMatSpace {
    type Element = IntMat;

    #[inline]
    fn default(&self) -> IntMat {
        self.default()
    }
}

impl MatrixSpace<IntegerRing> for IntMatSpace {
    type Element = IntMat;
    
    #[inline]
    fn default(&self) -> IntMat {
        self.default()
    }

    #[inline]
    fn base_ring(&self) -> IntegerRing {
        IntegerRing {}
    }
    
    #[inline]
    fn nrows(&self) -> usize {
        self.nrows().try_into().unwrap()
    }
    
    #[inline]
    fn ncols(&self) -> usize {
        self.ncols().try_into().unwrap()
    }
}

impl Element for IntMat {
    type Parent = IntMatSpace;

    #[inline]
    fn parent(&self) -> IntMatSpace {
        self.parent()
    }
}

impl MatrixSpaceElement<IntegerRing> for IntMat {
    type Parent = IntMatSpace;

    #[inline]
    fn parent(&self) -> IntMatSpace {
        self.parent()
    }

    #[inline]
    fn base_ring(&self) -> IntegerRing {
        IntegerRing {}
    }

    #[inline]
    fn nrows(&self) -> usize {
        self.nrows().try_into().unwrap()
    }
    
    #[inline]
    fn ncols(&self) -> usize {
        self.ncols().try_into().unwrap()
    }
}
