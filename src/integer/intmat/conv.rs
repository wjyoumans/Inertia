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

impl_from! {
    IntMat, IntModMat
    {
        fn from(x: &IntModMat) -> IntMat {
            IntMat { data: IntMatData { elem: x.data.elem.mat[0].clone() } }
        }
    }
}

impl_from! {
    String, IntMat
    {
        fn from(x: &IntMat) -> String {
            x.get_str()
        }
    }
}

impl_from! {
    matrix
    IntMat, Integer {u64 u32 u16 u8 i64 i32 i16 i8 IntMod PadicElem}
}

impl From<&[&[Integer]]> for IntMat {
    fn from(mat: &[&[Integer]]) -> IntMat {
        let m = mat.len() as c_long;
        let n = mat.iter().map(|x| x.len()).max().unwrap() as c_long;
        let mut res = <IntMat>::zero(m, n);

        for (i, row) in mat.iter().enumerate() {
            for (j, x) in row.iter().enumerate() {
                res.set_entry(i, j, x);
            }
        }
        res
    }
}

impl From<&[Vec<Integer>]> for IntMat {
    fn from(mat: &[Vec<Integer>]) -> IntMat {
        let m = mat.len() as c_long;
        let n = mat.iter().map(|x| x.len()).max().unwrap() as c_long;
        let mut res = <IntMat>::zero(m, n);

        for (i, row) in mat.iter().enumerate() {
            for (j, x) in row.iter().enumerate() {
                res.set_entry(i, j, x);
            }
        }
        res
    }
}

impl From<Vec<&[Integer]>> for IntMat {
    #[inline]
    fn from(mat: Vec<&[Integer]>) -> IntMat {
        <IntMat>::from(mat.as_slice())
    }
}

impl From<Vec<Vec<Integer>>> for IntMat {
    #[inline]
    fn from(mat: Vec<Vec<Integer>>) -> IntMat {
        <IntMat>::from(mat.as_slice())
    }
}
