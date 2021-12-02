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

use antic_sys::nf_struct;
use antic_sys::nf_elem_struct;

use crate::traits::*;

// NumberField //

/// A number field.
pub struct NumberField {
    pub ctx: <Self as Parent>::Data,
}

// NumFldElem //

/// A number field element.
pub type NumFldElem = Elem<NumberField>;

impl NumFldElem {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// Antic via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &nf_elem_struct {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with Antic via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut nf_elem_struct {
        &mut self.data
    }

    /// A reference to the struct holding context information. This is only needed to interface
    /// directly with Antic via the FFI.
    #[inline]
    pub fn ctx_as_ptr(&self) -> &nf_struct {
        &self.ctx.0
    }
    
    /*
    /// Return a [String] representation of a number field element.
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
    */
}
