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


impl fmt::Display for IntPolyRing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Polynomial ring in {} over the integers", self.gens()[0])
    }
}

impl Clone for IntPoly {
    fn clone(&self) -> Self {
        let mut res = IntPoly::default();
        unsafe { flint_sys::fmpz_poly::fmpz_poly_set(res.as_mut_ptr(), self.as_ptr()); }
        res
    }
}

/*
impl fmt::Debug for IntPoly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IntPoly")
            .field("data", &self.data)
            .finish()
    }
}*/

impl Default for IntPoly {
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_init(z.as_mut_ptr());
            IntPoly { data: IntPolyData { x: Arc::new("x".to_owned()), elem: z.assume_init() } }
        }
    }
}

impl fmt::Display for IntPoly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Hash for IntPoly {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coefficients().hash(state);
    }
}
