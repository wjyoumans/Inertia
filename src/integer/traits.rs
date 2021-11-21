
use std::fmt::{self, Debug};
use std::mem::MaybeUninit;

use flint_sys::fmpz::fmpz as fmpz;

use crate::traits::*;
use crate::integer::src::{Integer, IntegerRing};

// IntegerRing //

impl Parent for IntegerRing {
    type Data = ();
    type Element = Integer;
}

// Integer //

impl Element for Integer {
    type Data = fmpz;
    type Parent = IntegerRing;
}

impl Clone for Integer {
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe { 
            flint_sys::fmpz::fmpz_init_set(z.as_mut_ptr(), &self.data); 
            Integer { data: z.assume_init() }
        }
    }
}

/*
impl Debug for Integer {

}*/

impl Default for Integer {
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz::fmpz_init(z.as_mut_ptr());
            Integer { data: z.assume_init() }
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for Integer {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpz::fmpz_clear(self.as_mut_ptr());}
    }
}

impl Eq for Integer {}
impl PartialEq for Integer {
    fn eq(&self, rhs: &Integer) -> bool {
        unsafe { flint_sys::fmpz::fmpz_equal(self.as_ptr(), rhs.as_ptr()) == 1}
    }
}

macro_rules! impl_eq {
    ($func:path; $cast:ident {$($t:ty)*}) => ($(
        impl PartialEq<$t> for Integer {
            fn eq(&self, rhs: &$t) -> bool {
                unsafe { $func(self.as_ptr(), *rhs as $cast) == 1}
            }
        }
        impl PartialEq<Integer> for $t {
            fn eq(&self, rhs: &Integer) -> bool {
                unsafe { $func(rhs.as_ptr(), *self as $cast) == 1}
            }
        }
    )*)
}

impl_eq! { flint_sys::fmpz::fmpz_equal_ui; u64 { usize u64 u32 u16 u8 }}
impl_eq! { flint_sys::fmpz::fmpz_equal_si; i64 { isize i64 i32 i16 i8 }}

// Hash
