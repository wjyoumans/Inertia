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

use std::fmt::Debug;

use crate::integer::src::Integer;
use crate::intpol::src::IntPol;


macro_rules! impl_from_prim {
    ($cast:ident $func:path; $($t:ident)*) => ($(
        impl_from! {
            impl From<&$t> for IntPol {
                fn from(src: &$t) -> IntPol {
                    let mut res = IntPol::default();
                    unsafe { $func(res.as_mut_ptr(), *src as $cast); }
                    res
                }
            }
        }

    )*)
}

impl_from_prim! {u64 flint_sys::fmpz_poly::fmpz_poly_set_ui; usize u64 u32 u16 u8 }
impl_from_prim! {i64 flint_sys::fmpz_poly::fmpz_poly_set_si; isize i64 i32 i16 i8 }

impl_from! {
    impl From<&Integer> for IntPol {
        fn from(src: &Integer) -> IntPol {
            let mut res = IntPol::default();
            unsafe {
                flint_sys::fmpz_poly::fmpz_poly_set_fmpz(
                    res.as_mut_ptr(),
                    src.as_ptr()
                );
            }
            res
        }
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


impl From<&IntPol> for String {
    fn from(x: &IntPol) -> String {
        x.get_str_pretty("x")
    }
}

impl<'a> From<IntPol> for String {
    fn from(x: IntPol) -> String {
        String::from(&x)
    }
}
