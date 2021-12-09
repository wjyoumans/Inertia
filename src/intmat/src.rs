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


use std::convert::TryInto;
use std::mem::MaybeUninit;

use flint_sys::fmpz_mat::fmpz_mat_struct;
use libc::c_long;

use crate::*;


/// The vector space of `rows` by `cols` [Integer] matrices.
#[derive(Default, Debug, Hash, Clone, Copy)]
pub struct IntMatSpace {
    rows: c_long,
    cols: c_long,
}

impl Parent for IntMatSpace {
    type Data = ();
    type Extra = ();
    type Element = IntMat;

    #[inline]
    fn default(&self) -> IntMat {
        IntMat::zero(self.rows, self.cols)
    }
}

impl Additive for IntMatSpace {
    #[inline]
    fn zero(&self) -> IntMat {
        IntMat::zero(self.rows, self.cols)
    }
}

impl Multiplicative for IntMatSpace {
    #[inline]
    fn one(&self) -> IntMat {
        IntMat::one(self.rows, self.cols)
    }
}

impl AdditiveGroup for IntMatSpace {}

impl Module for IntMatSpace {}

impl VectorSpace for IntMatSpace {
    type BaseRing = IntegerRing;
    fn base_ring(&self) -> IntegerRing {
        IntegerRing {}
    }
}

impl MatrixSpace for IntMatSpace {}

