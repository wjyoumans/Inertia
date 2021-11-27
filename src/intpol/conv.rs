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

use crate::integer::src::Integer;
use crate::intpol::src::IntPol;


impl_from_unsafe! {
    IntPol, u64 {usize u64 u32 u16 u8}
    flint_sys::fmpz_poly::fmpz_poly_set_ui
}

impl_from_unsafe! {
    IntPol, i64 {isize i64 i32 i16 i8}
    flint_sys::fmpz_poly::fmpz_poly_set_si
}

impl_from_unsafe! {
    IntPol, Integer
    flint_sys::fmpz_poly::fmpz_poly_set_fmpz
}

impl_from! {
    String, IntPol
    {
        fn from(x: &IntPol) -> String {
            x.get_str_pretty("x")
        }
    }
}

impl<'a, T> From<&'a [T]> for IntPol where &'a T: Into<Integer>
{
    fn from(src: &'a [T]) -> IntPol {
        let mut res = IntPol::default();
        for (i, x) in src.iter().enumerate() {
            res.set_coeff(i, &x.into());
        }
        res
    }
}

impl From<&[Integer]> for IntPol {
    fn from(src: &[Integer]) -> IntPol {
        let mut res = IntPol::default();
        for (i, x) in src.iter().enumerate() {
            res.set_coeff(i, x);
        }
        res
    }
}
