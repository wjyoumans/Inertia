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
use std::mem::MaybeUninit;
use std::sync::Arc;

use flint_sys::fq_default_poly::fq_default_poly_struct as fq_poly_struct;
use flint_sys::fq_default::fq_default_ctx_struct as fq_ctx_struct;
use libc::c_long;
use num_traits::PrimInt;

use crate::traits::*;
use crate::integer::src::Integer;
use crate::intpol::src::IntPol;
use crate::finfld::src::{FqCtx, FinFldElem, FiniteField};


/// The finite field with `p^k` elements for `p` prime.
pub struct FinFldPolRing {
    pub ctx: <Self as Parent>::Data,
}

impl Parent for FinFldPolRing {
    type Data = Arc<FqCtx>;
    type Element = FinFldPol;
}

/*
impl ParentInit2<&Integer, c_long> for FinFldPolRing {
    /// Construct the ring of polynomials over the finite field with `p^k` elements.
    #[inline]
    fn init(p: &Integer, k: c_long) -> Self {
        let ff = FiniteField::init(p, k);
        FinFldPolRing { ctx: Arc::clone(&ff.ctx) }
    }
}

impl<T> ParentInit2<T, c_long> for FinFldPolRing where
    T: PrimInt + Into<Integer>
{
    /// Construct the ring of polynomials over the finite field with `p^k` elements.
    #[inline]
    fn init(p: T, k: c_long) -> Self {
        let ff = FiniteField::init(p, k);
        FinFldPolRing { ctx: Arc::clone(&ff.ctx) }
    }
}
*/

impl New<&IntPol> for FinFldPolRing {
    /// Construct a polynomial over a finite field.
    #[inline]
    fn new(&self, n: &IntPol) -> FinFldPol {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fq_default_poly::fq_default_poly_set_fmpz_poly(
                z.as_mut_ptr(),
                n.as_ptr(),
                &self.ctx.0
            );
            FinFldPol { ctx: Arc::clone(&self.ctx), data: z.assume_init() }
        }
    }
}

impl<T> New<T> for FinFldPolRing where
    T: Into<IntPol>
{
    /// Construct a polynomial over a finite field.
    #[inline]
    fn new(&self, n: T) -> FinFldPol {
        self.new(&n.into())
    }
}

/// An element of a finite field.
pub type FinFldPol = Elem<FinFldPolRing>;

impl Element for FinFldPol {
    type Data = fq_poly_struct;
    type Parent = FinFldPolRing;
}

impl FinFldPol {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fq_poly_struct {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fq_poly_struct {
        &mut self.data
    }

    /// A reference to the struct holding context information. This is only needed to interface
    /// directly with FLINT via the FFI.
    pub fn ctx_ptr(&self) -> &fq_ctx_struct {
        &self.ctx.0
    }
    
    /// Return a [String] representation of a polynomial over a finite field.
    #[inline]
    pub fn get_str(&self) -> String {
        unsafe {
            let s = flint_sys::fq_default_poly::fq_default_poly_get_str(
                self.as_ptr(),
                self.ctx_ptr()
            );
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
    
    /// Return a pretty-printed [String] representation of a polynomial over a finite field.
    #[inline]
    pub fn get_str_pretty(&self, var: &str) -> String {
        let v = CString::new(var).unwrap();
        unsafe {
            let s = flint_sys::fq_default_poly::fq_default_poly_get_str_pretty(
                self.as_ptr(), 
                v.as_ptr(),
                self.ctx_ptr()
            );
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
    
    /// Return the length of the polynomial, equivalently, the degree plus one.
    pub fn len(&self) -> c_long {
        unsafe {
            flint_sys::fq_default_poly::fq_default_poly_length(
                self.as_ptr(),
                self.ctx_ptr()
            )
        }
    }

    /// Get the i-th coefficient of a polynomial over a finite field.
    #[inline]
    pub fn get_coeff(&self, i: usize) -> FinFldElem {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fq_default::fq_default_init(z.as_mut_ptr(), self.ctx_ptr());
            flint_sys::fq_default_poly::fq_default_poly_get_coeff(
                z.as_mut_ptr(), 
                self.as_ptr(), 
                i as i64,
                self.ctx_ptr()
            );
            FinFldElem { ctx: Arc::clone(&self.ctx), data: z.assume_init() }
        }
    }
    
    /// Set the i-th coefficient of a polynomial over a finite field.
    #[inline]
    pub fn set_coeff(&mut self, i: usize, coeff: &FinFldElem) {
        unsafe {
            flint_sys::fq_default_poly::fq_default_poly_set_coeff(
                self.as_mut_ptr(), 
                i as c_long, 
                coeff.as_ptr(),
                self.ctx_ptr()
            );
        }
    }
    
    // NOTE: there is also fq_default_poly_get/set_coeff_fmpz

    /// Return an [FinFldElem] vector containing the coefficients of the polynomial.
    #[inline]
    pub fn coefficients(&self) -> Vec<FinFldElem> {
        let len = self.len();

        let mut vec = Vec::<FinFldElem>::default();
        for i in 0..len {
            vec.push(self.get_coeff(i as usize));
        }
        vec
    }
}
