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


//use std::ffi::{CStr, CString};
//use std::hash::{Hash, Hasher};

use crate::*;

impl Clone for QadicElem {
    fn clone(&self) -> Self {
        let mut res = self.parent().default();
        unsafe { 
            flint_sys::qadic::qadic_set(
                res.as_mut_ptr(), 
                self.as_ptr(),
                self.ctx_as_ptr()
            ); 
            res
        }
    }
}

/*
impl fmt::Debug for QadicElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("QadicElem")
            .field("ctx", &self.ctx)
            .field("extra", &self.extra)
            .field("data", &self.data)
            .finish()
    }
}
*/

/*
impl fmt::Display for PadicElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}*/


/*
impl Hash for PadicElem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        IntPol::from(self).hash(state);
    }
}*/
