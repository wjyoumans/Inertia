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
use std::mem::{MaybeUninit, ManuallyDrop};
use std::sync::Arc;

use flint_sys::fmpz_poly_q::fmpz_poly_q_struct;

use crate::*;

#[derive(Default, Debug, Hash, Clone)]
pub struct RatFuncField {
    pub x: Arc<String>,
}

impl Parent for RatFuncField {
    type Element = RatFunc;
    
    #[inline]
    fn default(&self) -> RatFunc {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_poly_q::fmpz_poly_q_init(z.as_mut_ptr());
            RatFunc { 
                data: RatFuncData {
                    x: Arc::clone(&self.x), 
                    elem: z.assume_init() 
                }
            }
        }
    }
}

impl Additive for RatFuncField {
    #[inline]
    fn zero(&self) -> RatFunc {
        self.default()
    }
}

impl Multiplicative for RatFuncField {
    #[inline]
    fn one(&self) -> RatFunc {
        let mut res = self.default();
        unsafe { flint_sys::fmpz_poly_q::fmpz_poly_q_one(res.as_mut_ptr()); }
        res
    }
}

impl AdditiveGroup for RatFuncField {}

impl MultiplicativeGroup for RatFuncField {}

impl Ring for RatFuncField {}

impl Field for RatFuncField {
    type BaseField = RatFuncField;

    #[inline]
    fn base_field(&self) -> RatFuncField {
        self.clone()
    }
}

impl Init1<&str> for RatFuncField {
    #[inline]
    fn init(x: &str) -> RatFuncField {
        RatFuncField { x: Arc::new(x.to_owned()) }
    }
}

impl New<&IntPol> for RatFuncField {
    fn new(&self, x: &IntPol) -> RatFunc {
        let mut res = self.default();
        let num = ManuallyDrop::new(x.clone());
        let den = ManuallyDrop::new(IntPol::from(1));
        
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_set(&mut res.data.elem.num, num.as_ptr());
            flint_sys::fmpz_poly::fmpz_poly_set(&mut res.data.elem.den, den.as_ptr());
        }

        res
    }
}

impl<T> New<T> for RatFuncField where
    T: Into<IntPol>
{
    #[inline]
    fn new(&self, x: T) -> RatFunc {
        self.new(&x.into())
    }
}

/// A rational function represented as the quotient of integer polynomials. The field `data` is a
/// FLINT [fmpz_poly_q][flint_sys::fmpz_poly_q::fmpz_poly_q_struct]
pub type RatFunc = Elem<RatFuncField>;

#[derive(Debug)]
pub struct RatFuncData {
    pub elem: fmpz_poly_q_struct,
    pub x: Arc<String>,
}

impl Drop for RatFuncData {
    fn drop(&mut self) { 
        unsafe { flint_sys::fmpz_poly_q::fmpz_poly_q_clear(&mut self.elem);}
    }
}

impl Element for RatFunc {
    type Data = RatFuncData;
    type Parent = RatFuncField;

    #[inline]
    fn parent(&self) -> RatFuncField {
        RatFuncField { x: Arc::clone(&self.data.x) }
    }
}

impl AdditiveElement for RatFunc {    
    /// Return true if the rational function is zero.
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe {flint_sys::fmpz_poly_q::fmpz_poly_q_is_zero(self.as_ptr()) == 1}
    }
}

impl MultiplicativeElement for RatFunc {    
    /// Return true if the rational function is one.
    #[inline]
    fn is_one(&self) -> bool {
        unsafe {flint_sys::fmpz_poly_q::fmpz_poly_q_is_one(self.as_ptr()) == 1}
    }
}

impl AdditiveGroupElement for RatFunc {}

impl MultiplicativeGroupElement for RatFunc {}

impl RingElement for RatFunc {}

impl RatFunc {
    /// A pointer to the underlying FFI type. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fmpz_poly_q_struct {
        &self.data.elem
    }
    
    /// A mutable pointer to the underlying FFI type. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpz_poly_q_struct {
        &mut self.data.elem
    }
    
    /// Return a [String] representation of a rational function.
    #[inline]
    pub fn get_str(&self) -> String {
        unsafe {
            let s = flint_sys::fmpz_poly_q::fmpz_poly_q_get_str(self.as_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
    
    /// Return a pretty-printed [String] representation of a rational function.
    #[inline]
    pub fn get_str_pretty(&self) -> String {
        let v = CString::new((*self.data.x).clone()).unwrap();
        unsafe {
            let s = flint_sys::fmpz_poly_q::fmpz_poly_q_get_str_pretty(self.as_ptr(), v.as_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }

    /*
    /// Return the numerator of a rational function as an integer polynomial.
    #[inline]
    pub fn numerator(&self) -> IntPol {
        let mut num = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_set(num.as_mut_ptr(), &self.data.num);
        }
        num
    }

    /// Return the denominator of a rational function as an integer polynomial.
    #[inline]
    pub fn denominator(&self) -> IntPol {
        let mut den = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_set(den.as_mut_ptr(), &self.data.den);
        }
        den
    }
    
    /// Return the numerator and denominator of a rational function as integer polynomials.
    #[inline]
    pub fn num_den(&self) -> (IntPol, IntPol) {
        (self.numerator(), self.denominator())
    }
    
    /// Set the numerator of a rational function to a given integer polynomial.
    #[inline]
    pub fn set_numerator(&mut self, num: &IntPol) {
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_set(&mut self.data.num, num.as_ptr());
        }
    }
    
    /// Set the denominator of a rational function to a given integer polynomial.
    #[inline]
    pub fn set_denominator(&mut self, den: &IntPol) {
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_set(&mut self.data.den, den.as_ptr());
        }
    }*/
    
    /* TODO: need num_den
    // TODO: is it canonical by default?
    /// Return the degree of a rational function, defined as the maximum of the degrees of the
    /// numerator and denominator after canonicalization.
    #[inline]
    pub fn degree(&self) -> c_long {
        let (num, den) = self.num_den();
        std::cmp::max(num.degree(), den.degree())
    }
    
    /// Return the relative degree of a rational function, defined as the difference of the
    /// numerator and denominator after canonicalization. If the denominator has a larger degree
    /// then the result will be negative.
    #[inline]
    pub fn relative_degree(&self) -> c_long {
        let (num, den) = self.num_den();
        num.degree() - den.degree()
    }
    */
    
}
