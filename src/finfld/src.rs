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

use flint_sys::fq_default::fq_default_struct as fq_struct;
use flint_sys::fq_default::fq_default_ctx_struct as fq_ctx_struct;
use libc::c_long;

use crate::traits::*;
use crate::integer::src::Integer;

/// The finite field with `p^k` elements for `p` prime.
pub struct FiniteField {
    pub ctx: <Self as Parent>::Data,
}

impl FiniteField {
    /// Construct the finite field with `p^k` elements for `p` prime.
    pub fn init(p: &Integer, k: c_long) -> Self {
        assert!(p.is_prime());
        assert!(k > 0);

        let var = CString::new("o").unwrap();
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fq_default::fq_default_ctx_init(z.as_mut_ptr(), p.as_ptr(), k, var.as_ptr());
            FiniteField { ctx: Arc::new(Wrap { wrap: z.assume_init() }) }
        }
    }

    /// Construct an element of a finite field.
    pub fn new<T: Into<FinFldElem>>(&self, x: T) -> FinFldElem {
        x.into()
    }
}

/// An element of a finite field.
pub type FinFldElem = Elem<FiniteField>;

impl FinFldElem {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fq_struct {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fq_struct {
        &mut self.data
    }

    /// A reference to the struct holding context information. This is only needed to interface
    /// directly with FLINT via the FFI.
    pub fn ctx_ptr(&self) -> &fq_ctx_struct {
        &self.ctx.wrap
    }
    
    /// Return a [String] representation of a finite field element.
    #[inline]
    pub fn get_str(&self) -> String {
        unsafe {
            let s = flint_sys::fq_default::fq_default_get_str(self.as_ptr(), self.ctx_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
    
    /// Return a pretty-printed [String] representation of a finite field element.
    #[inline]
    pub fn get_str_pretty(&self) -> String {
        unsafe {
            let s = flint_sys::fq_default::fq_default_get_str_pretty(self.as_ptr(), self.ctx_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
}
