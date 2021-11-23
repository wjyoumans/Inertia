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


macro_rules! impl_from_prim {
    ($cast:ident $func:path; $($t:ident)*) => ($(
        impl_from! {
            impl From<&$t> for Integer {
                fn from(src: &$t) -> Integer {
                    let mut res = Integer::default();
                    unsafe { $func(res.as_mut_ptr(), *src as $cast); }
                    res
                }
            }
        }

    )*)
}

impl_from_prim! {u64 flint_sys::fmpz::fmpz_set_ui; usize u64 u32 u16 u8 }
impl_from_prim! {i64 flint_sys::fmpz::fmpz_set_si; isize i64 i32 i16 i8 }

impl From<Integer> for String {
    fn from(x: Integer) -> String {
        String::from(&x)
    }
}

impl From<&Integer> for String {
    fn from(x: &Integer) -> String {
        format!("{}", &x.to_str_radix(10))
    }
}
