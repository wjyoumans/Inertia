
use flint_sys::fmpz::fmpz;
use flint_sys::fmpq::fmpq;

use crate::integer::src::Integer;

// RationalField //

#[derive(Default, Debug, Hash, Clone, Copy)]
pub struct RationalField {}

impl RationalField {
    pub fn init() -> Self {
        RationalField {}
    }
    
    pub fn new<T: Into<Integer>>(&self, n: T, d: T) -> Rational {
        let mut z = Rational::default();
        unsafe {
            flint_sys::fmpq::fmpq_set_fmpz_frac(z.as_mut_ptr(), n.into().as_ptr(), d.into().as_ptr());
        }
        z
    }
}

#[repr(transparent)]
pub struct Rational {
    pub data: fmpq,
}

impl Rational {
    #[inline]
    pub fn as_ptr(&self) -> &fmpq {
        &self.data
    }
   
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpq {
        &mut self.data
    }

    #[inline]
    pub fn numerator(&self) -> Integer {
        Integer {
            data: self.data.num
        }
    }
    
    #[inline]
    pub fn denominator(&self) -> Integer {
        Integer {
            data: self.data.num
        }
    }
}
