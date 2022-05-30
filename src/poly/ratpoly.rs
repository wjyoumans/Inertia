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

use crate::*;

impl Parent for RatPolyRing {
    type Element = RatPoly;

    #[inline]
    fn default(&self) -> RatPoly {
        self.default()
    }
}

impl Ring for RatPolyRing {
    type Element = RatPoly;
    type PolynomialRing = GenericPolyRing<Self>;
    
    #[inline]
    fn default(&self) -> RatPoly {
        self.default()
    }
}

impl PolynomialRing<RationalField> for RatPolyRing {
    type Element = RatPoly;
    
    #[inline]
    fn default(&self) -> RatPoly {
        self.default()
    }
    
    #[inline]
    fn init(_: &RationalField, var: &str) -> Self {
        RatPolyRing::init(var)
    }

    #[inline]
    fn base_ring(&self) -> RationalField {
        RationalField {}
    }

    #[inline]
    fn var(&self) -> String {
        self.var()
    }
    
    #[inline]
    fn set_var<S: AsRef<str>>(&self, var: S) {
        self.set_var(var);
    }
}

impl Element for RatPoly {
    type Parent = RatPolyRing;
    
    #[inline]
    fn parent(&self) -> Self::Parent {
        self.parent()
    }
}

impl RingElement for RatPoly {
    type Parent = RatPolyRing;
    
    #[inline]
    fn parent(&self) -> RatPolyRing {
        self.parent()
    }
    
    #[inline]
    fn is_zero(&self) -> bool {
        self.is_zero()
    }
}

impl PolynomialRingElement<RationalField> for RatPoly {
    type Parent = RatPolyRing;
    
    #[inline]
    fn parent(&self) -> RatPolyRing {
        self.parent()
    }
    
    #[inline]
    fn base_ring(&self) -> RationalField {
        RationalField {}
    }

    #[inline]
    fn var(&self) -> String {
        self.var()
    }
    
    #[inline]
    fn set_var<S: AsRef<str>>(&self, var: S) {
        self.set_var(var);
    }
    
    #[inline]
    fn len(&self) -> usize {
        self.len().try_into().unwrap()
    }
    
    #[inline]
    fn degree(&self) -> i64 {
        self.degree()
    }
    
    #[inline]
    fn get_coeff(&self, i: usize) -> Rational {
        self.get_coeff(i.try_into().unwrap())
    }
    
    #[inline]
    fn set_coeff<'a, S>(&mut self, i: usize, coeff: S) where
        S: Into<ValOrRef<'a, Rational>>
    {
        self.set_coeff(i.try_into().unwrap(), coeff);
    }

    #[inline]
    fn coefficients(&self) -> Vec<Rational> {
        self.coefficients()
    }
}
