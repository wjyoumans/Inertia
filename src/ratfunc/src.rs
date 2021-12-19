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

use flint_sys::fmpz_poly_q::fmpz_poly_q_struct;

use crate::*;

// RatFunc //

#[derive(Default, Debug, Hash, Clone)]
pub struct RatFuncField {
    pub x: Arc<String>,
}

impl Parent for RatFuncField {
    type Data = ();
    type Extra = Arc<String>;
    type Element = RatFunc;
    
    #[inline]
    fn default(&self) -> RatFunc {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_poly_q::fmpz_poly_q_init(z.as_mut_ptr());
            RatFunc { ctx: (), extra: Arc::clone(&self.x), data: z.assume_init() }
        }
    }
}

impl RatFuncField {
    /// Construct the field of rational functions.
    pub fn init() -> Self {
        RatFuncField {}
    }
    
    /// Create a new rational function.
    pub fn new<T: Into<RatFunc>>(&self, x: T) -> RatFunc {
        x.into()
    }
}

// RatPol //

/// A rational function represented as the quotient of integer polynomials. The field `data` is a
/// FLINT [fmpz_poly_q][flint_sys::fmpz_poly_q::fmpz_poly_q_struct]
pub type RatFunc = Elem<RatFuncField>;

impl RatFunc {
    /// A pointer to the underlying FFI type. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fmpz_poly_q_struct {
        &self.data
    }
    
    /// A mutable pointer to the underlying FFI type. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpz_poly_q_struct {
        &mut self.data
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
    pub fn get_str_pretty(&self, var: &str) -> String {
        let v = CString::new(var).unwrap();
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

    /// Return true if the rational function is zero.
    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe {flint_sys::fmpz_poly_q::fmpz_poly_q_is_zero(self.as_ptr()) == 1}
    }

    /// Return true if the rational function is one.
    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe {flint_sys::fmpz_poly_q::fmpz_poly_q_is_one(self.as_ptr()) == 1}
    }
    
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
