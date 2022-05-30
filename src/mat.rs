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

mod generic;
mod arith;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MatSpace<T: Ring> {
    inner: Rc<T::MatrixSpace>,
}

impl<T: Ring> fmt::Display for MatSpace<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner.to_string())
        /*
        write!(
            f,
            "Space of {} by {} matrices over {}",
            self.nrows(),
            self.ncols(),
            self.base_ring()
        )*/
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
            inner: MatrixSpace::default(&*self.inner) 
        }
    }

    #[inline]
    fn init<S>(ring: &T, nrows: S, ncols: S) -> Self where 
            S: TryInto<usize>,
            <S as TryInto<usize>>::Error: fmt::Debug {
        MatSpace {
            inner: Rc::new(T::MatrixSpace::init(ring, nrows, ncols))
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Mat<T: Ring> {
    ctx: Rc<T::MatrixSpace>,
    inner: <T::MatrixSpace as MatrixSpace<T>>::Element,
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
        MatSpace { inner: Rc::clone(&self.ctx) }
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

/*
impl<'a, S, T> New<S> for PolyRing<T> where
    T: 'a + Ring,
    S: Into<ValOrRef<'a, <T as Ring>::Element>>,
{
    fn new(&self, x: S) -> Poly<T> {
        let mut p = PolynomialRing::default(self);
        let inner = x.into();
        p.set_coeff(0, inner);
        p
    }
}*/

/*
impl<'a, S, T> New<&'a [S]> for PolyRing<T> where
    T: 'a + Ring,
    &'a S: Into<ValOrRef<'a, <T as Ring>::Element>>,
{
    fn new(&self, x: &'a [S]) -> Poly<T> {
        let mut res = PolynomialRing::default(self);
        for (i, x) in x.iter().enumerate() {
            res.set_coeff(i, x);
        }
        res
    }
}
*/

pub use generic::*;

#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn mat() {
    }
}
