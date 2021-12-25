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


use std::mem::MaybeUninit;
use std::ops::Rem;

use flint_sys::flint::{flint_rand_s, flint_bitcnt_t};
use flint_sys::fmpz::fmpz;
use libc::{c_int, c_long, c_ulong};
use num_traits::Zero;
use rug::ops::Pow;
use rustc_hash::FxHashMap;

use crate::*;


/// An integer ring that can be used as an [Integer] "factory".
#[derive(Default, Debug, Hash, Clone, Copy)]
pub struct IntegerRing {}

impl Parent for IntegerRing {
    type Element = Integer;

    #[inline]
    fn default(&self) -> Integer {
        Integer::default()
    }
}

impl Additive for IntegerRing {
    #[inline]
    fn zero(&self) -> Integer {
        Integer::default()
    }
}

impl Multiplicative for IntegerRing {
    #[inline]
    fn one(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_one(res.as_mut_ptr()); }
        res
    }
}

impl AdditiveGroup for IntegerRing {}

impl Ring for IntegerRing {}

impl Init for IntegerRing {
    #[inline]
    fn init() -> Self {
        IntegerRing {}
    }
}

impl New<&Integer> for IntegerRing {
    #[inline]
    fn new(&self, x: &Integer) -> Integer {
        x.clone()
    }
}

impl<T> New<T> for IntegerRing where 
    T: Into<Integer>
{
    #[inline]
    fn new(&self, x: T) -> Integer {
        x.into()
    }
}

/// An arbitrary precision integer. The field `data` is a FLINT [fmpz][flint_sys::fmpz::fmpz].
pub type Integer = Elem<IntegerRing>;

#[derive(Debug)]
pub struct IntegerData {
    pub elem: fmpz,
}

impl Drop for IntegerData {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpz::fmpz_clear(&mut self.elem);}
    }
}

impl Element for Integer {
    type Data = IntegerData;
    type Parent = IntegerRing;

    #[inline]
    fn parent(&self) -> IntegerRing {
        IntegerRing {}
    }
}

impl AdditiveElement for Integer {
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe { flint_sys::fmpz::fmpz_is_zero(self.as_ptr()) == 1 }
    }
}

impl MultiplicativeElement for Integer {
    #[inline]
    fn is_one(&self) -> bool {
        unsafe { flint_sys::fmpz::fmpz_is_one(self.as_ptr()) == 1 }
    }
}

impl AdditiveGroupElement for Integer {}

impl RingElement for Integer {}

impl Integer {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fmpz {
        &self.data.elem
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpz {
        &mut self.data.elem
    }

    /// Convert an [Integer] to a string in base `base`.
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

    /// Return true if an [Integer] is even, false otherwise.
    #[inline]
    pub fn is_even(&self) -> bool {
        unsafe {flint_sys::fmpz::fmpz_is_even(self.as_ptr()) == 1}
    }
    
    /// Return true if an [Integer] is odd, false otherwise.
    #[inline]
    pub fn is_odd(&self) -> bool {
        unsafe {flint_sys::fmpz::fmpz_is_odd(self.as_ptr()) == 1}
    }
    
    /// Returns -1 an [Integer] is negative, +1 if it is positive, and 0 otherwise.
    #[inline]
    pub fn sign(&self) -> i32 {
        unsafe {
            flint_sys::fmpz::fmpz_sgn(self.as_ptr())
        }
    }