impl<T> Init2<T, T> for IntMatSpace where 
    T: TryInto<c_long>,
{
    #[inline]
    fn init(r: T, c: T) -> Self {
        match r.try_into() {
            Ok(rr) =>
                match c.try_into() {
                    Ok(cc) => IntMatSpace { rows: rr, cols: cc},
                    Err(_) => panic!("Input cannot be converted into a signed long!"),
                },
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
}

impl New<&IntMat> for IntMatSpace {
    fn new(&self, x: &IntMat) -> IntMat {
        x.clone()
    }
}

impl<T> New<T> for IntMatSpace where 
    T: Into<IntMat>
{
    #[inline]
    fn new(&self, x: T) -> IntMat {
        x.into()
    }
}

/// A matrix of arbitrary precision [Integer]s. The field `data` is a FLINT
/// [fmpz_mat_struct][flint_sys::fmpz_mat::fmpz_mat_struct].
pub type IntMat = Elem<IntMatSpace>;

impl Element for IntMat {
    type Data = fmpz_mat_struct;
    type Parent = IntMatSpace;

    #[inline]
    fn parent(&self) -> IntMatSpace {
        IntMatSpace { rows: self.nrows(), cols: self.ncols() }
    }
}

impl AdditiveElement for IntMat {
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe { flint_sys::fmpz_mat::fmpz_mat_is_zero(self.as_ptr()) == 1 }
    }
}

impl MultiplicativeElement for IntMat {
    #[inline]
    fn is_one(&self) -> bool {
        unsafe { flint_sys::fmpz_mat::fmpz_mat_is_one(self.as_ptr()) == 1 }
    }
}

impl AdditiveGroupElement for IntMat {}

impl ModuleElement for IntMat {}

impl VectorSpaceElement for IntMat {
    type BaseRingElement = Integer;
}

impl MatrixSpaceElement for IntMat {
    /// Return the number of rows of an integer matrix.
    #[inline]
    fn nrows(&self) -> c_long {
        unsafe {
            flint_sys::fmpz_mat::fmpz_mat_nrows(self.as_ptr())
        }
    }

    /// Return the number of columns of an integer matrix.
    #[inline]
    fn ncols(&self) -> c_long {
        unsafe {
            flint_sys::fmpz_mat::fmpz_mat_ncols(self.as_ptr())
        }
    }
    
    /// Get the `(i, j)`-th entry of an integer matrix.
    #[inline]
    fn get_entry(&self, i: usize, j: usize) -> Integer {
        let mut res = Integer::default();
        unsafe {
            let x = flint_sys::fmpz_mat::fmpz_mat_entry(self.as_ptr(), i as c_long, j as c_long);
            flint_sys::fmpz::fmpz_set(res.as_mut_ptr(), x);
        }
        res
    }
    
    /// Set the `(i, j)`-th entry of an integer matrix to the [Integer] `e`.
    #[inline]
    fn set_entry(&mut self, i: usize, j: usize, e: &Integer) {
        unsafe {
            let x = flint_sys::fmpz_mat::fmpz_mat_entry(self.as_ptr(), i as c_long, j as c_long);
            flint_sys::fmpz::fmpz_set(x, e.as_ptr());
        }
    }
}

impl IntMat {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with FLINT
    /// via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fmpz_mat_struct {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpz_mat_struct {
        &mut self.data
    }

    /// Swap two integer matrices. The dimensions are allowed to be different.
    #[inline]
    pub fn swap(&mut self, other: &mut IntMat) {
        unsafe { 
            flint_sys::fmpz_mat::fmpz_mat_swap(self.as_mut_ptr(), other.as_mut_ptr()); 
        }
    }

    /// Swap the rows `r` and `s` of an integer matrix. 
    #[inline]
    pub fn swap_rows(&mut self, r: c_long, s: c_long) {
        assert!(r < self.nrows());
        assert!(s < self.nrows());

        unsafe { 
            flint_sys::fmpz_mat::fmpz_mat_swap_rows(
                self.as_mut_ptr(), 
                std::ptr::null(),
                r,
                s
            ); 
        }
    }
    
    /// Swap the columns `r` and `s` of an integer matrix. 
    #[inline]
    pub fn swap_cols(&mut self, r: c_long, s: c_long) {
        assert!(r < self.ncols());
        assert!(s < self.ncols());

        unsafe { 
            flint_sys::fmpz_mat::fmpz_mat_swap_rows(
                self.as_mut_ptr(), 
                std::ptr::null(),
                r,
                s
            ); 
        }
    }
    
    /// Swap row `i` and `r - i` of an integer matrix for `0 <= i < r/2` where `r` is the number
    /// of rows of the input matrix.
    #[inline]
    pub fn invert_rows(&mut self) {
        unsafe { 
            flint_sys::fmpz_mat::fmpz_mat_invert_rows(
                self.as_mut_ptr(), 
                std::ptr::null()
            ); 
        }
    }
    
    /// Swap columns `i` and `c - i` of an integer matrix for `0 <= i < c/2` where `c` is the number
    /// of columns of the input matrix.
    #[inline]
    pub fn invert_columns(&mut self) {
        unsafe { 
            flint_sys::fmpz_mat::fmpz_mat_invert_cols(
                self.as_mut_ptr(), 
                std::ptr::null()
            ); 
        }
    }
    
   
    /* TODO: function missing from bindings
    /// Swap two integer matrices by swapping the individual entries rather than swapping the
    /// contents of their structs.
    #[inline]
    pub fn swap_entrywise(&mut self, other: &mut IntMat) {
        unsafe { 
            flint_sys::fmpz_mat::fmpz_mat_swap_entrywise(self.as_mut_ptr(), other.as_mut_ptr()); 
        }
    }
    */

    /// Return an `m` by `n` integer zero matrix.
    #[inline]
    pub fn zero(m: c_long, n: c_long) -> IntMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_mat::fmpz_mat_init(z.as_mut_ptr(), m, n);
            IntMat { ctx: (), extra: (), data: z.assume_init() }
        }
    }

    /// Return an `m` by `n` integer identity matrix, truncated if `m != n`.
    #[inline]
    pub fn one(m: c_long, n: c_long) -> IntMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_mat::fmpz_mat_init(z.as_mut_ptr(), m, n);
            flint_sys::fmpz_mat::fmpz_mat_one(z.as_mut_ptr());
            IntMat { ctx: (), extra: (), data: z.assume_init() }
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
        unsafe {
            flint_sys::fmpz_mat::fmpz_mat_is_zero_row(self.as_ptr(), i as c_long) != 0
        }
    }

    /// Return true if column `i` is all zeros.
    // TODO: Does an additional allocation compared to `is_zero_row`.
    #[inline]
    pub fn is_zero_col(&self, i: usize) -> bool {
        self.col(i).is_zero()
    }


    /// Return the transpose of an integer matrix.
    #[inline]
    pub fn transpose(&self) -> IntMat {
        let mut res = IntMat::zero(self.ncols(), self.nrows());
        unsafe {
            flint_sys::fmpz_mat::fmpz_mat_transpose(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }

    /// Compute the transpose of a square integer matrix in place.
    #[inline]
    pub fn transpose_assign(&mut self) {
        assert!(self.is_square());
        unsafe { flint_sys::fmpz_mat::fmpz_mat_transpose(self.as_mut_ptr(), self.as_ptr()); }
    }
    

    /// Get the `(i, j)`-th entry of an integer matrix and assign it to `out`. Avoids extra
    /// allocation.
    #[inline]
    pub fn get_entry_assign(&self, out: &mut Integer, i: usize, j: usize) {
        unsafe {
            let x = flint_sys::fmpz_mat::fmpz_mat_entry(self.as_ptr(), i as c_long, j as c_long);
            flint_sys::fmpz::fmpz_set(out.as_mut_ptr(), x);
        }
    }

    /// Return the matrix obtained by horizontally concatenating `self` with `other` in that order.
    /// The number of rows of both matrices must agree.
    #[inline]
    pub fn hcat(&self, other: &IntMat) -> IntMat {
        assert_eq!(self.nrows(), other.nrows());
        let mut res = IntMat::zero(self.nrows(), self.ncols() + other.ncols());
        unsafe {
            flint_sys::fmpz_mat::fmpz_mat_concat_horizontal(
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
    pub fn vcat(&self, other: &IntMat) -> IntMat {
        assert_eq!(self.ncols(), other.ncols());
        let mut res = IntMat::zero(self.nrows() + other.nrows(), self.ncols());
        unsafe {
            flint_sys::fmpz_mat::fmpz_mat_concat_vertical(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            );
        }
        res
    }
   
    /// Return a new matrix containing the `r2 - r1` by `c2 - c1` submatrix of an integer matrix whose
    /// `(0, 0)` entry is the `(r1, c1)` entry of the input.
    #[inline]
    pub fn submatrix(&self, r1: usize, c1: usize, r2: usize, c2: usize) -> IntMat {
        assert!((r2+r1) as c_long <= self.nrows());
        assert!((c2+c1) as c_long  <= self.ncols());

        let mut res = IntMat::zero((r2-r1) as c_long, (c2-c1) as c_long);
        let mut win = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_mat::fmpz_mat_window_init(
                win.as_mut_ptr(), 
                self.as_ptr(),
                r1 as c_long,
                c1 as c_long,
                r2 as c_long,
                c2 as c_long);
            flint_sys::fmpz_mat::fmpz_mat_set(res.as_mut_ptr(), win.as_ptr());
            flint_sys::fmpz_mat::fmpz_mat_window_clear(win.as_mut_ptr());
        }
        res
    }
    
    /// Return row `i` as an integer matrix.
    #[inline]
    pub fn row(&self, i: usize) -> IntMat {
        self.submatrix(i, 0, i + 1, self.ncols() as usize)
    }
   
    /// Return column `j` as an integer matrix.
    #[inline]
    pub fn col(&self, j: usize) -> IntMat {
        self.submatrix(0, j, self.nrows() as usize, j + 1)
    }

    /// Return the square of an integer matrix. The matrix must be square.
    #[inline]
    pub fn square(&self) -> Self {
        assert!(self.is_square());
        let mut res = IntMat::zero(self.nrows(), self.ncols());
        unsafe { flint_sys::fmpz_mat::fmpz_mat_sqr(res.as_mut_ptr(), self.as_ptr()) }
        res
    }
    
    /// Compute the square of an integer matrix in place.
    #[inline]
    pub fn square_assign(&mut self) {
        assert!(self.is_square());
        unsafe { flint_sys::fmpz_mat::fmpz_mat_sqr(self.as_mut_ptr(), self.as_ptr()) }
    }

    /// Return the kronecker product of two integer matrices.
    #[inline]
    pub fn kronecker_product(&self, other: &IntMat) -> IntMat {
        let mut res = IntMat::zero(self.nrows()*other.nrows(), self.ncols()*other.ncols());
        unsafe { 
            flint_sys::fmpz_mat::fmpz_mat_kronecker_product(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            ); 
        }
        res
    }
    
    /// Compute the trace of a square integer matrix.
    #[inline]
    pub fn trace(&self) -> Integer {
        assert!(self.is_square());
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz_mat::fmpz_mat_trace(res.as_mut_ptr(), self.as_ptr()); }
        res
    }

    /// Return the content of an integer matrix, that is, the gcd of all its entries. Returns zero
    /// if the matrix is empty.
    #[inline]
    pub fn content(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz_mat::fmpz_mat_content(res.as_mut_ptr(), self.as_ptr()); }
        res
    }
    
    /// Compute the determinant of a square integer matrix.
    #[inline]
    pub fn det(&self) -> Integer {
        assert!(self.is_square());
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz_mat::fmpz_mat_det(res.as_mut_ptr(), self.as_ptr()); }
        res
    }
    
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
    }

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

/* TODO: RatMat 
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
}*/
