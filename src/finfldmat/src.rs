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


use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::sync::Arc;

use flint_sys::fq_default::fq_default_ctx_struct;
use flint_sys::fq_default_mat::fq_default_mat_struct;
use libc::c_long;

use crate::traits::*;
use crate::integer::src::Integer;
use crate::finfld::src::{FqCtx, FinFldElem, FiniteField};


/// The vector space of matrices with entries in a finite field.
pub struct FinFldMatSpace {
    rows: c_long,
    cols: c_long,
    ctx: <Self as Parent>::Data,
}

impl Parent for FinFldMatSpace {
    type Data = Arc<FqCtx>;
    type Element = FinFldMat;
}

impl Additive for FinFldMatSpace {
    #[inline]
    fn zero(&self) -> FinFldMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fq_default_mat::fq_default_mat_init(
                z.as_mut_ptr(), 
                self.rows, 
                self.cols, 
                self.as_ptr()
            );
            flint_sys::fq_default_mat::fq_default_mat_zero(z.as_mut_ptr(), self.as_ptr());
            FinFldMat { ctx: Arc::clone(&self.ctx), data: z.assume_init() }
        }
    }
}

impl Multiplicative for FinFldMatSpace {
    #[inline]
    fn one(&self) -> FinFldMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fq_default_mat::fq_default_mat_init(
                z.as_mut_ptr(), 
                self.rows, 
                self.cols, 
                self.as_ptr()
            );
            flint_sys::fq_default_mat::fq_default_mat_one(z.as_mut_ptr(), self.as_ptr());
            FinFldMat { ctx: Arc::clone(&self.ctx), data: z.assume_init() }
        }
    }
}

impl AdditiveGroup for FinFldMatSpace {}

impl Module for FinFldMatSpace {}

impl VectorSpace for FinFldMatSpace {}

impl MatrixSpace for FinFldMatSpace {}

impl FinFldMatSpace {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fq_default_ctx_struct {
        &self.ctx.0
    }

    /// Construct the ring of polynomials with coefficients integers mod `n`.
    pub fn init(rows: c_long, cols: c_long, p: &Integer, k: c_long) -> Self {
        assert!(p.is_prime());
        assert!(k > 0);
    
        let var = CString::new("o").unwrap();
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fq_default::fq_default_ctx_init(z.as_mut_ptr(), p.as_ptr(), k, var.as_ptr());
            FinFldMatSpace { rows: rows, cols: cols, ctx: Arc::new(FqCtx(z.assume_init())) }
        }
    }

    /// Create a new polynomial over integers mod `n`.
    #[inline]
    pub fn new<T: Into<FinFldMat>>(&self, x: T) -> FinFldMat {
        x.into()
    }

    /*
    pub fn modulus(&self) -> Integer {
        Integer { ctx: (), data: self.ctx.0 }
    }*/
}

/// An element of the ring of integers mod `n`.
pub type FinFldMat = Elem<FinFldMatSpace>;

impl Element for FinFldMat {
    type Data = fq_default_mat_struct;
    type Parent = FinFldMatSpace;
}

impl AdditiveElement for FinFldMat {
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe { 
            flint_sys::fq_default_mat::fq_default_mat_is_zero(self.as_ptr(), self.ctx_as_ptr()) == 1 
        }
    }
}

impl MultiplicativeElement for FinFldMat {
    #[inline]
    fn is_one(&self) -> bool {
        unsafe { 
            flint_sys::fq_default_mat::fq_default_mat_is_one(self.as_ptr(), self.ctx_as_ptr()) == 1 
        }
    }
}

impl AdditiveGroupElement for FinFldMat {}

impl ModuleElement for FinFldMat {}

impl VectorSpaceElement for FinFldMat {}

impl MatrixSpaceElement for FinFldMat {}

impl FinFldMat {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fq_default_mat_struct {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fq_default_mat_struct {
        &mut self.data
    }

    /// A reference to the struct holding context information. This is only needed to interface
    /// directly with FLINT via the FFI.
    pub fn ctx_as_ptr(&self) -> &fq_default_ctx_struct {
        &self.ctx.0
    }
   
    /*
    /// Return the modulus `n` of a matrix with entries in integers mod `n`.
    pub fn modulus(&self) -> Integer {
        Integer { ctx: (), data: self.ctx.0 }
    }*/

    /// Return an `r` by `c` zero matrix with entries in the finite field with `p^k` elements.
    #[inline]
    pub fn zero(r: c_long, c: c_long, p: &Integer, k: c_long) -> FinFldMat {
        let ff = FinFldMatSpace::init(r, c, p, k);
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fq_default_mat::fq_default_mat_init(z.as_mut_ptr(), r, c, ff.as_ptr());
            FinFldMat { ctx: Arc::clone(&ff.ctx), data: z.assume_init() }
        }
    }

    /// Return an `r` by `c` identity matrix with entries in the finite field with `p^k` elements,
    /// truncated if `m != n`.
    #[inline]
    pub fn one(r: c_long, c: c_long, p: &Integer, k: c_long) -> FinFldMat {
        let ff = FinFldMatSpace::init(r, c, p, k);
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fq_default_mat::fq_default_mat_init(z.as_mut_ptr(), r, c, ff.as_ptr());
            flint_sys::fq_default_mat::fq_default_mat_one(z.as_mut_ptr(), ff.as_ptr());
            FinFldMat { ctx: Arc::clone(&ff.ctx), data: z.assume_init() }
        }
    }

    /// Return the number of rows of a matrix with entries in a finite field.
    #[inline]
    pub fn nrows(&self) -> c_long {
        unsafe {
            flint_sys::fq_default_mat::fq_default_mat_nrows(self.as_ptr(), self.ctx_as_ptr())
        }
    }

    /// Return the number of columns of a matrix with entries in a finite field.
    #[inline]
    pub fn ncols(&self) -> c_long {
        unsafe {
            flint_sys::fq_default_mat::fq_default_mat_ncols(self.as_ptr(), self.ctx_as_ptr())
        }
    }
}
