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


use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::sync::Arc;

use flint_sys::fmpz::fmpz;
use flint_sys::fmpz_mod::fmpz_mod_ctx_struct;
use num_traits::PrimInt;

use crate::traits::*;
use crate::integer::src::Integer;
use crate::intmod::src::{IntMod, IntModRing};


/// A trait for implementing different initializations of the ring of integers mod `n`.
pub trait IntModRingInit<T> {
    fn init(x: T) -> Self;
}

/// A trait for constructing elements of the ring of integers mod `n`.
pub trait IntModRingNew<T> {
    fn new(&self, x: T) -> IntMod;
}

// IntModRing //

pub struct IntModCtx(pub fmpz_mod_ctx_struct);

impl Drop for IntModCtx {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpz_mod::fmpz_mod_ctx_clear(&mut self.0); }
    }
}

impl Parent for IntModRing {
    type Data = Arc<IntModCtx>;
    type Element = IntMod;
}

impl IntModRingInit<&Integer> for IntModRing {
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

impl<T> IntModRingInit<T> for IntModRing where
    T: PrimInt + Into<Integer>
{
    /// Construct the ring of integers mod `n`.
    #[inline]
    fn init(n: T) -> Self {
        Self::init(&n.into())
    }
}

impl IntModRingNew<&Integer> for IntModRing {
    /// Construct an element of the ring of integers mod `n`.
    #[inline]
    fn new(&self, n: &Integer) -> IntMod {
        IntMod { ctx: Arc::clone(&self.ctx), data: n.data }
    }
}

impl<T> IntModRingNew<T> for IntModRing where
    T: PrimInt + Into<Integer>
{
    /// Construct an element of the ring of integers mod `n`.
    #[inline]
    fn new(&self, n: T) -> IntMod {
        self.new(&n.into())
    }
}

// IntMod //

impl Element for IntMod {
    type Data = fmpz;
    type Parent = IntModRing;
}

impl Clone for IntMod {
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe { 
            flint_sys::fmpz::fmpz_init(z.as_mut_ptr());
            flint_sys::fmpz_mod::fmpz_mod_set_fmpz(
                z.as_mut_ptr(), 
                self.as_ptr(), 
                self.ctx_ptr()
            ); 
            IntMod { ctx: Arc::clone(&self.ctx), data: z.assume_init() }
        }
    }
}

impl fmt::Display for IntMod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for IntMod {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpz::fmpz_clear(self.as_mut_ptr());}
    }
}

impl Hash for IntMod {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Integer::from(self).hash(state);
    }
}
