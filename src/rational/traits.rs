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

use flint_sys::fmpz::fmpz as fmpq;

use crate::traits::*;
use crate::product::src::Product;
use crate::integer::src::Integer;
use crate::rational::src::{Rational, RationalField};

// RationalField //

impl Parent for RationalField {
    type Data = ();
    type Element = Rational;
}

// Integer //

impl Element for Rational {
    type Data = ();
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

impl Default for Rational {
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpq::fmpq_init(z.as_mut_ptr());
            Rational { data: z.assume_init() }
        }
    }
}


impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for Rational {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpq::fmpq_clear(self.as_mut_ptr());}
    }
}

impl Hash for Rational {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.numerator().hash(state);
        self.denominator().hash(state);
    }
}

impl Factorizable for Rational {
    type Output = Product<Integer>;
    fn factor(&self) -> Self::Output {
        assert!(self != &0);
        
        if self == &1 {
            Product::from(Integer::from(1))
        } else { 
            self.numerator().factor() * self.denominator().factor().inv()
        }
    }
}
