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
    IntPol, u64 {usize u64 u32 u16 u8}
    flint_sys::fmpz_poly::fmpz_poly_set_ui
}

impl_from_unsafe! {
    None
    IntPol, i64 {isize i64 i32 i16 i8}
    flint_sys::fmpz_poly::fmpz_poly_set_si
}

impl_from_unsafe! {
    None
    IntPol, Integer
    flint_sys::fmpz_poly::fmpz_poly_set_fmpz
}

impl_from! {
    IntPol, IntMod
    {
        fn from(x: &IntMod) -> IntPol {
            let mut res = IntPol::default();
            unsafe {
                flint_sys::fmpz_poly::fmpz_poly_set_fmpz(res.as_mut_ptr(), x.as_ptr());
            }
            res
        }
    }
}

impl_from! {
    IntPol, PadicElem
    {
        fn from(x: &PadicElem) -> IntPol {
            let mut res = IntPol::default();
            let tmp = Integer::from(x);
            res.set_coeff(0, &tmp);
            res
        }
    }
}

impl_from! {
    IntPol, IntModPol
    {
        fn from(x: &IntModPol) -> IntPol {
            let mut res = IntPol::default();
            unsafe { 
                flint_sys::fmpz_mod_poly::fmpz_mod_poly_get_fmpz_poly(
                    res.as_mut_ptr(),
                    x.as_ptr(),
                    x.ctx_as_ptr(),
                );
            }
            res
        }
    }
}

// PadicPol

impl_from! {
    IntPol, FinFldElem
    {
        fn from(x: &FinFldElem) -> IntPol {
            let mut res = IntPol::default();
            unsafe {
                flint_sys::fq_default::fq_default_get_fmpz_poly(
                    res.as_mut_ptr(), 
                    x.as_ptr(), 
                    x.ctx_as_ptr()
                );
            }
            res
        }
    }
}

impl_from! {
    String, IntPol
    {
        fn from(x: &IntPol) -> String {
            x.get_str_pretty()
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