    /// Returns the absolute value of an [Integer].
    #[inline]
    pub fn abs(&self) -> Integer {
        unsafe {
            let mut res = Integer::default();
            flint_sys::fmpz::fmpz_abs(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }
   
    /// Determines the size of the absolute value of an [Integer] in base `base` in terms of number
    /// of digits. The base can be between 2 and 62, inclusive.
    #[inline]
    pub fn sizeinbase(&self, base: u8) -> usize {
        unsafe { flint_sys::fmpz::fmpz_sizeinbase(self.as_ptr(), base as i32) as usize }
    }
   
    /// Returns the number of limbs required to store the absolute value of an [Integer]. Returns
    /// zero if the [Integer] is zero.
    #[inline]
    pub fn size(&self) -> c_long {
        unsafe { flint_sys::fmpz::fmpz_size(self.as_ptr()) }
    }
   
    /// Returns the number of bits required to store the absolute value of an [Integer]. Returns zero
    /// if the [Integer] is zero.
    #[inline]
    pub fn bits(&self) -> c_ulong {
        unsafe { flint_sys::fmpz::fmpz_bits(self.as_ptr()) }
    }
   
    /// Determine if the [Integer] fits in a signed long.
    #[inline]
    pub fn fits_si(&self) -> bool {
        unsafe { flint_sys::fmpz::fmpz_fits_si(self.as_ptr()) == 1 }
    }
    
    /// Determine if the absolute value of an [Integer] fits in an unsigned long.
    #[inline]
    pub fn abs_fits_ui(&self) -> bool {
        unsafe { flint_sys::fmpz::fmpz_abs_fits_ui(self.as_ptr()) == 1 }
    }
   
    /// If the input [Integer] fits in an signed long we return it in an [Option].
    #[inline]
    pub fn get_si(&self) -> Option<c_long> {
        if self.fits_si() {
            unsafe { 
                Some(flint_sys::fmpz::fmpz_get_si(self.as_ptr()))
            }
        } else {
            None
        }
    }

    /// If the input [Integer] fits in an unsigned long we return it in an [Option].
    #[inline]
    pub fn get_ui(&self) -> Option<c_ulong> {
        if self.sign() < 0 {
            return None;
        }
        if self.abs_fits_ui() {
            unsafe { 
                Some(flint_sys::fmpz::fmpz_get_ui(self.as_ptr())) 
            }
        } else {
            None
        }
    }

    /// Return a vector `A` of unsigned longs such that the original [Integer] can be written as 
    /// `a[0] + a[1]*x + ... + a[n-1]*x^(n-1)` where `x = 2^FLINT_BITS`.
    #[inline]
    pub fn get_ui_vector(&self) -> Vec<c_ulong> {
        assert!(self > &0);

        let n = self.size();
        let mut out = Vec::<c_ulong>::with_capacity(n as usize);
        unsafe {
            flint_sys::fmpz::fmpz_get_ui_array(out.as_mut_ptr(), n, self.as_ptr());
            out.set_len(n as usize);
        }
        out
    }

    /// Set `self` to the nonnegative [Integer] `vec[0] + vec[1]*x + ... + vec[n-1]*x^(n-1)` where `x =
    /// 2^FLINT_BITS`.
    #[inline]
    pub fn set_ui_vector(&mut self, vec: Vec<c_ulong>) {
        unsafe {
            flint_sys::fmpz::fmpz_set_ui_array(self.as_mut_ptr(), vec.as_ptr(), vec.len() as c_long);
        }
    }

    /// Sets the bit index `bit_index` of an [Integer].
    #[inline]
    pub fn setbit(&mut self, bit_index: usize) {
        unsafe { flint_sys::fmpz::fmpz_setbit(self.as_mut_ptr(), bit_index as c_ulong) }
    }

    /// Test the bit index `bit_index` of an [Integer]. Return `true` if it is 1, `false` if it is
    /// zero.
    #[inline]
    pub fn testbit(&self, bit_index: usize) -> bool {
        unsafe { flint_sys::fmpz::fmpz_tstbit(self.as_ptr(), bit_index as c_ulong) == 1 }
    }

    // TODO: All Rand functions need work.

    /// Not implemented.
    #[inline]
    pub fn rand_bits(st: flint_rand_s, bt: flint_bitcnt_t) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randbits(res.as_mut_ptr(), &st, bt);}
        res
    }
    
