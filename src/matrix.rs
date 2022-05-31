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

//! Generic matrices over rings.
//!
//! [MatSpace] is a convenient wrapper for working with matrices over
//! arbitrary rings.

use crate::*;
use std::fmt;
use std::rc::Rc;

mod arith;
mod generic;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MatSpace<T: Ring> {
    inner: Rc<T::MatrixSpace>,
}

impl<T: Ring> fmt::Display for MatSpace<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner.to_string())
    }
}

impl<T: Ring> Parent for MatSpace<T> {
    type Element = Mat<T>;

    #[inline]
    fn default(&self) -> Self::Element {
        MatrixSpace::default(self)
    }
}

impl<T: Ring> MatrixSpace<T> for MatSpace<T> {
    type Element = Mat<T>;

    #[inline]
    fn default(&self) -> <Self as MatrixSpace<T>>::Element {
        Mat {
            ctx: Rc::clone(&self.inner),
            inner: MatrixSpace::default(&*self.inner),
        }
    }

    #[inline]
    fn init<S>(ring: &T, nrows: S, ncols: S) -> Self
    where
        S: TryInto<usize>,
        <S as TryInto<usize>>::Error: fmt::Debug,
    {
        MatSpace {
            inner: Rc::new(T::MatrixSpace::init(ring, nrows, ncols)),
        }
    }

    #[inline]
    fn base_ring(&self) -> T {
        self.inner.base_ring()
    }

    #[inline]
    fn nrows(&self) -> usize {
        self.inner.nrows()
    }

    #[inline]
    fn ncols(&self) -> usize {
        self.inner.ncols()
    }
}

#[derive(Clone, Debug, Eq, Hash)]
pub struct Mat<T: Ring> {
    ctx: Rc<T::MatrixSpace>,
    inner: <T::MatrixSpace as MatrixSpace<T>>::Element,
}

impl<S, T> PartialEq<Mat<S>> for Mat<T> where
    S: Ring,
    T: Ring,
    <T::MatrixSpace as MatrixSpace<T>>::Element: 
        PartialEq<<S::MatrixSpace as MatrixSpace<S>>::Element>
{
    #[inline]
    fn eq(&self, rhs: &Mat<S>) -> bool {
        self.inner == rhs.inner
    }
}

impl<S, T> PartialEq<&Mat<S>> for Mat<T> where
    S: Ring,
    T: Ring,
    <T::MatrixSpace as MatrixSpace<T>>::Element: 
        PartialEq<<S::MatrixSpace as MatrixSpace<S>>::Element>
{
    #[inline]
    fn eq(&self, rhs: &&Mat<S>) -> bool {
        self.inner == rhs.inner
    }
}

impl<S, T> PartialEq<Mat<S>> for &Mat<T> where
    S: Ring,
    T: Ring,
    <T::MatrixSpace as MatrixSpace<T>>::Element: 
        PartialEq<<S::MatrixSpace as MatrixSpace<S>>::Element>
{
    #[inline]
    fn eq(&self, rhs: &Mat<S>) -> bool {
        self.inner == rhs.inner
    }
}

impl<T: Ring> fmt::Display for Mat<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<T: Ring> Element for Mat<T> {
    type Parent = MatSpace<T>;

    #[inline]
    fn parent(&self) -> Self::Parent {
        MatrixSpaceElement::parent(self)
    }
}

impl<T: Ring> MatrixSpaceElement<T> for Mat<T> {
    type Parent = MatSpace<T>;

    #[inline]
    fn parent(&self) -> <Self as MatrixSpaceElement<T>>::Parent {
        MatSpace {
            inner: Rc::clone(&self.ctx),
        }
    }

    #[inline]
    fn base_ring(&self) -> T {
        self.inner.base_ring()
    }

    #[inline]
    fn nrows(&self) -> usize {
        self.inner.nrows()
    }

    #[inline]
    fn ncols(&self) -> usize {
        self.inner.ncols()
    }

    /*
    #[inline]
    fn get_entry(&self, i: usize, j: usize) -> <T as Ring>::Element {
        self.inner.get_entry(i, j)        
    }

    #[inline]
    fn set_entry<'a, S>(&mut self, i: usize, j: usize, entry: S)
        where
            <T as Ring>::Element: 'a,
            S: Into<ValOrRef<'a, <T as Ring>::Element>> 
    {
        self.inner.set_entry(i, j, entry)
    }

    #[inline]
    fn entries(&self) -> Vec<<T as Ring>::Element> {
        self.inner.entries()
    }*/
}

impl<T, A> New<A> for MatSpace<T>
where
    T: Ring,
    T::MatrixSpace: 
        New<A, Output = <T::MatrixSpace as MatrixSpace<T>>::Element>,
{
    type Output = Mat<T>;
    fn new(&self, a: A) -> Mat<T> {
        Mat {
            ctx: Rc::clone(&self.inner),
            inner: self.inner.new(a),
        }
    }
}


impl Mat<IntegerRing> {
    pub fn new<'a, S, T>(m: S, n: S, x: &'a [T]) -> Self where
        S: TryInto<i64>,
        <S as TryInto<i64>>::Error: fmt::Debug,
        &'a T: Into<ValOrRef<'a, Integer>>
    {
        let m = m.try_into().unwrap();
        let n = n.try_into().unwrap();
        let ms = IntMatSpace::init(m, n);
        Mat {
            ctx: Rc::new(ms),
            inner: ms.new(x)
        }
    }
            
    pub fn snf(&self) -> Self {
        Mat { 
            ctx: Rc::clone(&self.ctx),
            inner: self.inner.snf() 
        }
    }
}

pub use generic::*;

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn mat() {}
}
