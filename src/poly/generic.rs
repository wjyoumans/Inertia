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
// GenericPolyRing
///////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct GenericPolyRing<T> where 
    T: Ring<PolynomialRing = GenericPolyRing<T>>,
{
    base_ring: Rc<T>,
    var: Rc<RefCell<String>>,
}

impl<T> Eq for GenericPolyRing<T> where
    T: Ring<PolynomialRing = GenericPolyRing<T>>,
{}

impl<T> PartialEq for GenericPolyRing<T> where
    T: Ring<PolynomialRing = GenericPolyRing<T>>,
{
    fn eq(&self, rhs: &GenericPolyRing<T>) -> bool {
        self.base_ring() == rhs.base_ring()
    }
}

impl<T> fmt::Display for GenericPolyRing<T> where
    T: Ring<PolynomialRing = GenericPolyRing<T>>,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Univariate polynomial ring in {} over {}",
            self.var(),
            self.base_ring()
        )
    }
}

impl<T> Hash for GenericPolyRing<T> where
    T: Ring<PolynomialRing = GenericPolyRing<T>>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_ring().hash(state);
        self.nvars().hash(state);
    }
}

impl<T> Parent for GenericPolyRing<T> where
    T: Ring<PolynomialRing = GenericPolyRing<T>>,
{
    type Element = GenericPoly<T>;

    #[inline]
    default fn default(&self) -> Self::Element {
        PolynomialRing::<T>::default(self)
    }
}

impl<T> Ring for GenericPolyRing<T> where 
    T: Ring<PolynomialRing = GenericPolyRing<T>>,
{
    type Element = GenericPoly<T>;
    type PolynomialRing = GenericPolyRing<Self>;
    
    #[inline]
    default fn default(&self) -> <Self as Ring>::Element {
        PolynomialRing::<T>::default(self)
    }
}

impl<T> PolynomialRing<T> for GenericPolyRing<T> where
    T: Ring<PolynomialRing = GenericPolyRing<T>>,
{
    type Element = GenericPoly<T>;

    #[inline]
    default fn default(&self) -> <Self as PolynomialRing<T>>::Element {
        let mut vec = Vec::new();
        vec.push(Ring::default(&*self.base_ring));
        GenericPoly {
            base_ring: Rc::clone(&self.base_ring),
            var: Rc::clone(&self.var),
            coeffs: RefCell::new(vec)
        }
    }
    
    default fn init(ring: &T, var: &str) -> Self {
        GenericPolyRing {
            base_ring: Rc::new(ring.clone()),
            var: Rc::new(RefCell::new(var.to_string()))
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
pub struct GenericPoly<T> where
    T: Ring<PolynomialRing = GenericPolyRing<T>>,
{
    base_ring: Rc<T>,
    var: Rc<RefCell<String>>,
    coeffs: RefCell<Vec<<T as Ring>::Element>>
}

impl<T> Eq for GenericPoly<T> where
    T: Ring<PolynomialRing = GenericPolyRing<T>>,
{}

impl<T> PartialEq for GenericPoly<T> where
    T: Ring<PolynomialRing = GenericPolyRing<T>>,
{
    fn eq(&self, rhs: &GenericPoly<T>) -> bool {
        let len = self.len();
        if rhs.len() != len {
            return false;
        }

        let c1 = self.coeffs.borrow();
        let c2 = rhs.coeffs.borrow();
        for i in 0..len-1 {
            if c1[i] != c2[i] {
                return false;
            }
        }
        true
    }
}

impl<T> fmt::Display for GenericPoly<T> where
    T: Ring<PolynomialRing = GenericPolyRing<T>>,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    }
}

// TODO
impl<T> Hash for GenericPoly<T> where
    T: Ring<PolynomialRing = GenericPolyRing<T>>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        PolynomialRingElement::parent(self).hash(state);
        //self.coefficients().hash(state);
    }
}

impl<T> Element for GenericPoly<T> where
    T: Ring<PolynomialRing = GenericPolyRing<T>>,
{
    type Parent = GenericPolyRing<T>;
    
    #[inline]
    fn parent(&self) -> Self::Parent {
        GenericPolyRing {
            base_ring: Rc::clone(&self.base_ring),
            var: Rc::clone(&self.var),
        }
    }
}

