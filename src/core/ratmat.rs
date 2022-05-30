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

impl Parent for RatMatSpace {
    type Element = RatMat;

    #[inline]
    fn default(&self) -> RatMat {
        self.default()
    }
}

impl MatrixSpace<RationalField> for RatMatSpace {
    type Element = RatMat;
    
    #[inline]
    fn default(&self) -> RatMat {
        self.default()
    }

    #[inline]
    fn base_ring(&self) -> RationalField {
        RationalField {}
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

impl Element for RatMat {
    type Parent = RatMatSpace;

    #[inline]
    fn parent(&self) -> RatMatSpace {
        self.parent()
    }
}

impl MatrixSpaceElement<RationalField> for RatMat {
    type Parent = RatMatSpace;

    #[inline]
    fn parent(&self) -> RatMatSpace {
        self.parent()
    }

    #[inline]
    fn base_ring(&self) -> RationalField {
        RationalField {}
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
