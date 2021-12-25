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

use crate::*;


#[derive(Debug)]
pub struct FmpzModCtx(pub fmpz_mod_ctx_struct);

impl Drop for FmpzModCtx {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpz_mod::fmpz_mod_ctx_clear(&mut self.0); }
    }
}

/// The ring of integers mod `n` for any integer `n`.
#[derive(Debug, Clone)]
pub struct IntModRing {
    pub ctx: Arc<FmpzModCtx>,
}

impl Parent for IntModRing {
    type Element = IntMod;

    #[inline]
    fn default(&self) -> IntMod {
        let tmp = Integer::default();
        IntMod { data: IntModData { ctx: Arc::clone(&self.ctx), elem: tmp.data.elem } }
    }
}

impl Additive for IntModRing {
    #[inline]
    fn zero(&self) -> IntMod {
        self.default()
    }
}

impl Multiplicative for IntModRing {
    #[inline]
    fn one(&self) -> IntMod {
        let z = Integer::from(1);
        IntMod { data: IntModData { ctx: Arc::clone(&self.ctx), elem: z.data.elem } }
    }
}

impl AdditiveGroup for IntModRing {}

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
    T: Into<Integer>
{
    /// Construct the ring of integers mod `n`.
    #[inline]
    fn init(n: T) -> Self {
        Self::init(&n.into())
    }
}

impl_new_unsafe! {
    IntModRing, u64 {u64 u32 u16 u8}
    flint_sys::fmpz::fmpz_set_ui
}

impl_new_unsafe! {
    IntModRing, i64 {i64 i32 i16 i8}
    flint_sys::fmpz::fmpz_set_si
}

impl_new_unsafe! {
    IntModRing, Integer
    flint_sys::fmpz::fmpz_set
}

impl_new_unsafe! {
    IntModRing, IntMod
    flint_sys::fmpz::fmpz_set
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

#[derive(Debug)]
pub struct IntModData {
    pub elem: fmpz,
    pub ctx: Arc<FmpzModCtx>,
}

impl Drop for IntModData {
    fn drop(&mut self) {
        unsafe { 
            flint_sys::fmpz::fmpz_clear(&mut self.elem);
        }
    }
}

impl Element for IntMod {
    type Data = IntModData;
    type Parent = IntModRing;

    #[inline]
    fn parent(&self) -> IntModRing {
        IntModRing { ctx: Arc::clone(&self.data.ctx) }
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

impl RingElement for IntMod {}

impl IntMod {
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

    /// A reference to the struct holding context information. This is only needed to interface
    /// directly with FLINT via the FFI.
    pub fn ctx_as_ptr(&self) -> &fmpz_mod_ctx_struct {
        &self.data.ctx.0
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
