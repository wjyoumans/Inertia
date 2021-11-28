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

use flint_sys::fmpq_mat::fmpq_mat_struct;
use libc::c_long;

use crate::integer::src::Integer;
use crate::intmat::src::IntMat;
use crate::rational::src::Rational;


/// The vector space of `rows` by `cols` [Rational] matrices.
#[derive(Default, Debug, Hash, Clone, Copy)]
pub struct RatMatSpace {
    rows: c_long,
    cols: c_long,
}

impl RatMatSpace {
    /// Construct the space of dimension `m` by `n` [Rational] matrices.
    #[inline]
    pub fn init(m: c_long, n: c_long) -> Self {
        RatMatSpace { rows: m, cols: n }
    }

    /// Create a new [RatMat].
    #[inline]
    pub fn new<T: Into<RatMat>>(&self, x: T) -> RatMat {
        x.into()
    }
}

/// A matrix of arbitrary precision [Rationals][Rational]. The field `data` is a FLINT
/// [fmpq_mat_struct][flint_sys::fmpq_mat::fmpq_mat_struct].
#[derive(Debug)]
#[repr(transparent)]
pub struct RatMat {
    pub data: fmpq_mat_struct,
}

impl RatMat {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with FLINT
    /// via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fmpq_mat_struct {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpq_mat_struct {
        &mut self.data
    }

    /* No `get_str` for matrices in FLINT
    /// Return a [String] representation of an integer polynomial.
    #[inline]
    pub fn get_str(&self) -> String {
        unsafe {
            let s = flint_sys::fmpz_mat::fmpz_mat_get_str(self.as_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
    
    /// Return a pretty-printed [String] representation of an integer polynomial.
    #[inline]
    pub fn get_str_pretty(&self, var: &str) -> String {
        let v = CString::new(var).unwrap();
        unsafe {
            let s = flint_sys::fmpz_mat::fmpz_mat_get_str_pretty(self.as_ptr(), v.as_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }*/


    /// Return two integer matrices containing the numerator and denominator of each entry of a
    /// rational matrix.
    #[inline]
    pub fn num_den_entrywise(&self) -> (IntMat, IntMat) {
        let mut num = IntMat::zero(self.nrows(), self.ncols());
        let mut den = IntMat::zero(self.nrows(), self.ncols());
        unsafe {
            flint_sys::fmpq_mat::fmpq_mat_get_fmpz_mat_entrywise(
                num.as_mut_ptr(),
                den.as_mut_ptr(),
                self.as_ptr()
            );
        }
        (num, den)
    }
    
    /// Return the numerator and denominator of a rational matrix as an integer matrix and integer,
    /// obtained by clearing the denominators of the input matrix.
    #[inline]
    pub fn num_den(&self) -> (IntMat, Integer) {
        let mut num = IntMat::zero(self.nrows(), self.ncols());
        let mut den = Integer::default();
        unsafe {
            flint_sys::fmpq_mat::fmpq_mat_get_fmpz_mat_matwise(
                num.as_mut_ptr(),
                den.as_mut_ptr(),
                self.as_ptr()
            );
        }
        (num, den)
    }

    /// Return the numerator of the rational matrix as an [IntMat].
    #[inline]
    pub fn numerator(&self) -> IntMat {
        self.num_den().0
    }
    
    /// Return the denominator of the rational matrix as an [Integer].
    #[inline]
    pub fn denominator(&self) -> Integer {
        self.num_den().1
    }

    /// Swap two rational matrices. The dimensions are allowed to be different.
    #[inline]
    pub fn swap(&mut self, other: &mut RatMat) {
        unsafe { 
            flint_sys::fmpq_mat::fmpq_mat_swap(self.as_mut_ptr(), other.as_mut_ptr()); 
        }
    }

    /// Swap the rows `r` and `s` of a rational matrix. 
    #[inline]
    pub fn swap_rows(&mut self, r: c_long, s: c_long) {
        assert!(r < self.nrows());
        assert!(s < self.nrows());

        unsafe { 
            flint_sys::fmpq_mat::fmpq_mat_swap_rows(
                self.as_mut_ptr(), 
                std::ptr::null(),
                r,
                s
            ); 
        }
    }
    
    /// Swap the columns `r` and `s` of an rational matrix. 
    #[inline]
    pub fn swap_cols(&mut self, r: c_long, s: c_long) {
        assert!(r < self.ncols());
        assert!(s < self.ncols());

        unsafe { 
            flint_sys::fmpq_mat::fmpq_mat_swap_rows(
                self.as_mut_ptr(), 
                std::ptr::null(),
                r,
                s
            ); 
        }
    }
    
    /// Swap row `i` and `r - i` of a rational matrix for `0 <= i < r/2` where `r` is the number
    /// of rows of the input matrix.
    #[inline]
    pub fn invert_rows(&mut self) {
        unsafe { 
            flint_sys::fmpq_mat::fmpq_mat_invert_rows(
                self.as_mut_ptr(), 
                std::ptr::null()
            ); 
        }
    }
    
    /// Swap columns `i` and `c - i` of a rational matrix for `0 <= i < c/2` where `c` is the number
    /// of columns of the input matrix.
    #[inline]
    pub fn invert_columns(&mut self) {
        unsafe { 
            flint_sys::fmpq_mat::fmpq_mat_invert_cols(
                self.as_mut_ptr(), 
                std::ptr::null()
            ); 
        }
    }
    
   
    /* TODO: function missing from bindings
    /// Swap two rational matrices by swapping the individual entries rather than swapping the
    /// contents of their structs.
    #[inline]
    pub fn swap_entrywise(&mut self, other: &mut RatMat) {
        unsafe { 
            flint_sys::fmpq_mat::fmpq_mat_swap_entrywise(self.as_mut_ptr(), other.as_mut_ptr()); 
        }
    }
    */

    /// Return an `m` by `n` rational zero matrix.
    #[inline]
    pub fn zero(m: c_long, n: c_long) -> RatMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpq_mat::fmpq_mat_init(z.as_mut_ptr(), m, n);
            RatMat { data: z.assume_init() }
        }
    }

    /// Return the square `m` by `m` rational identity matrix.
    #[inline]
    pub fn one(m: c_long) -> RatMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpq_mat::fmpq_mat_init(z.as_mut_ptr(), m, m);
            flint_sys::fmpq_mat::fmpq_mat_one(z.as_mut_ptr());
            RatMat { data: z.assume_init() }
        }
    }

    /// Return true if the matrix contains all zeros.
    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe { flint_sys::fmpq_mat::fmpq_mat_is_zero(self.as_ptr()) == 1 } 
    }
    
    /// Return true if the matrix is the identity.
    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe { flint_sys::fmpq_mat::fmpq_mat_is_one(self.as_ptr()) == 1 } 
    }

    /// Return true if the number of rows or columns is zero.
    #[inline]
    pub fn is_empty(&self) -> bool {
        unsafe { flint_sys::fmpq_mat::fmpq_mat_is_empty(self.as_ptr()) != 0 }
    }

    /// Return true if the matrix is square.
    #[inline]
    pub fn is_square(&self) -> bool {
        unsafe {
            flint_sys::fmpq_mat::fmpq_mat_is_square(self.as_ptr()) != 0
        }
    }
    
    /// Return true if the matrix is invertible.
    #[inline]
    pub fn is_invertible(&self) -> bool {
        self.is_square() && !self.det().is_zero()
    }

    /// Return true if row `i` is all zeros.
    #[inline]
    pub fn is_zero_row(&self, i: usize) -> bool {
        self.row(i).is_zero()
    }

    /// Return true if column `i` is all zeros.
    #[inline]
    pub fn is_zero_col(&self, i: usize) -> bool {
        self.col(i).is_zero()
    }

    /// Return the number of rows of a rational matrix.
    #[inline]
    pub fn nrows(&self) -> c_long {
        unsafe {
            flint_sys::fmpq_mat::fmpq_mat_nrows(self.as_ptr())
        }
    }

    /// Return the number of columns of an rational matrix.
    #[inline]
    pub fn ncols(&self) -> c_long {
        unsafe {
            flint_sys::fmpq_mat::fmpq_mat_ncols(self.as_ptr())
        }
    }

    /// Return the transpose of an rational matrix.
    #[inline]
    pub fn transpose(&self) -> RatMat {
        let mut res = RatMat::zero(self.ncols(), self.nrows());
        unsafe {
            flint_sys::fmpq_mat::fmpq_mat_transpose(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }

    /// Compute the transpose of a square rational matrix in place.
    #[inline]
    pub fn transpose_assign(&mut self) {
        assert!(self.is_square());
        unsafe { flint_sys::fmpq_mat::fmpq_mat_transpose(self.as_mut_ptr(), self.as_ptr()); }
    }
    
    /// Get the `(i, j)`-th entry of an rational matrix.
    #[inline]
    pub fn get_entry(&self, i: usize, j: usize) -> Rational {
        let mut res = Rational::default();
        unsafe {
            let x = flint_sys::fmpq_mat::fmpq_mat_entry(self.as_ptr(), i as c_long, j as c_long);
            flint_sys::fmpq::fmpq_set(res.as_mut_ptr(), x);
        }
        res
    }

    /// Get the `(i, j)`-th entry of an rational matrix and assign it to `out`. Avoids extra
    /// allocation.
    #[inline]
    pub fn get_entry_assign(&self, out: &mut Rational, i: usize, j: usize) {
        unsafe {
            let x = flint_sys::fmpq_mat::fmpq_mat_entry(self.as_ptr(), i as c_long, j as c_long);
            flint_sys::fmpq::fmpq_set(out.as_mut_ptr(), x);
        }
    }
    
    /// Set the `(i, j)`-th entry of a rational matrix to the [Rational] `e`.
    #[inline]
    pub fn set_entry(&mut self, i: usize, j: usize, e: &Rational) {
        unsafe {
            let x = flint_sys::fmpq_mat::fmpq_mat_entry(self.as_ptr(), i as c_long, j as c_long);
            flint_sys::fmpq::fmpq_set(x, e.as_ptr());
        }
    }

    /// Return the matrix obtained by horizontally concatenating `self` with `other` in that order.
    /// The number of rows of both matrices must agree.
    #[inline]
    pub fn hcat(&self, other: &RatMat) -> RatMat {
        assert_eq!(self.nrows(), other.nrows());
        let mut res = RatMat::zero(self.nrows(), self.ncols() + other.ncols());
        unsafe {
            flint_sys::fmpq_mat::fmpq_mat_concat_horizontal(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            );
        }
        res
    }
   
    /// Return the matrix obtained by vertically concatenating `self` with `other` in that order.
    /// The number of columns of both matrices must agree. 
    #[inline]
    pub fn vcat(&self, other: &RatMat) -> RatMat {
        assert_eq!(self.ncols(), other.ncols());
        let mut res = RatMat::zero(self.nrows() + other.nrows(), self.ncols());
        unsafe {
            flint_sys::fmpq_mat::fmpq_mat_concat_vertical(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            );
        }
        res
    }
   
    /// Return a new matrix containing the `r2 - r1` by `c2 - c1` submatrix of a rational matrix whose
    /// `(0, 0)` entry is the `(r1, c1)` entry of the input.
    #[inline]
    pub fn submatrix(&self, r1: usize, c1: usize, r2: usize, c2: usize) -> RatMat {
        assert!((r2+r1) as c_long <= self.nrows());
        assert!((c2+c1) as c_long  <= self.ncols());

        let mut res = RatMat::zero((r2-r1) as c_long, (c2-c1) as c_long);
        let mut win = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpq_mat::fmpq_mat_window_init(
                win.as_mut_ptr(), 
                self.as_ptr(),
                r1 as c_long,
                c1 as c_long,
                r2 as c_long,
                c2 as c_long);
            flint_sys::fmpq_mat::fmpq_mat_set(res.as_mut_ptr(), win.as_ptr());
            flint_sys::fmpq_mat::fmpq_mat_window_clear(win.as_mut_ptr());
        }
        res
    }
    
    /// Return row `i` as a rational matrix.
    #[inline]
    pub fn row(&self, i: usize) -> RatMat {
        self.submatrix(i, 0, i + 1, self.ncols() as usize)
    }
   
    /// Return column `j` as a rational matrix.
    #[inline]
    pub fn col(&self, j: usize) -> RatMat {
        self.submatrix(0, j, self.nrows() as usize, j + 1)
    }

    /// Return the square of an rational matrix. The matrix must be square.
    #[inline]
    pub fn square(&self) -> Self {
        assert!(self.is_square());
        let mut res = RatMat::zero(self.nrows(), self.ncols());
        unsafe { flint_sys::fmpq_mat::fmpq_mat_mul(res.as_mut_ptr(), self.as_ptr(), self.as_ptr()) }
        res
    }
    
    /// Compute the square of a rational matrix in place.
    #[inline]
    pub fn square_assign(&mut self) {
        assert!(self.is_square());
        unsafe { flint_sys::fmpq_mat::fmpq_mat_mul(self.as_mut_ptr(), self.as_ptr(), self.as_ptr()) }
    }

    /// Return the kronecker product of two rational matrices.
    #[inline]
    pub fn kronecker_product(&self, other: &RatMat) -> RatMat {
        let mut res = RatMat::zero(self.nrows()*other.nrows(), self.ncols()*other.ncols());
        unsafe { 
            flint_sys::fmpq_mat::fmpq_mat_kronecker_product(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            ); 
        }
        res
    }
    
    /// Compute the trace of a square rational matrix.
    #[inline]
    pub fn trace(&self) -> Rational {
        assert!(self.is_square());
        let mut res = Rational::default();
        unsafe { flint_sys::fmpq_mat::fmpq_mat_trace(res.as_mut_ptr(), self.as_ptr()); }
        res
    }

    /*
    /// Return the content of a rational matrix, that is, the gcd of all its entries. Returns zero
    /// if the matrix is empty.
    #[inline]
    pub fn content(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz_mat::fmpz_mat_content(res.as_mut_ptr(), self.as_ptr()); }
        res
    }*/
    
    /// Compute the determinant of a square rational matrix.
    #[inline]
    pub fn det(&self) -> Rational {
        assert!(self.is_square());
        let mut res = Rational::default();
        unsafe { flint_sys::fmpq_mat::fmpq_mat_det(res.as_mut_ptr(), self.as_ptr()); }
        res
    }
    
    /*
    /// Return an absolute upper bound on the determinant of a square integer matrix computed from
    /// the Hadamard inequality.
    #[inline]
    pub fn det_bound(&self) -> Integer {
        assert!(self.is_square());
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz_mat::fmpz_mat_det_bound(res.as_mut_ptr(), self.as_ptr()); }
        res
    }
    
    /// Return a positive divisor of the determinant of a square integer matrix. If the determinant
    /// is zero this will always return zero.
    #[inline]
    pub fn det_divisor(&self) -> Integer {
        assert!(self.is_square());
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz_mat::fmpz_mat_det_divisor(res.as_mut_ptr(), self.as_ptr()); }
        res
    }
    
    /// Applies a similarity transform to an `n` by `n` integer matrix. If `P` is the identity
    /// matrix whose zero entries in row `r` have been replaced by `d`, this transform is
    /// equivalent to `P^-1 * M * P`. 
    #[inline]
    pub fn similarity(&self, r: c_long, d: &Integer) -> IntMat {
        assert!(self.is_square());
        let mut res = self.clone();
        unsafe { flint_sys::fmpz_mat::fmpz_mat_similarity(res.as_mut_ptr(), r, d.as_ptr()); }
        res
    }
    
    /// Applies a similarity transform to an `n` by `n` integer matrix in place.
    #[inline]
    pub fn similarity_assign(&mut self, r: c_long, d: &Integer) {
        assert!(self.is_square());
        unsafe { flint_sys::fmpz_mat::fmpz_mat_similarity(self.as_mut_ptr(), r, d.as_ptr()); }
    }
  
    /// Return the characteristic polynomial of a square integer matrix.
    #[inline]
    pub fn charpoly(&self) -> IntPol {
        assert!(self.is_square());
        let mut res = IntPol::default();
        unsafe { flint_sys::fmpz_mat::fmpz_mat_charpoly(res.as_mut_ptr(), self.as_ptr()); }
        res
    }
    
    /// Return the minimal polynomial of a square integer matrix.
    #[inline]
    pub fn minpoly(&self) -> IntPol {
        assert!(self.is_square());
        let mut res = IntPol::default();
        unsafe { flint_sys::fmpz_mat::fmpz_mat_minpoly(res.as_mut_ptr(), self.as_ptr()); }
        res
    }

    /// Return the rank of a matrix, that is, the number of linearly independent columns
    /// (equivalently, rows) of an integer matrix. The rank is computed by row reducing a copy of
    /// the input matrix.
    #[inline]
    pub fn rank(&self) -> c_long {
        unsafe { flint_sys::fmpz_mat::fmpz_mat_rank(self.as_ptr()) }
    }*/

    /* todo: RatMat
    pub fn solve<'a, T>(&self, B: &'a T) -> Option<RatMat<'a>> where &'a T: Into<IntMat<'a>> {
        let B = B.into();
        assert_eq!(self.nrows(), B.nrows());

        let mut res = RatMat::zero(self.ncols(), B.ncols());
        unsafe { 
            let x = flint_sys::fmpq_mat::fmpq_mat_solve_fmpz_mat(
                res.as_mut_ptr(), 
                self.as_ptr(),
                B.as_ptr()
            );
            if x == 0 {
                None
            } else {
                Some(res)
            }
        }
    }
    
    pub fn solve_fraction_free<'a, T>(&self, B: &'a T) -> Option<RatMat> where &'a T: Into<IntMat<'a>> {
        let B = B.into();
        assert_eq!(self.nrows(), B.nrows());

        let mut res = RatMat::zero(self.ncols(), B.ncols());
        unsafe { 
            let x = flint_sys::fmpq_mat::fmpq_mat_solve_fmpz_mat_fraction_free(
                res.as_mut_ptr(), 
                self.as_ptr(),
                B.as_ptr()
            );
            if x == 0 {
                None
            } else {
                Some(res)
            }
        }
    }
    
    pub fn solve_dixon<'a, T>(&self, B: &'a T) -> Option<RatMat> where &'a T: Into<IntMat<'a>> {
        let B = B.into();
        assert_eq!(self.nrows(), B.nrows());

        let mut res = RatMat::zero(self.ncols(), B.ncols());
        unsafe { 
            let x = flint_sys::fmpq_mat::fmpq_mat_solve_fmpz_mat_dixon(
                res.as_mut_ptr(), 
                self.as_ptr(),
                B.as_ptr()
            );
            if x == 0 {
                None
            } else {
                Some(res)
            }
        }
    }
    
    pub fn solve_multi_mod<'a, T>(&self, B: &'a T) -> Option<RatMat> where &'a T: Into<IntMat<'a>> {
        let B = B.into();
        assert_eq!(self.nrows(), B.nrows());

        let mut res = RatMat::zero(self.ncols(), B.ncols());
        unsafe { 
            let x = flint_sys::fmpq_mat::fmpq_mat_solve_fmpz_mat_multi_mod(
                res.as_mut_ptr(), 
                self.as_ptr(),
                B.as_ptr()
            );
            if x == 0 {
                None
            } else {
                Some(res)
            }
        }
    }
    
    pub fn solve_fflu<'a, T>(&self, B: &'a T) -> Option<RatMat> where &'a T: Into<IntMat<'a>> {
        let B = B.into();
        assert_eq!(self.nrows(), B.nrows());

        let mut res = IntMat<'a>::zero(self.ncols(), B.ncols());
        let mut den = Integer::default();
        unsafe { 
            let x = flint_sys::flint_sys::fmpz_mat::fmpz_mat_solve_fflu(
                res.as_mut_ptr(),
                den.as_mut_ptr(),
                self.as_ptr(),
                B.as_ptr()
            );
            if x == 0 {
                None
            } else {
                Some(res/den)
            }
        }
    }
    
    pub fn solve_cramer<'a, T>(&self, B: &'a T) -> Option<RatMat> where &'a T: Into<IntMat<'a>> {
        let B = B.into();
        assert_eq!(self.nrows(), B.nrows());

        let mut res = IntMat<'a>::zero(self.ncols(), B.ncols());
        let mut den = Integer::default();
        unsafe { 
            let x = flint_sys::flint_sys::fmpz_mat::fmpz_mat_solve_cramer(
                res.as_mut_ptr(), 
                den.as_mut_ptr(),
                self.as_ptr(),
                B.as_ptr()
            );
            if x == 0 {
                None
            } else {
                Some(res/den)
            }
        }
    }
    
    /* flint functions missing
    pub fn can_solve<'a, T>(&self, B: &'a T) -> Option<RatMat> where &'a T: Into<IntMat<'a>> {
        let B = B.into();
        assert_eq!(self.nrows(), B.nrows());
        
        let mut res = IntMat<'a>::zero(self.ncols(), 1);
        let mut den = Integer::default();
        unsafe { 
            let x = flint_sys::fmpz_mat::fmpz_mat_can_solve(
                res.as_mut_ptr(), 
                den.as_mut_ptr(),
                self.as_ptr(),
                B.as_ptr()
            );
            if x == 1 {
                Some(res/den)
            } else {
                None
            }
        }
    }
    
    pub fn can_solve_fflu<'a, T>(&self, B: &'a T) -> Option<RatMat> where &'a T: Into<IntMat<'a>> {
        let B = B.into();
        assert_eq!(self.nrows(), B.nrows());
        
        let mut res = IntMat<'a>::zero(self.ncols(), 1);
        let mut den = Integer::default();
        unsafe { 
            let x = flint_sys::fmpz_mat::fmpz_mat_can_solve_fflu(
                res.as_mut_ptr(), 
                den.as_mut_ptr(),
                self.as_ptr(),
                B.as_ptr()
            );
            if x == 1 {
                Some(res/den)
            } else {
                None
            }
        }
    }*/

    pub fn solve_bound(&self, B: &IntMat<'a>) -> (Integer, Integer) {
        let mut N = Integer::default();
        let mut D = Integer::default();
        
        unsafe {
            flint_sys::fmpz_mat::fmpz_mat_solve_bound(
                N.as_mut_ptr(), 
                D.as_mut_ptr(), 
                self.as_ptr(), 
                B.as_ptr()
            );
        }
        (N, D)
    }

    pub fn fflu(&self) -> (RatMat, c_long) {
        let mut B = IntMat<'a>::zero(self.nrows(), self.ncols());
        let mut den = Integer::default();

        unsafe {
            let rank = flint_sys::fmpz_mat::fmpz_mat_fflu(
                B.as_mut_ptr(), 
                den.as_mut_ptr(), 
                std::ptr::null(), 
                self.as_ptr(), 
                0
            );
            (B/den, rank)
        }
    }
    
    pub fn rref(&self) -> (RatMat, c_long) {
        let mut B = IntMat<'a>::zero(self.nrows(), self.ncols());
        let mut den = Integer::default();

        unsafe {
            let rank = flint_sys::fmpz_mat::fmpz_mat_rref(
                B.as_mut_ptr(), 
                den.as_mut_ptr(), 
                self.as_ptr()
            );
            (B/den, rank)
        }
    }
    
    pub fn rref_mod<'b, T>(&self, modulus: &'b T) -> (IntMat<'a>, c_long) where &'b T: Into<Integer> {
        let mut B = self.clone();

        unsafe {
            let rank = flint_sys::fmpz_mat::fmpz_mat_rref_mod(
                std::ptr::null_mut(),
                B.as_mut_ptr(),
                modulus.into().as_ptr()
            );
            (B, rank)
        }
    }

    pub fn gram_schmidt(&self) -> RatMat {
        RatMat::from(self).gram_schmidt()
    }

    pub fn strong_echelon_form_mod<'b, T>(&self, modulus: &'b T) -> IntMat<'a> where 
        &'b T: Into<Integer> 
    {
        let mut B = self.clone();

        unsafe {
            flint_sys::fmpz_mat::fmpz_mat_strong_echelon_form_mod(
                B.as_mut_ptr(),
                modulus.into().as_ptr()
            );
            B
        }
    }
    
    pub fn howell_form_mod<'b, T>(&self, modulus: &'b T) -> (IntMat<'a>, c_long) where 
        &'b T: Into<Integer> 
    {
        assert!(self.ncols() <= self.nrows());
        let mut B = self.clone();

        unsafe {
            // is the output the rank? docs unclear
            let rank = flint_sys::fmpz_mat::fmpz_mat_howell_form_mod(
                B.as_mut_ptr(),
                modulus.into().as_ptr()
            );
            (B, rank)
        }
    }
    
    pub fn nullspace(&self) -> IntMat<'a> {
        let mut B = IntMat<'a>::zero(self.ncols(), self.ncols());

        unsafe {
            let rank = flint_sys::fmpz_mat::fmpz_mat_nullspace(
                B.as_mut_ptr(),
                self.as_ptr()
            );
            B.submatrix(0, 0, B.nrows() as usize, rank as usize)
        }
    }
    
    pub fn hnf(&self) -> IntMat<'a> {
        let mut H = IntMat<'a>::zero(self.nrows(), self.ncols());
        unsafe { flint_sys::fmpz_mat::fmpz_mat_hnf(H.as_mut_ptr(), self.as_ptr()); }
        H
    }
    
    pub fn hnf_transform(&self) -> (IntMat<'a>, IntMat<'a>) {
        let mut H = IntMat<'a>::zero(self.nrows(), self.ncols());
        let mut U = IntMat<'a>::zero(self.nrows(), self.nrows());
        unsafe { 
            flint_sys::fmpz_mat::fmpz_mat_hnf_transform(H.as_mut_ptr(), U.as_mut_ptr(), self.as_ptr()); 
        }
        (H, U)
    }
    
    pub fn is_hnf(&self) -> bool {
        unsafe { flint_sys::fmpz_mat::fmpz_mat_is_in_hnf(self.as_ptr()) == 1 }
    }
    
    pub fn snf(&self) -> IntMat<'a> {
        let mut H = IntMat<'a>::zero(self.nrows(), self.ncols());
        unsafe { flint_sys::fmpz_mat::fmpz_mat_snf(H.as_mut_ptr(), self.as_ptr()); }
        H
    }
    
    pub fn is_snf(&self) -> bool {
        unsafe { flint_sys::fmpz_mat::fmpz_mat_is_in_snf(self.as_ptr()) == 1 }
    }
    
    /* TODO: per docs "dims must be compatible" is the output the same dim as self?
    pub fn gram(&self) -> IntMat<'a> {
        let mut B = IntMat<'a>::zero(self.nrows(), self.ncols());
        unsafe { flint_sys::fmpz_mat::fmpz_mat_gram(B.as_mut_ptr(), self.as_ptr()); }
        B
    }
    */

    pub fn is_hadamard(&self) -> bool {
        unsafe { flint_sys::fmpz_mat::fmpz_mat_is_hadamard(self.as_ptr()) != 0 }
    }

    pub fn hadamard(n: c_long) -> IntMat<'a> {
        let mut H = IntMat<'a>::zero(n, n);
        unsafe { flint_sys::fmpz_mat::fmpz_mat_hadamard(H.as_mut_ptr());}
        H
    }
   
    /* TODO: requires d_mat
    pub fn chol_d(&self) -> IntMat<'a> {
        assert!(self.is_symmetric());
        assert!(self.is_positive_definite());
        let mut R = IntMat<'a>::zero(?, ?);
        unsafe { flint_sys::fmpz_mat::fmpz_mat_chol_d(R.as_mut_ptr(), self.as_ptr());}
        R
    }
    */
   
    // TODO: default delta/eta? 
    pub fn lll<'b, T>(&self, delta: &'b T, eta: &'b T) -> IntMat<'a> where &'b T: Into<Rational> {
        let mut B = self.clone();
        unsafe { 
            flint_sys::fmpz_mat::fmpz_mat_lll_storjohann(
                B.as_mut_ptr(), 
                delta.into().as_ptr(), 
                eta.into().as_ptr()
            );
        }
        B
    }
    
    pub fn lll_original<'b, T>(&self, delta: &'b T, eta: &'b T) -> IntMat<'a> where &'b T: Into<Rational> {
        let mut B = self.clone();
        unsafe { 
            flint_sys::fmpz_mat::fmpz_mat_lll_original(
                B.as_mut_ptr(), 
                delta.into().as_ptr(), 
                eta.into().as_ptr()
            );
        }
        B
    }

    pub fn rational_reconstruction<'a, T>(&self, modulus: &'a T) -> RatMat where &'a T: Into<Integer> {
        let mut res = RatMat::from(self);
        unsafe {
            flint_sys::fmpq_mat::fmpq_mat_set_fmpz_mat_mod_fmpz(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                modulus.into().as_ptr()
            );
        }
        res
    }*/
}
