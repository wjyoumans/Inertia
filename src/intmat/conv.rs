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

use crate::integer::src::Integer;
use crate::intmat::src::IntMat;


impl_from! {
    String, IntMat
    {
        fn from(x: &IntMat) -> String {
            let r = x.nrows() as usize;
            let c = x.ncols() as usize;
            let mut out = Vec::<String>::with_capacity(r);

            for i in 0usize..r {
                let mut row = Vec::<String>::with_capacity(c+2);
                row.push("[".to_string());
                for j in 0usize..c {
                    row.push(format!(" {} ", x.get_entry(i, j)));
                }
                if i == r-1 {
                    row.push("]".to_string());
                } else {
                    row.push("]\n".to_string());
                }
                out.push(row.join(""));
            }
            out.join("")
        }
    }
}

impl From<&IntMat> for Vec<Integer> {
    fn from(x: &IntMat) -> Vec<Integer> {
        let r = x.nrows() as usize;
        let c = x.ncols() as usize;
        let mut out = Vec::<Integer>::with_capacity(r*c);

        for i in 0usize..r {
            for j in 0usize..c {
                out.push(x.get_entry(i, j));
            }
        }
        out
    }
}

impl From<IntMat> for Vec<Integer> {
    fn from(x: IntMat) -> Vec<Integer> {
        Vec::from(&x)
    }
}

impl<'a, T> From<&'a [Vec<T>]> for IntMat where &'a T: Into<Integer> {
    fn from(mat: &'a [Vec<T>]) -> IntMat {
        let m = mat.len() as c_long;
        let n = mat.iter().map(|x| x.len()).max().unwrap() as c_long;

        let mut res = IntMat::zero(m, n);
        for (i, row) in mat.iter().enumerate() {
            for (j, x) in row.iter().enumerate() {
                res.set_entry(i, j, &x.into());
            }
        }
        res
    }
}

