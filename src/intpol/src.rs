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

#![allow(non_snake_case)]

use flint_sys::fmpz_poly::fmpz_poly_struct;
use libc::{c_int, c_long, c_ulong};

use crate::traits::Element;
use crate::integer::src::Integer;

// IntPol //

#[derive(Default, Debug, Hash, Clone, Copy)]
pub struct IntPolRing {}

impl IntPolRing {
    pub fn init() -> Self {
        IntPolRing {}
    }
    
    pub fn new<T: Into<IntPol>>(&self, x: T) -> IntPol {
        x.into()
    }
}

// IntPol //

#[derive(Debug)]
#[repr(transparent)]
pub struct IntPol {
    pub data: <Self as Element>::Data,
}

impl IntPol {
    #[inline]
    pub fn as_ptr(&self) -> &fmpz_poly_struct {
        &self.data
    }
    
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpz_poly_struct {
        &mut self.data
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == 0
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe {flint_sys::fmpz_poly::fmpz_poly_is_one(self.as_ptr()) == 1}
    }

    // NOTE: we mean invertible in Q(x) here (to guarantee division is possible)
    #[inline]
    pub fn is_invertible(&self) -> bool {
        !self.is_zero()
    }

    #[inline]
    pub fn len(&self) -> c_long {
        unsafe { flint_sys::fmpz_poly::fmpz_poly_length(self.as_ptr())}
    }
    
    #[inline]
    pub fn degree(&self) -> c_long {
        unsafe { flint_sys::fmpz_poly::fmpz_poly_degree(self.as_ptr())}
    }
    
