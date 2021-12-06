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
//use std::fmt;
//use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::sync::Arc;

use crate::*;

impl Clone for QadicElem {
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe { 
            flint_sys::qadic::qadic_init(z.as_mut_ptr());
            flint_sys::qadic::qadic_set(
                z.as_mut_ptr(), 
                self.as_ptr(),
                self.ctx_as_ptr()
            ); 
            QadicElem { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
        }
    }
}

/*
impl fmt::Display for PadicElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}*/

impl Drop for QadicElem {
    fn drop(&mut self) {
        unsafe { flint_sys::qadic::qadic_clear(self.as_mut_ptr()); }
    }
}

/*
impl Hash for PadicElem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        IntPol::from(self).hash(state);
    }
}*/
