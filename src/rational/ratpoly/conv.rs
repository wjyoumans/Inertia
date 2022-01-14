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
    RatPoly, u64 {usize u64 u32 u16 u8} 
    flint_sys::fmpq_poly::fmpq_poly_set_ui
}

impl_from_unsafe! {
    None
    RatPoly, i64 {isize i64 i32 i16 i8} 
    flint_sys::fmpq_poly::fmpq_poly_set_si
}

impl_from_unsafe! {
    None
    RatPoly, Integer
    flint_sys::fmpq_poly::fmpq_poly_set_fmpz
}

impl_from_unsafe! {
    None
    RatPoly, IntMod
    flint_sys::fmpq_poly::fmpq_poly_set_fmpz
}

impl_from_unsafe! {
    None
    RatPoly, Rational
    flint_sys::fmpq_poly::fmpq_poly_set_fmpq
}

impl_from_unsafe! {
    None
    RatPoly, IntPoly
    flint_sys::fmpq_poly::fmpq_poly_set_fmpz_poly
}

impl_from! {
    RatPoly, IntModPoly
    {
        fn from(x: &IntModPoly) -> RatPoly {
            RatPoly::from(IntPoly::from(x))
        }
    }
}

impl_from_unsafe! {
    ctx
    RatPoly, NumFldElem
    antic_sys::nf_elem::nf_elem_get_fmpq_poly
}

impl_from! {
    String, RatPoly
    {
        fn from(x: &RatPoly) -> String {
            x.get_str_pretty()
        }
    }
}


impl_from! {
    pol
    RatPoly, Rational {u64 u32 u16 u8 i64 i32 i16 i8 Integer IntMod PadicElem}
}

impl From<&[Rational]> for RatPoly {
    #[inline]
    fn from(src: &[Rational]) -> RatPoly {
        let mut res = RatPoly::default();
        for (i, x) in src.iter().enumerate() {
            res.set_coeff(i, x);
        }
        res
    }
}

impl From<Vec<Rational>> for RatPoly {
    #[inline]
    fn from(src: Vec<Rational>) -> RatPoly {
        RatPoly::from(src.as_slice())
    }
}
