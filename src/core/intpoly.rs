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

impl Parent for IntPolyRing {
    type Element = IntPoly;

    #[inline]
    fn default(&self) -> IntPoly {
        self.default()
    }
}

impl Ring for IntPolyRing {
    type Element = IntPoly;
    type PolynomialRing = GenericPolyRing<Self>;
    type MatrixSpace = GenericMatSpace<Self>;

    #[inline]
    fn default(&self) -> IntPoly {
        self.default()
    }
}

impl PolynomialRing<IntegerRing> for IntPolyRing {
    type Element = IntPoly;

    #[inline]
    fn default(&self) -> IntPoly {
        self.default()
    }

    #[inline]
    fn init(_: &IntegerRing, var: &str) -> Self {
        IntPolyRing::init(var)
    }

    #[inline]
    fn base_ring(&self) -> IntegerRing {
        IntegerRing {}
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

impl Element for IntPoly {
    type Parent = IntPolyRing;

    #[inline]
    fn parent(&self) -> IntPolyRing {
        self.parent()
    }
}

impl RingElement for IntPoly {
    type Parent = IntPolyRing;

    #[inline]
    fn parent(&self) -> IntPolyRing {
        self.parent()
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.is_zero()
    }
}

impl PolynomialRingElement<IntegerRing> for IntPoly {
    type Parent = IntPolyRing;

    #[inline]
    fn parent(&self) -> IntPolyRing {
        self.parent()
    }

    #[inline]
    fn base_ring(&self) -> IntegerRing {
        IntegerRing {}
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
    fn get_coeff(&self, i: usize) -> Integer {
        self.get_coeff(i.try_into().unwrap())
    }

    #[inline]
    fn set_coeff<'a, S>(&mut self, i: usize, coeff: S)
    where
        S: Into<ValOrRef<'a, Integer>>,
    {
        self.set_coeff(i.try_into().unwrap(), coeff);
    }

    #[inline]
    fn coefficients(&self) -> Vec<Integer> {
        self.coefficients()
    }
}

impl<T> New<T> for IntPolyRing
where
    T: Into<IntPoly>,
{
    type Output = IntPoly;
    #[inline]
    fn new(&self, x: T) -> IntPoly {
        x.into()
    }
}
