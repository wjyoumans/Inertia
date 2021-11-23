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

use flint_sys::fmpq::fmpq;
use crate::integer::src::Integer;

// RationalField //

#[derive(Default, Debug, Hash, Clone, Copy)]
pub struct RationalField {}

impl RationalField {
    pub fn init() -> Self {
        RationalField {}
    }
    
    pub fn new<'a, T: Into<&'a Integer>>(&self, n: T, d: T) -> Rational {
        let mut z = Rational::default();
        unsafe {
            flint_sys::fmpq::fmpq_set_fmpz_frac(z.as_mut_ptr(), n.into().as_ptr(), d.into().as_ptr());
        }
        z
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Rational {
    pub data: fmpq,
}

impl Rational {
    #[inline]
    pub fn as_ptr(&self) -> &fmpq {
        &self.data
    }
   
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpq {
        &mut self.data
    }

    #[inline]
    pub fn numerator(&self) -> Integer {
        Integer {
            data: self.data.num
        }
    }
    
    #[inline]
    pub fn denominator(&self) -> Integer {
        Integer {
            data: self.data.num
        }
    }

    #[inline]
    pub fn floor(&self) -> Integer {
        Integer::fdiv(&self.numerator(), &self.denominator())
    }

    #[inline]
    pub fn ceil(&self) -> Integer {
        Integer::cdiv(&self.numerator(), &self.denominator())
    }
    
    #[inline]
    pub fn round(&self) -> Integer {
        Integer::tdiv(&self.numerator(), &self.denominator())
    }
    
    
    #[inline]
    pub fn sign(&self) -> i32 {
        unsafe {
            flint_sys::fmpq::fmpq_sgn(self.as_ptr())
        }
    }

    #[inline]
    pub fn abs(&self) -> Rational {
        unsafe {
            let mut res = Rational::default();
            flint_sys::fmpq::fmpq_abs(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }

    #[inline]
    pub fn height(&self) -> Integer {
        unsafe {
            let mut res = Integer::default();
            flint_sys::fmpq::fmpq_height(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }
    
    #[inline]
    pub fn inv(&self) -> Rational {
        unsafe {
            let mut res = Rational::default();
            flint_sys::fmpq::fmpq_inv(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }

/*
    // TODO: RANDOM GENERATION

    #[inline]
    pub fn gcd(&self, other: &Rational) -> Rational {
        unsafe {
            let mut res = Rational::default();
            flint_sys::fmpq::fmpq_gcd(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            res
        }
    }

    #[inline]
    pub fn xgcd(&self, other: &Rational) -> (Rational, Integer, Integer) {
        unsafe {
            let mut d = Rational::default();
            let mut a = Integer::default();
            let mut b = Integer::default();
            flint_sys::fmpq::fmpq_gcd_cofactors(
                d.as_mut_ptr(), 
                a.as_mut_ptr(), 
                b.as_mut_ptr(),
                self.as_ptr(), 
                other.as_ptr());
            (d, a, b)
        }
    }
    */
}
