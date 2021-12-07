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

use crate::*;


/// The vector space of matrices with entries in a finite field.
pub struct FinFldMatSpace {
    rows: c_long,
    cols: c_long,
    ctx: <Self as Parent>::Data,
}

impl Parent for FinFldMatSpace {
    type Data = Arc<FqCtx>;
    type Extra = ();
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
            FinFldMat { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
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
            FinFldMat { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
        }
    }
}

impl AdditiveGroup for FinFldMatSpace {}

impl Module for FinFldMatSpace {}

impl VectorSpace for FinFldMatSpace {
    type BaseRing = FiniteField;
    fn base_ring(&self) -> FiniteField {
        FiniteField { ctx: Arc::clone(&self.ctx) }
    }
}

impl MatrixSpace for FinFldMatSpace {}

impl<T> Init5<T, T, &Integer, T, &str> for FinFldMatSpace where
    T: TryInto<c_long>
{
    fn init(r: T, c: T, p: &Integer, k: T, var: &str) -> FinFldMatSpace {
        let ff = FiniteField::init(p, k, var);
        match r.try_into() {
            Ok(rr) => 
                match c.try_into() {
                    Ok(cc) => 
                        FinFldMatSpace { 
                            rows: rr, 
                            cols: cc, 
                            ctx: Arc::clone(&ff.ctx)
                        },
                    Err(_) => panic!("Input cannot be converted into a signed long!"),
                },
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
}

impl<T, U> Init5<T, T, U, T, &str> for FinFldMatSpace where
    T: TryInto<c_long>,
    U : Into<Integer>,
{
    #[inline]
    fn init(r: T, c: T, p: U, k: T, var: &str) -> FinFldMatSpace {
        Self::init(r, c, &p.into(), k, var)
    }
}

impl<'a, T> New<&'a [Vec<T>]> for FinFldMatSpace where
    &'a [Vec<T>]: Into<IntMat>,
{
    #[inline]
    fn new(&self, x: &'a [Vec<T>]) -> FinFldMat {
        self.new(x.into())

    }
}

impl New<&IntMat> for FinFldMatSpace {
    #[inline]
    fn new(&self, x: &IntMat) -> FinFldMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fq_default_mat::fq_default_mat_set_fmpz_mat(
                z.as_mut_ptr(),
                x.as_ptr(),
                self.as_ptr()
            );
            FinFldMat { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
        }
    }
}

impl New<IntMat> for FinFldMatSpace {
    #[inline]
    fn new(&self, x: IntMat) -> FinFldMat {
        self.new(&x)
    }
}

impl New<&IntModMat> for FinFldMatSpace {
    #[inline]
    fn new(&self, x: &IntModMat) -> FinFldMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fq_default_mat::fq_default_mat_set_fmpz_mod_mat(
                z.as_mut_ptr(),
                x.as_ptr(),
                self.as_ptr()
            );
            FinFldMat { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
        }
    }
}

impl New<IntModMat> for FinFldMatSpace {
    #[inline]
    fn new(&self, x: IntModMat) -> FinFldMat {
        self.new(&x)
    }
}

impl FinFldMatSpace {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fq_default_ctx_struct {
        &self.ctx.0
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

impl VectorSpaceElement for FinFldMat {
    type BaseRingElement = FinFldElem;
}

//impl MatrixSpaceElement for FinFldMat {}

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

    /*
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
    }*/

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

    /*
    /// Get the `(i, j)`-th entry of the matrix.
    #[inline]
    fn get_entry(&self, i: usize, j: usize) -> FinFldElem {
        let mut z = MaybeUninit::uninit();
        let mut ctx = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz::fmpz_init(z.as_mut_ptr());
            flint_sys::fmpz_mod_mat::fmpz_mod_mat_get_entry(
                z.as_mut_ptr(), 
                self.as_ptr(),
                i as c_long, 
                j as c_long
            );
            flint_sys::fmpz_mod::fmpz_mod_ctx_init(ctx.as_mut_ptr(), &self.ctx.0);
            IntMod { ctx: Arc::new(FmpzModCtx(ctx.assume_init())), extra: (), data: z.assume_init() } 
        }
    }*/

}
