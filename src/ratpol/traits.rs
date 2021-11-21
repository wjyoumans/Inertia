
use std::fmt::{self, Debug};
use std::mem::{self, MaybeUninit};

use flint_sys::fmpq_poly::fmpq_poly_struct;
use rug::Assign;

use crate::traits::*;
use crate::intpol::src::IntPol;
use crate::ratpol::src::{RatPol, RatPolRing};

// RatPolRing //

impl Parent for RatPolRing {
    type Data = ();
    type Element = RatPol;
}

// RatPol //

impl Element for RatPol {
    type Data = fmpq_poly_struct;
    type Parent = RatPolRing;
}

impl Clone for RatPol {
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe { 
            flint_sys::fmpq_poly::fmpq_poly_init(z.as_mut_ptr());
            flint_sys::fmpq_poly::fmpq_poly_set(z.as_mut_ptr(), &self.data); 
            RatPol { data: z.assume_init() }
        }
    }
}

impl Default for RatPol {
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpq_poly::fmpq_poly_init(z.as_mut_ptr());
            RatPol { data: z.assume_init() }
        }
    }
}

impl fmt::Display for RatPol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for RatPol {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpq_poly::fmpq_poly_clear(self.as_mut_ptr());}
    }
}

impl Eq for RatPol {}
impl PartialEq for RatPol {
    fn eq(&self, rhs: &RatPol) -> bool {
        unsafe { flint_sys::fmpq_poly::fmpq_poly_equal(self.as_ptr(), rhs.as_ptr()) == 1}
    }
}

// Hash

impl Assign for RatPol {
    #[inline]
    fn assign(&mut self, src: RatPol) {
        drop(mem::replace(self, src));
    }
}

impl Assign<&RatPol> for RatPol {
    #[inline]
    fn assign(&mut self, src: &RatPol) {
        unsafe { flint_sys::fmpq_poly::fmpq_poly_set(self.as_mut_ptr(), src.as_ptr()); }
    }
}


