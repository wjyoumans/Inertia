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

use std::fmt::Debug;
use std::mem::{self, MaybeUninit};

use flint_sys::fmpz::fmpz as fmpq;

use crate::traits::*;
use crate::integer::src::Integer;
use crate::rational::src::{Rational, RationalField};

// RationalField //

impl Parent for RationalField {
    type Data = ();
    type Element = Rational;
}

// Integer //

impl Element for Rational {
    type Data = fmpq;
    type Parent = RationalField;
}

impl Clone for Rational {
    fn clone(&self) -> Self {
        let mut z = Rational::default();
        unsafe {
            flint_sys::fmpq::fmpq_set(z.as_mut_ptr(), &self.data); 
        }
        z
    }
}

/*
impl Debug for Integer {

}*/

impl Default for Rational {
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpq::fmpq_init(z.as_mut_ptr());
            Rational { data: z.assume_init() }
        }
    }
}


impl Drop for Rational {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpq::fmpq_clear(self.as_mut_ptr());}
    }
}


impl Eq for Rational {}
impl PartialEq for Rational {
    fn eq(&self, rhs: &Rational) -> bool {
        unsafe { flint_sys::fmpq::fmpq_equal(self.as_ptr(), rhs.as_ptr()) == 1}
    }
}

// Hash
