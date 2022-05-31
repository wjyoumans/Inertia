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

use std::fmt;
use crate::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct GrpAbFG {
    mat: Mat<IntegerRing>
}

impl fmt::Display for GrpAbFG {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO")
    }
}

impl GrpAbFG {
    pub fn init<M>(mat: M) -> Self 
    where
        M: Into<Mat<IntegerRing>>
    {
        GrpAbFG { mat: mat.into().snf() }
    }

    fn is_snf(&self) -> bool {
        if !self.mat.is_diagonal() {
            return false;
        }

        let m = std::cmp::min(self.mat.nrows(), self.mat.ncols());
        let mut d1 = 1;
        let mut d2;
        for i in 0..m {
            d2 = self.mat.get_entry(i, i);
            if d2 % d1 != 0 {
                return false;
            }
            d1 = d2;
        }
        true
    }

    /*
    pub fn elementary_divisors(&self) -> Vec<Integer> {

    }
    */
}

pub trait Group {}
pub trait AbelianGroup {}
pub trait FinitelyGenerated {}

