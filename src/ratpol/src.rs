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


use std::ffi::{CStr, CString};
use std::sync::Arc;

use flint_sys::fmpq_poly::fmpq_poly_struct;
use libc::{c_long, c_ulong};

use crate::*;

// RatPol //

#[derive(Default, Debug, Hash, Clone)]
pub struct RatPolRing {
    pub x: Arc<String>,
}

impl Parent for RatPolRing {
    type Element = RatPol;

    #[inline]
    fn default(&self) -> RatPol {
        RatPol::default()
    }
}

impl Additive for RatPolRing {
    #[inline]
    fn zero(&self) -> RatPol {
        RatPol::default()
    }
}

impl Multiplicative for RatPolRing {
    #[inline]
    fn one(&self) -> RatPol {
        let mut res = RatPol::default();
        unsafe { flint_sys::fmpq_poly::fmpq_poly_one(res.as_mut_ptr()); }
        res
    }
}

impl AdditiveGroup for RatPolRing {}

impl Ring for RatPolRing {}

impl PolynomialRing for RatPolRing {
    type BaseRing = RationalField;

    #[inline]
    fn base_ring(&self) -> RationalField {
        RationalField {}
    }

    #[inline]
    fn gens(&self) -> Vec<RatPol> {
        vec![RatPol::from(vec![0,1].as_slice())]
    }
}

impl Init1<&str> for RatPolRing {
    #[inline]
    fn init(x: &str) -> Self {
        RatPolRing { x: Arc::new(x.to_owned()) }
    }
}

impl New<&RatPol> for RatPolRing {
    #[inline]
    fn new(&self, x: &RatPol) -> RatPol {
        x.clone()
    }
}

impl<T> New<T> for RatPolRing where 
    T: Into<RatPol>
{
    #[inline]
    fn new(&self, x: T) -> RatPol {
        x.into()
    }
}


// RatPol //

pub type RatPol = Elem<RatPolRing>;

#[derive(Debug)]
pub struct RatPolData {
    pub elem: fmpq_poly_struct,
    pub x: Arc<String>,
}

impl Drop for RatPolData {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpq_poly::fmpq_poly_clear(&mut self.elem); }
    }
}

impl Element for RatPol {
    type Data = RatPolData;
    type Parent = RatPolRing;

    #[inline]
    fn parent(&self) -> RatPolRing {
        RatPolRing { x: Arc::clone(&self.data.x) }
    }
}

impl AdditiveElement for RatPol {
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe {flint_sys::fmpq_poly::fmpq_poly_is_zero(self.as_ptr()) == 1}
    }
}

impl MultiplicativeElement for RatPol {
    #[inline]
    fn is_one(&self) -> bool {
        unsafe { flint_sys::fmpq_poly::fmpq_poly_is_one(self.as_ptr()) == 1 }
    }
}

impl AdditiveGroupElement for RatPol {}

impl RingElement for RatPol {}

impl PolynomialRingElement for RatPol {
    type BaseRingElement = Rational;

    /// Return the length of the polynomial, equivalently, the degree plus one.
    #[inline]
    fn len(&self) -> c_long {
        unsafe { flint_sys::fmpq_poly::fmpq_poly_length(self.as_ptr())}
    }
    
    #[inline]
    fn degree(&self) -> c_long {
        unsafe { flint_sys::fmpq_poly::fmpq_poly_degree(self.as_ptr())}
    }
    
    #[inline]
    fn get_coeff(&self, i: usize) -> Rational {
        let mut res = Rational::default();
        unsafe {
            flint_sys::fmpq_poly::fmpq_poly_get_coeff_fmpq(res.as_mut_ptr(), self.as_ptr(), i as i64);
            res
        }
    }
    
    #[inline]
    fn set_coeff(&mut self, i: usize, coeff: &Rational) {
        unsafe {
            flint_sys::fmpq_poly::fmpq_poly_set_coeff_fmpq(
                self.as_mut_ptr(), 
                i as c_long, 
                coeff.as_ptr()
            );
        }
    }
}

impl RatPol {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fmpq_poly_struct {
        &self.data.elem
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpq_poly_struct {
        &mut self.data.elem
    }
    
    #[inline]
    pub fn get_str(&self) -> String {
        unsafe {
            let s = flint_sys::fmpq_poly::fmpq_poly_get_str(self.as_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
    
    #[inline]
    pub fn get_str_pretty(&self) -> String {
        let v = CString::new((*self.data.x).clone()).unwrap();
        unsafe {
            let s = flint_sys::fmpq_poly::fmpq_poly_get_str_pretty(self.as_ptr(), v.as_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
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
    pub fn set_coeff_ui<T>(&mut self, i: usize, coeff: T) where
        T: Into<c_ulong>
    {
        unsafe {
            flint_sys::fmpq_poly::fmpq_poly_set_coeff_ui(
                self.as_mut_ptr(), 
                i as c_long, 
                coeff.into()
            );
        }
    }
    
    #[inline]
    pub fn set_coeff_si<T>(&mut self, i: usize, coeff: T) where
        T: Into<c_long>
    {
        unsafe {
            flint_sys::fmpq_poly::fmpq_poly_set_coeff_si(
                self.as_mut_ptr(), 
                i as c_long, 
                coeff.into()
            );
        }
    }
}

impl<T> Evaluate<T> for RatPol where
    T: Into<Rational>
{
    type Output = Rational;
    #[inline]
    fn evaluate(&self, x: T) -> Self::Output {
        self.evaluate(&x.into())
    }
}

impl Evaluate<&Rational> for RatPol {
    type Output = Rational;
    #[inline]
    fn evaluate(&self, x: &Rational) -> Self::Output {
        let mut res = Rational::default();
        unsafe {
            flint_sys::fmpq_poly::fmpq_poly_evaluate_fmpq(
                res.as_mut_ptr(),
                self.as_ptr(),
                x.as_ptr()
            );
        }
        res
    }
}