impl<T> RingElement for GenericPoly<T> where
    T: Ring<PolynomialRing = GenericPolyRing<T>>,
{
    type Parent = GenericPolyRing<T>;
    
    #[inline]
    fn parent(&self) -> <Self as RingElement>::Parent {
        GenericPolyRing {
            base_ring: Rc::clone(&self.base_ring),
            var: Rc::clone(&self.var),
        }
    }

    #[inline]
    fn is_zero(&self) -> bool {
        for c in self.coeffs.borrow().iter() {
            if !c.is_zero() {
                return false;
            }
        }
        true
    }
}

impl<T> PolynomialRingElement<T> for GenericPoly<T> where
    T: Ring<PolynomialRing = GenericPolyRing<T>>,
{
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
    default fn set_coeff<'a, S>(&mut self, i: usize, coeff: S) where
        <T as Ring>::Element: 'a,
        S: Into<ValOrRef<'a, <T as Ring>::Element>>
    {
        let z = Ring::default(&*self.base_ring);
        let mut coeffs = self.coeffs.borrow_mut();
        if i + 1 > coeffs.len() {
            coeffs.resize(i + 1, z);
        }
        coeffs.push(coeff.into().clone());
        coeffs.swap_remove(i);
        //self.normalize();
    }

    #[inline]
    default fn coefficients(&self) -> Vec<<T as Ring>::Element> {
        self.coeffs.borrow().clone()
    }

}

impl<T> GenericPoly<T> where
    T: Ring<PolynomialRing = GenericPolyRing<T>>,
{
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

/*
impl<'a, T: 'a + Ring> GenericPoly<T> {
    fn normalize(&mut self) {
        // remove trailing zeros and ensure len >= 1
        let len = self.len();
        if len != 1 {
            let d = self.base_ring().default();
            if let Some(pos) = self.coeffs.iter().rev().position(|x| x != &d) {
                self.coeffs.truncate(len - pos);
            } else {
                self.coeffs.clear();
            }
        }
    }

    #[inline]
    pub fn parent(&self) -> <Self as Element>::Parent {
        GenericPolyRing {
            base_ring: Arc::clone(&self.base_ring),
            var: Arc::clone(&self.var),
        }
    }

    #[inline]
    pub fn base_ring(&self) -> T {
        (*self.base_ring).clone()
    }

    #[inline]
    pub fn nvars(&self) -> i32 {
        1
    }

    /// Return the variable of the polynomial as a `&str`.
    #[inline]
    pub fn var(&self) -> String {
        self.var.read().unwrap().to_string()
    }

    /// Change the variable of the polynomial.
    #[inline]
    pub fn set_var<S: AsRef<String>>(&self, var: S) {
        *self.var.write().unwrap() = var.as_ref().to_string()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.coeffs.len()
    }

    #[inline]
    pub fn degree(&self) -> usize {
        self.coeffs.len() - 1
    }

    #[inline]
    pub fn get_coeff(&self, i: usize) -> <T as Ring>::Element {
        self.coeffs
            .get(i)
            .unwrap_or(&self.base_ring().default())
            .clone()
    }

    #[inline]
    pub fn set_coeff<S>(&mut self, i: usize, coeff: S)
    where
        S: Into<ValOrRef<'a, <T as Ring>::Element>>,
    {
        if i >= self.len() {
            let d = self.base_ring().default();
            self.coeffs.resize_with(i + 1, || d.clone());
            // pad with zeros
        }

        *self.coeffs.get_mut(i).unwrap() = coeff.into().clone();
        self.normalize();
    }

    #[inline]
    pub fn coefficients(&self) -> Vec<<T as Ring>::Element> {
        self.coeffs.clone()
    }
}
*/

/*
impl<T: Ring> Add for GenericPoly<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        for (&a, b) in self.coeffs.iter().zip(&rhs.coeffs) {
            a += b
        }
        self
    }
}
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
