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

use std::ffi::CString;

use num_traits::PrimInt;
use crate::*;

impl_from_unsafe! {
    Rational, u64 {usize u64 u32 u16 u8}
    flint_sys::fmpq::fmpq_set_ui_den1
}

impl_from_unsafe! {
    Rational, i64 {isize i64 i32 i16 i8}
    flint_sys::fmpq::fmpq_set_si_den1
}

impl_from_unsafe! {
    Rational, Integer
    flint_sys::fmpq::fmpq_set_fmpz_den1
}

impl_from! {
    Rational, IntMod
    {
        fn from(x: &IntMod) -> Rational {
            let mut res = Rational::default();
            unsafe { flint_sys::fmpq::fmpq_set_fmpz_den1(res.as_mut_ptr(), x.as_ptr()); }
            res
        }
    }
}

impl_from! {
    String, Rational
    {
        fn from(x: &Rational) -> String {
            if x.denominator() == 1 {
                x.numerator().to_str_radix(10)
            } else {
                format!("{}/{}", &x.numerator().to_str_radix(10), &x.denominator().to_str_radix(10))
            }
        }
    }
}


impl<T> From<[T; 2]> for Rational where
    T: PrimInt + Into<Integer>
{
    #[inline]
    fn from(src: [T; 2]) -> Rational {
        Rational::from([&src[0].into(), &src[1].into()])
    }
}

impl From<[&Integer; 2]> for Rational {
    #[inline]
    fn from(src: [&Integer; 2]) -> Rational {
        assert!(!src[1].is_zero());
        let mut res = Rational::default();
        unsafe { 
            flint_sys::fmpq::fmpq_set_fmpz_frac(
                res.as_mut_ptr(), 
                src[0].as_ptr(),
                src[1].as_ptr()
            ); 
        }
        res
    }
}

impl From<&str> for Rational {
    fn from(s :&str) -> Rational {
        let c_str = CString::new(s).expect("String contains 0 byte.");

        let mut z = Rational::default();
        unsafe {
            let res = flint_sys::fmpq::fmpq_set_str(z.as_mut_ptr(), c_str.as_ptr(), 10);
            assert_eq!(res, 0);
            z
        }
    }
}
