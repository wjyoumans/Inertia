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

use std::cell::RefCell;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
//use serde::{Serialize, Deserialize};
use crate::*;

///////////////////////////////////////////////////////////////////////
// GenericPolyRing
///////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct GenericPolyRing<T: Ring> {
    base_ring: Rc<T>,
    var: Rc<RefCell<String>>,
}

impl<T: Ring> Eq for GenericPolyRing<T> {}

impl<T: Ring> PartialEq for GenericPolyRing<T> {
    default fn eq(&self, rhs: &GenericPolyRing<T>) -> bool {
        self.base_ring() == rhs.base_ring()
    }
}

impl<T: Ring> fmt::Display for GenericPolyRing<T> {
    #[inline]
    default fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Univariate polynomial ring in {} over {}",
            self.var(),
            self.base_ring()
        )
    }
}

impl<T: Ring> Hash for GenericPolyRing<T> {
    default fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_ring().hash(state);
        self.nvars().hash(state);
    }
}

impl<T: Ring> Parent for GenericPolyRing<T> {
    type Element = GenericPoly<T>;

    #[inline]
    default fn default(&self) -> Self::Element {
        PolynomialRing::<T>::default(self)
    }
}

impl<T: Ring> Ring for GenericPolyRing<T> {
    type Element = GenericPoly<T>;
    type PolynomialRing = GenericPolyRing<Self>;
    type MatrixSpace = GenericMatSpace<Self>;

    #[inline]
    default fn default(&self) -> <Self as Ring>::Element {
        PolynomialRing::<T>::default(self)
    }
}

impl<T: Ring> PolynomialRing<T> for GenericPolyRing<T> {
    type Element = GenericPoly<T>;

    #[inline]
    default fn default(&self) -> <Self as PolynomialRing<T>>::Element {
        let mut vec = Vec::new();
        vec.push(Ring::default(&*self.base_ring));
        GenericPoly {
            base_ring: Rc::clone(&self.base_ring),
            var: Rc::clone(&self.var),
            coeffs: RefCell::new(vec),
        }
    }

    default fn init(ring: &T, var: &str) -> Self {
        GenericPolyRing {
            base_ring: Rc::new(ring.clone()),
            var: Rc::new(RefCell::new(var.to_string())),
        }
    }

    #[inline]
    default fn base_ring(&self) -> T {
        (*self.base_ring).clone()
    }

    #[inline]
    default fn var(&self) -> String {
        self.var.borrow().to_string()
    }

    #[inline]
    default fn set_var<S: AsRef<str>>(&self, var: S) {
        self.var.replace(var.as_ref().to_string());
    }
}

/*
impl<'a, X, T: 'a + Ring> New<X> for GenericPolyRing<T>
where
    X: Into<Self::Element>,
{
    fn new(&self, x: X) -> Self::Element {
        x.into()
    }
}*/

///////////////////////////////////////////////////////////////////////
// GenericPoly
///////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct GenericPoly<T: Ring> {
    base_ring: Rc<T>,
    var: Rc<RefCell<String>>,
    coeffs: RefCell<Vec<<T as Ring>::Element>>,
}

impl<T: Ring> Eq for GenericPoly<T> {}

impl<S, T> PartialEq<GenericPoly<S>> for GenericPoly<T> where
    S: Ring,
    T: Ring,
    <S as Ring>::Element: PartialEq<<T as Ring>::Element>
{
    default fn eq(&self, rhs: &GenericPoly<S>) -> bool {
        let len = self.len();
        if rhs.len() != len {
            return false;
        }

        let c1 = self.coeffs.borrow();
        let c2 = rhs.coeffs.borrow();
        for i in 0..len {
            if c2[i] != c1[i] {
                return false;
            }
        }
        true
    }
}

impl<S, T> PartialEq<&GenericPoly<S>> for GenericPoly<T> where
    S: Ring,
    T: Ring,
    <S as Ring>::Element: PartialEq<<T as Ring>::Element>
{
    #[inline]
    default fn eq(&self, rhs: &&GenericPoly<S>) -> bool {
        (&self).eq(rhs)
    }
}

impl<S, T> PartialEq<GenericPoly<S>> for &GenericPoly<T> where
    S: Ring,
    T: Ring,
    <S as Ring>::Element: PartialEq<<T as Ring>::Element>
{
    #[inline]
    default fn eq(&self, rhs: &GenericPoly<S>) -> bool {
        self.eq(&rhs)
    }
}

