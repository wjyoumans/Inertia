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
use std::sync::Arc;

use flint_sys::fmpz::fmpz;
use flint_sys::fmpz_mod_mat::fmpz_mod_mat_struct;
use libc::c_long;

use crate::traits::*;
use crate::integer::src::Integer;
use crate::intmod::src::{IntMod, IntModRing};
use crate::intpol::src::IntPol;


pub struct IntModMatCtx(fmpz);

impl Drop for IntModMatCtx {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpz::fmpz_clear(&mut self.0); }
    }
}

/// The vector space of matrices with entries integers mod `n`.
pub struct IntModMatSpace {
    rows: c_long,
    cols: c_long,
    ctx: <Self as Parent>::Data,
}

impl Parent for IntModMatSpace {
    type Data = Arc<IntModMatCtx>;
    type Element = IntModMat;
}

impl Additive for IntModMatSpace {
    #[inline]
    fn zero(&self) -> IntModMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_mod_mat::fmpz_mod_mat_init(
                z.as_mut_ptr(), 
                self.rows, 
                self.cols, 
                self.as_ptr()
            );
            flint_sys::fmpz_mod_mat::fmpz_mod_mat_zero(z.as_mut_ptr());
            IntModMat { ctx: Arc::clone(&self.ctx), data: z.assume_init() }
        }
    }
}

impl Multiplicative for IntModMatSpace {
    #[inline]
    fn one(&self) -> IntModMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_mod_mat::fmpz_mod_mat_init(
                z.as_mut_ptr(), 
                self.rows, 
                self.cols, 
                self.as_ptr()
            );
            flint_sys::fmpz_mod_mat::fmpz_mod_mat_one(z.as_mut_ptr());
            IntModMat { ctx: Arc::clone(&self.ctx), data: z.assume_init() }
        }
    }
}

impl AdditiveGroup for IntModMatSpace {}

impl Module for IntModMatSpace {}

impl VectorSpace for IntModMatSpace {}

impl MatrixSpace for IntModMatSpace {}

impl IntModMatSpace {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fmpz {
        &self.ctx.0
    }

    /// Construct the ring of polynomials with coefficients integers mod `n`.
    pub fn init(rows: c_long, cols: c_long, n: &Integer) -> Self {
        IntModMatSpace { rows: rows, cols: cols, ctx: Arc::new(IntModMatCtx(n.data)) }
    }

    /// Create a new polynomial over integers mod `n`.
    pub fn new<T: Into<IntModMat>>(&self, x: T) -> IntModMat {
        x.into()
    }

    pub fn modulus(&self) -> Integer {
        Integer { ctx: (), data: self.ctx.0 }
    }
}

/// An element of the ring of integers mod `n`.
pub type IntModMat = Elem<IntModMatSpace>;

impl Element for IntModMat {
    type Data = fmpz_mod_mat_struct;
    type Parent = IntModMatSpace;
}

impl AdditiveElement for IntModMat {
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe { flint_sys::fmpz_mod_mat::fmpz_mod_mat_is_zero(self.as_ptr()) == 1 }
    }
}

impl MultiplicativeElement for IntModMat {
    #[inline]
    fn is_one(&self) -> bool {
        let tmp = IntModMat::one(self.nrows(), self.ncols(), &self.modulus());
        self == tmp
    }
}

impl AdditiveGroupElement for IntModMat {}

impl ModuleElement for IntModMat {}

impl VectorSpaceElement for IntModMat {}

impl MatrixSpaceElement for IntModMat {}

impl IntModMat {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fmpz_mod_mat_struct {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpz_mod_mat_struct {
        &mut self.data
    }

    /// A reference to the struct holding context information. This is only needed to interface
    /// directly with FLINT via the FFI.
    pub fn ctx_as_ptr(&self) -> &fmpz {
        &self.ctx.0
    }
   
    /// Return the modulus `n` of a matrix with entries in integers mod `n`.
    pub fn modulus(&self) -> Integer {
        Integer { ctx: (), data: self.ctx.0 }
    }

    /// Return an `r` by `c` zero matrix with entries in integers mod `n`.
    #[inline]
    pub fn zero(r: c_long, c: c_long, n: &Integer) -> IntModMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_mod_mat::fmpz_mod_mat_init(z.as_mut_ptr(), r, c, n.as_ptr());
            IntModMat { ctx: Arc::new(IntModMatCtx(n.data)), data: z.assume_init() }
        }
    }

    /// Return an `r` by `c` identity matrix with entries in integers mod `n`, truncated if `m != n`.
    #[inline]
    pub fn one(r: c_long, c: c_long, n: &Integer) -> IntModMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_mod_mat::fmpz_mod_mat_init(z.as_mut_ptr(), r, c, n.as_ptr());
            flint_sys::fmpz_mod_mat::fmpz_mod_mat_one(z.as_mut_ptr());
            IntModMat { ctx: Arc::new(IntModMatCtx(n.data)), data: z.assume_init() }
        }
    }

    /// Return the number of rows of a matrix with entries in integers mod `n`.
    #[inline]
    pub fn nrows(&self) -> c_long {
        unsafe {
            flint_sys::fmpz_mod_mat::fmpz_mod_mat_nrows(self.as_ptr())
        }
    }

    /// Return the number of columns of a matrix with entries in integers mod `n`.
    #[inline]
    pub fn ncols(&self) -> c_long {
        unsafe {
            flint_sys::fmpz_mod_mat::fmpz_mod_mat_ncols(self.as_ptr())
        }
    }
}
