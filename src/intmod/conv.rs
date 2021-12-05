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
use crate::intmod::src::IntMod;

impl_from! {
    Integer, IntMod
    {
        fn from(x: &IntMod) -> Integer {
            Integer { ctx: (), extra: (), data: x.data }
        }
    }
}

impl_from! {
    String, IntMod
    {
        fn from(x: &IntMod) -> String {
            String::from(Integer::from(x))
        }
    }
}
