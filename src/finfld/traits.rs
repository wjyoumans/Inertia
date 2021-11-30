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
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::sync::Arc;

use flint_sys::fq_default::fq_default_struct as fq_struct;
use flint_sys::fq_default::fq_default_ctx_struct as fq_ctx_struct;
use libc::c_long;
use num_traits::PrimInt;

use crate::traits::*;
use crate::integer::src::Integer;
use crate::intpol::src::IntPol;
use crate::finfld::src::{FiniteField, FinFldElem};


/// A trait for implementing different initializations of a finite field.
pub trait FiniteFieldInit<T, U> {
    fn init(p: T, k: U) -> Self;
}

/// A trait for constructing elements of a finite field.
pub trait FiniteFieldNew<T> {
    fn new(&self, x: T) -> FinFldElem;
}

// FiniteField //

pub struct FqCtx(pub fq_ctx_struct);

impl Drop for FqCtx {
    fn drop(&mut self) {
        unsafe { flint_sys::fq_default::fq_default_ctx_clear(&mut self.0); }
    }
}

impl Parent for FiniteField {
    type Data = Arc<FqCtx>;
    type Element = FinFldElem;
}

impl FiniteFieldInit<&Integer, c_long> for FiniteField {
    /// Construct the finite field with `p^k` elements.
    #[inline]
    fn init(p: &Integer, k: c_long) -> Self {
        assert!(p.is_prime());
        assert!(k > 0);
    
        let var = CString::new("o").unwrap();
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fq_default::fq_default_ctx_init(z.as_mut_ptr(), p.as_ptr(), k, var.as_ptr());
            FiniteField { ctx: Arc::new(FqCtx(z.assume_init())) }
        }
    }
}

impl<T> FiniteFieldInit<T, c_long> for FiniteField where
    T: PrimInt + Into<Integer>
{
    /// Construct the finite field with `p^k` elements.
    #[inline]
    fn init(p: T, k: c_long) -> Self {
        Self::init(&p.into(), k)
    }
}

impl FiniteFieldNew<&Integer> for FiniteField {
    /// Construct an element of a finite field.
    #[inline]
    fn new(&self, n: &Integer) -> FinFldElem {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fq_default::fq_default_set_fmpz(
                z.as_mut_ptr(),
                n.as_ptr(),
                &self.ctx.0
            );
            FinFldElem { ctx: Arc::clone(&self.ctx), data: z.assume_init() }
        }
    }
}

impl<T> FiniteFieldNew<T> for FiniteField where
    T: PrimInt + Into<Integer>
{
    /// Construct an element of a finite field.
    #[inline]
    fn new(&self, n: T) -> FinFldElem {
        self.new(&n.into())
    }
}

// FinFldElem //

impl Element for FinFldElem {
    type Data = fq_struct;
    type Parent = FiniteField;
}

impl Clone for FinFldElem {
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe { 
            flint_sys::fq_default::fq_default_init(z.as_mut_ptr(), self.ctx_ptr());
            flint_sys::fq_default::fq_default_set(
                z.as_mut_ptr(), 
                self.as_ptr(),
                self.ctx_ptr()
            ); 
            FinFldElem { ctx: Arc::clone(&self.ctx), data: z.assume_init() }
        }
    }
}

impl fmt::Display for FinFldElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for FinFldElem {
    fn drop(&mut self) {
        unsafe { flint_sys::fq_default::fq_default_clear(self.as_mut_ptr(), self.ctx_ptr());}
    }
}

impl Hash for FinFldElem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        IntPol::from(self).hash(state);
    }
}
