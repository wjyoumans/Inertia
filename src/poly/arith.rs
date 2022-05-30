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
use std::ops::{Add, Sub, Mul};
use std::rc::Rc;

// TODO: placeholder only, need lots of boilerplate

type InnerPoly<T> = <<T as Ring>::PolynomialRing as PolynomialRing<T>>::Element;

impl<T> Add for Poly<T> where
    T: Ring,
    InnerPoly<T>: Add<Output = InnerPoly<T>>
{
    type Output = Poly<T>;
    fn add(self, rhs: Poly<T>) -> Self::Output {
        let inner = self.inner + rhs.inner;
        Poly { ctx: Rc::clone(&self.ctx), inner }
    }
}

impl<T> Sub for Poly<T> where
    T: Ring,
    InnerPoly<T>: Sub<Output = InnerPoly<T>>
{
    type Output = Poly<T>;
    fn sub(self, rhs: Poly<T>) -> Self::Output {
        let inner = self.inner - rhs.inner;
        Poly { ctx: Rc::clone(&self.ctx), inner }
    }
}

impl<T> Mul for Poly<T> where
    T: Ring,
    InnerPoly<T>: Mul<Output = InnerPoly<T>>
{
    type Output = Poly<T>;
    fn mul(self, rhs: Poly<T>) -> Self::Output {
        let inner = self.inner * rhs.inner;
        Poly { ctx: Rc::clone(&self.ctx), inner }
    }
}
