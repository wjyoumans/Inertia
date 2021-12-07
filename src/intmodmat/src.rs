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
use std::sync::Arc;

use flint_sys::fmpz_mod::fmpz_mod_ctx_struct;
use flint_sys::fmpz_mod_mat::fmpz_mod_mat_struct;
use libc::c_long;
use num_traits::PrimInt;

use crate::*;


/// The vector space of matrices with entries integers mod `n`.
pub struct IntModMatSpace {
    rows: c_long,
    cols: c_long,
    ctx: <Self as Parent>::Data,
}

impl Parent for IntModMatSpace {
    type Data = Arc<FmpzModCtx>;
    type Extra = ();
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
                self.modulus().as_ptr()
            );
            flint_sys::fmpz_mod_mat::fmpz_mod_mat_zero(z.as_mut_ptr());
            IntModMat { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
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
                self.modulus().as_ptr()
            );
            flint_sys::fmpz_mod_mat::fmpz_mod_mat_one(z.as_mut_ptr());
            IntModMat { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
        }
    }
}

impl AdditiveGroup for IntModMatSpace {}

impl Module for IntModMatSpace {}

/*
impl VectorSpace for IntModMatSpace {
    type BaseRing = IntModRing;
    fn base_ring(&self) -> IntModRing {
        IntModRing { ctx: Arc}
    }
}

impl MatrixSpace for IntModMatSpace {}
*/

impl<T> Init3<T, T, &Integer> for IntModMatSpace where 
    T: TryInto<c_long>,
{
    #[inline]
    fn init(r: T, c: T, n: &Integer) -> Self {
        match r.try_into() {
            Ok(rr) =>
                match c.try_into() {
                    Ok(cc) => IntModMatSpace { 
                        rows: rr, 
                        cols: cc, 
                        ctx: Arc::clone(&IntModRing::init(n).ctx)
                    },
                    Err(_) => panic!("Input cannot be converted into a signed long!"),
                },
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
}

impl<T, U> Init3<T, T, U> for IntModMatSpace where 
    T: TryInto<c_long>,
    U: PrimInt + Into<Integer>,
{
    fn init(r: T, c: T, n: U) -> Self {
        Self::init(r, c, &n.into())
    }
}

impl New<&IntModMat> for IntModMatSpace {
    fn new(&self, x: &IntModMat) -> IntModMat {
        let mut res = x.clone();
        unsafe {
            flint_sys::fmpz_mod_mat::_fmpz_mod_mat_set_mod(
                res.as_mut_ptr(), 
                self.modulus().as_ptr()
            );
            IntModMat { ctx: Arc::clone(&self.ctx), extra: (), data: res.data }
        }
    }
}

impl New<IntModMat> for IntModMatSpace {
    #[inline]
    fn new(&self, x: IntModMat) -> IntModMat {
        self.new(&x)
    }
}

impl New<&IntMat> for IntModMatSpace {
    #[inline]
    fn new(&self, x: &IntMat) -> IntModMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_mod_mat::fmpz_mod_mat_init(
                z.as_mut_ptr(), 
                self.rows, 
                self.cols, 
                self.modulus().as_ptr()
            );
            let mut z = z.assume_init();
            z.mat[0] = x.data;
            IntModMat { ctx: Arc::clone(&self.ctx), extra: (), data: z }
        }
    }
}

impl New<IntMat> for IntModMatSpace {
    #[inline]
    fn new(&self, x: IntMat) -> IntModMat {
        self.new(&x)
    }
}

impl<'a, T> New<&'a [Vec<T>]> for IntModMatSpace where
    &'a [Vec<T>]: Into<IntMat>,
{
    #[inline]
    fn new(&self, x: &'a [Vec<T>]) -> IntModMat {
        self.new(&x.into())

    }
}

impl IntModMatSpace {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fmpz_mod_ctx_struct {
        &self.ctx.0
    }

    /// Return the modulus `n` of the integers mod `n`.
    pub fn modulus(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { 
            let n = flint_sys::fmpz_mod::fmpz_mod_ctx_modulus(self.as_ptr()); 
            flint_sys::fmpz::fmpz_set(res.as_mut_ptr(), n);
        }
        res
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

impl VectorSpaceElement for IntModMat {
    type BaseRingElement = IntMod;
}

impl MatrixSpaceElement for IntModMat {
    /// Return the number of rows of a matrix with entries in integers mod `n`.
    #[inline]
    fn nrows(&self) -> c_long {
        unsafe {
            flint_sys::fmpz_mod_mat::fmpz_mod_mat_nrows(self.as_ptr())
        }
    }

    /// Return the number of columns of a matrix with entries in integers mod `n`.
    #[inline]
    fn ncols(&self) -> c_long {
        unsafe {
            flint_sys::fmpz_mod_mat::fmpz_mod_mat_ncols(self.as_ptr())
        }
    }

    /// Get the `(i, j)`-th entry of an integer matrix.
    #[inline]
    fn get_entry(&self, i: usize, j: usize) -> IntMod {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz::fmpz_init(z.as_mut_ptr());
            flint_sys::fmpz_mod_mat::fmpz_mod_mat_get_entry(
                z.as_mut_ptr(), 
                self.as_ptr(),
                i as c_long, 
                j as c_long
            );
            IntMod { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() } 
        }
    }

}

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
    pub fn ctx_as_ptr(&self) -> &fmpz_mod_ctx_struct {
        &self.ctx.0
    }
   
    /// Return the modulus `n` of a matrix with entries in integers mod `n`.
    pub fn modulus(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { 
            let n = flint_sys::fmpz_mod::fmpz_mod_ctx_modulus(self.ctx_as_ptr()); 
            flint_sys::fmpz::fmpz_set(res.as_mut_ptr(), n);
        }
        res
    }

    /// Return an `r` by `c` zero matrix with entries in integers mod `n`.
    #[inline]
    pub fn zero(r: c_long, c: c_long, n: &Integer) -> IntModMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_mod_mat::fmpz_mod_mat_init(z.as_mut_ptr(), r, c, n.as_ptr());
            IntModMat { 
                ctx: Arc::clone(&IntModRing::init(n).ctx), 
                extra: (), 
                data: z.assume_init() 
            }
        }
    }

    /// Return an `r` by `c` identity matrix with entries in integers mod `n`, truncated if `m != n`.
    #[inline]
    pub fn one(r: c_long, c: c_long, n: &Integer) -> IntModMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_mod_mat::fmpz_mod_mat_init(z.as_mut_ptr(), r, c, n.as_ptr());
            flint_sys::fmpz_mod_mat::fmpz_mod_mat_one(z.as_mut_ptr());
            IntModMat { 
                ctx: Arc::clone(&IntModRing::init(n).ctx), 
                extra: (), 
                data: z.assume_init() 
            }
        }
    }
}
