
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
