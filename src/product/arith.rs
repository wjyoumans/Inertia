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

use std::ops::Mul;
use std::hash::Hash;

use crate::product::src::Product;


impl<S, T> Mul<Product<S>> for Product<T> where 
    Product<S>: Into<Product<T>>,
    S: Eq + Hash,
    T: Eq + Hash + Clone,
{
    type Output = Product<T>;
    //default fn mul(mut self, rhs: Product<S>) -> Product<T> {
    fn mul(mut self, rhs: Product<S>) -> Product<T> {
        let rhs: Product<T> = rhs.into();

        for (k, v) in rhs.hashmap.iter() {
            if self.hashmap.contains_key(k) {
                *self.hashmap.get_mut(k).unwrap() += v;
            } else {
                self.hashmap.insert((*k).clone(), (*v).clone());
            }
        }

        self
    }
}
