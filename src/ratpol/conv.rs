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

impl_from_unsafe! {
    None
    RatPol, u64 {usize u64 u32 u16 u8} 
    flint_sys::fmpq_poly::fmpq_poly_set_ui
}

impl_from_unsafe! {
    None
    RatPol, i64 {isize i64 i32 i16 i8} 
    flint_sys::fmpq_poly::fmpq_poly_set_si
}

impl_from_unsafe! {
    None
    RatPol, Integer
    flint_sys::fmpq_poly::fmpq_poly_set_fmpz
}

impl_from_unsafe! {
    None
    RatPol, IntMod
    flint_sys::fmpq_poly::fmpq_poly_set_fmpz
}

impl_from_unsafe! {
    None
    RatPol, Rational
    flint_sys::fmpq_poly::fmpq_poly_set_fmpq
}

impl_from_unsafe! {
    None
    RatPol, IntPol
    flint_sys::fmpq_poly::fmpq_poly_set_fmpz_poly
}

impl_from! {
    RatPol, IntModPol
    {
        fn from(x: &IntModPol) -> RatPol {
            RatPol::from(IntPol::from(x))
        }
    }
}

impl_from! {
    String, RatPol
    {
        fn from(x: &RatPol) -> String {
            x.get_str_pretty()
        }
    }
}

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
}
