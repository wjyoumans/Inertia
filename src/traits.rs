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

use crate::ValOrRef;
use std::fmt;
use std::hash::Hash;

// hash, serialize/deserialize, display, Eq, PartialEq
pub trait BaseTrait: Clone + fmt::Debug + fmt::Display + Eq + Hash + PartialEq {}

impl<T> BaseTrait for T where
    T: Clone + fmt::Debug + fmt::Display + Eq + Hash + PartialEq
{}

pub trait Parent: BaseTrait {
    type Element: BaseTrait;

    fn default(&self) -> Self::Element;
}

pub trait Ring: Parent {
    type Element: RingElement;
    type PolynomialRing: PolynomialRing<Self>;
    
    fn default(&self) -> <Self as Ring>::Element;
}

pub trait PolynomialRing<T: Ring>: Ring {
    type Element: PolynomialRingElement<T>;

    fn default(&self) -> <Self as PolynomialRing<T>>::Element;
    fn init(ring: &T, var: &str) -> Self;
    fn base_ring(&self) -> T;
    fn var(&self) -> String;
    fn set_var<S: AsRef<str>>(&self, var: S);

    #[inline]
    fn nvars(&self) -> i64 {
        1
    }
    
    //#[inline]
    //fn gen(&self) -> <Self as PolynomialRing<T>>::Element {
    //    let mut p = PolynomialRing::default(self)
    //}
}

pub trait New<T>: Parent {
    fn new(&self, x: T) -> Self::Element;
}

////////////////////////////////////////////////////////////////////////

pub trait Element: BaseTrait {
    type Parent: BaseTrait;
    fn parent(&self) -> Self::Parent;
}

pub trait RingElement: Element {// + Add + AddAssign + Sub + SubAssign + Mul + MulAssign {
    type Parent: Ring;
    fn parent(&self) -> <Self as RingElement>::Parent;
    fn is_zero(&self) -> bool;
}

pub trait PolynomialRingElement<T: Ring>: RingElement {
    type Parent: PolynomialRing<T>;
    
    fn parent(&self) -> <Self as PolynomialRingElement<T>>::Parent;
    fn base_ring(&self) -> T;
    fn var(&self) -> String;
    fn set_var<S: AsRef<str>>(&self, var: S);
    fn degree(&self) -> i64;
    fn len(&self) -> usize;
    fn get_coeff(&self, i: usize) -> <T as Ring>::Element;
    fn set_coeff<'a, S>(&mut self, i: usize, coeff: S) where
        <T as Ring>::Element: 'a,
        S: Into<ValOrRef<'a, <T as Ring>::Element>>;
    fn coefficients(&self) -> Vec<<T as Ring>::Element>;
}
