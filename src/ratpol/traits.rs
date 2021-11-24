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
use std::mem::{self, MaybeUninit};

use flint_sys::fmpq_poly::fmpq_poly_struct;
use rug::Assign;

use crate::traits::*;
use crate::ratpol::src::{RatPol, RatPolRing};

// RatPolRing //

impl Parent for RatPolRing {
    type Data = ();
    type Element = RatPol;
}

// RatPol //

impl Element for RatPol {
    type Data = fmpq_poly_struct;
    type Parent = RatPolRing;
}

impl Clone for RatPol {
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe { 
            flint_sys::fmpq_poly::fmpq_poly_init(z.as_mut_ptr());
            flint_sys::fmpq_poly::fmpq_poly_set(z.as_mut_ptr(), &self.data); 
            RatPol { data: z.assume_init() }
        }
    }
}

impl Default for RatPol {
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpq_poly::fmpq_poly_init(z.as_mut_ptr());
            RatPol { data: z.assume_init() }
        }
    }
}

impl fmt::Display for RatPol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for RatPol {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpq_poly::fmpq_poly_clear(self.as_mut_ptr());}
    }
}

impl Hash for RatPol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coefficients().hash(state);
    }
}

