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

use std::ops::*;
use rug::ops::NegAssign;

use crate::traits::*;
use crate::integer::src::Integer;
use crate::intmat::src::IntMat;

impl_cmp_unsafe! {
    eq
    IntMat
    flint_sys::fmpz_mat::fmpz_mat_equal
}

/* no default
impl_unop_unsafe! {
    IntMat
    Neg {neg}
    NegAssign {neg_assign}
    flint_sys::fmpz_mat::fmpz_mat_neg
}*/

/* need RatMat
impl Inv for IntMat {
    type Output = Self;
    fn inv(&self) -> Self::Output {
        assert!(self.is_square());

        let mut res = IntMat::zero(self.nrows(), self.ncols());
        let mut den = Integer::default();
        unsafe { 
            let x = flint_sys::fmpz_mat::fmpz_mat_inv(res.as_mut_ptr(), den.as_mut_ptr(), self.as_ptr()); 
            if x == 0 {
                None
            } else {
                Some((res, den))
            }
        }
    }
}
*/
