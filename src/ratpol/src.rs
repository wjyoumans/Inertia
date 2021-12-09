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
    type Data = ();
    type Extra = Arc<String>;
    type Element = RatPol;
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

impl MultiplicativeGroup for RatPolRing {}

impl Ring for RatPolRing {}

impl PolynomialRing for RatPolRing {
    type BaseRing = RationalField;

    #[inline]
    fn base_ring(&self) -> RationalField {
        RationalField {}
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

impl Element for RatPol {
    type Data = fmpq_poly_struct;
    type Parent = RatPolRing;

    #[inline]
    fn parent(&self) -> RatPolRing {
        RatPolRing { x: Arc::clone(&self.extra) }
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

impl MultiplicativeGroupElement for RatPol {}

impl RingElement for RatPol {}

impl PolynomialRingElement for RatPol {}

impl RatPol {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fmpq_poly_struct {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpq_poly_struct {
        &mut self.data
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
        let v = CString::new((*self.extra).clone()).unwrap();
        unsafe {
            let s = flint_sys::fmpq_poly::fmpq_poly_get_str_pretty(self.as_ptr(), v.as_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }

    /// Return the length of the polynomial, equivalently, the degree plus one.
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
    pub fn set_coeff(&mut self, i: usize, coeff: &Rational) {
        unsafe {
            flint_sys::fmpq_poly::fmpq_poly_set_coeff_fmpq(
                self.as_mut_ptr(), 
                i as c_long, 
                coeff.as_ptr()
            );
        }
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
