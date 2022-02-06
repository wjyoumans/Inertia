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

use crate::*;


impl AsRef<RatMat> for RatMat {
    fn as_ref(&self) -> &RatMat {
        self
    }
}

impl fmt::Display for RatMatSpace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The space of {}x{} matrices over the rationals", self.nrows(), self.ncols())
    }
}

impl Clone for RatMat {
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpq_mat::fmpq_mat_init_set(z.as_mut_ptr(), self.as_ptr());
            RatMat { data: z.assume_init() }
        }
    }
}

/*
impl fmt::Debug for RatMat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RatMat")
            .field("ctx", &self.ctx)
            .field("extra", &self.extra)
            .field("data", &self.data)
            .finish()
    }
}*/

impl fmt::Display for RatMat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Hash for RatMat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entries().hash(state);
    }
}
