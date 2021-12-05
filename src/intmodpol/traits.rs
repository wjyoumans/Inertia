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


impl Clone for IntModPol {
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe { 
            flint_sys::fmpz_mod_poly::fmpz_mod_poly_init(z.as_mut_ptr(), self.ctx_as_ptr());
            flint_sys::fmpz_mod_poly::fmpz_mod_poly_set(
                z.as_mut_ptr(), 
                self.as_ptr(), 
                self.ctx_as_ptr()
            ); 
            IntModPol { 
                ctx: Arc::clone(&self.ctx), 
                extra: Arc::clone(&self.extra), 
                data: z.assume_init() 
            }
        }
    }
}

impl fmt::Display for IntModPol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for IntModPol {
    fn drop(&mut self) {
        unsafe { 
            flint_sys::fmpz_mod_poly::fmpz_mod_poly_clear(self.as_mut_ptr(), self.ctx_as_ptr());
        }
    }
}

impl Hash for IntModPol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        IntPol::from(self).hash(state);
    }
}
