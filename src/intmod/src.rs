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

use crate::traits::*;
use crate::integer::src::Integer;

/// The ring of integers mod `n` for any integer `n`.
pub struct IntModRing {
    pub ctx: Arc<Wrap<fmpz_mod_ctx_struct>>,
}

impl IntModRing {
    /// Construct the ring of integers mod `n`.
    pub fn init(n: &Integer) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_mod::fmpz_mod_ctx_init(z.as_mut_ptr(), n.as_ptr());
            IntModRing { ctx: Arc::new(Wrap { wrap: z.assume_init() }) }
        }
    }

    /// Create a new integer mod `n`.
    pub fn new<T: Into<IntMod>>(&self, x: T) -> IntMod {
        x.into()
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
        &self.ctx.wrap
    }
}
