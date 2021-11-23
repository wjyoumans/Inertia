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

use std::collections::HashMap;
use std::hash::Hash;

use crate::integer::src::Integer;

#[derive(Default, Debug, Clone)]
pub struct Product<T: Hash> {
    pub hashmap: HashMap<T, Integer>,
}

impl<T> Product<T> where T: Eq + Hash
{
    pub fn new(p: T, k: Integer) -> Self {
        let mut fac = HashMap::<T, Integer>::new();
        fac.insert(p, k);
        Product { hashmap: fac}
    }
    
    pub fn len(&self) -> usize {
        self.hashmap.len()
    }
    
    pub fn inv(&mut self) -> &Self {
        for v in self.hashmap.values_mut() {
            *v *= -1;
        }
        self
    }
}