    /// Not implemented.
    #[inline]
    pub fn rand_max_bits(st: flint_rand_s, bt: flint_bitcnt_t) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randtest(res.as_mut_ptr(), &st, bt);}
        res
    }
    
    /// Not implemented.
    #[inline]
    pub fn rand_max_bits_ui(st: flint_rand_s, bt: flint_bitcnt_t) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randtest_unsigned(res.as_mut_ptr(), &st, bt);}
        res
    }
    
    /// Not implemented.
    #[inline]
    pub fn rand_max_bits_non_zero(st: flint_rand_s, bt: flint_bitcnt_t) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randtest_not_zero(res.as_mut_ptr(), &st, bt);}
        res
    }
    
    /// Not implemented.
    #[inline]
    pub fn rand(st: flint_rand_s, m: &Integer) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randm(res.as_mut_ptr(), &st, m.as_ptr());}
        res
    }
    
    /// Not implemented.
    #[inline]
    pub fn rand_mod(st: flint_rand_s, m: &Integer) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randtest_mod(res.as_mut_ptr(), &st, m.as_ptr());}
        res
    }
    
    /// Not implemented.
    #[inline]
    pub fn rand_mod_si(st: flint_rand_s, m: &Integer) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randtest_mod_signed(res.as_mut_ptr(), &st, m.as_ptr());}
        res
    }
    
    /// Not implemented.
    #[inline]
    pub fn rand_prime(st: flint_rand_s, bt: flint_bitcnt_t) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randprime(res.as_mut_ptr(), &st, bt, 1);}
        res
    }

    /// Return the quotient `self/other` rounded up towards infinity.
    #[inline]
    pub fn cdiv(&self, other: &Integer) -> Integer {
        assert!(!other.is_zero());
        unsafe {
            let mut res = Integer::default();
            flint_sys::fmpz::fmpz_cdiv_q(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            res
        }
    }
    
    /// Return the quotient `self/other` rounded down towards negative infinity.
    #[inline]
    pub fn fdiv(&self, other: &Integer) -> Integer {
        assert!(!other.is_zero());
        unsafe {
            let mut res = Integer::default();
            flint_sys::fmpz::fmpz_fdiv_q(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            res
        }
    }
    
    /// Return the quotient `self/other` rounded to the nearest [Integer].
    #[inline]
    pub fn tdiv(&self, other: &Integer) -> Integer {
        assert!(!other.is_zero());
        unsafe {
            let mut res = Integer::default();
            flint_sys::fmpz::fmpz_tdiv_q(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            res
        }
    }
    
    /// Return the quotient `self/other` rounded up towards infinity as well as the remainder.
    #[inline]
    pub fn cdivrem(&self, other: &Integer) -> (Integer, Integer) {
        assert!(!other.is_zero());
        unsafe {
            let mut q = Integer::default();
            let mut r = Integer::default();
            flint_sys::fmpz::fmpz_cdiv_qr(
                q.as_mut_ptr(), 
                r.as_mut_ptr(), 
                self.as_ptr(),
                other.as_ptr()
            );
            (q, r)
        }
    }
    
    /// Return the quotient `self/other` rounded down towards negative infinity as well as the 
    /// remainder.
    #[inline]
    pub fn fdivrem(&self, other: &Integer) -> (Integer, Integer) {
        assert!(!other.is_zero());
        unsafe {
            let mut q = Integer::default();
            let mut r = Integer::default();
            flint_sys::fmpz::fmpz_fdiv_qr(
                q.as_mut_ptr(), 
                r.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            );
            (q, r)
        }
    }
    
    /// Return the quotient `self/other` rounded towards zero as well as the remainder.
    #[inline]
    pub fn divrem(&self, other: &Integer) -> (Integer, Integer) {
        assert!(!other.is_zero());
        unsafe {
            let mut q = Integer::default();
            let mut r = Integer::default();
            flint_sys::fmpz::fmpz_tdiv_qr(
                q.as_mut_ptr(), 
                r.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            );
            (q, r)
        }
    }
   
    /// Exact division of `self/other`. If the division is not exact the output [Result] will be an
    /// [Err].
    #[inline]
    pub fn divexact(&self, other: &Integer) -> Result<Integer, ()> {
        assert!(!other.is_zero());
        if self.rem(other) != 0 {
            Err(())
        } else {
            let mut res = Integer::default();
            unsafe { flint_sys::fmpz::fmpz_divexact(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());}
            Ok(res)
        }
    }
    
    /// Exact division of `self/other`. If the division is not exact the output [Result] will be an
    /// [Err].
    #[inline]
    pub fn divexact_ui(&self, other: c_ulong) -> Result<Integer, ()> {
        assert!(!other.is_zero());
        if self.rem(other) != 0 {
            Err(())
        } else {
            let mut res = Integer::default();
            unsafe { flint_sys::fmpz::fmpz_divexact_ui(res.as_mut_ptr(), self.as_ptr(), other);}
            Ok(res)
        }
    }
    
    /// Exact division of `self/other`. If the division is not exact the output [Result] will be an
    /// [Err].
    #[inline]
    pub fn divexact_si(&self, other: c_long) -> Result<Integer, ()> {
        assert!(!other.is_zero());
        if self.rem(other) != 0 {
            Err(())
        } else {
            let mut res = Integer::default();
            unsafe { flint_sys::fmpz::fmpz_divexact_si(res.as_mut_ptr(), self.as_ptr(), other);}
            Ok(res)
        }
    }
   
    /// The symmetric remainder of an [Integer] modulo `n` will be in the range 
    /// `[-(n-1)/2, ..., (n-1)/2]` symmetric around zero.
    #[inline]
    pub fn srem(&self, modulus: &Integer) -> Integer {
        assert!(modulus > &0);
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_smod(res.as_mut_ptr(), self.as_ptr(), modulus.as_ptr());
        }
        res
    }
   
    /// Raises an [Integer] to the power `exp` modulo `modulo`. If the exponent is negative and no
    /// inverse exists then the output [Result] will be an [Err].
    #[inline]
    pub fn powm(&self, exp: &Integer, modulus: &Integer) -> Result<Integer, ()> {
        assert!(modulus > &0);
        if exp < &0 && !self.is_coprime(modulus) {
            Err(())
        } else {
            let mut res = Integer::default();
            unsafe {
                flint_sys::fmpz::fmpz_powm(
                    res.as_mut_ptr(), 
                    self.as_ptr(), 
                    exp.as_ptr(), 
                    modulus.as_ptr()
                );
            }
            Ok(res)
        }
    }

    /// Raises an [Integer] to the power `exp` modulo `modulo`, assigning it to the input. If 
    /// the exponent is negative and no inverse exists then the output [Result] will be an [Err].
    #[inline]
    pub fn powm_assign(&mut self, exp: &Integer, modulus: &Integer) -> Result<(), ()> {
        assert!(modulus > &0);
        if exp < &0 && !self.is_coprime(modulus) {
            Err(())
        } else {
            unsafe {
                flint_sys::fmpz::fmpz_powm(
                    self.as_mut_ptr(), 
                    self.as_ptr(), 
                    exp.as_ptr(), 
                    modulus.as_ptr()
                );
            }
            Ok(())
        }
    }
    
    /// Raises an [Integer] to an unsigned integer `exp` modulo `modulo`.
    #[inline]
    pub fn powm_ui<T>(&self, exp: T, modulus: &Integer) -> Integer where
        T: Into<c_ulong>
    {
        assert!(modulus > &0);
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_powm_ui(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                exp.into(), 
                modulus.as_ptr()
            );
        }
        res
    }
    
    /// Raises an [Integer] to an unsigned integer `exp` modulo `modulo`, assigning the result to the 
    /// input.
    #[inline]
    pub fn powm_ui_assign<T>(&mut self, exp: T, modulus: &Integer) where
        T: Into<c_ulong>
    {
        assert!(modulus > &0);
        unsafe {
            flint_sys::fmpz::fmpz_powm_ui(
                self.as_mut_ptr(), 
                self.as_ptr(), 
                exp.into(), 
                modulus.as_ptr()
            );
        }
    }
    
    /// Determine whether `self` divides `other`.
    #[inline]
    pub fn divides(&self, other: &Integer) -> bool {
        unsafe { flint_sys::fmpz::fmpz_divisible(other.as_ptr(), self.as_ptr()) == 1 }
    }

    /// Compute the natural logarithm of an [Integer] as a double precision float. If the input 
    /// is less than or equal to zero the [Result] will be an [Err]. (For logarithms of negative 
    /// integers use (the Complex/arb crate, not yet complete.) 
    #[inline]
    pub fn log(&self) -> Result<f64, ()> {
        if self <= &0 {
            Err(())
        } else {
            unsafe { 
                Ok(flint_sys::fmpz::fmpz_dlog(self.as_ptr()))
            }
        }
    }

    /// Return the logarithm of an [Integer] at base `base` rounded up towards infinity. Requires
    /// `self >= 1`, `base >= 2` and that the output will fit in a signed long.
    #[inline]
    pub fn clog(&self, base: &Integer) -> Result<c_long, ()> {
        if self < &1 {
            Err(())
        } else {
            unsafe { 
                Ok(flint_sys::fmpz::fmpz_clog(self.as_ptr(), base.as_ptr()))
            }
        }
    }

    /// Return the logarithm of an [Integer] at base `base` rounded up towards infinity. Requires
    /// `self >= 1`, `base >= 2` and that the output will fit in a signed long.
    #[inline]
    pub fn clog_ui<T: Into<c_ulong>>(&self, base: T) -> c_long {
        let base = base.into();
        unsafe { flint_sys::fmpz::fmpz_clog_ui(self.as_ptr(), base)}
    }
    
    /// Return the logarithm of an [Integer] at base `base` rounded down towards negative infinity. 
    /// Requires `self >= 1`, `base >= 2` and that the output will fit in a signed long.
    #[inline]
    pub fn flog(&self, base: &Integer) -> c_long {
        unsafe { flint_sys::fmpz::fmpz_flog(self.as_ptr(), base.as_ptr())}
    }

    /// Return the logarithm of an [Integer] at base `base` rounded down towards negative infinity. 
    /// Requires `self >= 1`, `base >= 2` and that the output will fit in a signed long.
    #[inline]
    pub fn flog_ui<T: Into<c_ulong>>(&self, base: T) -> c_long {
        let base = base.into();
        unsafe { flint_sys::fmpz::fmpz_flog_ui(self.as_ptr(), base)}
    }

    /// Return the square root of an [Integer] modulo `n` if it exists.
    #[inline]
    pub fn sqrtmod(&self, n: &Integer) -> Result<Integer, ()> {
        let mut res = Integer::default();
        unsafe { 
            let r = flint_sys::fmpz::fmpz_sqrtmod(res.as_mut_ptr(), self.as_ptr(), n.as_ptr());
      
            if r == 0 {
                Err(())
            } else {
                Ok(res)
            }
        }
    }

    /// Return the integer part `a` of the square root of an [Integer] and it's remainder `b`, that
    /// is, the difference `self - b^2`. Returns an [Err] if the input is negative. 
    #[inline]
    pub fn sqrtrem(&self) -> Result<(Integer, Integer), ()> {
        if self < &0 {
            Err(())
        } else {
            let mut s = Integer::default();
            let mut r = Integer::default();
            unsafe { 
                flint_sys::fmpz::fmpz_sqrtrem(s.as_mut_ptr(), r.as_mut_ptr(), self.as_ptr());
            }
            Ok((s, r))
        }
    }
   
    /// Return true if the [Integer] is a square.
    #[inline]
    pub fn is_square(&self) -> bool {
        unsafe { flint_sys::fmpz::fmpz_is_square(self.as_ptr()) != 0}
    }

    /// Return the integer part of the square root of an [Integer]. Returns an [Err] if the input
    /// is negative.
    #[inline]
    pub fn sqrt(&self) -> Result<Integer, ()> {
        if self < &0 {
            Err(())
        } else {
            let mut res = Integer::default();
            unsafe { flint_sys::fmpz::fmpz_sqrt(res.as_mut_ptr(), self.as_ptr());}
            Ok(res)
        }
    }

    /// Return the integer part of the n-th root of an [Integer]. Requires `n > 0` and that if `n`
    /// is even then the input is nonnegative, otherwise an [Err] is returned.
    #[inline]
    pub fn root<T: Into<c_long>>(&self, n: T) -> Result<Integer, ()> {
        let n = n.into();
        
        if n < 1 || (Integer::from(n).is_even() && self < &0) {
            Err(())
        } else {
            let mut res = Integer::default();
            unsafe { flint_sys::fmpz::fmpz_root(res.as_mut_ptr(), self.as_ptr(), n);}
            Ok(res)
        }
    }
  
    /// If the input [Integer] is a perfect power then return the root and exponent, otherwise 
    /// return an [Err].
    #[inline]
    pub fn perfect_power(&self) -> Result<(Integer, c_int), ()> {
        let mut res = Integer::default();
        unsafe { 
            let k = flint_sys::fmpz::fmpz_is_perfect_power(res.as_mut_ptr(), self.as_ptr());

            if k != 0 {
                Ok((res, k))
            } else {
                Err(())
            }
        }
    }
   
    /// Return the n-th Fibonacci number.
    #[inline]
    pub fn fibonacci(n: c_ulong) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_fib_ui(res.as_mut_ptr(), n);}
        res
    }
    
    /// Return the binomial coefficient n choose k.
    #[inline]
    pub fn binomial(n: c_ulong, k: c_ulong) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_bin_uiui(res.as_mut_ptr(), n, k);}
        res
    }
   
    /// Return the factorial of an [Integer].
    #[inline]
    pub fn factorial(&self) -> Integer {
        assert!(self.abs_fits_ui());
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_fac_ui(res.as_mut_ptr(), self.get_ui().unwrap());}
        res
    }

    /// Return the rising factorial `x(x+1)(x+2)...(x+k-1)`.
    #[inline]
    pub fn rising_factorial(&self, k: c_ulong) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_rfac_ui(res.as_mut_ptr(), self.as_ptr(), k);}
        res
    }

    /// Return the greatest common divisor of two integers.
    #[inline]
    pub fn gcd(&self, other: &Integer) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_gcd(res.as_mut_ptr(), self.as_ptr(), other.as_ptr()); }
        res
    }
    
    /// Return true if two integers are coprime, false otherwise.
    #[inline]
    pub fn is_coprime(&self, other: &Integer) -> bool {
        self.gcd(other) == 1
    }

    /// Return the least common multiple of two integers.
    #[inline]
    pub fn lcm(&self, other: &Integer) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_lcm(res.as_mut_ptr(), self.as_ptr(), other.as_ptr()); }
        res
    }

    /// Compute the extended GCD of two integers. Call the input integers `f` and `g`. Then we return
    /// `(d, a, b)` where `d = gcd(f, g)` and `a*f + b*g = d`.
    #[inline]
    pub fn xgcd(&self, other: &Integer) -> (Integer, Integer, Integer) {
        unsafe {
            let mut d = Integer::default();
            let mut a = Integer::default();
            let mut b = Integer::default();
            flint_sys::fmpz::fmpz_xgcd(
                d.as_mut_ptr(), 
                a.as_mut_ptr(), 
                b.as_mut_ptr(),
                self.as_ptr(), 
                other.as_ptr());
            (d, a, b)
        }
    } 
   
    /// Attempt to reconstruct a [Rational] from it's residue mod `m`. This is just
    /// [rational_reconstruction2][crate::integer::src::Integer::rational_reconstruction2] with the 
    /// numerator and denominator bounds `n == d == floor(sqrt((m-1)/2))`. If a solution with these 
    /// constraints exists then it is unique.
    #[inline]
    pub fn rational_reconstruction(&self, m: &Integer) -> Result<Rational,()> {
        let mut res = Rational::default();
        unsafe {
            let b = flint_sys::fmpq::fmpq_reconstruct_fmpz(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                m.as_ptr()
            );
            if b == 0 {
                Err(())
            } else {
                Ok(res)
            }
        }
    }
    
    /// Given bounds `n` and `d` satisfying `2*n*d < m`, attempt to reconstruct a [Rational] from it's 
    /// residue mod `m` with numerator and denominator absolutely bounded by `n` and `d`
    /// respectively. We also require `gcd(n, d) = 1` and `n = a*d % m`. If a solution exists then
    /// it is unique.
    #[inline]
    pub fn rational_reconstruction2(&self, m: &Integer, n: &Integer, d: &Integer) 
        -> Result<Rational, ()> 
    {
        let mut res = Rational::default();
        unsafe {
            let b = flint_sys::fmpq::fmpq_reconstruct_fmpz_2(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                m.as_ptr(),
                n.as_ptr(), 
                d.as_ptr()
            );
            if b == 0 {
                Err(())
            } else {
                Ok(res)
            }
        }
    }

    /// Attempt to invert an [Integer] modulo `modulus`.
    #[inline]
    pub fn invmod(&self, modulus: &Integer) -> Result<Integer, ()> {
        assert!(modulus > &0);

        let mut res = Integer::default();
        unsafe{ 
            let r = flint_sys::fmpz::fmpz_invmod(res.as_mut_ptr(), self.as_ptr(), modulus.as_ptr());
        
            if r == 0 {
                Err(())
            } else {
                Ok(res)
            }
        }
    }
    
    /// Remove all occurences of the factor `factor` from an [Integer].
    #[inline]
    pub fn remove(&mut self, factor: &Integer) {
        assert!(factor > &1);
        unsafe {
            flint_sys::fmpz::fmpz_remove(self.as_mut_ptr(), self.as_ptr(), factor.as_ptr());
        }
    }

    /// Negate an [Integer] modulo `modulo`.
    #[inline]
    pub fn negmod(&self, modulus: &Integer) -> Integer {
        assert!(!modulus.is_zero());
        if self > modulus {
            let mut res = self.rem(modulus);
            unsafe {
                flint_sys::fmpz::fmpz_negmod(res.as_mut_ptr(), res.as_ptr(), modulus.as_ptr());
            }
            res
        } else {
            let mut res = Integer::default();
            unsafe {
                flint_sys::fmpz::fmpz_negmod(res.as_mut_ptr(), self.as_ptr(), modulus.as_ptr());
            }
            res
        }
    }

    /// Compute the jacobi symbol `(a/n)` for any `a` and odd positive `n`.
    #[inline]
    pub fn jacobi(&self, n: &Integer) -> c_int {
        assert!(n > &0 && n.is_odd());
        unsafe { flint_sys::fmpz::fmpz_jacobi(self.as_ptr(), n.as_ptr()) }
    }
    
    /// Compute the kronecker symbol `(a/n)` for any `a` and any `n`.
    #[inline]
    pub fn kronecker(&self, n: &Integer) -> c_int {
        unsafe { flint_sys::fmpz::fmpz_kronecker(self.as_ptr(), n.as_ptr()) }
    }

    // TODO: BIT PACKING
   
    /// Set the i-th bit of an [Integer] to zero.
    #[inline]
    pub fn clear_bit(&mut self, i: c_ulong) {
        unsafe { flint_sys::fmpz::fmpz_clrbit(self.as_mut_ptr(), i);}
    }
    
    /// Complement the i-th bit of an [Integer].
    #[inline]
    pub fn complement_bit(&mut self, i: c_ulong) {
        unsafe { flint_sys::fmpz::fmpz_combit(self.as_mut_ptr(), i);}
    }

    /// Use the Chinese Remainder Theorem to return the unique value `0 <= x < M` congruent to `r1`
    /// modulo `m1` and `r2` modulo `m2` where `M = m1 * m2`. We require that the moduli are
    /// greater than one and coprime and `0 <= r1 < m1`, `0 <= r2 < m2`.
    #[inline]
    pub fn crt(r1: &Integer, m1: &Integer, r2: &Integer, m2: &Integer) -> Integer {
        assert!(m1 > &1 && m2 > &1);
        assert!(r1 >= &0 && r2 >= &0);
        assert!(m1 > r1 && m2 > r2);
        assert!(m1.is_coprime(m2));

        let mut res = Integer::default();
        unsafe { 
            flint_sys::fmpz::fmpz_CRT(
                res.as_mut_ptr(), 
                r1.as_ptr(), 
                m1.as_ptr(),
                r2.as_ptr(), 
                m2.as_ptr(),
                0
            );
        }
        res
    }
    
    /// Use the Chinese Remainder Theorem to compute the unique [Integer] that is congruent to `r[i]`
    /// modulo `m[i]` for all `i`. This uses the same assumptions as
    /// [crt][crate::integer::src::Integer::crt], also requiring the inputs to have the same length.
    #[inline]
    pub fn multi_crt(r: &[Integer], m: &[Integer]) -> Integer {
        assert!(r.len() == m.len());
        let mut res = Integer::default(); 
       
        let len = r.len();
        let vr: Vec<flint_sys::fmpz::fmpz> = r.iter().map(|x| x.as_ptr().clone()).collect();
        let vm: Vec<flint_sys::fmpz::fmpz> = m.iter().map(|x| x.as_ptr().clone()).collect();

        unsafe { 
            let b = flint_sys::fmpz::fmpz_multi_crt(
                res.as_mut_ptr(), 
                vm.as_ptr(), 
                vr.as_ptr(),
                len as c_long
            );
            assert!(b == 1, "The CRT assumptions were not satisfied and the output is undefined.");
        }
        res
    }

    // PRIMALITY TESTING
    // TODO: probable prime tests?

    /// Returns true if the [Integer] is a prime.
    #[inline]
    pub fn is_prime(&self) -> bool {
        unsafe {
            flint_sys::fmpz::fmpz_is_prime(self.as_ptr()) == 1
        }
    }

    // guaranteed prime
    /// Returns the next prime greater than the input.
    #[inline]
    pub fn next_prime(&self) -> Integer {
        unsafe {
            let mut res = Integer::default();
            flint_sys::fmpz::fmpz_nextprime(res.as_mut_ptr(), self.as_ptr(), 1);
            res
        }
    }
    
    /// Outputs the primorial of `n`, the product of all primes less than or equal to `n`.
    #[inline]
    pub fn primorial(n: c_ulong) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_primorial(res.as_mut_ptr(), n);}
        res
    }

    /// Returns the value of the Euler totient/phi function at an [Integer] `n`, the number of 
    /// positive integers up to `n` inclusive that are coprime to `n`. The input must be greater
    /// than zero.
    #[inline]
    pub fn euler_phi(&self) -> Integer {
        assert!(self > &0);
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_euler_phi(res.as_mut_ptr(), self.as_ptr());}
        res
    }
    
    /// Compute the Moebius mu function at an [Integer] `n`, which is defined to be 0 if `n` has
    /// a prime factor of multiplicity greater than one, -1 if `n` has an odd number of distinct
    /// prime factors, and 1 otherwise.
    #[inline]
    pub fn moebius_mu(&self) -> c_int {
        unsafe { flint_sys::fmpz::fmpz_moebius_mu(self.as_ptr())}
    }
   
    /// Compute the divisor function `sigma_k(n)` of an [Integer] `n`, which is the sum of `k`-th
    /// powers of the divisors of `n`. If `k = 0` then it counts the number of divisors.
    #[inline]
    pub fn divisor_sigma(&self, k: c_ulong) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_divisor_sigma(res.as_mut_ptr(), self.as_ptr(), k);}
        res
    }
}

