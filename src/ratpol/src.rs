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

use flint_sys::fmpq_poly::fmpq_poly_struct;
use libc::c_long;
use crate::traits::Element;
use crate::integer::src::Integer;
use crate::rational::src::Rational;
use crate::intpol::src::IntPol;

// RatPol //

#[derive(Default, Debug, Hash, Clone, Copy)]
pub struct RatPolRing {}

impl RatPolRing {
    pub fn init() -> Self {
        RatPolRing {}
    }
    
    pub fn new<T: Into<RatPol>>(&self, x: T) -> RatPol {
        x.into()
    }
}

// RatPol //

#[derive(Debug)]
#[repr(transparent)]
pub struct RatPol {
    pub data: <Self as Element>::Data,
}

impl RatPol {
    #[inline]
    pub fn as_ptr(&self) -> &fmpq_poly_struct {
        &self.data
    }
    
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpq_poly_struct {
        &mut self.data
    }

    #[inline]
    pub fn len(&self) -> c_long {
        unsafe { flint_sys::fmpq_poly::fmpq_poly_length(self.as_ptr())}
    }
    
    #[inline]
    pub fn degree(&self) -> c_long {
        unsafe { flint_sys::fmpq_poly::fmpq_poly_degree(self.as_ptr())}
    }
    
    #[inline]
    pub fn numerator(&self) -> IntPol {
        let mut res = IntPol::default();
        unsafe {
            flint_sys::fmpq_poly::fmpq_poly_get_numerator(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
    
    #[inline]
    pub fn denominator(&self) -> Integer {
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpq_poly::fmpq_poly_get_denominator(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
    
    #[inline]
    pub fn get_coeff(&self, i: usize) -> Rational {
        let mut res = Rational::default();
        unsafe {
            flint_sys::fmpq_poly::fmpq_poly_get_coeff_fmpq(res.as_mut_ptr(), self.as_ptr(), i as i64);
            res
        }
    }
    
    #[inline]
    pub fn set_coeff<T>(&mut self, i: usize, coeff: T) where T: Into<Rational> {
        unsafe {
            flint_sys::fmpq_poly::fmpq_poly_set_coeff_fmpq(
                self.as_mut_ptr(), 
                i as c_long, 
                coeff.into().as_ptr()
            );
        }
    }

    #[inline]
    pub fn coefficients(&self) -> Vec<Rational> {
        let len = self.len();

        let mut vec = Vec::<Rational>::default();
        for i in 0..len {
            vec.push(self.get_coeff(i as usize));
        }
        vec
    }
}
