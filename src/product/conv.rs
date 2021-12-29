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

use std::hash::Hash;

use rustc_hash::FxHashMap;

use crate::*;


impl<T> From<FxHashMap<T, Integer>> for Product<T> where
    T: Eq + Hash
{
    fn from(other: FxHashMap<T, Integer>) -> Product<T> {
        Product { hashmap: other}
    }

}

impl<T> From<T> for Product<T> where
    T: Eq + Hash,
{
    fn from(other: T) -> Product<T> {
        let mut hashmap = FxHashMap::<T, Integer>::default();
        hashmap.insert(other, Integer::from(1));
        Product { hashmap: hashmap}
    }
}
