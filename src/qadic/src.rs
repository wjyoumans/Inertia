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

use flint_sys::qadic::qadic_ctx_struct;
use flint_sys::qadic::qadic_struct;
use libc::c_long;
use num_traits::PrimInt;

use crate::traits::*;
use crate::integer::src::Integer;


pub struct QadicCtx(pub qadic_ctx_struct);

impl Drop for QadicCtx {
    fn drop(&mut self) {
        unsafe { flint_sys::qadic::qadic_ctx_clear(&mut self.0); }
    }
}

/// An unramified extension of the p-adic numbers.
pub struct QadicField {
    pub ctx: <Self as Parent>::Data,
}

impl Parent for QadicField {
    type Data = Arc<QadicCtx>;
    type Extra = ();
    type Element = QadicElem;
}

impl Additive for QadicField {
    #[inline]
    fn zero(&self) -> QadicElem {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::qadic::qadic_init(z.as_mut_ptr());
            flint_sys::qadic::qadic_zero(z.as_mut_ptr());
            QadicElem { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
        }
    }
}

impl Multiplicative for QadicField {
    #[inline]
    fn one(&self) -> QadicElem {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::qadic::qadic_init(z.as_mut_ptr());
            flint_sys::qadic::qadic_one(z.as_mut_ptr());
            QadicElem { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
        }
    }
}

impl AdditiveGroup for QadicField {}

impl MultiplicativeGroup for QadicField {}

impl Ring for QadicField {}

impl Field for QadicField {}

/// An element of a q-adic field.
pub type QadicElem = Elem<QadicField>;

impl Element for QadicElem {
    type Data = qadic_struct;
    type Parent = QadicField;
}

impl AdditiveElement for QadicElem {
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe { flint_sys::qadic::qadic_is_zero(self.as_ptr()) == 1 }
    }
}

impl MultiplicativeElement for QadicElem {
    #[inline]
    fn is_one(&self) -> bool {
        unsafe { flint_sys::qadic::qadic_is_one(self.as_ptr()) == 1 }
    }
}

impl AdditiveGroupElement for QadicElem {}

impl MultiplicativeGroupElement for QadicElem {}

impl RingElement for QadicElem {}

impl FieldElement for QadicElem {}

impl QadicElem {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &qadic_struct {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut qadic_struct {
        &mut self.data
    }

    /// A reference to the struct holding context information. This is only needed to interface
    /// directly with FLINT via the FFI.
    pub fn ctx_as_ptr(&self) -> &qadic_ctx_struct {
        &self.ctx.0
    }
}
