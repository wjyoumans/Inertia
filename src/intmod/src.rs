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
use flint_sys::fmpz_mod::fmpz_mod_ctx_struct;
use num_traits::PrimInt;

use crate::*;


#[derive(Debug)]
pub struct FmpzModCtx(pub fmpz_mod_ctx_struct);

impl Drop for FmpzModCtx {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpz_mod::fmpz_mod_ctx_clear(&mut self.0); }
    }
}

/// The ring of integers mod `n` for any integer `n`.
pub struct IntModRing {
    pub ctx: <Self as Parent>::Data,
}

impl Parent for IntModRing {
    type Data = Arc<FmpzModCtx>;
    type Extra = ();
    type Element = IntMod;
}

impl Additive for IntModRing {
    #[inline]
    fn zero(&self) -> IntMod {
        let z = Integer::default();
        IntMod { ctx: Arc::clone(&self.ctx), extra: (), data: z.data }
    }
}

impl Multiplicative for IntModRing {
    #[inline]
    fn one(&self) -> IntMod {
        let z = Integer::from(1);
        IntMod { ctx: Arc::clone(&self.ctx), extra: (), data: z.data }
    }
}

impl AdditiveGroup for IntModRing {}

impl MultiplicativeGroup for IntModRing {}

impl Ring for IntModRing {}

impl Init1<&Integer> for IntModRing {
    /// Construct the ring of integers mod `n`.
    #[inline]
    fn init(n: &Integer) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_mod::fmpz_mod_ctx_init(z.as_mut_ptr(), n.as_ptr());
            IntModRing { ctx: Arc::new(FmpzModCtx(z.assume_init())) }
        }
    }
}

impl<T> Init1<T> for IntModRing where
    T: PrimInt + Into<Integer>
{
    /// Construct the ring of integers mod `n`.
    #[inline]
    fn init(n: T) -> Self {
        Self::init(&n.into())
    }
}

impl New<&IntMod> for IntModRing {
    /// Construct an element of the ring of integers mod `n`.
    #[inline]
    fn new(&self, n: &IntMod) -> IntMod {
        IntMod { ctx: Arc::clone(&self.ctx), extra: (), data: n.data }
    }
}

impl New<&Integer> for IntModRing {
    /// Construct an element of the ring of integers mod `n`.
    #[inline]
    fn new(&self, n: &Integer) -> IntMod {
        IntMod { ctx: Arc::clone(&self.ctx), extra: (), data: n.data }
    }
}

impl<T> New<T> for IntModRing where
    T: PrimInt + Into<Integer>
{
    /// Construct an element of the ring of integers mod `n`.
    #[inline]
    fn new(&self, n: T) -> IntMod {
        self.new(&n.into())
    }
}

impl IntModRing {
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
pub type IntMod = Elem<IntModRing>;

impl Element for IntMod {
    type Data = fmpz;
    type Parent = IntModRing;

    #[inline]
    fn parent(&self) -> IntModRing {
        IntModRing { ctx: Arc::clone(&self.ctx) }
    }
}

impl AdditiveElement for IntMod {
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe { flint_sys::fmpz::fmpz_is_zero(self.as_ptr()) == 1 }
    }
}

impl MultiplicativeElement for IntMod {
    #[inline]
    fn is_one(&self) -> bool {
        unsafe { flint_sys::fmpz::fmpz_is_one(self.as_ptr()) == 1 }
    }
}

impl AdditiveGroupElement for IntMod {}

impl MultiplicativeGroupElement for IntMod {}

impl RingElement for IntMod {}

impl IntMod {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fmpz {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpz {
        &mut self.data
    }

    /// A reference to the struct holding context information. This is only needed to interface
    /// directly with FLINT via the FFI.
    pub fn ctx_as_ptr(&self) -> &fmpz_mod_ctx_struct {
        &self.ctx.0
    }
    
    /// Return the modulus `n` of the integers mod `n`.
    pub fn modulus(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { 
            let n = flint_sys::fmpz_mod::fmpz_mod_ctx_modulus(self.ctx_as_ptr()); 
            flint_sys::fmpz::fmpz_set(res.as_mut_ptr(), n);
        }
        res
    }
}
