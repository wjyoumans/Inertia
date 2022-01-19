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


impl AsRef<Integer> for Integer {
    fn as_ref(&self) -> &Integer {
        self
    }
}

impl fmt::Display for IntegerRing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Integer ring")
    }
}

impl Clone for Integer {
    fn clone(&self) -> Self {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_init_set(res.as_mut_ptr(), self.as_ptr()); }
        res
    }
}

impl fmt::Debug for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Integer")
            .field("data", &self.data)
            .finish()
    }
}

impl Default for Integer {
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz::fmpz_init(z.as_mut_ptr());
            Integer { data: IntegerData { elem: z.assume_init() } }
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Hash for Integer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_ui_vector().hash(state);
    }
}

