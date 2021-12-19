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


use libc::c_long;

use crate::*;


impl_from_unsafe! {
    matrix
    RatMat, IntMat
    flint_sys::fmpq_mat::fmpq_mat_set_fmpz_mat
}

impl_from! {
    RatMat, IntModMat
    {
        fn from(x: &IntModMat) -> RatMat {
            RatMat::from(IntMat::from(x))
        }
    }
}

impl_from! {
    String, RatMat
    {
        fn from(x: &RatMat) -> String {
            x.get_str()
        }
    }
}

impl_from! {
    matrix
    RatMat, Rational {u64 u32 u16 u8 i64 i32 i16 i8 Integer IntMod PadicElem}
}

impl From<&[&[Rational]]> for RatMat {
    fn from(mat: &[&[Rational]]) -> RatMat {
        let m = mat.len() as c_long;
        let n = mat.iter().map(|x| x.len()).max().unwrap() as c_long;
        let mut res = <RatMat>::zero(m, n);

        for (i, row) in mat.iter().enumerate() {
            for (j, x) in row.iter().enumerate() {
                res.set_entry(i, j, x);
            }
        }
        res
    }
}

impl From<&[Vec<Rational>]> for RatMat {
    fn from(mat: &[Vec<Rational>]) -> RatMat {
        let m = mat.len() as c_long;
        let n = mat.iter().map(|x| x.len()).max().unwrap() as c_long;
        let mut res = <RatMat>::zero(m, n);

        for (i, row) in mat.iter().enumerate() {
            for (j, x) in row.iter().enumerate() {
                res.set_entry(i, j, x);
            }
        }
        res
    }
}

impl From<Vec<&[Rational]>> for RatMat {
    #[inline]
    fn from(mat: Vec<&[Rational]>) -> RatMat {
        <RatMat>::from(mat.as_slice())
    }
}

impl From<Vec<Vec<Rational>>> for RatMat {
    #[inline]
    fn from(mat: Vec<Vec<Rational>>) -> RatMat {
        <RatMat>::from(mat.as_slice())
    }
}
