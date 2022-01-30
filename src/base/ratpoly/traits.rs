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
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::sync::Arc;

use crate::*;


impl AsRef<RatPoly> for RatPoly {
    fn as_ref(&self) -> &RatPoly {
        self
    }
}

impl fmt::Display for RatPolyRing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Polynomial ring in {} over the rationals", self.gens()[0])
    }
}

impl Clone for RatPoly {
    fn clone(&self) -> Self {
        let mut res = self.parent().default();
        unsafe { 
            flint_sys::fmpq_poly::fmpq_poly_set(res.as_mut_ptr(), self.as_ptr()); 
        }
        res
    }
}

/*
impl fmt::Debug for RatPoly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IntPol")
            .field("ctx", &self.ctx)
            .field("extra", &self.extra)
            .field("data", &self.data)
            .finish()
    }
}*/

impl Default for RatPoly {
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpq_poly::fmpq_poly_init(z.as_mut_ptr());
            RatPoly { data: RatPolyData { x: Arc::new("x".to_owned()), elem: z.assume_init() } }
        }
    }
}

impl fmt::Display for RatPoly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Hash for RatPoly {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coefficients().hash(state);
    }
}