    #[inline]
    pub fn get_coeff(&self, i: usize) -> Integer {
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_get_coeff_fmpz(res.as_mut_ptr(), self.as_ptr(), i as i64);
            res
        }
    }
    
    #[inline]
    pub fn set_coeff<T>(&mut self, i: usize, coeff: T) where T: Into<Integer> {
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_set_coeff_fmpz(
                self.as_mut_ptr(), 
                i as c_long, 
                coeff.into().as_ptr()
            );
        }
    }

    #[inline]
    pub fn coefficients(&self) -> Vec<Integer> {
        let len = self.len();

        let mut vec = Vec::<Integer>::default();
        for i in 0..len {
            vec.push(self.get_coeff(i as usize));
        }
        vec
    }
    
    #[inline]
    pub fn is_unit(&self) -> bool {
        unsafe {flint_sys::fmpz_poly::fmpz_poly_is_unit(self.as_ptr()) == 1}
    }
    
    #[inline]
    pub fn is_gen(&self) -> bool {
        unsafe {flint_sys::fmpz_poly::fmpz_poly_is_gen(self.as_ptr()) == 1}
    }
    
    // 1 == true?
    #[inline]
    pub fn is_squarefree(&self) -> bool {
        unsafe {flint_sys::fmpz_poly::fmpz_poly_is_squarefree(self.as_ptr()) == 1}
    }

    #[inline]
    pub fn is_monic(&self) -> bool {
        self.get_coeff(self.degree() as usize).is_one()
    }

    pub fn is_constant(&self) -> bool {
        self.len() <= 1
    }

    #[inline]
    pub fn max_limbs(&self) -> c_ulong {
        unsafe { flint_sys::fmpz_poly::fmpz_poly_max_limbs(self.as_ptr())}
    }
    
    #[inline]
    pub fn max_bits(&self) -> c_long {
        unsafe { flint_sys::fmpz_poly::fmpz_poly_max_bits(self.as_ptr())}
    }

    #[inline]
    pub fn abs(&self) -> IntPol {
        unsafe {
            let mut res = IntPol::default();
            flint_sys::fmpz_poly::fmpz_poly_scalar_abs(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }

    #[inline]
    pub fn height(&self) -> Integer {
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_height(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }

    #[inline]
    pub fn l2_norm(&self) -> Integer {
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_2norm(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
    
    #[inline]
    pub fn discriminant(&self) -> Integer {
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_discriminant(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
    
    #[inline]
    pub fn content(&self) -> Integer {
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_content(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
    
    #[inline]
    pub fn primitive_part(&self) -> IntPol {
        let mut res = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_primitive_part(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
    
    #[inline]
    pub fn reverse(&mut self) {
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_reverse(self.as_mut_ptr(), self.as_ptr(), self.len());
        }
    }
    
    #[inline]
    pub fn truncate(&mut self, n: c_long) {
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_truncate(self.as_mut_ptr(), n);
        }
    }
    
    // no cdiv in flint

    #[inline]
    pub fn fdiv<T>(&self, other: T) -> IntPol where T: Into<Integer> {
        let other = other.into();
        assert!(!other.is_zero());
        let mut res = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_scalar_fdiv_fmpz(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr());
            res
        }
    }
    
    #[inline]
    pub fn tdiv<T>(&self, other: T) -> IntPol where T: Into<Integer> {
        let other = other.into();
        assert!(!other.is_zero());
        let mut res = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_scalar_tdiv_fmpz(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr());
            res
        }
    }
 
    #[inline]
    pub fn divexact<T>(&self, other: T) -> IntPol where T: Into<Integer> {
        let other = other.into();
        assert!(!other.is_zero());
        
        let coeffs = self.coefficients();
        for coeff in coeffs {
            assert!(other.divides(&coeff));
        }

        let mut res = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_scalar_divexact_fmpz(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr());
            res
        }
    }
    
    #[inline]
    pub fn srem<T>(&self, other: T) -> IntPol where T: Into<Integer> {
        let other = other.into();
        assert!(!other.is_zero());
        let mut res = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_scalar_smod_fmpz(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr());
            res
        }
    }
    
    #[inline]
    pub fn divrem<T>(&self, other: T) -> (IntPol, IntPol) where T: Into<IntPol> {
        let other = other.into();
        assert!(!other.is_zero());
        let mut q = IntPol::default();
        let mut r = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_divrem(
                q.as_mut_ptr(), 
                r.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr());
            (q, r)
        }
    }
   
    // is this more efficient than multiplication?
    #[inline]
    pub fn square(&self) -> IntPol {
        let mut res = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_sqr(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }
    
    #[inline]
    pub fn shift_left(&mut self, n: c_long) {
        unsafe { flint_sys::fmpz_poly::fmpz_poly_shift_left(self.as_mut_ptr(), self.as_ptr(), n);}
    }
    
    #[inline]
    pub fn shift_right(&mut self, n: c_long) {
        unsafe { flint_sys::fmpz_poly::fmpz_poly_shift_right(self.as_mut_ptr(), self.as_ptr(), n);}
    }

    #[inline]
    pub fn gcd<T>(&self, other: T) -> IntPol where T: Into<IntPol> {
        let mut res = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_gcd(res.as_mut_ptr(), self.as_ptr(), other.into().as_ptr());
            res
        }
    }

    #[inline]
    pub fn lcm<T>(&self, other: T) -> IntPol where T: Into<IntPol> {
        let mut res = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_lcm(res.as_mut_ptr(), self.as_ptr(), other.into().as_ptr());
            res
        }
    }

    #[inline]
    pub fn xgcd<T>(&self, other: T) -> (Integer, IntPol, IntPol) where T: Into<IntPol> {
        unsafe {
            let mut d = Integer::default();
            let mut a = IntPol::default();
            let mut b = IntPol::default();
            flint_sys::fmpz_poly::fmpz_poly_xgcd(
                d.as_mut_ptr(), 
                a.as_mut_ptr(), 
                b.as_mut_ptr(),
                self.as_ptr(), 
                other.into().as_ptr()
            );
            (d, a, b)
        }
    }
    
    #[inline]
    pub fn resultant<T>(&self, other: T) -> Integer where T: Into<IntPol> {
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_resultant(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.into().as_ptr()
            );
            res
        }
    }
    
    // unoptimized per flint doc
    #[inline]
    pub fn divides<T>(&self, other: T) -> bool where T: Into<IntPol> {
        let mut res = IntPol::default();
        unsafe { flint_sys::fmpz_poly::fmpz_poly_divides(
            res.as_mut_ptr(), 
            other.into().as_ptr(), 
            self.as_ptr()) == 1 
        }
    }
    
    #[inline]
    pub fn remove<T>(&mut self, other: T) -> c_int where T: Into<IntPol> {
        let other = other.into();
        assert!(!other.is_zero());
        assert!(other.abs() != 1);
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_divides(
                self.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr())
        }
    }
    
    #[inline]
    pub fn inv_series(&self, n: c_long) -> IntPol {
        assert!(self.get_coeff(0).abs() == 1);
        assert!(n >= 1);

        let mut res = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_inv_series(res.as_mut_ptr(), self.as_ptr(), n);
            res
        }
    }
    
    #[inline]
    pub fn div_series<T>(&self, other: T, n: c_long) -> IntPol where T: Into<IntPol> {
        let other = other.into();
        assert!(other.get_coeff(0).abs() == 1);
        assert!(n >= 1);

        let mut res = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_div_series(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr(), 
                n);
            res
        }
    }
    
    #[inline]
    pub fn derivative(&self) -> IntPol {
        let mut res = IntPol::default();
        unsafe { flint_sys::fmpz_poly::fmpz_poly_derivative(res.as_mut_ptr(), self.as_ptr());}
        res
    }

    // TODO: Flint inexact error thrown if output is rational polynomial (use RatPol::interpolate).
    #[inline]
    pub fn interpolate<'b, T>(x: &'b [T], y: &'b [T]) -> IntPol where &'b T: Into<Integer> {
        assert_eq!(x.len(), y.len());
        let n = x.len();

        // TODO: check x has no repeated elements?
        //let mut set: HashSet<&Integer> = HashSet::from_iter(x.iter().clone());
        //assert_eq!(set.len(), n);

        let vx = Vec::from_iter(x.iter().map(|x| x.into().as_ptr().clone()));
        let vy = Vec::from_iter(y.iter().map(|y| y.into().as_ptr().clone()));

        let mut res = IntPol::default();
        unsafe { 
            flint_sys::fmpz_poly::fmpz_poly_interpolate_fmpz_vec(
                res.as_mut_ptr(),
                vx.as_ptr(),
                vy.as_ptr(),
                n as c_long
            );
        }
        res
    }

    #[inline]
    pub fn compose<T>(&self, other: T) -> IntPol where T: Into<IntPol> {
        let mut res = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_compose(res.as_mut_ptr(), self.as_ptr(), other.into().as_ptr());
        }
        res
    }
    
    #[inline]
    pub fn inflate(&self, n: c_ulong) -> IntPol {
        let mut res = IntPol::default();
        unsafe { flint_sys::fmpz_poly::fmpz_poly_inflate(res.as_mut_ptr(), self.as_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn deflate(&self, n: c_ulong) -> IntPol {
        let mut res = IntPol::default();
        unsafe { flint_sys::fmpz_poly::fmpz_poly_deflate(res.as_mut_ptr(), self.as_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn deflation(&self) -> c_ulong {
        unsafe { flint_sys::fmpz_poly::fmpz_poly_deflation(self.as_ptr())}
    }

    #[inline]
    pub fn taylor_shift<T>(&self, c: T) -> IntPol where T: Into<Integer> {
        let mut res = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_taylor_shift(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                c.into().as_ptr()
            );
        }
        res
    }
    
    #[inline]
    pub fn compose_series<T>(&self, other: T, n: c_long) -> IntPol where T: Into<IntPol> {
        let mut res = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_compose_series(
                res.as_mut_ptr(), 
                self.as_ptr(),
                other.into().as_ptr(),
                n
            );
        }
        res
    }
    
    #[inline]
    pub fn revert_series(&self, n: c_long) -> IntPol {
        let mut res = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_revert_series(
                res.as_mut_ptr(), 
                self.as_ptr(),
                n);
        }
        res
    }

    #[inline]
    pub fn sqrt(&self) -> IntPol {
        let mut res = IntPol::default();
        unsafe {
            let n = flint_sys::fmpz_poly::fmpz_poly_sqrt(res.as_mut_ptr(), self.as_ptr());
            assert_eq!(n, 1);
        }
        res
    }
    
    #[inline]
    pub fn sqrt_series(&self, n: c_long) -> IntPol {
        let mut res = IntPol::default();
        assert!(n > 0);
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_sqrt_series(res.as_mut_ptr(), self.as_ptr(), n);
        }
        res
    }
    
    #[inline]
    pub fn power_sums_naive(&self, n: c_long) -> IntPol {
        let mut res = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_power_sums_naive(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                n);
        }
        res
    }
    
    #[inline]
    pub fn power_sums(&self, n: c_long) -> IntPol {
        let mut res = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_power_sums(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                n);
        }
        res
    }
    
    #[inline]
    pub fn power_sums_to_poly(&self) -> IntPol {
        let mut res = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_power_sums_to_poly(
                res.as_mut_ptr(), 
                self.as_ptr()); 
        }
        res
    }
   
    #[inline]
    pub fn signature(&self) -> (c_long, c_long) {
        assert!(self.is_squarefree());

        let mut r1 = 0 as c_long;
        let mut r2 = 0 as c_long;
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_signature(
                &mut r1,
                &mut r2,
                self.as_ptr()); 
        }
        (r1, r2)
    }

    #[inline]
    pub fn hensel_lift<'b, 'c, S, T>(
        &self, 
        g: &'b S, 
        h: &'b S, 
        a: &'b S, 
        b: &'b S, 
        p: &'c T, 
        p1: &'c T
    ) -> (IntPol, IntPol, IntPol, IntPol) where 
        &'b S: Into<IntPol>,
        &'c T: Into<Integer>
{

        let mut G = IntPol::default();
        let mut H = IntPol::default();
        let mut A = IntPol::default();
        let mut B = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_hensel_lift(
                G.as_mut_ptr(),
                H.as_mut_ptr(),
                A.as_mut_ptr(),
                B.as_mut_ptr(),
                self.as_ptr(),
                g.into().as_ptr(),
                h.into().as_ptr(),
                a.into().as_ptr(),
                b.into().as_ptr(),
                p.into().as_ptr(),
                p1.into().as_ptr());
        }
        (G, H, A, B)
    }
    
    #[inline]
    pub fn hensel_lift_only_inv<'b, 'c, S, T>(
        G: &'b S, 
        H: &'b S, 
        a: &'b S, 
        b: &'b S, 
        p: &'c T, 
        p1: &'c T
    ) -> (IntPol, IntPol) where 
        &'b S: Into<IntPol>,
        &'c T: Into<Integer>
{

        let mut A = IntPol::default();
        let mut B = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_hensel_lift_only_inverse(
                A.as_mut_ptr(),
                B.as_mut_ptr(),
                G.into().as_ptr(),
                H.into().as_ptr(),
                a.into().as_ptr(),
                b.into().as_ptr(),
                p.into().as_ptr(),
                p1.into().as_ptr());
        }
        (A, B)
    }
    
    #[inline]
    pub fn hensel_lift_no_inv<'b, 'c, S, T>(
        &self,
        g: &'b S, 
        h: &'b S, 
        a: &'b S, 
        b: &'b S, 
        p: &'c T, 
        p1: &'c T
    ) -> (IntPol, IntPol) where
        &'b S: Into<IntPol>,
        &'c T: Into<Integer>
{

        let mut G = IntPol::default();
        let mut H = IntPol::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_hensel_lift_without_inverse(
                G.as_mut_ptr(),
                H.as_mut_ptr(),
                self.as_ptr(),
                g.into().as_ptr(),
                h.into().as_ptr(),
                a.into().as_ptr(),
                b.into().as_ptr(),
                p.into().as_ptr(),
                p1.into().as_ptr());
        }
        (G, H)
    }

    // CRT once nmod poly implemented

    #[inline]
    pub fn bound_roots(&self) -> Integer {
        let mut res = Integer::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_bound_roots(res.as_mut_ptr(), self.as_ptr());}
        res
    }
    
    #[inline]
    pub fn num_real_roots(&self) -> c_long {
        unsafe {flint_sys::fmpz_poly::fmpz_poly_num_real_roots(self.as_ptr())}
    }
    
    #[inline]
    pub fn cyclotomic(n: c_ulong) -> IntPol {
        let mut res = IntPol::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_cyclotomic(res.as_mut_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn cos_minpoly(n: c_ulong) -> IntPol {
        let mut res = IntPol::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_cos_minpoly(res.as_mut_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn swinnerton_dyer(n: c_ulong) -> IntPol {
        let mut res = IntPol::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_swinnerton_dyer(res.as_mut_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn chebyshev_t(n: c_ulong) -> IntPol {
        let mut res = IntPol::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_chebyshev_t(res.as_mut_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn chebyshev_u(n: c_ulong) -> IntPol {
        let mut res = IntPol::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_chebyshev_u(res.as_mut_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn legendre_pt(n: c_ulong) -> IntPol {
        let mut res = IntPol::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_legendre_pt(res.as_mut_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn hermite_h(n: c_ulong) -> IntPol {
        let mut res = IntPol::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_hermite_h(res.as_mut_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn hermite_he(n: c_ulong) -> IntPol {
        let mut res = IntPol::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_hermite_he(res.as_mut_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn fibonacci(n: c_ulong) -> IntPol {
        let mut res = IntPol::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_fibonacci(res.as_mut_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn eta_qexp(r: c_long, n: c_long) -> IntPol {
        let mut res = IntPol::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_eta_qexp(res.as_mut_ptr(), r, n);}
        res
    }
    
    #[inline]
    pub fn theta_qexp(r: c_long, n: c_long) -> IntPol {
        let mut res = IntPol::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_theta_qexp(res.as_mut_ptr(), r, n);}
        res
    }
    
    #[inline]
    pub fn CLD_bound(&self, n: c_long) -> Integer {
        let mut res = Integer::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_CLD_bound(res.as_mut_ptr(), self.as_ptr(), n);}
        res
    }
}
