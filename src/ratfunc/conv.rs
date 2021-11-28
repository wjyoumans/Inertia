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
use crate::rational::src::Rational;
use crate::ratpol::src::RatPol;
use crate::ratfunc::src::RatFunc;

/*
impl_from_unsafe! {
    RatFunc, u64 {usize u64 u32 u16 u8} 
    flint_sys::fmpz_poly_q::fmpz_poly_q_set_ui
}*/

impl_from_unsafe! {
    RatFunc, i64 {isize i64 i32 i16 i8} 
    flint_sys::fmpz_poly_q::fmpz_poly_q_set_si
}

/*
impl_from_unsafe! {
    RatFunc, Integer
    flint_sys::fmpz_poly_q::fmpz_poly_q_set_fmpz
}

impl_from_unsafe! {
    RatFunc, Rational
    flint_sys::fmpz_poly_q::fmpz_poly_q_set_fmpq
}

impl_from_unsafe! {
    RatFunc, IntPol
    flint_sys::fmpz_poly_q::fmpz_poly_q_set_fmpz_poly
}*/

impl_from! {
    String, RatFunc
    {
        fn from(x: &RatFunc) -> String {
            x.get_str_pretty("x")
        }
    }
}

/*
impl<'a, T> From<&'a [T]> for RatPol where &'a T: Into<Rational>
{
    fn from(src: &'a [T]) -> RatPol {
        let mut res = RatPol::default();
        for (i, x) in src.iter().enumerate() {
            res.set_coeff(i, &x.into());
        }
        res
    }
}

impl From<&[Rational]> for RatPol {
    fn from(src: &[Rational]) -> RatPol {
        let mut res = RatPol::default();
        for (i, x) in src.iter().enumerate() {
            res.set_coeff(i, x);
        }
        res
    }
}*/
