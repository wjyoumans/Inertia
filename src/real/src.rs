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

use arb_sys::arb::arb_struct;
use libc::{c_int, c_long, c_ulong};
use num_traits::Zero;

use crate::traits::*;
use crate::rational::src::Rational;

/// The field of real numbers with initial precision given by `ctx`.
pub struct RealField {
    pub ctx: <Self as Parent>::Data,
}

impl ParentInit1<c_long> for RealField {
    fn init(prec: c_long) -> Self {
        RealField { ctx: Arc::new(prec)}
    }
}

impl<T: Into<Real>> ParentNew<T> for RealField {
    fn new(&self, x: T) -> Real {
        x.into()
    }
}

/// A real number represented as a ball over the real numbers, that is, an interval `[m +/- r] = 
/// [m - r, m + r]` where the midpoint `m` and the radius `r` are (extended) real numbers and `r` is 
/// nonnegative (possibly infinite).
pub type Real = Elem<RealField>;

impl Real {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// Arb via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &arb_struct {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with Arb via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut arb_struct {
        &mut self.data
    }
    
    /// Return a [String] representation of the real number.
    #[inline]
    pub fn get_str(&self, n: c_long) -> String {
        unsafe {
            let s = arb_sys::arb::arb_get_str(self.as_ptr(), n, 0);
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Arb returned invalid UTF-8!")
            }
        }
    }
}
