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

use arb_sys::acb::acb_struct;
use arb_sys::arb::arb_struct;
use libc::{c_int, c_long, c_ulong};
use num_traits::Zero;

use crate::traits::*;
use crate::rational::src::Rational;

/// The field of complex numbers with initial precision given by `ctx`.
pub struct ComplexField {
    pub ctx: <Self as Parent>::Data,
}

impl ParentInit1 for ComplexField {
    fn init(prec: c_long) -> Self {
        ComplexField { ctx: Arc::new(prec)}
    }
}

impl<T: Into<Complex>> ParentNew<T> for ComplexField {
    fn new(&self, x: T) -> Complex {
        x.into()
    }
}

/// A complex number represented as a pair of [Reals][Real], representing real and imaginary parts
/// with separate error bounds.
pub type Complex = Elem<ComplexField>;

impl Complex {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// Arb via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &acb_struct {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with Arb via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut acb_struct {
        &mut self.data
    }
    
    /// A reference to the underlying FFI struct of the real part of a complex number. This is only 
    /// needed to interface directly with Arb via the FFI.
    #[inline]
    pub fn real_as_ptr(&self) -> &arb_struct {
        &self.data.real
    }
    
    /// A mutable reference to the underlying FFI struct of the real part of a complex number. This is
    /// only needed to interface directly with Arb via the FFI.
    #[inline]
    pub fn real_as_mut_ptr(&mut self) -> &mut arb_struct {
        &mut self.data.real
    }
    
    /// A reference to the underlying FFI struct of the imaginary part of a complex number. This is 
    /// only needed to interface directly with Arb via the FFI.
    #[inline]
    pub fn imag_as_ptr(&self) -> &arb_struct {
        &self.data.imag
    }
    
    /// A mutable reference to the underlying FFI struct of the imaginary part of a complex number. 
    /// This is only needed to interface directly with Arb via the FFI.
    #[inline]
    pub fn imag_as_mut_ptr(&mut self) -> &mut arb_struct {
        &mut self.data.imag
    }
    
    /// Return a [String] representation of the complex number.
    #[inline]
    pub fn get_str(&self, n: c_long) -> String {
        unsafe {
            let r = CStr::from_ptr(arb_sys::arb::arb_get_str(self.real_as_ptr(), n, 0)).to_str();
            let i = CStr::from_ptr(arb_sys::arb::arb_get_str(self.imag_as_ptr(), n, 0)).to_str();
            if r.is_ok() && i.is_ok() {
                format!("{} + i*{}", r.unwrap(), i.unwrap())
            } else {
                panic!("Arb returned invalid UTF-8!")
            }
        }
    }
}
