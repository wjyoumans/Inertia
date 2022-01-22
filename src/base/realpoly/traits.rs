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

use std::fmt;
use std::mem::MaybeUninit;
use std::sync::{Arc, RwLock};

use crate::*;

impl Clone for RealPoly {
    fn clone(&self) -> Self {
        let mut res = self.parent().default();
        unsafe { arb_sys::arb_poly::arb_poly_set(res.as_mut_ptr(), self.as_ptr()); }
        res
    }
}

impl Default for RealPoly {
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            arb_sys::arb_poly::arb_poly_init(z.as_mut_ptr());
            RealPoly {
                data: RealPolyData {
                    prec: Arc::new(RealCtx(RwLock::new(ARB_DEFAULT_PREC))), 
                    elem: z.assume_init(),
                    x: Arc::new("x".to_owned()),
                }
            }
        }
    }
}

impl fmt::Display for RealPoly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
