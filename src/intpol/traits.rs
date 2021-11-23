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
use std::mem::{self, MaybeUninit};

use flint_sys::fmpz_poly::fmpz_poly_struct;
use rug::Assign;

use crate::traits::*;
use crate::intpol::src::{IntPol, IntPolRing};

// IntPolRing //

impl Parent for IntPolRing {
    type Data = ();
    type Element = IntPol;
}

// IntPol //

impl Element for IntPol {
    type Data = fmpz_poly_struct;
    type Parent = IntPolRing;
}

impl Clone for IntPol {
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe { 
            flint_sys::fmpz_poly::fmpz_poly_init(z.as_mut_ptr());
            flint_sys::fmpz_poly::fmpz_poly_set(z.as_mut_ptr(), &self.data); 
            IntPol { data: z.assume_init() }
        }
    }
}

impl Default for IntPol {
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_init(z.as_mut_ptr());
            IntPol { data: z.assume_init() }
        }
    }
}

impl fmt::Display for IntPol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for IntPol {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpz_poly::fmpz_poly_clear(self.as_mut_ptr());}
    }
}

// Hash

impl Assign for IntPol {
    #[inline]
    fn assign(&mut self, src: IntPol) {
        drop(mem::replace(self, src));
    }
}

impl Assign<&IntPol> for IntPol {
    #[inline]
    fn assign(&mut self, src: &IntPol) {
        unsafe { flint_sys::fmpz_poly::fmpz_poly_set(self.as_mut_ptr(), src.as_ptr()); }
    }
}


