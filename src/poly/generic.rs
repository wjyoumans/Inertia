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
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
//use serde::{Serialize, Deserialize};
use crate::{
    ValOrRef,
    BaseTrait, 
    Parent, 
    Element, 
    Ring,
};

// Generic polynomial ring implementation
#[derive(Clone, Debug)]
pub struct GenericPolyRing<T: Ring> {
    base_ring: Arc<T>,
    var: Arc<RwLock<String>>
}

impl<T: Ring> Eq for GenericPolyRing<T> {}

impl<T: Ring> PartialEq for GenericPolyRing<T> {
    fn eq(&self, rhs: &GenericPolyRing<T>) -> bool {
        self.base_ring() == rhs.base_ring()
    }
}

impl<T: Ring> fmt::Display for GenericPolyRing<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Univariate polynomial ring in {} over {}", self.var(), self.base_ring())
    }
}

impl<T: Ring> Hash for GenericPolyRing<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_ring().hash(state);
        self.nvars().hash(state);
    }
}

impl<T: Ring> BaseTrait for GenericPolyRing<T> {}

impl<T: Ring> Parent for GenericPolyRing<T> {
    type Element = GenericPoly<T>;

    #[inline]
    fn default(&self) -> Self::Element {
        GenericPoly {
            base_ring: Arc::clone(&self.base_ring),
            var: Arc::clone(&self.var),
            coeffs: vec![self.base_ring.default()]
        }
    }
}

impl<T: Ring> Ring for GenericPolyRing<T> {}

impl<T: Ring> GenericPolyRing<T> {
    #[inline]
    pub fn init(ring: &T, var: &str) -> Self {
        GenericPolyRing {
            base_ring: Arc::new(ring.clone()),
            var: Arc::new(RwLock::new(var.to_string()))
        }
    }

    #[inline]
    pub fn new(&self, x: Vec<T::Element>) -> GenericPoly<T> {
        GenericPoly {
            base_ring: Arc::clone(&self.base_ring),
            var: Arc::clone(&self.var),
            coeffs: x
        }
    }
    
    #[inline]
    pub fn nvars(&self) -> i64 {
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
    pub fn base_ring(&self) -> T {
        (*self.base_ring).clone()
    }
}

// Generic polynomial implementation
#[derive(Clone, Debug)]
pub struct GenericPoly<T: Ring> {
    base_ring: Arc<T>,
    var: Arc<RwLock<String>>,
    coeffs: Vec<T::Element>,
}

impl<T: Ring> Eq for GenericPoly<T> {}

impl<T: Ring> PartialEq for GenericPoly<T> {
    fn eq(&self, rhs: &GenericPoly<T>) -> bool {
        self.coefficients() == rhs.coefficients()
    }
}

impl<T: Ring> fmt::Display for GenericPoly<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.coeffs)
    }
}

impl<T: Ring> Hash for GenericPoly<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parent().hash(state);
        self.coefficients().hash(state);
    }
}

impl<T: Ring> BaseTrait for GenericPoly<T> {}

impl<T: Ring> Element for GenericPoly<T> {
    type Parent = GenericPolyRing<T>;
}

impl<'a, T: 'a + Ring> GenericPoly<T> {
    #[inline]
    pub fn parent(&self) -> <Self as Element>::Parent {
        GenericPolyRing {
            base_ring: Arc::clone(&self.base_ring),
            var: Arc::clone(&self.var)
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
    pub fn get_coeff(&self, i: usize) -> T::Element {
        self.coeffs.get(i).unwrap_or(&self.base_ring().default()).clone()
    }
    
    #[inline]
    pub fn set_coeff<S>(&mut self, i: usize, coeff: S) where
        S: Into<ValOrRef<'a, T::Element>>
    {
        if let Some(x) = self.coeffs.get_mut(i) {
            *x = coeff.into().clone();
        } else {
            let d = self.base_ring().default();
            self.coeffs.resize_with(i-1, || d.clone());
        }
    }

    #[inline]
    pub fn coefficients(&self) -> Vec<T::Element> {
        self.coeffs.clone()
    }
}

