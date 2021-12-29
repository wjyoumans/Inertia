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

use crate::*;

/*
impl Hash for FiniteField { 
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.prime().hash(state);
        self.modulus().hash(state);
    }
}*/

impl fmt::Display for FiniteField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Finite field with {}^{} elements", self.prime(), self.degree())
    }
}

impl Clone for FinFldElem {
    fn clone(&self) -> Self {
        let mut res = self.parent().default();
        unsafe { 
            flint_sys::fq_default::fq_default_set(
                res.as_mut_ptr(), 
                self.as_ptr(),
                self.ctx_as_ptr()
            ); 
        }
        res
    }
}

impl fmt::Display for FinFldElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Hash for FinFldElem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        IntPoly::from(self).hash(state);
        self.parent().hash(state);
    }
}
