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


use flint_sys::flint::{flint_rand_s, flint_bitcnt_t};
use flint_sys::fmpz::fmpz;
use libc::{c_int, c_long, c_ulong};

use crate::traits::Element;
use crate::rational::src::Rational;

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

    /// A pointer to the underlying FFI type. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fmpz {
        &self.data
    }
    
    /// A mutable pointer to the underlying FFI type. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpz {
        &mut self.data
    }
    
    /// Convert the `Integer` to a string in base `base`.
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
    
    /// Return true if the integer is zero, false otherwise.
    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe { flint_sys::fmpz::fmpz_is_zero(self.as_ptr()) == 1 }
    }
   
    /// Return true if the integer is one, false otherwise.
    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe {
            flint_sys::fmpz::fmpz_is_one(self.as_ptr()) == 1
        }
    }

    /// Return true if the integer is even, false otherwise.
    #[inline]
    pub fn is_even(&self) -> bool {
        unsafe {flint_sys::fmpz::fmpz_is_even(self.as_ptr()) == 1}
    }
    
    /// Return true if the integer is odd, false otherwise.
    #[inline]
    pub fn is_odd(&self) -> bool {
        unsafe {flint_sys::fmpz::fmpz_is_odd(self.as_ptr()) == 1}
    }
    
    /// Returns -1 if the `Integer` is negative, +1 if the `Integer` is positive, and 0 otherwise.
    #[inline]
    pub fn sign(&self) -> i32 {
        unsafe {
            flint_sys::fmpz::fmpz_sgn(self.as_ptr())
        }
    }

    /// Returns the absolute value of an `Integer`.
    #[inline]
    pub fn abs(&self) -> Integer {
        unsafe {
            let mut res = Integer::default();
            flint_sys::fmpz::fmpz_abs(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }
   
    /// Determines the size of the absolute value of an `Integer` in base `base` in terms of number
    /// of digits. The base can be between 2 and 62, inclusive.
    #[inline]
    pub fn sizeinbase(&self, base: u8) -> usize {
        unsafe { flint_sys::fmpz::fmpz_sizeinbase(self.as_ptr(), base as i32) as usize }
    }
   
    /// Returns the number of limbs required to store the absolute value of an `Integer`. Returns
    /// zero if the `Integer` is zero.
    #[inline]
    pub fn size(&self) -> c_long {
        unsafe { flint_sys::fmpz::fmpz_size(self.as_ptr()) }
    }
   
    /// Returns the number of bits required to store the absolute value of an `Integer`. Returns zero
    /// if the `Integer` is zero.
    #[inline]
    pub fn bits(&self) -> c_ulong {
        unsafe { flint_sys::fmpz::fmpz_bits(self.as_ptr()) }
    }
   
    /// Determine if the `Integer` fits in a signed long.
    #[inline]
    pub fn fits_si(&self) -> bool {
        unsafe { flint_sys::fmpz::fmpz_fits_si(self.as_ptr()) == 1 }
    }
    
    /// Determine if the absolute value of an `Integer` fits in an unsigned long.
    #[inline]
    pub fn abs_fits_ui(&self) -> bool {
        unsafe { flint_sys::fmpz::fmpz_abs_fits_ui(self.as_ptr()) == 1 }
    }
   
    /// Return an `Option` containing the input as a signed long (`libc::c_long`) if possible, 
    /// otherwise it is `None`.
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

    /// Return an `Option` containing the input as an unsigned long (`libc::c_ulong`) if possible, 
    /// otherwise it is `None`.
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

    /// Sets the bit index `bit_index` of an `Integer`.
    #[inline]
    pub fn setbit(&mut self, bit_index: usize) {
        unsafe { flint_sys::fmpz::fmpz_setbit(self.as_mut_ptr(), bit_index as c_ulong) }
    }

    /// Test the bit index `bit_index` of an `Integer`. Return `true` if it is 1, `false` if it is
    /// zero.
    #[inline]
    pub fn testbit(&self, bit_index: usize) -> bool {
        unsafe { flint_sys::fmpz::fmpz_tstbit(self.as_ptr(), bit_index as c_ulong) == 1 }
    }

    // TODO: All Rand functions need work.
    #[inline]
    pub fn rand_bits(st: flint_rand_s, bt: flint_bitcnt_t) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randbits(res.as_mut_ptr(), &st, bt);}
        res
    }
    
    #[inline]
    pub fn rand_max_bits(st: flint_rand_s, bt: flint_bitcnt_t) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randtest(res.as_mut_ptr(), &st, bt);}
        res
    }
    
    #[inline]
    pub fn rand_max_bits_ui(st: flint_rand_s, bt: flint_bitcnt_t) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randtest_unsigned(res.as_mut_ptr(), &st, bt);}
        res
    }
    
    #[inline]
    pub fn rand_max_bits_non_zero(st: flint_rand_s, bt: flint_bitcnt_t) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randtest_not_zero(res.as_mut_ptr(), &st, bt);}
        res
    }
    
    #[inline]
    pub fn rand(st: flint_rand_s, m: &Integer) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randm(res.as_mut_ptr(), &st, m.as_ptr());}
        res
    }
    
    #[inline]
    pub fn rand_mod(st: flint_rand_s, m: &Integer) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randtest_mod(res.as_mut_ptr(), &st, m.as_ptr());}
        res
    }
    
    #[inline]
    pub fn rand_mod_si(st: flint_rand_s, m: &Integer) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randtest_mod_signed(res.as_mut_ptr(), &st, m.as_ptr());}
        res
    }
    
    #[inline]
    pub fn rand_prime(st: flint_rand_s, bt: flint_bitcnt_t) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randprime(res.as_mut_ptr(), &st, bt, 1);}
        res
    }

    /// Return the quotient self/other rounded up towards infinity.
    #[inline]
    pub fn cdiv<'a, T>(&self, other: T) -> Integer where T: Into<&'a Integer> {
        let other = other.into();
        assert!(!other.is_zero());
        unsafe {
            let mut res = Integer::default();
            flint_sys::fmpz::fmpz_cdiv_q(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            res
        }
    }
    
    /// Return the quotient self/other rounded down towards negative infinity.
    #[inline]
    pub fn fdiv<'a, T>(&self, other: T) -> Integer where T: Into<&'a Integer> {
        let other = other.into();
        assert!(!other.is_zero());
        unsafe {
            let mut res = Integer::default();
            flint_sys::fmpz::fmpz_fdiv_q(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            res
        }
    }
    
    /// Return the quotient self/other rounded to the nearest integer.
    #[inline]
    pub fn tdiv<'a, T>(&self, other: T) -> Integer where T: Into<&'a Integer> {
        let other = other.into();
        assert!(!other.is_zero());
        unsafe {
            let mut res = Integer::default();
            flint_sys::fmpz::fmpz_tdiv_q(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            res
        }
    }
    
    /// Return the quotient self/other rounded up towards infinity and the remainder r.
    #[inline]
    pub fn cdivrem<'a, T>(&self, other: T) -> (Integer, Integer) where T: Into<&'a Integer> {
        let other = other.into();
        assert!(!other.is_zero());
        unsafe {
            let mut q = Integer::default();
            let mut r = Integer::default();
            flint_sys::fmpz::fmpz_cdiv_qr(q.as_mut_ptr(), r.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            (q, r)
        }
    }
    
    /// Return the quotient self/other rounded down towards negative infinity and the remainder r.
    #[inline]
    pub fn fdivrem<'a, T>(&self, other: T) -> (Integer, Integer) where T: Into<&'a Integer> {
        let other = other.into();
        assert!(!other.is_zero());
        unsafe {
            let mut q = Integer::default();
            let mut r = Integer::default();
            flint_sys::fmpz::fmpz_fdiv_qr(q.as_mut_ptr(), r.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            (q, r)
        }
    }
    
    /// Return the quotient self/other rounded down towards zero and the remainder r.
    #[inline]
    pub fn divrem<'a, T>(&self, other: T) -> (Integer, Integer) where T: Into<&'a Integer> {
        let other = other.into();
        assert!(!other.is_zero());
        unsafe {
            let mut q = Integer::default();
            let mut r = Integer::default();
            flint_sys::fmpz::fmpz_tdiv_qr(q.as_mut_ptr(), r.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            (q, r)
        }
    }
   
    /// Return the quotient self/other or `None` if the division is not exact.
    #[inline]
    pub fn divexact<'a, T>(&self, other: T) -> Option<Integer> where T: Into<&'a Integer> {
        let other = other.into();
        assert!(!other.is_zero());

        if !other.divides(self) {
            return None
        }

        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_divexact(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());}
        Some(res)
    }
   
    /// Symmetric remainder. The `%` remainder operator is overloaded for modular arithmetic but
    /// `srem` will reduce an `Integer` modulo n to the range -(n-1)/2, ..., 0, ..., (n-1)/2
    /// symmetric around zero.
    #[inline]
    pub fn srem<'a, T>(&self, other: T) -> Integer where T: Into<&'a Integer> {
        let other = other.into();
        assert!(!other.is_zero());
        unsafe {
            let mut res = Integer::default();
            flint_sys::fmpz::fmpz_smod(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            res
        }
    }
  
    // Replace with Powm trait
    #[deprecated]
    #[inline]
    pub fn powm_ui<T: Into<c_ulong>>(&self, exp: T, modulus: &Integer) -> Integer {
        assert!(!modulus.is_zero());
        let exp = exp.into();
        unsafe {
            let mut res = Integer::default();
            flint_sys::fmpz::fmpz_powm_ui(res.as_mut_ptr(), self.as_ptr(), exp, modulus.as_ptr());
            res
        }
    }
   
    /// Determine whether self divides other.
    #[inline]
    pub fn divides<'a, T>(&self, other: T) -> bool where T: Into<&'a Integer> {
        unsafe { flint_sys::fmpz::fmpz_divisible(other.into().as_ptr(), self.as_ptr()) == 1 }
    }

    // Use arb/acb for result.
    #[inline]
    pub fn log(&self) -> f64 {
        unsafe { flint_sys::fmpz::fmpz_dlog(self.as_ptr())}
    }

    /// Return ceil(log(self)).
    #[inline]
    pub fn clog<'a, T>(&self, base: T) -> c_long where T: Into<&'a Integer> {
        unsafe { flint_sys::fmpz::fmpz_clog(self.as_ptr(), base.into().as_ptr())}
    }

    #[inline]
    pub fn clog_ui<T: Into<c_ulong>>(&self, base: T) -> c_long {
        let base = base.into();
        unsafe { flint_sys::fmpz::fmpz_clog_ui(self.as_ptr(), base)}
    }
    
    #[inline]
    pub fn flog(&self, base: &Integer) -> c_long {
        unsafe { flint_sys::fmpz::fmpz_flog(self.as_ptr(), base.as_ptr())}
    }

    #[inline]
    pub fn flog_ui<T: Into<c_ulong>>(&self, base: T) -> c_long {
        let base = base.into();
        unsafe { flint_sys::fmpz::fmpz_flog_ui(self.as_ptr(), base)}
    }

    // doc says: if n not prime and r == 1, the value of res is meaningless?
    #[inline]
    pub fn sqrt_mod(&self, n: &Integer) -> Option<Integer> {
        assert!(!n.is_zero());
        let mut res = Integer::default();
        unsafe { 
            let r = flint_sys::fmpz::fmpz_sqrtmod(res.as_mut_ptr(), self.as_ptr(), n.as_ptr());
      
            if r == 0 {
                None
            } else {
                Some(res)
            }
        }
    }

    #[inline]
    pub fn quadratic_residue(&self, n: &Integer) -> Option<Integer> {
        self.sqrt_mod(n)
    }
    
    #[inline]
    pub fn sqrt_rem(&self) -> (Integer, Integer) {
        let mut q = Integer::default();
        let r = Integer::default();
        unsafe { 
            flint_sys::fmpz::fmpz_sqrtrem(q.as_mut_ptr(), r.as_ptr(), self.as_ptr());
        }
        (q, r)
    }
    
    #[inline]
    pub fn is_square(&self) -> bool {
        unsafe { flint_sys::fmpz::fmpz_is_square(self.as_ptr()) != 0}
    }

    /*
    #[inline]
    pub fn sqrt(&self) -> Integer {
        assert!(self > 0);
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_sqrt(res.as_mut_ptr(), self.as_ptr());}
        res
    }

    #[inline]
    pub fn root<T: Into<i64>>(&self, n: T) -> Integer {
        let n = n.into();
        
        assert!(n > 0);
        if Integer::from(n).is_even() {
            assert!(self >= &0)
        }

        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_root(res.as_mut_ptr(), self.as_ptr(), n);}
        res
    }
   
    // return bool? (c_int = 0 then not perfect power)
    #[inline]
    pub fn perfect_power(&self) -> (c_int, Option<Integer>) {
        let mut res = Integer::default();
        unsafe { 
            let k = flint_sys::fmpz::fmpz_is_perfect_power(res.as_mut_ptr(), self.as_ptr());

            if k != 0 {
                (k, Some(res))
            } else {
                (0, None)
            }
        }
    }
    
    #[inline]
    pub fn fibonacci(&self) -> Integer {
        assert!(self > 0);
        assert!(self.abs_fits_ui());
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_fib_ui(res.as_mut_ptr(), self.get_ui().unwrap());}
        res
    }
    
    #[inline]
    pub fn binomial(&self, k: c_ulong) -> Integer {
        assert!(self > 0);
        assert!(self.abs_fits_ui());
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_bin_uiui(res.as_mut_ptr(), self.get_ui().unwrap(), k);}
        res
    }
    
    #[inline]
    pub fn factorial(&self) -> Integer {
        assert!(self > 0);
        assert!(self.abs_fits_ui());
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_fac_ui(res.as_mut_ptr(), self.get_ui().unwrap());}
        res
    }

    #[inline]
    pub fn rising_factorial(&self, k: c_ulong) -> Integer {
        assert!(self > 0);
        assert!(self.abs_fits_ui());
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_rfac_uiui(res.as_mut_ptr(), self.get_ui().unwrap(), k);}
        res
    }

    #[inline]
    pub fn gcd(&self, other: &Integer) -> Integer {
        unsafe {
            let mut res = Integer::default();
            flint_sys::fmpz::fmpz_gcd(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            res
        }
    }

    #[inline]
    pub fn lcm(&self, other: &Integer) -> Integer {
        unsafe {
            let mut res = Integer::default();
            flint_sys::fmpz::fmpz_lcm(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            res
        }
    }

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
    
    #[inline]
    pub fn rational_reconstruction(&self, modulus: &Integer) -> Rational {
        unsafe {
            let mut res = Rational::default();
            flint_sys::fmpq::fmpq_reconstruct_fmpz(res.as_mut_ptr(), self.as_ptr(), modulus.as_ptr());
            res
        }
    }
    
    #[inline]
    pub fn rational_reconstruction2(&self, modulus: &Integer, n: &Integer, d: &Integer) -> Rational {
        unsafe {
            let mut res = Rational::default();
            flint_sys::fmpq::fmpq_reconstruct_fmpz_2(res.as_mut_ptr(), self.as_ptr(), modulus.as_ptr(),
                n.as_ptr(), d.as_ptr());
            res
        }
    }

    #[inline]
    pub fn invmod<'a, T>(&self, modulus: T) -> Option<Integer>  where 
        T: Into<&'a Integer>
    {
        let modulus = modulus.into();
        assert!(!modulus.is_zero());

        let mut res = Integer::default();
        unsafe{ 
            let r = flint_sys::fmpz::fmpz_invmod(res.as_mut_ptr(), self.as_ptr(), modulus.as_ptr());
        
            if r == 0 {
                None
            } else {
                Some(res)
            }
        }
    }
    
    #[inline]
    pub fn remove(&mut self, factor: &Integer) {
        assert!(!factor.is_zero());
        unsafe {flint_sys::fmpz::fmpz_remove(self.as_mut_ptr(), self.as_ptr(), factor.as_ptr());}
    }

    #[inline]
    pub fn negmod(&self, modulus: &Integer) -> Integer {
        assert!(!modulus.is_zero());
        let mut res = self.rem(modulus);
        unsafe {
            flint_sys::fmpz::fmpz_negmod(res.as_mut_ptr(), res.as_ptr(), modulus.as_ptr());
            res
        }
    }

    // remove prime assertion?
    #[inline]
    pub fn jacobi(&self, p: &Integer) -> i32 {
        assert!(p > &0);
        assert!(p.is_prime());
        unsafe {
            flint_sys::fmpz::fmpz_jacobi(self.as_ptr(), p.as_ptr())
        }
    }

    // TODO: BIT PACKING
    
    #[inline]
    pub fn complement(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_complement(res.as_mut_ptr(), self.as_ptr());}
        res
    }
    
    #[inline]
    pub fn clear_bit(&mut self, i: c_ulong) {
        unsafe { flint_sys::fmpz::fmpz_clrbit(self.as_mut_ptr(), i);}
    }
    
    #[inline]
    pub fn complement_bit(&mut self, i: c_ulong) {
        unsafe { flint_sys::fmpz::fmpz_combit(self.as_mut_ptr(), i);}
    }
    
    #[inline]
    pub fn and(&self, other: &Integer) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_and(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());}
        res
    }
    
    #[inline]
    pub fn or(&self, other: &Integer) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_or(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());}
        res
    }
    
    #[inline]
    pub fn xor(&self, other: &Integer) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_xor(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());}
        res
    }

    #[inline]
    pub fn crt(v1: &Integer, m1: &Integer, v2: &Integer, m2: &Integer) -> Integer {
        let mut res = Integer::default();
        unsafe { 
            flint_sys::fmpz::fmpz_CRT(
                res.as_mut_ptr(), 
                v1.as_ptr(), 
                m1.as_ptr(),
                v2.as_ptr(), 
                m2.as_ptr(),
                0);
        }
        res
    }
    
    #[inline]
    pub fn multi_crt(v: &[Integer], m: &[Integer]) -> Integer {
        let mut res = Integer::default();        
       
        let len = v.len();
        let vv: Vec<flint_sys::fmpz::fmpz> = v.iter().map(|x| x.as_ptr().clone()).collect();
        let vm: Vec<flint_sys::fmpz::fmpz> = m.iter().map(|y| y.as_ptr().clone()).collect();

        unsafe { 
            flint_sys::fmpz::fmpz_multi_crt(
                res.as_mut_ptr(), 
                vm.as_ptr(), 
                vv.as_ptr(),
                len as c_long);
        }
        res
    }

    // PRIMALITY TESTING
    // TODO: probable prime tests?

    #[inline]
    pub fn is_prime(&self) -> bool {
        unsafe {
            flint_sys::fmpz::fmpz_is_prime(self.as_ptr()) == 1
        }
    }

    // guaranteed prime
    #[inline]
    pub fn next_prime(&self) -> Integer {
        unsafe {
            let mut res = Integer::default();
            flint_sys::fmpz::fmpz_nextprime(res.as_mut_ptr(), self.as_ptr(), 1);
            res
        }
    }
    
    #[inline]
    pub fn primorial(n: c_ulong) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_primorial(res.as_mut_ptr(), n);}
        res
    }

    #[inline]
    pub fn euler_phi(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_euler_phi(res.as_mut_ptr(), self.as_ptr());}
        res
    }
    
    #[inline]
    pub fn moebius_mu(&self) -> c_int {
        unsafe { flint_sys::fmpz::fmpz_moebius_mu(self.as_ptr())}
    }
    
    #[inline]
    pub fn divisor_sigma(&self, k: c_ulong) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_divisor_sigma(res.as_mut_ptr(), self.as_ptr(), k);}
        res
    }*/
}
