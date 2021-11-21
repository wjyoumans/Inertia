
use flint_sys::fmpz::fmpz;
use libc::c_int;

use crate::traits::Element;

// IntegerRing //

#[derive(Default, Debug, Hash, Clone, Copy)]
pub struct IntegerRing {}

impl IntegerRing {
    pub fn init() -> Self {
        IntegerRing {}
    }
    
    pub fn new<T: Into<Integer>>(&self, x: T) -> Integer {
        x.into()
    }
}

// Integer //

#[derive(Debug)]
#[repr(transparent)]
pub struct Integer {
    pub data: <Self as Element>::Data,
}

impl Integer {
    #[inline]
    pub fn as_ptr(&self) -> &fmpz {
        &self.data
    }
    
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpz {
        &mut self.data
    }
    
    /// Convert the `Integer` to a string in base `base`.
    ///
    /// ```
    /// use inertia::integer::src::Integer;
    ///
    /// let x = Integer::from(1024);
    /// assert_eq!(x.to_str_radix(2), "10000000000")
    /// ```
    pub fn to_str_radix(&self, base: u8) -> String {
        unsafe {
            // Extra two bytes are for possible minus sign and null terminator
            let len = flint_sys::fmpz::fmpz_sizeinbase(self.as_ptr(), base as c_int) as usize + 2;

            // Allocate and write into a raw *c_char of the correct length
            let mut vector: Vec<u8> = Vec::with_capacity(len);
            vector.set_len(len);

            flint_sys::fmpz::fmpz_get_str(vector.as_mut_ptr() as *mut _, base as c_int, self.as_ptr());

            let mut first_nul = None;
            let mut index : usize = 0;
            for elem in &vector {
                if *elem == 0 {
                    first_nul = Some(index);
                    break;
                }
                index += 1;
            }
            let first_nul = first_nul.unwrap_or(len);

            vector.truncate(first_nul);
            match String::from_utf8(vector) {
                Ok(s)  => s,
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe { flint_sys::fmpz::fmpz_is_zero(self.as_ptr()) == 1 }
    }
    
    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe {
            flint_sys::fmpz::fmpz_is_one(self.as_ptr()) == 1
        }
    }
    
    /// Returns -1 if the `Integer` is negative, +1 if the `Integer` is positive, and 0 otherwise.
    ///
    /// ```
    /// use inertia::integer::src::Integer;
    ///
    /// let z = Integer::from(-12);
    /// assert_eq!(z.sign(), -1);
    ///
    /// let z = Integer::from(0);
    /// assert_eq!(z.sign(), 0);
    ///
    /// let z = Integer::from(12);
    /// assert_eq!(z.sign(), 1);
    /// ```
    #[inline]
    pub fn sign(&self) -> c_int {
        unsafe {
            flint_sys::fmpz::fmpz_sgn(self.as_ptr())
        }
    }

    /// Returns the absolute value of an `Integer`.
    ///
    /// ```
    /// use inertia::integer::src::Integer;
    ///
    /// let z = Integer::from(-99);
    /// assert_eq!(z.abs(), Integer::from(99));
    /// ```
    #[inline]
    pub fn abs(&self) -> Integer {
        unsafe {
            let mut res = Integer::default();
            flint_sys::fmpz::fmpz_abs(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }
}
