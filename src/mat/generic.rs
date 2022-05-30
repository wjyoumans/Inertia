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

use std::fmt;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
//use serde::{Serialize, Deserialize};
use crate::*;

///////////////////////////////////////////////////////////////////////
// GenericMatSpace
///////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct GenericMatSpace<T: Ring> {
    base_ring: Rc<T>,
    nrows: usize,
    ncols: usize,
}

impl<T: Ring> Eq for GenericMatSpace<T> {}

impl<T: Ring> PartialEq for GenericMatSpace<T> {
    default fn eq(&self, rhs: &GenericMatSpace<T>) -> bool {
        self.base_ring() == rhs.base_ring() && 
            self.nrows() == rhs.nrows() && self.ncols() == rhs.ncols()
    }
}

impl<T: Ring> fmt::Display for GenericMatSpace<T> {
    #[inline]
    default fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Space of {} by {} matrices over {}",
            self.nrows(),
            self.ncols(),
            self.base_ring()
        )
    }
}

impl<T: Ring> Hash for GenericMatSpace<T> {
    default fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_ring().hash(state);
        self.nrows().hash(state);
        self.ncols().hash(state);
    }
}

impl<T: Ring> Parent for GenericMatSpace<T> {
    type Element = GenericMat<T>;

    #[inline]
    default fn default(&self) -> Self::Element {
        MatrixSpace::<T>::default(self)
    }
}

impl<T: Ring> MatrixSpace<T> for GenericMatSpace<T> {
    type Element = GenericMat<T>;

    #[inline]
    default fn default(&self) -> <Self as MatrixSpace<T>>::Element {
        let vec = vec![Ring::default(&*self.base_ring); self.nrows*self.ncols];
        GenericMat {
            base_ring: Rc::clone(&self.base_ring),
            entries: RefCell::new(vec),
            ncols: self.ncols(),
            nrows: self.nrows()
        }
    }
    
    default fn init<S>(ring: &T, nrows: S, ncols: S) -> Self where 
        S: TryInto<usize>,
        <S as TryInto<usize>>::Error: fmt::Debug,
    {
        GenericMatSpace {
            base_ring: Rc::new(ring.clone()),
            nrows: nrows.try_into().unwrap(),
            ncols: ncols.try_into().unwrap()
        }
    }
    
    #[inline]
    default fn base_ring(&self) -> T {
        (*self.base_ring).clone()
    }

    #[inline]
    default fn nrows(&self) -> usize {
        self.nrows
    }
    
    #[inline]
    default fn ncols(&self) -> usize {
        self.ncols
    }
}

/*
impl<'a, X, T: 'a + Ring> New<X> for GenericMatSpace<T>
where
    X: Into<Self::Element>,
{
    fn new(&self, x: X) -> Self::Element {
        x.into()
    }
}*/

///////////////////////////////////////////////////////////////////////
// GenericMat
///////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct GenericMat<T: Ring> {
    base_ring: Rc<T>,
    entries: RefCell<Vec<<T as Ring>::Element>>,
    nrows: usize,
    ncols: usize
}

impl<T: Ring> Eq for GenericMat<T> {}

impl<T: Ring> PartialEq for GenericMat<T> {
    default fn eq(&self, rhs: &GenericMat<T>) -> bool {
        let m = self.nrows();
        let n = self.ncols();

        if rhs.nrows() != m || rhs.ncols() != n {
            return false;
        }

        let c1 = self.entries.borrow();
        let c2 = rhs.entries.borrow();
        for i in 0..m*n {
            if c1[i] != c2[i] {
                return false;
            }
        }
        true
    }
}

impl<T: Ring> fmt::Display for GenericMat<T> {
    default fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /*
        let coeffs = self.coeffs.borrow();
        let len = coeffs.len();
        let x = self.var();

        let mut out = vec![];
        if len > 0 && !coeffs[0].is_zero() {
            out.push(format!("({})", coeffs[0]));
        }
        if len > 1 && !coeffs[1].is_zero() {
            out.push(format!("({})*{}", coeffs[1], x));
        }
        if len > 2 {
            for i in 2..len {
                if !coeffs[i].is_zero() {
                    out.push(format!("({})*{}^{}", coeffs[i], x, i));
                }
            }
        }
        out.reverse();
        write!(f, "{}", out.join(" + "))
        */
        write!(f, "TODO")
    }
}

// TODO
impl<T: Ring> Hash for GenericMat<T> {
    default fn hash<H: Hasher>(&self, state: &mut H) {
        MatrixSpaceElement::parent(self).hash(state);
        //self.entries().hash(state);
    }
}

impl<T: Ring> Element for GenericMat<T> {
    type Parent = GenericMatSpace<T>;
    
    #[inline]
    default fn parent(&self) -> Self::Parent {
        GenericMatSpace {
            base_ring: Rc::clone(&self.base_ring),
            nrows: self.nrows(),
            ncols: self.ncols()
        }
    }
}

impl<T: Ring> MatrixSpaceElement<T> for GenericMat<T> {
    type Parent = GenericMatSpace<T>;
    
    #[inline]
    default fn parent(&self) -> <Self as MatrixSpaceElement<T>>::Parent {
        GenericMatSpace {
            base_ring: Rc::clone(&self.base_ring),
            nrows: self.nrows(),
            ncols: self.ncols()
        }
    }
    
    #[inline]
    default fn base_ring(&self) -> T {
        (*self.base_ring).clone()
    }
    
    #[inline]
    default fn nrows(&self) -> usize {
        self.nrows
    }
    
    #[inline]
    default fn ncols(&self) -> usize {
        self.ncols
    }
}

/*
impl<T: Ring> GenericPoly<T> {
    // remove trailing zeros and ensure len >= 1
    fn normalize(&self) {    
        let len = self.len();
        let mut coeffs = self.coeffs.borrow_mut();
        if len != 1 {
            let d = Ring::default(&self.base_ring());
            if let Some(pos) = coeffs.iter().rev().position(|x| x != &d) {
                coeffs.truncate(len - pos);
            } else {
                coeffs.clear();
            }
        }
    }
    
}*/

///////////////////////////////////////////////////////////////////////
// Ops
///////////////////////////////////////////////////////////////////////

/*
use std::ops::{Add, AddAssign};

impl<'a, T: Ring> Add for GenericPoly<T> where 
    <T as Ring>::Element: 'a + AddAssign<&'a <T as Ring>::Element>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let m = std::cmp::min(self.len(), rhs.len());

        let mut res = self.coeffs.borrow_mut();
        let c = rhs.coeffs.borrow();
        for i in 0..m {
            res[i] += &c[i];
        }
        self
    }
}
*/

/*
impl<T: Ring> AddAssign for GenericPoly<T> {
    fn add_assign(&mut self, rhs: Self) {}
}

impl<T: Ring> Sub for GenericPoly<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        for (&a, b) in self.coeffs.iter().zip(&rhs.coeffs) {
            a -= b
        }
        self
    }
}
impl<T: Ring> SubAssign for GenericPoly<T> {
    fn sub_assign(&mut self, rhs: Self) {}
}

// TODO
impl<T: Ring> Mul for GenericPoly<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        for (&a, b) in self.coeffs.iter().zip(&rhs.coeffs) {
            a -= b
        }
        self
    }
}
impl<T: Ring> MulAssign for GenericPoly<T> {
    fn mul_assign(&mut self, rhs: Self) {}
}
*/
