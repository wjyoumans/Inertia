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
use rustc_hash::FxHashMap;

use inertia_core::ops::Pow;


#[derive(Debug, Clone)]
pub struct Prod<A, B> where
    A: Pow<B>,
    <A as Pow<B>>::Output: Mul<Output=<A as Pow<B>>::Output>
{
    inner: FxHashMap<A, B>,
}

impl<A, B> Default for Prod<A, B> where 
    A: Pow<B>,
    <A as Pow<B>>::Output: Mul<Output=<A as Pow<B>>::Output>
{
    #[inline]
    fn default() -> Self {
        Prod { inner: FxHashMap::<A, B>::default() }
    }
}

impl<A, B> Prod<A, B> where
    A: Pow<B>,
    <A as Pow<B>>::Output: Mul<Output=<A as Pow<B>>::Output>
{
    #[inline]
    pub fn new(map: FxHashMap<A, B>) -> Self {
        Prod { inner: map }
    }

    #[inline]
    pub fn hashmap(self) -> FxHashMap<A, B> {
        self.inner
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
   
    pub fn evaluate(self) -> Option<<A as Pow<B>>::Output> {
        let mut it = self.inner.into_iter();
        if let Some((a, b)) = it.next() {
            let mut x = a.pow(b); // <A as Pow<B>>::Output
            for (a, b) in it {
                x = x*(a.pow(b)); // <<A as Pow<B>>::Output as Mul>::Output
            }
            Some(x)
        } else {
            None
        }
    }

    #[inline]
    pub fn eval(self) -> Option<<A as Pow<B>>::Output> {
        self.evaluate()
    }
}

#[allow(unused_macros)]
macro_rules! prod {
    ($(($a:expr, $b:expr)),+ $(,)?) => {
        {
            let mut map = FxHashMap::default();
            $(map.insert($a, $b);)+
            Prod::new(map)
        }
    };
}

#[cfg(test)]
mod test {
    use super::{FxHashMap, Prod};
    use crate::IntPoly;

    #[test]
    fn prod() {
        let a1 = IntPoly::from(vec![1,0,1]);
        let a2 = IntPoly::from(vec![-1,0,1]);

        let p = prod!(
            (a1, 2u32),
            (a2, 2u32)
        );
        assert_eq!(IntPoly::from(vec![1,0,0,0,-2,0,0,0,1]), p.eval().unwrap());
    }
}
