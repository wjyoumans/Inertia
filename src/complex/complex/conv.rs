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
    None
    Complex, u64 {usize u64 u32 u16 u8}
    arb_sys::acb::acb_set_ui
}

impl_from_unsafe! {
    None
    Complex, i64 {isize i64 i32 i16 i8}
    arb_sys::acb::acb_set_si
}

impl_from_unsafe! {
    None
    Complex, f64 {f64}
    arb_sys::acb::acb_set_d
}

impl_from_unsafe! {
    None
    Complex, Integer
    arb_sys::acb::acb_set_fmpz
}

impl_from_unsafe! {
    None
    Complex, Rational, ARB_DEFAULT_PREC;
    arb_sys::acb::acb_set_fmpq
}

impl_from_unsafe! {
    None
    Complex, Real
    arb_sys::acb::acb_set_arb
}

impl_from! {
    String, Complex
    {
        fn from(x: &Complex) -> String {
           format!("{}", &x.get_str())
        }
    }
}

impl<T> From<[T; 2]> for Complex where 
    T: PrimInt + Into<Integer>
{
    #[inline]
    fn from(src: [T; 2]) -> Complex {
        Complex::from([&src[0].into(), &src[1].into()])
    }
}

impl From<[f64; 2]> for Complex {
    #[inline]
    fn from(src: [f64; 2]) -> Complex {
        let mut res = Complex::default();
        unsafe { 
            arb_sys::acb::acb_set_d_d(
                res.as_mut_ptr(),
                src[0],
                src[1]
            ); 
        }
        res
    }
}

impl From<[&Integer; 2]> for Complex {
    #[inline]
    fn from(src: [&Integer; 2]) -> Complex {
        let mut res = Complex::default();
        unsafe { 
            arb_sys::acb::acb_set_fmpz_fmpz(
                res.as_mut_ptr(),
                src[0].as_ptr(),
                src[1].as_ptr()
            ); 
        }
        res
    }
}

impl From<[&Rational; 2]> for Complex {
    #[inline]
    fn from(src: [&Rational; 2]) -> Complex {
        let mut res = Complex::default();
        unsafe { 
            arb_sys::acb::acb_set_arb_arb(
                res.as_mut_ptr(),
                Real::from(src[0]).as_ptr(),
                Real::from(src[1]).as_ptr()
            ); 
        }
        res
    }
}

impl From<[&Real; 2]> for Complex {
    #[inline]
    fn from(src: [&Real; 2]) -> Complex {
        let mut res = Complex::default();
        unsafe { 
            arb_sys::acb::acb_set_arb_arb(
                res.as_mut_ptr(),
                src[0].as_ptr(),
                src[1].as_ptr()
            ); 
        }
        res
    }
}
