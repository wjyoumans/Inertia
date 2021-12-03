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

use crate::integer::src::Integer;
use crate::intmod::src::IntMod;


impl Clone for IntMod {
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe { 
            flint_sys::fmpz::fmpz_init(z.as_mut_ptr());
            flint_sys::fmpz_mod::fmpz_mod_set_fmpz(
                z.as_mut_ptr(), 
                self.as_ptr(), 
                self.ctx_as_ptr()
            ); 
            IntMod { ctx: Arc::clone(&self.ctx), data: z.assume_init() }
        }
    }
}

impl fmt::Display for IntMod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for IntMod {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpz::fmpz_clear(self.as_mut_ptr());}
    }
}

impl Hash for IntMod {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Integer::from(self).hash(state);
    }
}