impl Factorizable for Integer {
    type Output = Product<Integer>;
    fn factor(&self) -> Self::Output {
        assert!(self != &0);
        if self == &1 {
            return Product::from(Integer::from(1))
        };
       
        let mut fac = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_factor::fmpz_factor_init(fac.as_mut_ptr());
            let mut fac = fac.assume_init();
            
            flint_sys::fmpz_factor::fmpz_factor(&mut fac, self.as_ptr());

            let n = fac.num as usize;
            let base = std::slice::from_raw_parts(fac.p, n);
            let exp = std::slice::from_raw_parts(fac.exp, n);
            
            let mut hashmap = FxHashMap::<Integer, Integer>::default();
            for (p, k) in base.iter().zip(exp) {
                hashmap.insert(Integer { data: IntegerData { elem: p.clone() }}, Integer::from(k));
            }
            
            flint_sys::fmpz_factor::fmpz_factor_clear(&mut fac);
            let fac = Product::<Integer>::from(hashmap);
            fac
        }
    }
}

impl EvaluateProduct for Product<Integer> {
    type Output = Rational;
    fn evaluate(&self) -> Rational {
        let mut x = Rational::from(1);
        for (p, k) in self.hashmap.iter() {
            x *= p.pow(k);
        }
        x
    }
}

impl EvaluateProductMod<Integer> for Product<Integer> {
    type Output = Result<Integer, ()>;
    #[inline]
    fn evaluate_mod(&self, modulus: Integer) -> Result<Integer, ()> {
        self.evaluate_mod(&modulus)
    }
}

impl EvaluateProductMod<&Integer> for Product<Integer> {
    type Output = Result<Integer, ()>;
    fn evaluate_mod(&self, modulus: &Integer) -> Result<Integer, ()> {
        let mut x = Integer::from(1);
        for (p, k) in self.hashmap.iter() {
            x *= p.powm(k, modulus)?;
            x %= modulus;
        }
        Ok(x)
    }
}
