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
use crate::rational::src::Rational;
use crate::ratpol::src::RatPol;


macro_rules! impl_from_prim {
    ($cast:ident $func:path; $($t:ident)*) => ($(
        impl_from! {
            impl From<&$t> for RatPol {
                fn from(src: &$t) -> RatPol {
                    let mut res = RatPol::default();
                    unsafe { $func(res.as_mut_ptr(), *src as $cast); }
                    res
                }
            }
        }

    )*)
}

impl_from_prim! {u64 flint_sys::fmpq_poly::fmpq_poly_set_ui; usize u64 u32 u16 u8 }
impl_from_prim! {i64 flint_sys::fmpq_poly::fmpq_poly_set_si; isize i64 i32 i16 i8 }

impl_from! {
    impl From<&Integer> for RatPol {
        fn from(src: &Integer) -> RatPol {
            let mut res = RatPol::default();
            unsafe {
                flint_sys::fmpq_poly::fmpq_poly_set_fmpz(
                    res.as_mut_ptr(),
                    src.as_ptr()
                    );
            }
            res
        }
    }
}

impl_from! {
    impl From<&Rational> for RatPol {
        fn from(src: &Rational) -> RatPol {
            let mut res = RatPol::default();
            unsafe {
                flint_sys::fmpq_poly::fmpq_poly_set_fmpq(
                    res.as_mut_ptr(),
                    src.as_ptr()
                    );
            }
            res
        }
    }
}

impl<'a, T: Debug> From<&'a [T]> for RatPol where &'a T: Into<Rational> {
    fn from(src: &'a [T]) -> RatPol {
        let mut res = RatPol::default();
        for (i, x) in src.iter().enumerate() {
            res.set_coeff(i, x);
        }
        res
    }
}

impl From<&RatPol> for String {
    fn from(x: &RatPol) -> String {
        format!("({})/{}", x.numerator(), x.denominator())
    }
}

impl<'a> From<RatPol> for String {
    fn from(x: RatPol) -> String {
        String::from(&x)
    }
}
