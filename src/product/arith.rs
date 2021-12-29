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

use std::ops::{Mul, MulAssign};
use std::hash::Hash;

use crate::*;

impl<T> Inv for Product<T> where
    T: Eq + Hash,
{
    type Output = Self;
    fn inv(mut self) -> Self::Output {
        self.inv_assign();
        self
    }
}

impl<T> Inv for &Product<T> where
    T: Eq + Hash,
    Product<T>: Clone,
{
    type Output = Product<T>;
    fn inv(self) -> Self::Output {
        let mut out = self.clone();
        out.inv_assign();
        out
    }
}

impl<T> InvAssign for Product<T> where
    T: Eq + Hash,
{
    fn inv_assign(&mut self) {
        for v in self.hashmap.values_mut() {
            *v *= -1;
        }
    }
}

impl<T> Mul for Product<T> where 
    T: Eq + Hash + Clone,
{
    type Output = Product<T>;
    fn mul(mut self, rhs: Product<T>) -> Product<T> {
        self.mul_assign(rhs);
        self
    }
}

impl<T> Mul<&Product<T>> for Product<T> where
    T: Eq + Hash + Clone,
{
    type Output = Product<T>;
    fn mul(mut self, rhs: &Product<T>) -> Product<T> {
        self.mul_assign(rhs);
        self
    }
}

impl<T> Mul<Product<T>> for &Product<T> where
    T: Eq + Hash + Clone,
{
    type Output = Product<T>;
    fn mul(self, mut rhs: Product<T>) -> Product<T> {
        rhs.mul_assign(self);
        rhs
    }
}

impl<T> Mul for &Product<T> where 
    T: Eq + Hash + Clone,
    Product<T>: Clone
{
    type Output = Product<T>;
    fn mul(self, rhs: &Product<T>) -> Product<T> {
        let mut out = self.clone();
        out.mul_assign(rhs);
        out
    }
}

impl<T> MulAssign for Product<T> where 
    T: Eq + Hash + Clone,
{
    fn mul_assign(&mut self, rhs: Product<T>) {
        self.mul_assign(&rhs)
    }
}

impl<T> MulAssign<&Product<T>> for Product<T> where 
    T: Eq + Hash + Clone,
{
    fn mul_assign(&mut self, rhs: &Product<T>) {
        for (k, v) in rhs.hashmap.iter() {
            if self.hashmap.contains_key(k) {
                *self.hashmap.get_mut(k).unwrap() += v;
            } else {
                self.hashmap.insert((*k).clone(), (*v).clone());
            }
        }
    }
}