impl<T: Ring> fmt::Display for GenericPoly<T> {
    default fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let coeffs = self.coeffs.borrow();
        let len = coeffs.len();
        let x = self.var();

        let mut out = Vec::with_capacity(len);
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
    }
}

// TODO
impl<T: Ring> Hash for GenericPoly<T> {
    default fn hash<H: Hasher>(&self, state: &mut H) {
        PolynomialRingElement::parent(self).hash(state);
        //self.coefficients().hash(state);
    }
}

impl<T: Ring> Element for GenericPoly<T> {
    type Parent = GenericPolyRing<T>;

    #[inline]
    default fn parent(&self) -> Self::Parent {
        GenericPolyRing {
            base_ring: Rc::clone(&self.base_ring),
            var: Rc::clone(&self.var),
        }
    }
}

impl<T: Ring> RingElement for GenericPoly<T> {
    type Parent = GenericPolyRing<T>;

    #[inline]
    default fn parent(&self) -> <Self as RingElement>::Parent {
        GenericPolyRing {
            base_ring: Rc::clone(&self.base_ring),
            var: Rc::clone(&self.var),
        }
    }

    #[inline]
    default fn is_zero(&self) -> bool {
        for c in self.coeffs.borrow().iter() {
            if !c.is_zero() {
                return false;
            }
        }
        true
    }
}

impl<T: Ring> PolynomialRingElement<T> for GenericPoly<T> {
    type Parent = GenericPolyRing<T>;

    #[inline]
    default fn parent(&self) -> <Self as PolynomialRingElement<T>>::Parent {
        GenericPolyRing {
            base_ring: Rc::clone(&self.base_ring),
            var: Rc::clone(&self.var),
        }
    }

    #[inline]
    default fn base_ring(&self) -> T {
        (*self.base_ring).clone()
    }

    #[inline]
    default fn var(&self) -> String {
        self.var.borrow().to_string()
    }

    #[inline]
    default fn set_var<S: AsRef<str>>(&self, var: S) {
        self.var.replace(var.as_ref().to_string());
    }

    #[inline]
    default fn degree(&self) -> i64 {
        let d = self.len() - 1;
        d.try_into().unwrap()
    }

    #[inline]
    default fn len(&self) -> usize {
        self.coeffs.borrow().len()
    }

    #[inline]
    default fn get_coeff(&self, i: usize) -> <T as Ring>::Element {
        self.coeffs.borrow()[i].clone()
    }

    #[inline]
    default fn set_coeff<'a, S>(&mut self, i: usize, coeff: S)
    where
        <T as Ring>::Element: 'a,
        S: Into<ValOrRef<'a, <T as Ring>::Element>>,
    {
        { // scope for mutable borrow
            let z = Ring::default(&*self.base_ring);
            let mut coeffs = self.coeffs.borrow_mut();
            if i + 1 > coeffs.len() {
                coeffs.resize(i + 1, z);
            }
            coeffs.push(coeff.into().into_owned());
            coeffs.swap_remove(i);
        }
        self.normalize();
    }

    #[inline]
    default fn coefficients(&self) -> Vec<<T as Ring>::Element> {
        self.coeffs.borrow().clone()
    }
}

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
}

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

/*
impl<'a, T, S> New<S> for GenericPolyRing<T> where
    T: 'a + Ring,
    S: Into<ValOrRef<'a, <T as Ring>::Element>>
{
    fn new(&self, x: S) -> GenericPoly<T> {
        let vec = vec![x.into()];
        GenericPoly {
            base_ring: Rc::clone(&self.base_ring),
            var: Rc::clone(&self.var),
            coeffs: RefCell::new(vec)
        }
    }
}*/

impl<'a, T, S> New<&'a [S]> for GenericPolyRing<T>
where
    T: 'a + Ring,
    &'a S: Into<ValOrRef<'a, <T as Ring>::Element>>,
{
    type Output = GenericPoly<T>;
    fn new(&self, x: &'a [S]) -> GenericPoly<T> {
        let mut res = PolynomialRing::default(self);
        
        for (i, coeff) in x.iter().enumerate() {
            res.set_coeff(i, coeff)
        }
        res
    }
}
