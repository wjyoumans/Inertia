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
use std::mem::MaybeUninit;

use flint_sys::fmpz_poly::fmpz_poly_struct;

use crate::traits::*;
use crate::integer::src::Integer;
use crate::rational::src::Rational;
use crate::intpol::src::{IntPol, IntPolRing};
use crate::ratpol::src::RatPol;

// IntPolRing //

impl Parent for IntPolRing {
    type Data = ();
    type Element = IntPol;
}

// IntPol //

impl Element for IntPol {
    type Data = ();
    type Parent = IntPolRing;
}

impl Clone for IntPol {
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe { 
            flint_sys::fmpz_poly::fmpz_poly_init(z.as_mut_ptr());
            flint_sys::fmpz_poly::fmpz_poly_set(z.as_mut_ptr(), &self.data); 
            IntPol { data: z.assume_init() }
        }
    }
}

impl Default for IntPol {
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_init(z.as_mut_ptr());
            IntPol { data: z.assume_init() }
        }
    }
}

impl fmt::Display for IntPol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for IntPol {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpz_poly::fmpz_poly_clear(self.as_mut_ptr());}
    }
}

impl Hash for IntPol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coefficients().hash(state);
    }
}


impl<T> Evaluate<T> for IntPol where
    T: Into<Integer>
{
    type Output = Integer;
    #[inline]
    fn evaluate(&self, x: T) -> Self::Output {
        self.evaluate(&x.into())
    }
}

impl Evaluate<&Integer> for IntPol {
    type Output = Integer;
    #[inline]
    fn evaluate(&self, x: &Integer) -> Self::Output {
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_evaluate_fmpz(
                res.as_mut_ptr(),
                self.as_ptr(),
                x.as_ptr()
            );
        }
        res
    }
}

impl Evaluate<Rational> for IntPol {
    type Output = Rational;
    #[inline]
    fn evaluate(&self, x: Rational) -> Self::Output {
        RatPol::from(self).evaluate(x)
    }
}

impl Evaluate<&Rational> for IntPol {
    type Output = Rational;
    #[inline]
    fn evaluate(&self, x: &Rational) -> Self::Output {
        RatPol::from(self).evaluate(x)
    }
}
