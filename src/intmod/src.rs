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

use crate::traits::*;
use crate::integer::src::Integer;
use crate::intmod::traits::IntModCtx;

/// The ring of integers mod `n` for any integer `n`.
pub struct IntModRing {
    pub ctx: <Self as Parent>::Data,
}

impl ParentInit1<&Integer> for IntModRing {
    /// Construct the ring of integers mod `n`.
    #[inline]
    fn init(n: &Integer) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_mod::fmpz_mod_ctx_init(z.as_mut_ptr(), n.as_ptr());
            IntModRing { ctx: Arc::new(IntModCtx(z.assume_init())) }
        }
    }
}

impl<T> ParentInit1<T> for IntModRing where
    T: PrimInt + Into<Integer>
{
    /// Construct the ring of integers mod `n`.
    #[inline]
    fn init(n: T) -> Self {
        Self::init(&n.into())
    }
}

impl ParentNew<&Integer> for IntModRing {
    /// Construct an element of the ring of integers mod `n`.
    #[inline]
    fn new(&self, n: &Integer) -> IntMod {
        IntMod { ctx: Arc::clone(&self.ctx), data: n.data }
    }
}

impl<T> ParentNew<T> for IntModRing where
    T: PrimInt + Into<Integer>
{
    /// Construct an element of the ring of integers mod `n`.
    #[inline]
    fn new(&self, n: T) -> IntMod {
        self.new(&n.into())
    }
}
/// An element of the ring of integers mod `n`.
pub type IntMod = Elem<IntModRing>;

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
    pub fn ctx_ptr(&self) -> &fmpz_mod_ctx_struct {
        &self.ctx.0
    }
}
