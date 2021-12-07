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

use crate::*;


impl_from_unsafe! {
    None
    Real, u64 {usize u64 u32 u16 u8}
    arb_sys::arb::arb_set_ui
}

impl_from_unsafe! {
    None
    Real, i64 {isize i64 i32 i16 i8}
    arb_sys::arb::arb_set_si
}

impl_from_unsafe! {
    None
    Real, f64 {f64}
    arb_sys::arb::arb_set_d
}

impl_from_unsafe! {
    None
    Real, Integer
    arb_sys::arb::arb_set_fmpz
}

impl_from_unsafe! {
    None
    Real, Rational, REAL_DEFAULT_PREC;
    arb_sys::arb::arb_set_fmpq
}

impl_from! {
    String, Real
    {
        fn from(x: &Real) -> String {
           format!("{}", &x.get_str(x.precision()))
        }
    }
}
