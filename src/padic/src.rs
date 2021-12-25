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

use flint_sys::padic::padic_ctx_struct;
use flint_sys::padic::padic_struct;
use libc::c_long;

use crate::*;


#[derive(Debug)]
pub struct PadicCtx(padic_ctx_struct);

impl Drop for PadicCtx {
    fn drop(&mut self) {
        unsafe { flint_sys::padic::padic_ctx_clear(&mut self.0); }
    }
}

/// The p-adic completion of the rational numbers.
pub struct PadicField {
    pub ctx: Arc<PadicCtx>,
}

impl Parent for PadicField {
    type Element = PadicElem;

    #[inline]
    fn default(&self) -> PadicElem {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::padic::padic_init(z.as_mut_ptr());
            PadicElem { 
                data: PadicData {
                    ctx: Arc::clone(&self.ctx), 
                    elem: z.assume_init() 
                }
            }
        }
    }
}

impl Additive for PadicField {
    #[inline]
    fn zero(&self) -> PadicElem {
        self.default()
    }
}

impl Multiplicative for PadicField {
    #[inline]
    fn one(&self) -> PadicElem {
        let mut res = self.default();
        unsafe { flint_sys::padic::padic_one(res.as_mut_ptr()); }
        res
    }
}

impl AdditiveGroup for PadicField {}

impl MultiplicativeGroup for PadicField {}

impl Ring for PadicField {}

impl Field for PadicField {
    type BaseField = PadicField;

    #[inline]
    fn base_field(&self) -> PadicField {
        PadicField { ctx: Arc::clone(&self.ctx) }
    }
}

impl<T> Init2<&Integer, T> for PadicField where
    T: TryInto<c_long>
{
    fn init(p: &Integer, k: T) -> Self {
        match k.try_into() {
            Ok(kk) => {
                let mut z = MaybeUninit::uninit();
                unsafe {
                    flint_sys::padic::padic_ctx_init(
                        z.as_mut_ptr(), 
                        p.as_ptr(), 
                        0, 
                        kk, 
                        PADIC_DEFAULT_PRINT_MODE
                    );
                    PadicField { ctx: Arc::new(PadicCtx(z.assume_init())) }
                }
            },
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
}

impl<T, U> Init2<T, U> for PadicField where
    T: Into<Integer>,
    U: TryInto<c_long>
{
    fn init(p: T, k: U) -> Self {
        Self::init(&p.into(), k)
    }
}

impl_new_unsafe! {
    ctx
    PadicField, u64 {u64 u32 u16 u8}
    flint_sys::padic::padic_set_ui
}

impl_new_unsafe! {
    ctx
    PadicField, i64 {i64 i32 i16 i8}
    flint_sys::padic::padic_set_si
}

impl_new_unsafe! {
    ctx
    PadicField, Integer
    flint_sys::padic::padic_set_fmpz
}

impl_new_unsafe! {
    ctx
    PadicField, IntMod
    flint_sys::padic::padic_set_fmpz
}

impl_new_unsafe! {
    ctx
    PadicField, Rational
    flint_sys::padic::padic_set_fmpq
}

impl PadicField {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &padic_ctx_struct {
        &self.ctx.0
    }
}

/// An element of a p-adic field.
pub type PadicElem = Elem<PadicField>;

#[derive(Debug)]
pub struct PadicData {
    pub elem: padic_struct,
    pub ctx: Arc<PadicCtx>,
}

impl Drop for PadicData {
    fn drop(&mut self) {
        unsafe { 
            flint_sys::padic::padic_clear(&mut self.elem);
        }
    }
}

impl Element for PadicElem {
    type Data = PadicData;
    type Parent = PadicField;

    #[inline]
    fn parent(&self) -> PadicField {
        PadicField { ctx: Arc::clone(&self.data.ctx) }
    }
}

impl AdditiveElement for PadicElem {
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe { flint_sys::padic::padic_is_zero(self.as_ptr()) == 1 }
    }
}

impl MultiplicativeElement for PadicElem {
    #[inline]
    fn is_one(&self) -> bool {
        unsafe { flint_sys::padic::padic_is_one(self.as_ptr()) == 1 }
    }
}

impl AdditiveGroupElement for PadicElem {}

impl MultiplicativeGroupElement for PadicElem {}

impl RingElement for PadicElem {}

impl FieldElement for PadicElem {}

impl PadicElem {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &padic_struct {
        &self.data.elem
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut padic_struct {
        &mut self.data.elem
    }

    /// A reference to the struct holding context information. This is only needed to interface
    /// directly with FLINT via the FFI.
    pub fn ctx_as_ptr(&self) -> &padic_ctx_struct {
        &self.data.ctx.0
    }

    /// Return a [String] representation of a padic number.
    #[inline]
    pub fn get_str(&self) -> String {
        unsafe {
            let s = flint_sys::padic::padic_get_str(
                std::ptr::null(), 
                self.as_ptr(), 
                self.ctx_as_ptr()
            );
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
}
